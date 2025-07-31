use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use tracing::warn;

#[cfg(feature = "zip")]
pub mod zip;

#[cfg(feature = "gz")]
pub mod gz;

#[cfg(feature = "tar")]
pub mod tar;

#[cfg(feature = "xz")]
pub mod xz;

#[cfg(feature = "targz")]
pub mod targz;

#[cfg(feature = "tarxz")]
pub mod tarxz;

pub trait DecoderFactory {
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Box<dyn Decoder>>;
    fn from_file<P: AsRef<Path>>(file: P) -> std::io::Result<Box<dyn Decoder>> {
        let file = fs::read(file)?;
        Self::from_bytes(file)
    }
    fn supports_file<P: AsRef<Path>>(file: P) -> bool {
        let bytes = fs::read(file);
        if let Err(err) = bytes {
            warn!("Error reading file in type checking: \n{}", err.to_string());
            return false;
        }
        Self::supports(&bytes.unwrap())
    }
    fn supports(file: &[u8]) -> bool;
}
pub trait Decoder {
    fn extract(&mut self, destination: PathBuf) -> Result<(), String>;
}
