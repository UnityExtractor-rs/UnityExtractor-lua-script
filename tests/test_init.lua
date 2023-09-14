
---@class ScriptRegister
ScriptRegister = {}

---@class UserEditableConfig
---@field idx string 该配置项的唯一标识，用于标识该config
---@field text string 该配置项供用户阅读文本
---@field tip string? 该配置的额外提示
---@field kind UserEditableConfigKind


---@class UserEditableConfigKind
---@field ty "switch" | "select" | "text"
---@field default string | boolean
---@field selects {[number]:SelectItem}


---@enum ConfigKind
ConfigKind ={
    Switch="switch",
    Select = "select",
    Text = "text"
}

---@class SelectItem
---@field idx string
---@field text string
SelectItem={}

---@param name string
function ScriptRegister:name(name) end
function operateA(unityObject)
    print("执行操作A")
end
maps = {
    ["A"] = operateA,
    ["B"] = function(b) return b -1 end
}

---addUserConfig
---@param config UserEditableConfig
function ScriptRegister:addUserConfig(config)end

---initScript
---@param register ScriptRegister
function initScript(register)

    print(maps["A"](11))
    print("init script test")
    register:name("test");
    register:addUserConfig({
        idx =  "1",
        text = "配置1",
        tip = "提示",
        kind = {
            ty = ConfigKind.Switch,
            default = "true"
        }
    })
    register:addUserConfig({
        idx="2",
        text = "配置2",
        kind={
            ty = ConfigKind.Select,
            selects = {
                [1] = {
                    idx = "0",
                    text ="A"
                },
                [2] = {
                    idx = "1",
                    text = "B"
                }
            },
            default = "1"
        }
    })
    register:configUpdateCallback("configUpdate")
    register:verifyApplicable("verifyApplicable")
    register:entryPoint("maps")
end

function configUpdate(configs,storage)
    for i, v in ipairs(configs) do
        print("config ", i , v)
    end

end

function verifyApplicable(unityObject,storage)
    return {
        ["A"] = {
            name = "操作A",
            entry_point="A"
        }
    }
end

