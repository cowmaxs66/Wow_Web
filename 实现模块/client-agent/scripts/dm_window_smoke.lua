local target_title = "World of Warcraft"
local target_class = ""

local function step(name, action)
    local ok, result = pcall(action)
    if not ok then
        error("dm_window_smoke failed at " .. name .. ": " .. tostring(result))
    end
    if log then
        log("dm_window_smoke " .. name .. " ok")
    end
    return result
end

step("dm.init", function()
    return dm.init("")
end)

local version = step("dm.ver", function()
    return dm.ver()
end)

local hwnd = step("dm.find_window", function()
    return dm.find_window(target_class, target_title)
end)

if hwnd <= 0 then
    local bridge_error = dm.last_bridge_error()
    dm.shutdown()
    return "dm_window_smoke|window=not_found"
        .. "|title=" .. target_title
        .. "|ver=" .. tostring(version)
        .. "|bridge_error=" .. tostring(bridge_error)
end

local ok, bind_error = dm.safe_bind_window(hwnd, "normal", "windows", "windows", 0)
if not ok then
    dm.shutdown()
    return "dm_window_smoke|bind=failed"
        .. "|hwnd=" .. tostring(hwnd)
        .. "|title=" .. target_title
        .. "|error=" .. tostring(bind_error)
end

local rgb = dm.get_color_rgb(0, 0)
dm.unbind_window()
dm.shutdown()

return "dm_window_smoke|window=found"
    .. "|hwnd=" .. tostring(hwnd)
    .. "|title=" .. target_title
    .. "|ver=" .. tostring(version)
    .. "|color00=" .. tostring(rgb.hex)
