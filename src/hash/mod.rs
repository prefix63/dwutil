use std::{fs, hash::DefaultHasher, path::Path};

use std::hash::Hasher as RustHasher;
use tracing::{debug, error, warn};

#[cfg(feature = "sha")]
pub mod sha;

#[cfg(feature = "md5")]
pub mod md5;

pub trait Hasher {
    type Err: std::fmt::Display;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err>;
}

impl Hasher for std::hash::DefaultHasher {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let mut hasher = DefaultHasher::new();
        hasher.write(bytes);
        let hash = hasher.finish();
        Ok(hex::encode(hash.to_be_bytes()))
    }
}

pub struct Hash<H: Hasher> {
    expect: String,
    __hasher: std::marker::PhantomData<H>,
}
impl<H: Hasher> Hash<H> {
    pub fn new(expect: String) -> Self {
        Self {
            expect,
            __hasher: Default::default(),
        }
    }
    pub fn check_bytes(&self, bytes: Vec<u8>) -> Option<()> {
        let hash = H::compute(&bytes);
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
    pub fn check_file<P: AsRef<Path>>(&self, file: P) -> std::io::Result<Option<()>> {
        let bytes = fs::read(file.as_ref())?;
        Ok(self.check_bytes(bytes))
    }
}
