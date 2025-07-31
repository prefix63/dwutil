use std::{
    fs::{self},
    io::{Cursor, Read},
};

use xz2::read::XzDecoder;

use crate::decompress::{Decoder, DecoderFactory};

pub struct Xz {
    decoder: xz2::read::XzDecoder<Cursor<Vec<u8>>>,
}
impl Decoder for Xz {
    fn extract(&mut self, destination: std::path::PathBuf) -> Result<(), String> {
        let mut buf = Vec::new();
        self.decoder
            .read_to_end(&mut buf)
            .map_err(|e| e.to_string())?;
        fs::write(destination, buf).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct XzFactory;
impl DecoderFactory for XzFactory {
    fn supports(file: &[u8]) -> bool {
        infer::is(file, "xz")
    }
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Box<dyn Decoder>> {
        let decoder = XzDecoder::new(Cursor::new(bytes));
        Ok(Box::new(Xz { decoder }))
    }
}
