use std::{
    fs::{self},
    io::{Cursor, Read},
};

pub struct Gz {
    decoder: flate2::read::GzDecoder<Cursor<Vec<u8>>>,
}
impl super::Decoder for Gz {
    fn extract(&mut self, destination: std::path::PathBuf) -> Result<(), String> {
        let mut buf = Vec::new();
        self.decoder
            .read_to_end(&mut buf)
            .map_err(|e| e.to_string())?;
        fs::write(destination.join("target"), buf).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct GzFactory;
impl super::DecoderFactory for GzFactory {
    fn supports(file: &[u8]) -> bool {
        infer::is(file, "gz")
    }
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Box<dyn super::Decoder>> {
        let decoder = flate2::read::GzDecoder::new(Cursor::new(bytes));
        Ok(Box::new(Gz { decoder }))
    }
}
