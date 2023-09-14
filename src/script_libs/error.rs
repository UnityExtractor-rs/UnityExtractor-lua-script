#[derive(Debug, thiserror::Error)]
pub enum InitScriptError {
    #[error(transparent)]
    Storage(Box<dyn std::error::Error + Send + Sync>),
    #[error("script name has been set `{0:?}`")]
    ScriptNameHasSet(String),
    #[error("script name not set yet")]
    ScriptNameNotSet,
    #[error("unknown user editable config type {0:?}")]
    UnknownConfigType(String),
    #[error("unexpect default value, expect kind[{0}], get {1:?}")]
    UnexpectDefaultValue(&'static str, String),
    #[error("select a key {0:?} out of range {1:?}")]
    SelectTargetNotInRage(String, Vec<String>),
    #[error("expect type {0:?}, but get type {1:?}")]
    UserConfigTypeNotMatch(&'static str, &'static str),
}

impl Into<mlua::Error> for InitScriptError {
    fn into(self) -> mlua::Error {
        mlua::Error::external(self)
    }
}
