use std::error::Error;
use std::fmt::{Debug, Formatter};

use std::rc::Rc;

pub type StorageResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub trait Storage {
    fn load(&self, script: &str, key: &str) -> StorageResult<Vec<u8>>;

    fn store(&self, script: &str, key: &str, value: &[u8]) -> StorageResult<()>;
}

pub type BoxedStorage = Rc<dyn Storage>;

#[derive(Clone)]
pub struct StorageManager(pub(super) BoxedStorage);

impl Storage for StorageManager {
    fn load(&self, script: &str, key: &str) -> StorageResult<Vec<u8>> {
        self.0.load(script, key)
    }

    fn store(&self, script: &str, key: &str, value: &[u8]) -> StorageResult<()> {
        self.0.store(script, key, value)
    }
}

impl Debug for StorageManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageManger").finish()
    }
}
