local impls = require("test_lua/impls")
local define = require("define")
local ConfigKind = define.ConfigKind


local math = require("math")

---initScript
---@return ScriptRegister
function InitScript()
    ---@type ScriptRegister
    local register = {
        identity = "test.script.1",
        name = "测试脚本",
        userConfig = {
            [1] = {
                identity = "user.config.1",
                text = "一号用户设定",
                tip = "测试数据",
                ---@type SwitchConfig
                kind = {
                    ty = ConfigKind.Switch,
                    default = false
                }
            },
            [2] = {
                identity = "user.config.2",
                text = "用户配置2",
                ---@type SelectConfig
                kind = {
                    ty = ConfigKind.Select,
                    selects = {
                        ["0"] = "选择1",
                        ["1"] = "选择2",
                    },
                    default = "0"
                }
            }
        },
        configUpdate = impls.configUpdate,
        getApplicableScripts = impls.verifyApplicable
    }
    return register
end
