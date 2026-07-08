local target_class = "Qt51514QWindowIcon"
local target_title = "微信"

local modes = {
    { "normal", "normal", "normal", 0 },
    { "normal", "windows", "windows", 0 },
    { "gdi", "windows", "windows", 0 },
    { "gdi2", "windows", "windows", 0 },
}

local function mode_label(mode)
    return table.concat({
        "display=" .. mode[1],
        "mouse=" .. mode[2],
        "keypad=" .. mode[3],
        "mode=" .. tostring(mode[4]),
    }, ",")
end

local hwnd = dm.find_window(target_class, target_title)
if hwnd <= 0 then
    return "dm_bind_probe|window=not_found|class=" .. target_class .. "|title=" .. target_title
end

local results = {}
for index, mode in ipairs(modes) do
    local label = mode_label(mode)
    local ok, err = dm.safe_bind_window(hwnd, mode[1], mode[2], mode[3], mode[4])
    if ok then
        local color = dm.get_color_rgb(10, 10)
        dm.unbind_window()
        local line = "ok#" .. tostring(index) .. "{" .. label .. ",color=" .. tostring(color.hex) .. "}"
        log("dm_bind_probe " .. line)
        table.insert(results, line)
    else
        pcall(function()
            dm.unbind_window()
        end)
        local line = "fail#" .. tostring(index) .. "{" .. label .. ",error=" .. tostring(err) .. "}"
        log("dm_bind_probe " .. line)
        table.insert(results, line)
    end
    dm.sleep_ms(100)
end

return "dm_bind_probe|hwnd=" .. tostring(hwnd)
    .. "|class=" .. target_class
    .. "|title=" .. target_title
    .. "|" .. table.concat(results, "|")
