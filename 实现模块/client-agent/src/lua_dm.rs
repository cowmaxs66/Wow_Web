use crate::config::AgentConfig;
use crate::dm_bridge::{DmBridge, DmBridgeError, resolve_bridge_path};
use mlua::{Error as LuaError, Lua, Table};
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_table(lua: &Lua, config: &AgentConfig) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    let bridge_state: Rc<RefCell<Option<DmBridge>>> = Rc::new(RefCell::new(None));

    register_lifecycle(lua, &table, config, &bridge_state)?;
    register_window_and_bind(lua, &table, config, &bridge_state)?;
    register_color_and_input(lua, &table, config, &bridge_state)?;

    Ok(table)
}

fn register_lifecycle(
    lua: &Lua,
    table: &Table,
    config: &AgentConfig,
    bridge_state: &Rc<RefCell<Option<DmBridge>>>,
) -> mlua::Result<()> {
    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "abi_version",
        lua.create_function(move |_, ()| {
            with_bridge(&fn_config, &state, |bridge| Ok(bridge.abi_version()))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "init",
        lua.create_function(move |_, dm_root: String| {
            with_bridge(&fn_config, &state, |bridge| bridge.init(&dm_root))?;
            Ok(true)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "shutdown",
        lua.create_function(move |_, ()| {
            with_bridge(&fn_config, &state, |bridge| bridge.shutdown())?;
            Ok(true)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "ver",
        lua.create_function(move |_, ()| with_bridge(&fn_config, &state, |bridge| bridge.ver()))?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "last_bridge_error",
        lua.create_function(move |_, ()| {
            with_bridge(&fn_config, &state, |bridge| Ok(bridge.last_bridge_error()))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "last_dm_error",
        lua.create_function(move |_, ()| {
            with_bridge(&fn_config, &state, |bridge| bridge.last_dm_error())
        })?,
    )?;

    Ok(())
}

fn register_window_and_bind(
    lua: &Lua,
    table: &Table,
    config: &AgentConfig,
    bridge_state: &Rc<RefCell<Option<DmBridge>>>,
) -> mlua::Result<()> {
    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "set_path",
        lua.create_function(move |_, path: String| {
            with_bridge(&fn_config, &state, |bridge| bridge.set_path(&path))?;
            Ok(true)
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "find_window",
        lua.create_function(move |_, (class_name, title_name): (String, String)| {
            with_bridge(&fn_config, &state, |bridge| {
                bridge.find_window(&class_name, &title_name)
            })
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "bind_window",
        lua.create_function(
            move |_, (hwnd, display, mouse, keypad, mode): (i32, String, String, String, i32)| {
                with_bridge(&fn_config, &state, |bridge| {
                    bridge.bind_window(hwnd, &display, &mouse, &keypad, mode)
                })
            },
        )?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "unbind_window",
        lua.create_function(move |_, ()| {
            with_bridge(&fn_config, &state, |bridge| bridge.unbind_window())
        })?,
    )?;

    Ok(())
}

fn register_color_and_input(
    lua: &Lua,
    table: &Table,
    config: &AgentConfig,
    bridge_state: &Rc<RefCell<Option<DmBridge>>>,
) -> mlua::Result<()> {
    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "get_color",
        lua.create_function(move |_, (x, y): (i32, i32)| {
            with_bridge(&fn_config, &state, |bridge| bridge.get_color(x, y))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "move_to",
        lua.create_function(move |_, (x, y): (i32, i32)| {
            with_bridge(&fn_config, &state, |bridge| bridge.move_to(x, y))
        })?,
    )?;

    let fn_config = config.clone();
    let state = Rc::clone(bridge_state);
    table.set(
        "left_click",
        lua.create_function(move |_, ()| {
            with_bridge(&fn_config, &state, |bridge| bridge.left_click())
        })?,
    )?;

    Ok(())
}

fn with_bridge<T>(
    config: &AgentConfig,
    state: &Rc<RefCell<Option<DmBridge>>>,
    action: impl FnOnce(&DmBridge) -> Result<T, DmBridgeError>,
) -> mlua::Result<T> {
    let mut bridge_slot = state.borrow_mut();

    if bridge_slot.is_none() {
        let path = resolve_bridge_path(&config.dm.bridge_path);
        let bridge = DmBridge::load(path).map_err(lua_dm_error)?;
        tracing::info!(path = %bridge.path().display(), "DmBridge 已加载");
        *bridge_slot = Some(bridge);
    }

    // Lua API 保持懒加载和同步执行。
    // 输入：Lua 调用参数和配置中的 DmBridge 路径。
    // 输出：安全 Rust 封装的返回值。
    // 边界：不把 libloading Symbol、裸指针或 C ABI 直接暴露给 Lua。
    action(bridge_slot.as_ref().expect("bridge must be loaded")).map_err(lua_dm_error)
}

fn lua_dm_error(error: DmBridgeError) -> LuaError {
    LuaError::RuntimeError(error.to_string())
}
