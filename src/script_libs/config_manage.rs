use crate::script_libs::error::InitScriptError;
use mlua::{FromLua, Lua, Table, Value};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum UserEditableConfigKind {
    Switch(bool),
    Select(BTreeMap<String, String>, String),
    Text(String),
}

impl UserEditableConfigKind {
    pub(crate) fn ty(&self) -> &'static str {
        match self {
            UserEditableConfigKind::Switch(_) => "switch",
            UserEditableConfigKind::Select(_, _) => "select",
            UserEditableConfigKind::Text(_) => "text",
        }
    }
}

impl<'lua> FromLua<'lua> for UserEditableConfigKind {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let tb = Table::from_lua(value, lua)?;
        let ty: String = tb.get("ty")?;
        let default: String = tb.get("default")?;
        let this = match ty.as_str() {
            "switch" => {
                let default = default.parse::<bool>().map_err::<mlua::Error, _>(|_| {
                    InitScriptError::UnexpectDefaultValue("bool", default).into()
                })?;
                Self::Switch(default)
            }
            "select" => {
                let selects = tb.get("selects")?;
                let selects = <BTreeMap<String, String>>::from_lua(selects, lua)?;
                if !selects.contains_key(&default) {
                    return Err(InitScriptError::SelectTargetNotInRage(
                        default,
                        selects.keys().cloned().collect(),
                    )
                    .into());
                }
                Self::Select(selects, default)
            }
            "text" => Self::Text(default),
            _ => {
                return Err(InitScriptError::UnknownConfigType(ty).into());
            }
        };

        Ok(this)
    }
}

#[derive(Debug, Clone)]
pub struct UserEditableConfig {
    pub(crate) identity: Rc<str>,
    pub text: Rc<str>,
    pub tip: Option<Rc<str>>,
    pub(crate) kind: Rc<UserEditableConfigKind>,
}

impl<'lua> FromLua<'lua> for UserEditableConfig {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let tb = Table::from_lua(value, lua)?;
        Ok(Self {
            identity: tb
                .get::<_, String>("identity")
                .map(|s| Rc::from(s.into_boxed_str()))?,
            text: tb
                .get::<_, String>("text")
                .map(|s| Rc::from(s.into_boxed_str()))?,
            tip: tb
                .get::<_, Option<String>>("tip")?
                .map(|s| Rc::from(s.into_boxed_str())),
            kind: tb.get("kind").map(Rc::new)?,
        })
    }
}
