use std::io::{Cursor, Read};

use flate2::read::GzDecoder;

use crate::decompress::{Decoder, DecoderFactory};

pub struct TarGz {
    decoder: flate2::read::GzDecoder<Cursor<Vec<u8>>>,
}
impl Decoder for TarGz {
    fn extract(&mut self, destination: std::path::PathBuf) -> Result<(), String> {
        let mut buf = Vec::new();
        self.decoder
            .read_to_end(&mut buf)
            .map_err(|e| e.to_string())?;
        let mut archive = tar::Archive::new(Cursor::new(buf));
        archive.unpack(destination).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct TarGzFactory;
impl DecoderFactory for TarGzFactory {
    fn supports(file: &[u8]) -> bool {
        if !infer::is(file, "gz") {
            return false;
        };
        let mut decoder = GzDecoder::new(Cursor::new(file));
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf).unwrap_or(0);
        infer::is(&buf, "tar")
    }
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Box<dyn Decoder>> {
        let decoder = GzDecoder::new(Cursor::new(bytes));
        Ok(Box::new(TarGz { decoder }))
    }
}
