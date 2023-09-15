use crate::script::error::{ScriptInnerError, StorageError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Lua engine Error : {0}")]
    Lua(#[from] mlua::Error),
    #[error("Script Not load yet")]
    ScriptNotLoad,
    #[error("Storage Error : {0}")]
    Storage(#[from] StorageError),

    #[error("SubScript Not Exist : {0:?}")]
    SubScriptNotExist(String),

    #[error("Script Inner Error : {0}")]
    ScriptInner(#[from] ScriptInnerError),
}
