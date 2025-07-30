use tracing::debug;

use super::Store;
use crate::hash::Hasher;
use std::{
    fs,
    hash::DefaultHasher,
    path::{Path, PathBuf},
};

pub struct DefaultStore {
    base: PathBuf,
}
impl DefaultStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            base: path.as_ref().to_path_buf(),
        }
    }
}
impl Store for DefaultStore {
    type Hash = DefaultHasher;
    type Err = std::io::Error;
    fn write(&mut self, file: Vec<u8>) -> Result<PathBuf, Self::Err> {
        let hash = Self::Hash::compute(&file)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let prefix = &hash[0..2];
        let parent = self.base.join(prefix);
        if !parent.exists() {
            fs::create_dir_all(&parent)?;
        }
        let path = parent.join(hash);
        debug!("Writing file in {}", path.to_string_lossy());
        fs::write(&path, file)?;
        Ok(path)
    }
}
