mod error;
mod ffi;
mod path;
mod strings;

pub use error::DmBridgeError;
pub use path::resolve_bridge_path;

use ffi::{DM_BRIDGE_ABI_VERSION, DM_BRIDGE_OK, RawDmBridge};
use libloading::Library;
use path::validate_process_architecture;
use std::path::{Path, PathBuf};
use strings::{call_wide_output, to_wide_nul};

pub struct DmBridge {
    path: PathBuf,
    _library: Library,
    raw: RawDmBridge,
}

impl DmBridge {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, DmBridgeError> {
        let path = resolve_bridge_path(path.as_ref());
        validate_process_architecture(&path)?;

        let library = unsafe { Library::new(&path) }
            .map_err(|error| DmBridgeError::load_failed(&path, error.to_string()))?;
        let raw = unsafe { RawDmBridge::load(&library)? };
        let bridge = Self {
            path,
            _library: library,
            raw,
        };

        let actual = bridge.abi_version();
        if actual != DM_BRIDGE_ABI_VERSION {
            return Err(DmBridgeError::InvalidAbi {
                expected: DM_BRIDGE_ABI_VERSION,
                actual,
            });
        }

        Ok(bridge)
    }

    pub fn abi_version(&self) -> i32 {
        unsafe { (self.raw.abi_version)() }
    }

    pub fn init(&self, dm_root: &str) -> Result<(), DmBridgeError> {
        let dm_root = to_wide_nul(dm_root);
        let status = unsafe { (self.raw.init)(dm_root.as_ptr()) };
        self.ensure_ok(status, "dm_bridge_init")
    }

    pub fn shutdown(&self) -> Result<(), DmBridgeError> {
        let status = unsafe { (self.raw.shutdown)() };
        self.ensure_ok(status, "dm_bridge_shutdown")
    }

    pub fn ver(&self) -> Result<String, DmBridgeError> {
        let (status, value) =
            call_wide_output(|buf, cap, len| unsafe { (self.raw.ver)(buf, cap, len) });
        self.ensure_ok(status, "dm_bridge_ver")?;
        Ok(value)
    }

    pub fn set_path(&self, path: &str) -> Result<(), DmBridgeError> {
        let path = to_wide_nul(path);
        let status = unsafe { (self.raw.set_path)(path.as_ptr()) };
        self.ensure_ok(status, "dm_bridge_set_path")
    }

    pub fn find_window(&self, class_name: &str, title_name: &str) -> Result<i32, DmBridgeError> {
        let class_name = to_wide_nul(class_name);
        let title_name = to_wide_nul(title_name);
        let mut hwnd = 0i32;
        let status =
            unsafe { (self.raw.find_window)(class_name.as_ptr(), title_name.as_ptr(), &mut hwnd) };
        self.ensure_ok(status, "dm_bridge_find_window")?;
        Ok(hwnd)
    }

    pub fn bind_window(
        &self,
        hwnd: i32,
        display: &str,
        mouse: &str,
        keypad: &str,
        mode: i32,
    ) -> Result<i32, DmBridgeError> {
        let display = to_wide_nul(display);
        let mouse = to_wide_nul(mouse);
        let keypad = to_wide_nul(keypad);
        let mut dm_ret = 0i32;
        let status = unsafe {
            (self.raw.bind_window)(
                hwnd,
                display.as_ptr(),
                mouse.as_ptr(),
                keypad.as_ptr(),
                mode,
                &mut dm_ret,
            )
        };
        self.ensure_ok(status, "dm_bridge_bind_window")?;
        Ok(dm_ret)
    }

    pub fn get_color(&self, x: i32, y: i32) -> Result<String, DmBridgeError> {
        let (status, value) =
            call_wide_output(|buf, cap, len| unsafe { (self.raw.get_color)(x, y, buf, cap, len) });
        self.ensure_ok(status, "dm_bridge_get_color")?;
        Ok(value)
    }

    pub fn move_to(&self, x: i32, y: i32) -> Result<i32, DmBridgeError> {
        let mut dm_ret = 0i32;
        let status = unsafe { (self.raw.move_to)(x, y, &mut dm_ret) };
        self.ensure_ok(status, "dm_bridge_move_to")?;
        Ok(dm_ret)
    }

    pub fn left_click(&self) -> Result<i32, DmBridgeError> {
        let mut dm_ret = 0i32;
        let status = unsafe { (self.raw.left_click)(&mut dm_ret) };
        self.ensure_ok(status, "dm_bridge_left_click")?;
        Ok(dm_ret)
    }

    pub fn unbind_window(&self) -> Result<i32, DmBridgeError> {
        let mut dm_ret = 0i32;
        let status = unsafe { (self.raw.unbind_window)(&mut dm_ret) };
        self.ensure_ok(status, "dm_bridge_unbind_window")?;
        Ok(dm_ret)
    }

    pub fn last_dm_error(&self) -> Result<i32, DmBridgeError> {
        let mut error_code = 0i32;
        let status = unsafe { (self.raw.get_last_dm_error)(&mut error_code) };
        self.ensure_ok(status, "dm_bridge_get_last_dm_error")?;
        Ok(error_code)
    }

    pub fn last_bridge_error(&self) -> i32 {
        unsafe { (self.raw.get_last_bridge_error)() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn ensure_ok(&self, status: i32, context: &'static str) -> Result<(), DmBridgeError> {
        if status == DM_BRIDGE_OK {
            return Ok(());
        }

        Err(DmBridgeError::BridgeFailed {
            context,
            status,
            message: self.last_bridge_message(),
        })
    }

    fn last_bridge_message(&self) -> String {
        let (_, message) =
            call_wide_output(|buf, cap, len| unsafe { (self.raw.get_last_message)(buf, cap, len) });
        message
    }
}

impl Drop for DmBridge {
    fn drop(&mut self) {
        let _ = unsafe { (self.raw.shutdown)() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_bridge_path_is_resolved_from_module_root() {
        let path = resolve_bridge_path(Path::new("../../target/dm-bridge/Win32/DmBridge.dll"));

        assert!(path.ends_with("target/dm-bridge/Win32/DmBridge.dll"));
        assert!(path.is_absolute());
    }

    #[test]
    fn dm_bridge_loads_abi_version_from_env_when_available() {
        let Some(path) = std::env::var_os("DM_BRIDGE_DLL") else {
            return;
        };

        let bridge = DmBridge::load(PathBuf::from(path)).expect("DmBridge must load");

        assert_eq!(bridge.abi_version(), DM_BRIDGE_ABI_VERSION);
    }

    #[test]
    fn dm_bridge_com_ver_and_color_smoke_when_enabled() {
        if std::env::var("DM_BRIDGE_COM_SMOKE").ok().as_deref() != Some("1") {
            return;
        }

        let path = std::env::var_os("DM_BRIDGE_DLL").expect("DM_BRIDGE_DLL must be set");
        let dm_root = std::env::var("DM_ROOT").unwrap_or_default();
        let bridge = DmBridge::load(PathBuf::from(path)).expect("DmBridge must load");

        bridge.init(&dm_root).expect("DmBridge init must work");
        let version = bridge.ver().expect("dm Ver must work");
        let color = bridge.get_color(0, 0).expect("dm GetColor must work");
        let move_ret = bridge.move_to(0, 0).expect("dm MoveTo must work");
        bridge.shutdown().expect("DmBridge shutdown must work");

        assert!(!version.trim().is_empty());
        assert!(!color.trim().is_empty());
        assert_eq!(move_ret, 1);
    }
}
