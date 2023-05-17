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

do
    for j = 0, 100 do
        if j % 2 == 0 then
            repeat
                if j % 10 then
                    print(j)
                end
                j = j - 1
            until j <= 0
        end
    end
end