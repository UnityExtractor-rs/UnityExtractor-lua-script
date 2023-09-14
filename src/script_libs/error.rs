#[derive(Debug, thiserror::Error)]
pub enum InitScriptError {
    #[error("script name has been set `{0:?}`")]
    ScriptNameHasSet(String),
    #[error("script name not set yet")]
    ScriptNameNotSet,
    #[error("unknown user editable config type {0:?}")]
    UnknownConfigType(String),
    #[error("unexpect default value, expect kind[{0}], get {1:?}")]
    UnexpectDefaultValue(&'static str, String),
}
