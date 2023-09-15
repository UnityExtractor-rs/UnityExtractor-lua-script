module = {}
Log = require("define").Log

---@param configs {[string]:string}
function module.configUpdate(configs)
    Log:info {
        ["update.config"] = configs
    }
    for i, v in pairs(configs) do
        Log:info({
            ["function"] = "update config",
            ["update"] = "Config",
            ["config." .. i] = v,
        })
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
            ---@param script ScriptConfig
            ---@param unityObject any
            ---@param manager any
            entry = function(script, unityObject, manager)
                Log:info({ ["entry"] = "Operate B" })
                Log:info({
                    ["script"] = "B",
                    ["state"] = "entry",
                    ["state.data"] = script:storageLoad("A"),
                    ["it.self"] = script.identity
                });

                local is_enable = script.config["user.config.1"]

                if is_enable.ty == "switch" then
                    if is_enable.value then
                        Log:info({
                            ["config"] = "user.config.1",
                            ["enable"] = true
                        })
                    else
                        Log:info({
                            ["config"] = "user.config.1",
                            ["enable"] = false,
                            ["action"] = "好好好"
                        })
                    end
                end
            end
        }
    }
    return ops
end

---@param script ScriptConfig
---@param unityBundle any
---@param manager any
function OperateA(script, unityBundle, manager)
    Log:info({ ["entry"] = "Operate A" })
    script:storageStore("A", "A");

    Log:info({ ["this is"] = script.identity })
    Log:info({
        ["config2.is"] = script.config["user.config.2"].value,
        ["config2.type"] = script.config["user.config.2"].ty
    })

    Log:info({ ["data.A"] = script:storageLoad("A") })
end

return module
