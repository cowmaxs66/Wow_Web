use crate::config::AgentConfig;
use crate::dm_bridge::{DmBridge, DmBridgeError, resolve_bridge_path};
use mlua::{Error as LuaError, Function, Lua, Table, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type SharedBridgeState = Rc<RefCell<Option<BridgeRuntime>>>;

struct BridgeRuntime {
    bridge: DmBridge,
    initialized: bool,
}

pub fn create_table(lua: &Lua, config: &AgentConfig) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    let bridge_state: SharedBridgeState = Rc::new(RefCell::new(None));

    register_lifecycle(lua, &table, config, &bridge_state)?;
    register_window_and_bind(lua, &table, config, &bridge_state)?;
    register_color_and_input(lua, &table, config, &bridge_state)?;
    register_helpers(lua, &table, config, &bridge_state)?;

    Ok(table)
}

fn register_lifecycle(
    lua: &Lua,
    table: &Table,
    config: &AgentConfig,
    bridge_state: &SharedBridgeState,
) -> mlua::Result<()> {
    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "abi_version",
        lua.create_function(move |_, ()| {
            with_bridge_loaded(&fn_config, &state, |bridge| Ok(bridge.abi_version()))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "init",
        lua.create_function(move |_, dm_root: String| {
            with_bridge_initialized(&fn_config, &state, &dm_root, |_| Ok(()))?;
            Ok(true)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "shutdown",
        lua.create_function(move |_, ()| {
            with_bridge_runtime(&fn_config, &state, |runtime| {
                runtime.bridge.shutdown()?;
                runtime.initialized = false;
                Ok(())
            })?;
            Ok(true)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "ver",
        lua.create_function(move |_, ()| {
            with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.ver())
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "last_bridge_error",
        lua.create_function(move |_, ()| {
            with_bridge_loaded(&fn_config, &state, |bridge| Ok(bridge.last_bridge_error()))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "last_dm_error",
        lua.create_function(move |_, ()| {
            with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.last_dm_error())
        })?,
    )?;

    Ok(())
}

fn register_window_and_bind(
    lua: &Lua,
    table: &Table,
    config: &AgentConfig,
    bridge_state: &SharedBridgeState,
) -> mlua::Result<()> {
    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "set_path",
        lua.create_function(move |_, path: String| {
            with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.set_path(&path))?;
            Ok(true)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "find_window",
        lua.create_function(move |_, (class_name, title_name): (String, String)| {
            let result = with_bridge_initialized_result(&fn_config, &state, "", |bridge| {
                bridge.find_window(&class_name, &title_name)
            });
            match result {
                Ok(hwnd) => Ok(hwnd),
                Err(error) if error.is_find_window_not_found() => Ok(0),
                Err(error) => Err(lua_dm_error(error)),
            }
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "find_window_required",
        lua.create_function(move |_, (class_name, title_name): (String, String)| {
            let result = with_bridge_initialized_result(&fn_config, &state, "", |bridge| {
                bridge.find_window(&class_name, &title_name)
            });
            let hwnd = match result {
                Ok(hwnd) => hwnd,
                Err(error) if error.is_find_window_not_found() => 0,
                Err(error) => return Err(lua_dm_error(error)),
            };
            if hwnd <= 0 {
                return Err(LuaError::runtime(format!(
                    "窗口不存在：class={class_name} title={title_name}"
                )));
            }
            Ok(hwnd)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "find_window_try",
        lua.create_function(move |lua, (class_name, title_name): (String, String)| {
            let result = with_bridge_initialized_result(&fn_config, &state, "", |bridge| {
                bridge.find_window(&class_name, &title_name)
            });
            let table = lua.create_table()?;
            match result {
                Ok(hwnd) => {
                    table.set("ok", true)?;
                    table.set("hwnd", hwnd)?;
                    table.set("error", "")?;
                }
                Err(error) if error.is_find_window_not_found() => {
                    table.set("ok", true)?;
                    table.set("hwnd", 0)?;
                    table.set(
                        "error",
                        format!("窗口不存在：class={class_name} title={title_name}"),
                    )?;
                }
                Err(error) => {
                    table.set("ok", false)?;
                    table.set("hwnd", 0)?;
                    table.set("error", error.to_string())?;
                }
            }
            Ok(table)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "bind_window",
        lua.create_function(
            move |_, (hwnd, display, mouse, keypad, mode): (i32, String, String, String, i32)| {
                with_bridge_initialized(&fn_config, &state, "", |bridge| {
                    bridge.bind_window(hwnd, &display, &mouse, &keypad, mode)
                })
            },
        )?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "bind_window_try",
        lua.create_function(
            move |lua, (hwnd, display, mouse, keypad, mode): (i32, String, String, String, i32)| {
                let result = with_bridge_initialized_result(&fn_config, &state, "", |bridge| {
                    bridge.bind_window(hwnd, &display, &mouse, &keypad, mode)
                });
                let table = lua.create_table()?;
                match result {
                    Ok(ret) => {
                        table.set("ok", true)?;
                        table.set("ret", ret)?;
                    }
                    Err(error) => {
                        table.set("ok", false)?;
                        table.set("ret", 0)?;
                        table.set("error", error.to_string())?;
                    }
                }
                Ok(table)
            },
        )?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "safe_bind_window",
        lua.create_function(
            move |_, (hwnd, display, mouse, keypad, mode): (i32, String, String, String, i32)| {
                match with_bridge_initialized_result(&fn_config, &state, "", |bridge| {
                    bridge.bind_window(hwnd, &display, &mouse, &keypad, mode)
                }) {
                    Ok(_) => Ok((true, String::new())),
                    Err(error) => Ok((false, error.to_string())),
                }
            },
        )?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "unbind_window",
        lua.create_function(move |_, ()| {
            with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.unbind_window())
        })?,
    )?;

    Ok(())
}

fn register_color_and_input(
    lua: &Lua,
    table: &Table,
    config: &AgentConfig,
    bridge_state: &SharedBridgeState,
) -> mlua::Result<()> {
    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "get_color",
        lua.create_function(move |_, (x, y): (i32, i32)| {
            with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.get_color(x, y))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "get_color_rgb",
        lua.create_function(move |lua, (x, y): (i32, i32)| {
            let color =
                with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.get_color(x, y))?;
            let (r, g, b) = parse_rgb_hex(&color).map_err(LuaError::runtime)?;
            let table = lua.create_table()?;
            table.set("hex", color)?;
            table.set("r", r)?;
            table.set("g", g)?;
            table.set("b", b)?;
            Ok(table)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "wait_color",
        lua.create_function(
            move |_, (x, y, expected, timeout_ms, interval_ms): (i32, i32, String, u64, u64)| {
                let timeout = Duration::from_millis(timeout_ms.min(60_000));
                let interval = Duration::from_millis(interval_ms.clamp(10, 5_000));
                let deadline = Instant::now() + timeout;

                loop {
                    let color = with_bridge_initialized(&fn_config, &state, "", |bridge| {
                        bridge.get_color(x, y)
                    })?;
                    if color.eq_ignore_ascii_case(expected.trim()) {
                        return Ok((true, color));
                    }
                    if Instant::now() >= deadline {
                        return Ok((false, color));
                    }
                    std::thread::sleep(interval);
                }
            },
        )?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "move_to",
        lua.create_function(move |_, (x, y): (i32, i32)| {
            with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.move_to(x, y))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "left_click",
        lua.create_function(move |_, ()| {
            with_bridge_initialized(&fn_config, &state, "", |bridge| bridge.left_click())
        })?,
    )?;

    Ok(())
}

fn register_helpers(
    lua: &Lua,
    table: &Table,
    config: &AgentConfig,
    bridge_state: &SharedBridgeState,
) -> mlua::Result<()> {
    table.set(
        "sleep_ms",
        lua.create_function(move |_, ms: u64| {
            std::thread::sleep(Duration::from_millis(ms.min(60_000)));
            Ok(true)
        })?,
    )?;

    table.set(
        "now_ms",
        lua.create_function(move |_, ()| {
            let ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis() as u64)
                .unwrap_or_default();
            Ok(ms)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "with_bound_window",
        lua.create_function(
            move |_,
                  (hwnd, display, mouse, keypad, mode, callback): (
                i32,
                String,
                String,
                String,
                i32,
                Function,
            )| {
                with_bridge_initialized(&fn_config, &state, "", |bridge| {
                    bridge.bind_window(hwnd, &display, &mouse, &keypad, mode)
                })?;

                let callback_result: mlua::Result<Value> = callback.call(());
                let unbind_result = with_bridge_initialized(&fn_config, &state, "", |bridge| {
                    bridge.unbind_window()
                });

                unbind_result?;

                callback_result
            },
        )?,
    )?;

    Ok(())
}

fn with_bridge_loaded<T>(
    config: &AgentConfig,
    state: &SharedBridgeState,
    action: impl FnOnce(&DmBridge) -> Result<T, DmBridgeError>,
) -> mlua::Result<T> {
    with_bridge_loaded_result(config, state, action).map_err(lua_dm_error)
}

fn with_bridge_initialized<T>(
    config: &AgentConfig,
    state: &SharedBridgeState,
    dm_root: &str,
    action: impl FnOnce(&DmBridge) -> Result<T, DmBridgeError>,
) -> mlua::Result<T> {
    with_bridge_initialized_result(config, state, dm_root, action).map_err(lua_dm_error)
}

fn with_bridge_runtime<T>(
    config: &AgentConfig,
    state: &SharedBridgeState,
    action: impl FnOnce(&mut BridgeRuntime) -> Result<T, DmBridgeError>,
) -> mlua::Result<T> {
    with_bridge_runtime_result(config, state, action).map_err(lua_dm_error)
}

fn with_bridge_loaded_result<T>(
    config: &AgentConfig,
    state: &SharedBridgeState,
    action: impl FnOnce(&DmBridge) -> Result<T, DmBridgeError>,
) -> Result<T, DmBridgeError> {
    with_bridge_runtime_result(config, state, |runtime| action(&runtime.bridge))
}

fn with_bridge_initialized_result<T>(
    config: &AgentConfig,
    state: &SharedBridgeState,
    dm_root: &str,
    action: impl FnOnce(&DmBridge) -> Result<T, DmBridgeError>,
) -> Result<T, DmBridgeError> {
    with_bridge_runtime_result(config, state, |runtime| {
        if !runtime.initialized {
            runtime.bridge.init(dm_root)?;
            runtime.initialized = true;
            tracing::info!(path = %runtime.bridge.path().display(), "DmBridge 已初始化");
        }

        action(&runtime.bridge)
    })
}

fn with_bridge_runtime_result<T>(
    config: &AgentConfig,
    state: &SharedBridgeState,
    action: impl FnOnce(&mut BridgeRuntime) -> Result<T, DmBridgeError>,
) -> Result<T, DmBridgeError> {
    let mut bridge_slot = state.borrow_mut();
    let runtime = ensure_bridge_runtime(config, &mut bridge_slot)?;

    // Lua API 保持懒加载、自动初始化和同步执行。
    // 输入：Lua 调用参数和配置中的 DmBridge 路径。
    // 输出：安全 Rust 封装的返回值。
    // 边界：不把 libloading Symbol、裸指针或 C ABI 直接暴露给 Lua。
    action(runtime)
}

fn ensure_bridge_runtime<'a>(
    config: &AgentConfig,
    bridge_slot: &'a mut Option<BridgeRuntime>,
) -> Result<&'a mut BridgeRuntime, DmBridgeError> {
    if bridge_slot.is_none() {
        let path = resolve_bridge_path(&config.dm.bridge_path);
        let bridge = DmBridge::load(path)?;
        tracing::info!(path = %bridge.path().display(), "DmBridge 已加载");
        *bridge_slot = Some(BridgeRuntime {
            bridge,
            initialized: false,
        });
    }

    Ok(bridge_slot.as_mut().expect("bridge runtime must be loaded"))
}

fn parse_rgb_hex(color: &str) -> Result<(u8, u8, u8), String> {
    let text = color.trim().trim_start_matches('#');
    if text.len() != 6 {
        return Err(format!("颜色值必须是 6 位十六进制：{color}"));
    }

    let value =
        u32::from_str_radix(text, 16).map_err(|_| format!("颜色值不是十六进制：{color}"))?;
    Ok((
        ((value >> 16) & 0xff) as u8,
        ((value >> 8) & 0xff) as u8,
        (value & 0xff) as u8,
    ))
}

fn lua_dm_error(error: DmBridgeError) -> LuaError {
    LuaError::RuntimeError(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rgb_hex_reads_uppercase_color() {
        assert_eq!(parse_rgb_hex("A1B2C3"), Ok((0xA1, 0xB2, 0xC3)));
    }

    #[test]
    fn parse_rgb_hex_rejects_invalid_color() {
        assert!(parse_rgb_hex("xyz").is_err());
    }
}
