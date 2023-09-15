use thiserror::Error;

use crate::script_loader::error::UserConfigError;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct StorageError(Box<dyn std::error::Error + Send + Sync>);

impl StorageError {
    pub fn from<T: std::error::Error + Send + Sync + 'static>(err: T) -> Self {
        Self(Box::new(err))
    }
}

#[derive(Debug, Error)]
pub enum ScriptInnerError {
    #[error("user config error :{0}")]
    UserConfig(#[from] UserConfigError),
    #[error("Lua runtime error :{0}")]
    Lua(#[from] mlua::Error),
    #[error("storage error :{0}")]
    Storage(#[from] StorageError),
}
