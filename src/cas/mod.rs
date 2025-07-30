use std::path::PathBuf;

use tracing::debug;

use crate::hash::Hasher;
use std::{io, path::Path};

pub mod default;

pub trait Store {
    type Hash: Hasher;
    type Err: std::fmt::Display + From<std::io::Error>;
    fn write(&mut self, file: Vec<u8>) -> Result<PathBuf, Self::Err>;
    fn create<P: AsRef<Path>>(&mut self, file: Vec<u8>, dst: P) -> Result<(), Self::Err> {
        let src = self.write(file)?;
        debug!(
            "Creating symlink {} -> {}",
            dst.as_ref().to_string_lossy(),
            src.to_string_lossy()
        );
        create_symlink(src, dst)?;
        Ok(())
    }
}

#[cfg(unix)]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    use std::fs;
    let src = src.as_ref();

    if src.is_dir() {
        std::os::windows::fs::symlink_dir(src, dst)
    } else {
        std::os::windows::fs::symlink_file(src, dst)
    }
}
