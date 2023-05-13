local a = {"Hello", "world"}

function log(val)
    print(val)
end

local function log_vararg(...)
    local args = {...}

    for _, v in pairs(args) do
        log(v)
    end
end

local i = 1
while i <= #a do
    log_vararg(a[i])
    i = i + 1
end