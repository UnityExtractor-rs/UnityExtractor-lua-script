use crate::script_libs::error::InitScriptError;
use mlua::{FromLua, Lua, Value};

#[derive(Debug, Clone)]
pub struct SelectItem {
    idx: String,
    text: String,
}

impl<'lua> FromLua<'lua> for SelectItem {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(tb) => {
                let idx = tb.get("idx")?;
                let text = tb.get("text")?;
                Ok(Self { idx, text })
            }
            _ => Err(mlua::Error::ToLuaConversionError {
                from: value.type_name(),
                to: "SelectItem",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserEditableConfigKind {
    Switch(bool),
    Select(Vec<SelectItem>, String),
    Text(String),
}

impl<'lua> FromLua<'lua> for UserEditableConfigKind {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(tb) => {
                let ty: String = tb.get("ty")?;
                let default: String = tb.get("default")?;
                let this = match ty.as_str() {
                    "switch" => {
                        let default = if default == "true" {
                            true
                        } else if default == "false" {
                            false
                        } else {
                            return Err(mlua::Error::external(
                                InitScriptError::UnexpectDefaultValue("bool", default),
                            ));
                        };
                        Self::Switch(default)
                    }
                    "select" => {
                        let selects = tb.get("selects")?;
                        let selects = <Vec<SelectItem>>::from_lua(selects, lua)?;

                        Self::Select(selects, default)
                    }
                    "text" => Self::Text(default),
                    _ => {
                        return Err(mlua::Error::external(InitScriptError::UnknownConfigType(
                            ty,
                        )));
                    }
                };

                Ok(this)
            }
            _ => Err(mlua::Error::ToLuaConversionError {
                from: value.type_name(),
                to: "UserEditableConfigKind",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserEditableConfig {
    idx: String,
    text: String,
    tip: Option<String>,
    kind: UserEditableConfigKind,
}

impl<'lua> FromLua<'lua> for UserEditableConfig {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(tb) => Ok(Self {
                idx: tb.get("idx")?,
                text: tb.get("text")?,
                tip: tb.get("tip")?,
                kind: tb.get("kind")?,
            }),
            _ => Err(mlua::Error::ToLuaConversionError {
                from: value.type_name(),
                to: "UserEditableConfig",
                message: None,
            }),
        }
    }
}
