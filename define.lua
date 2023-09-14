---@meta

---@class ScriptRegister
---@field identity string
---@field name string
---@field userConfig UserConfig
---@field configUpdate? function(config)
---@field getApplicableScripts function (unityBundle)-> { [string]: SubScriptEntry }

---@class UserConfig:{[number]: UserEditableConfigItem}

---@class UserEditableConfigItem
---@field identity string
---@field text string
---@field tip? string
---@field kind UserEditableConfigKind

---@class UserEditableConfigKind
---@field ty ConfigKind
---@field default string
---@field selects? Selects

---@class Selects : {[string] : string}

---@enum ConfigKind
ConfigKind = {
    Switch = "switch",
    Select = "select",
    Text = "text"
}

---@class ScriptConfig
---@field identity string
---@field config {[string] : UserConfigKind }
ScriptConfig = {}

---@class UserConfigKind
---@field kind string
---@field value string

---@param key string
---@return string
function ScriptConfig:storageLoad(key) end

---@param key string
---@param value string
function ScriptConfig:storageStore(key, value) end

---@class SubScriptEntry
---@field name string
---@field entry function(script,unityBundle,manager)


return {
    ConfigKind = ConfigKind,
    ScripScriptConfigt = ScriptConfig
}
