use std::{fmt::Debug, path::PathBuf};

use tracing::debug;

pub mod default;

/// Define a new store that manage a cas system
/// Implements:
///  - [Send]
///  - [Sync]
///  - [Debug]
pub trait Store: Send + Sync + Debug {
    /// Write a new file in hash filesystem and return the path
    fn write(&self, file: Vec<u8>) -> Result<PathBuf, String>;
    /// Write a new file and create a symlink to the original path
    fn create(&self, file: Vec<u8>, dst: PathBuf) -> Result<(), String> {
        let src = self.write(file)?;
        debug!(
            "Creating symlink {} -> {}",
            dst.to_string_lossy(),
            src.to_string_lossy()
        );
        crate::utils::create_symlink(src, dst).map_err(|e| e.to_string())?;
        Ok(())
    }
}
