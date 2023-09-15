use mlua::{IntoLua, Lua, UserData, UserDataFields, Value};
use typed_builder::TypedBuilder;

/// the user Edit able config type
#[derive(Debug, Clone)]
pub enum ConfigValue {
    /// a config which user can enable or disable it
    Switch(bool),
    /// a config which user can select item in one of provide selects
    Select(String),
    /// a config which user can enter text as config value
    Text(String),
}
impl ConfigValue {
    pub fn ty(&self) -> &'static str {
        match self {
            ConfigValue::Switch(_) => "switch",
            ConfigValue::Select(_) => "select",
            ConfigValue::Text(_) => "text",
        }
    }
}

impl<'lua> IntoLua<'lua> for &ConfigValue {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        Ok(Value::String(lua.create_string(self.ty().as_bytes())?))
    }
}

impl<'lua> IntoLua<'lua> for ConfigValue {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        Ok(Value::String(lua.create_string(self.ty().as_bytes())?))
    }
}

/// one of user editable config
///
///
#[derive(Debug, TypedBuilder)]
pub struct UserEditConfig {
    pub kind: ConfigValue,
}

impl UserEditConfig {
    pub fn value(&self) -> &str {
        match &self.kind {
            ConfigValue::Switch(v) => {
                if *v {
                    "true"
                } else {
                    "false"
                }
            }
            ConfigValue::Select(s) | ConfigValue::Text(s) => s,
        }
    }
}

impl UserData for UserEditConfig {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("ty", |_vm, this| Ok(this.kind.ty()));
        fields.add_field_method_get("value", |vm, this| match &this.kind {
            ConfigValue::Switch(bool) => bool.into_lua(vm),
            ConfigValue::Select(s) | ConfigValue::Text(s) => s.as_str().into_lua(vm),
        })
    }
}
