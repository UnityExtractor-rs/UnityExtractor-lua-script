use crate::script_loader::config::UserEditableConfig;
use mlua::{FromLua, Function, Lua, Table, Value};

pub mod config;
pub mod error;
mod register;

#[derive(Debug, Clone)]
pub struct ScriptRegister<'lua> {
    identity: String,
    name: Option<String>,

    user_editable_config: Vec<UserEditableConfig>,
    config_update_fn_name: Option<Function<'lua>>,
    verify_applicable_fn_name: Function<'lua>,
}

impl<'lua> FromLua<'lua> for ScriptRegister<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let table = Table::from_lua(value, lua)?;
        Ok(ScriptRegister {
            identity: table.get("identity")?,
            name: table.get("name")?,
            user_editable_config: table.get("userConfig")?,
            config_update_fn_name: table.get("configUpdate")?,
            verify_applicable_fn_name: table.get("getApplicableScripts")?,
        })
    }
}
