local function call_step(name, action)
    local ok, result = pcall(action)
    if not ok then
        error("DM smoke failed at " .. name .. ": " .. tostring(result))
    end
    return result
end

if log then
    log("dm smoke started")
end

local abi = call_step("abi_version", function()
    return dm.abi_version()
end)

call_step("init", function()
    return dm.init("")
end)

local version = call_step("ver", function()
    return dm.ver()
end)

local color = call_step("get_color", function()
    return dm.get_color(0, 0)
end)

local last_error = call_step("last_dm_error", function()
    return dm.last_dm_error()
end)

call_step("shutdown", function()
    return dm.shutdown()
end)

return "dm_smoke|abi=" .. tostring(abi)
    .. "|ver=" .. tostring(version)
    .. "|color00=" .. tostring(color)
    .. "|last_error=" .. tostring(last_error)
