use super::error::DmBridgeError;
use libloading::Library;

pub const DM_BRIDGE_ABI_VERSION: i32 = 1;
pub const DM_BRIDGE_OK: i32 = 1;
pub const DM_BRIDGE_BUFFER_TOO_SMALL: i32 = -4;

pub type AbiVersionFn = unsafe extern "system" fn() -> i32;
pub type InitFn = unsafe extern "system" fn(*const u16) -> i32;
pub type ShutdownFn = unsafe extern "system" fn() -> i32;
pub type GetLastBridgeErrorFn = unsafe extern "system" fn() -> i32;
pub type GetLastDmErrorFn = unsafe extern "system" fn(*mut i32) -> i32;
pub type GetLastMessageFn = unsafe extern "system" fn(*mut u16, u32, *mut u32) -> i32;
pub type VerFn = unsafe extern "system" fn(*mut u16, u32, *mut u32) -> i32;
pub type SetPathFn = unsafe extern "system" fn(*const u16) -> i32;
pub type FindWindowFn = unsafe extern "system" fn(*const u16, *const u16, *mut i32) -> i32;
pub type BindWindowFn =
    unsafe extern "system" fn(i32, *const u16, *const u16, *const u16, i32, *mut i32) -> i32;
pub type GetColorFn = unsafe extern "system" fn(i32, i32, *mut u16, u32, *mut u32) -> i32;
pub type MoveToFn = unsafe extern "system" fn(i32, i32, *mut i32) -> i32;
pub type LeftClickFn = unsafe extern "system" fn(*mut i32) -> i32;
pub type UnbindWindowFn = unsafe extern "system" fn(*mut i32) -> i32;

pub struct RawDmBridge {
    pub abi_version: AbiVersionFn,
    pub init: InitFn,
    pub shutdown: ShutdownFn,
    pub get_last_bridge_error: GetLastBridgeErrorFn,
    pub get_last_dm_error: GetLastDmErrorFn,
    pub get_last_message: GetLastMessageFn,
    pub ver: VerFn,
    pub set_path: SetPathFn,
    pub find_window: FindWindowFn,
    pub bind_window: BindWindowFn,
    pub get_color: GetColorFn,
    pub move_to: MoveToFn,
    pub left_click: LeftClickFn,
    pub unbind_window: UnbindWindowFn,
}

impl RawDmBridge {
    pub unsafe fn load(library: &Library) -> Result<Self, DmBridgeError> {
        Ok(Self {
            abi_version: unsafe { load_symbol(library, b"dm_bridge_abi_version\0")? },
            init: unsafe { load_symbol(library, b"dm_bridge_init\0")? },
            shutdown: unsafe { load_symbol(library, b"dm_bridge_shutdown\0")? },
            get_last_bridge_error: unsafe {
                load_symbol(library, b"dm_bridge_get_last_bridge_error\0")?
            },
            get_last_dm_error: unsafe { load_symbol(library, b"dm_bridge_get_last_dm_error\0")? },
            get_last_message: unsafe { load_symbol(library, b"dm_bridge_get_last_message\0")? },
            ver: unsafe { load_symbol(library, b"dm_bridge_ver\0")? },
            set_path: unsafe { load_symbol(library, b"dm_bridge_set_path\0")? },
            find_window: unsafe { load_symbol(library, b"dm_bridge_find_window\0")? },
            bind_window: unsafe { load_symbol(library, b"dm_bridge_bind_window\0")? },
            get_color: unsafe { load_symbol(library, b"dm_bridge_get_color\0")? },
            move_to: unsafe { load_symbol(library, b"dm_bridge_move_to\0")? },
            left_click: unsafe { load_symbol(library, b"dm_bridge_left_click\0")? },
            unbind_window: unsafe { load_symbol(library, b"dm_bridge_unbind_window\0")? },
        })
    }
}

unsafe fn load_symbol<T: Copy>(library: &Library, name: &[u8]) -> Result<T, DmBridgeError> {
    let symbol = unsafe { library.get::<T>(name) }
        .map_err(|error| DmBridgeError::symbol_missing(symbol_name(name), error.to_string()))?;
    Ok(*symbol)
}

fn symbol_name(name: &[u8]) -> String {
    let trimmed = name.strip_suffix(&[0]).unwrap_or(name);
    String::from_utf8_lossy(trimmed).to_string()
}
