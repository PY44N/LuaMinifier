local a = {"Hello", "world"}

local logger = {
    log = function(val)
        print(val)
    end
}

function logger:log_vararg(...)
    local args = {...}

    for _, v in pairs(args) do
        logger.log(v)
    end
end

local i = 1
while i <= #a do
    logger:log_vararg(a[i])
    i = i + 1
end

if true and (false or true) then
    print(true and (false or true) and (nil or false))
end

for i=1,10 do
    print(i)
end

do
    for j = 0, 100 do
        if j % 2 == 0 then
            repeat
                if j % 10 then
                    print(j)
                    break
                end
                j = j - 1
            until j <= 0
            return;
        end
    end
end