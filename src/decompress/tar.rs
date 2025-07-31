use std::io::Cursor;

use crate::decompress::{Decoder, DecoderFactory};

pub struct Tar {
    archive: tar::Archive<Cursor<Vec<u8>>>,
}
impl Decoder for Tar {
    fn extract(&mut self, destination: std::path::PathBuf) -> Result<(), String> {
        self.archive
            .unpack(destination)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct TarFactory;
impl DecoderFactory for TarFactory {
    fn supports(file: &[u8]) -> bool {
        infer::is(file, "tar")
    }
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Box<dyn Decoder>> {
        let archive = tar::Archive::new(Cursor::new(bytes));
        Ok(Box::new(Tar { archive }))
    }
}
