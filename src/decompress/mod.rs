use std::path::{Path, PathBuf};

pub enum ArchiveEntry {
    File(String),
    Directory(Box<Vec<ArchiveEntry>>),
    Symlink(String),
}
pub trait DecoderFactory {
    fn from_file(file: PathBuf) -> std::io::Result<Box<dyn Decoder>>;
    fn supports(file: &Path) -> bool;
}
pub trait Decoder {
    fn entries(&self) -> Vec<ArchiveEntry>;
    fn extract(&self, inner: String, destination: PathBuf);
}
