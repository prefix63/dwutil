use std::{fs, hash::DefaultHasher, path::Path};

use std::hash::Hasher as RustHasher;
use tracing::{debug, error, warn};

#[cfg(feature = "sha")]
pub mod sha;

#[cfg(feature = "md5")]
pub mod md5;

/// Common trait for the hashers
pub trait Hasher {
    /// Calculate a hash from bytes
    fn compute(bytes: &[u8]) -> Result<String, String>
    where
        Self: Sized;
}

impl Hasher for std::hash::DefaultHasher {
    fn compute(bytes: &[u8]) -> Result<String, String> {
        let mut hasher = DefaultHasher::new();
        hasher.write(bytes);
        let hash = hasher.finish();
        Ok(hex::encode(hash.to_be_bytes()))
    }
}

/// Hash check configuration
#[derive(Debug, Clone)]
pub struct Hash {
    /// Expected hash
    expect: String,
    /// Function that calculates the hash
    hasher: fn(&[u8]) -> Result<String, String>,
}
impl Hash {
    /// Creates a new configuration from the expected hash
    pub fn new<T: Hasher + 'static>(expect: &str) -> Self {
        Self {
            expect: expect.to_string(),
            hasher: T::compute,
        }
    }
    /// Check if the bytes matches with the expected hash
    /// Returns None if don't matches
    pub fn check_bytes(&self, bytes: Vec<u8>) -> Option<()> {
        let hash = (self.hasher)(&bytes);
        let hash = match hash {
            Result::Err(e) => {
                error!("Failed computing hash -- {e}");
                return None;
            }
            Result::Ok(hash) => hash,
        };
        debug!("HASH: {}", hash);
        if self.expect.ne(&hash) {
            warn!("Hash don't match");
            return None;
        }
        Some(())
    }
    /// Check if the file matches with the expected hash
    /// Returns None if don't matches
    pub fn check_file<P: AsRef<Path>>(&self, file: P) -> std::io::Result<Option<()>> {
        let bytes = fs::read(file.as_ref())?;
        Ok(self.check_bytes(bytes))
    }
}
