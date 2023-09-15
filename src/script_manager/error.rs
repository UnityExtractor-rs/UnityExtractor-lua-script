use crate::script_libs::error::InitScriptError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Lua engine Error : {0}")]
    Lua(#[from] mlua::Error),
    #[error("Script Not load yet")]
    ScriptNotLoad,
    #[error("Storage Error : {0}")]
    Storage(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Init Script Error : {0}")]
    InitScript(#[from] InitScriptError),

    #[error("SubScript Not Exist : {0:?}")]
    SubScriptNotExist(String),
}
