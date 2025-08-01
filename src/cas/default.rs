use tracing::debug;

use super::Store;
use crate::hash::Hasher;
use std::{
    fs,
    hash::DefaultHasher,
    path::{Path, PathBuf},
};

/// Default implementation for store, use the [std::hash::DefaultHasher] for hashing
#[derive(Debug)]
pub struct DefaultStore {
    base: PathBuf,
}
impl DefaultStore {
    /// Create a new store with a base path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            base: path.as_ref().to_path_buf(),
        }
    }
}
impl Store for DefaultStore {
    fn write(&self, file: Vec<u8>) -> Result<PathBuf, String> {
        let hash = DefaultHasher::compute(&file)?;
        let prefix = &hash[0..2];
        let parent = self.base.join(prefix);
        if !parent.exists() {
            fs::create_dir_all(&parent).map_err(|e| e.to_string())?;
        }
        let path = parent.join(hash);
        debug!("Writing file in {}", path.to_string_lossy());
        fs::write(&path, file).map_err(|e| e.to_string())?;
        Ok(path)
    }
}
