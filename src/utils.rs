use std::{fs, io, path::Path};

use std::sync::{Condvar, Mutex};
use tracing::warn;

// Semaphore utility for manage the current downloads
pub struct Semaphore {
    counter: Mutex<usize>,
    cvar: Condvar,
    max: usize,
}

impl Semaphore {
    /// Create a new semaphore with max current
    pub fn new(max: usize) -> Self {
        Self {
            counter: Mutex::new(0),
            cvar: Condvar::new(),
            max,
        }
    }

    /// Block the thread if is full the counter
    pub fn acquire(&self) {
        // Get the counter
        let mut count = self.counter.lock().unwrap();
        // Wait the counter
        while *count >= self.max {
            count = self.cvar.wait(count).unwrap();
        }
        *count += 1;
    }

    /// Release the thread
    pub fn release(&self) {
        let mut count = self.counter.lock().unwrap();
        *count -= 1;
        self.cvar.notify_one();
    }
}

/// Recursive copy a dir into other dir
pub fn rcopy<A: AsRef<Path>, B: AsRef<Path>>(src: A, dst: B) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            rcopy(&src_path, &dest_path)?;
        } else if file_type.is_file() {
            fs::copy(&src_path, &dest_path)?;
        } else if file_type.is_symlink() {
            let link = fs::read_link(&src_path)?;
            create_symlink(link, dest_path)?;
        } else {
            warn!("Unimplmented filetype copy attemp");
        }
    }

    Ok(())
}

#[cfg(unix)]
pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    use std::fs;
    let src = src.as_ref();

    if src.is_dir() {
        std::os::windows::fs::symlink_dir(src, dst)
    } else {
        std::os::windows::fs::symlink_file(src, dst)
    }
}
