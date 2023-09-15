use std::fmt::{Debug, Formatter};

use std::rc::Rc;

use super::error::StorageError;

pub type StorageResult<T> = Result<T, StorageError>;

pub trait Storage {
    type Error: std::error::Error + Send + Sync + 'static;

    fn contains_key(&self, script: &str, key: &str) -> Result<bool, Self::Error>;

    fn load(&self, script: &str, key: &str) -> Result<Option<Vec<u8>>, Self::Error>;

    fn store(&self, script: &str, key: &str, value: &[u8]) -> Result<(), Self::Error>;
}

pub trait StorageInner {
    fn contains_key(&self, script: &str, key: &str) -> StorageResult<bool>;

    fn load(&self, script: &str, key: &str) -> StorageResult<Option<Vec<u8>>>;

    fn store(&self, script: &str, key: &str, value: &[u8]) -> StorageResult<()>;
}

impl<T> StorageInner for T
where
    T: Storage,
{
    fn contains_key(&self, script: &str, key: &str) -> StorageResult<bool> {
        Storage::contains_key(self, script, key).map_err(StorageError::from)
    }

    fn load(&self, script: &str, key: &str) -> StorageResult<Option<Vec<u8>>> {
        Storage::load(self, script, key).map_err(StorageError::from)
    }

    fn store(&self, script: &str, key: &str, value: &[u8]) -> StorageResult<()> {
        Storage::store(self, script, key, value).map_err(StorageError::from)
    }
}

pub type BoxedStorage = Rc<dyn StorageInner>;

#[derive(Clone)]
pub struct StorageManager(pub(super) BoxedStorage);

impl Storage for StorageManager {
    fn contains_key(&self, script: &str, key: &str) -> StorageResult<bool> {
        self.0.contains_key(script, key)
    }

    fn load(&self, script: &str, key: &str) -> StorageResult<Option<Vec<u8>>> {
        self.0.load(script, key)
    }

    fn store(&self, script: &str, key: &str, value: &[u8]) -> StorageResult<()> {
        self.0.store(script, key, value)
    }

    type Error = StorageError;
}

impl Debug for StorageManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageManger").finish()
    }
}
