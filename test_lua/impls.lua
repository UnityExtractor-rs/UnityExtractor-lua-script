module = {}


---@param configs {[string]:string}
function module.configUpdate(configs)
    for i, v in pairs(configs) do
        print("config ", i, v)
    end
end

---@return nil | {[string]: SubScriptEntry}
function module.verifyApplicable(unityObject)
    ---@type {[string]: SubScriptEntry}
    local ops = {
        ["A"] = {
            name = "操作A",
            entry = OperateA
        },
        ["B"] = {
            name = "操作B",
            entry = function(script, unityObject, manager)
                print("this is func B")
                print(script:storageLoad("A"))
                print("this is ", script.identity)
            end
        }
    }
    return ops
end

---@param script ScriptConfig
---@param unityBundle any
---@param manager any
function OperateA(script, unityBundle, manager)
    script:storageStore("A", "A");
    print("this is ", script.identity)
    print("config 2 is ", script.config["user.config.2"].value, "kind is ", script.config["user.config.2"].kind)
    print(script:storageLoad("A"))
    print("process operat A")
end

return module
