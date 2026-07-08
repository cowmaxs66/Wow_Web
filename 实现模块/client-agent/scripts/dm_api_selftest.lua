local function step(name, action)
    local ok, result = pcall(action)
    if not ok then
        error("dm_api_selftest failed at " .. name .. ": " .. tostring(result))
    end
    if log then
        log("dm_api_selftest " .. name .. " ok")
    end
    return result
end

local client_id = "unknown"
if get_config then
    client_id = step("get_config", function()
        return get_config("client.id")
    end)
end

step("log", function()
    if log then
        log("dm_api_selftest started: client_id=" .. tostring(client_id))
    end
    return true
end)

local abi = step("dm.abi_version", function()
    return dm.abi_version()
end)

step("dm.init", function()
    return dm.init("")
end)

local version = step("dm.ver", function()
    return dm.ver()
end)

step("dm.set_path", function()
    return dm.set_path(".")
end)

local bridge_error = step("dm.last_bridge_error", function()
    return dm.last_bridge_error()
end)

local dm_error = step("dm.last_dm_error", function()
    return dm.last_dm_error()
end)

local color = step("dm.get_color", function()
    return dm.get_color(0, 0)
end)

local rgb = step("dm.get_color_rgb", function()
    return dm.get_color_rgb(0, 0)
end)

local wait_ok, wait_color = step("dm.wait_color", function()
    return dm.wait_color(0, 0, rgb.hex, 300, 50)
end)

local missing_hwnd = step("dm.find_window_missing", function()
    return dm.find_window("", "__WOW_FRAMEWORK_SELFTEST_WINDOW_SHOULD_NOT_EXIST__")
end)

local missing_probe = step("dm.find_window_try_missing", function()
    return dm.find_window_try("", "__WOW_FRAMEWORK_SELFTEST_WINDOW_SHOULD_NOT_EXIST__")
end)

local invalid_bind = step("dm.safe_bind_window_invalid", function()
    local ok, err = dm.safe_bind_window(0, "normal", "windows", "windows", 0)
    return { ok = ok, err = err }
end)

step("dm.sleep_ms", function()
    return dm.sleep_ms(10)
end)

local now_ms = step("dm.now_ms", function()
    return dm.now_ms()
end)

step("dm.shutdown", function()
    return dm.shutdown()
end)

return table.concat({
    "dm_api_selftest",
    "client=" .. tostring(client_id),
    "abi=" .. tostring(abi),
    "ver=" .. tostring(version),
    "color00=" .. tostring(color),
    "rgb=" .. tostring(rgb.hex) .. "/" .. tostring(rgb.r) .. "," .. tostring(rgb.g) .. "," .. tostring(rgb.b),
    "wait_color=" .. tostring(wait_ok) .. "/" .. tostring(wait_color),
    "missing_hwnd=" .. tostring(missing_hwnd),
    "missing_probe_ok=" .. tostring(missing_probe.ok),
    "invalid_bind_ok=" .. tostring(invalid_bind.ok),
    "bridge_error=" .. tostring(bridge_error),
    "dm_error=" .. tostring(dm_error),
    "now_ms=" .. tostring(now_ms),
}, "|")
