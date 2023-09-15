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

---@alias UserEditableConfigKind SwitchConfig | SelectConfig | TextConfig

---@class SwitchConfig
---@field ty "switch"
---@field default boolean

---@class SelectConfig
---@field ty "select"
---@field default string
---@field selects Selects

---@alias Selects  {[string] : string}

---@class TextConfig
---@field ty "text"
---@field default string

---@enum ConfigKind
ConfigKind = {
    Switch = "switch",
    Select = "select",
    Text = "text"
}

---@class ScriptConfig
---@field identity string
---@field config {[string] : UserConfigVariable }
ScriptConfig = {}

---@alias UserConfigVariable SwitchConfigVariable|SelectConfigVariable|TextConfigVariable

---@class SwitchConfigVariable
---@field ty "switch"
---@field value boolean

---@class SelectConfigVariable
---@field ty "select"
---@field value string

---@class TextConfigVariable
---@field ty "text"
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

---@class Log
Log = {}

---@param args {[string]: any}
function Log:info(args) end

---@param args {[string]: any}
function Log:debug(args) end

---@param args {[string]: any}
function Log:error(args) end

---@param args {[string]: any}
function Log:warn(args) end

---@param args {[string]: any}
function Log:trace(args) end

return {
    ConfigKind = ConfigKind,
    ScripScriptConfigt = ScriptConfig,
    Log = Log
}
