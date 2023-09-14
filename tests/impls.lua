model = {}

function model.configUpdate(configs,storage)
    for i, v in ipairs(configs) do
        print("config ", i , v)
    end

end

function model.verifyApplicable(unityObject,storage)
    return {
        ["A"] = {
            name = "操作A",
            entry_point="A"
        }
    }
end

local function operateA(unityObject)
    print("执行操作A")
end
model.maps = {
    ["A"] = operateA,
    ["B"] = function(b) return b -1 end
}

return model