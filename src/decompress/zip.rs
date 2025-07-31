use std::io::{Cursor, ErrorKind};

use zip::ZipArchive;

use super::{Decoder, DecoderFactory};

pub struct ZipFactory;

impl DecoderFactory for ZipFactory {
    fn supports(file: &[u8]) -> bool {
        infer::is(&file, "zip")
    }
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Box<dyn super::Decoder>> {
        let archive = ZipArchive::new(Cursor::new(bytes))
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;
        Ok(Box::new(Zip { zip: archive }))
    }
}

pub struct Zip {
    zip: zip::ZipArchive<Cursor<Vec<u8>>>,
}
impl Decoder for Zip {
    fn extract(&mut self, destination: std::path::PathBuf) -> Result<(), String> {
        self.zip.extract(destination).map_err(|e| e.to_string())
    }
}
