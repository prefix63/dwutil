use std::{
    fmt::Debug,
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use tempfile::{NamedTempFile, tempdir};
use tracing::error;

use crate::{decompress::DecoderFactory, indicator::IndicatorFactory, utils::Semaphore};

/// Content Addreseable Storage utilities and stores
pub mod cas;
/// Decompression utils, tar zip gz and xz support
pub mod decompress;
/// Hashing utilities, support sha*, md5 and default rust hasher
pub mod hash;
/// Indicators utilities, and default implementations for indicatif and tracing
pub mod indicator;
pub(crate) mod utils;

#[cfg(test)]
mod tests;

/// Define the size of the download buffer, default 1KB
pub const CHUNK_SIZE: usize = 1024;

/// Configuration for decompression function
#[derive(Debug, Clone)]
pub struct Decompression {
    /// Decoder function than create a decompressor
    decoder: fn(Vec<u8>) -> Result<Box<dyn crate::decompress::Decoder>, std::io::Error>,
    /// Decompression destination path
    dst: PathBuf,
    /// Exclude files or folders here
    exclude: Vec<String>,
}
impl Decompression {
    /// Create a new configuration
    pub fn new<T: DecoderFactory>() -> Self {
        Self {
            decoder: T::from_bytes,
            dst: PathBuf::new(),
            exclude: Vec::new(),
        }
    }
    /// Sets the destination path
    pub fn with_dst<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.dst = path.as_ref().to_path_buf();
        self
    }
    /// Add exclude file/path
    pub fn with_exclude<S: ToString>(mut self, str: S) -> Self {
        self.exclude.push(str.to_string());
        self
    }
    /// Sets the exclude list
    pub fn with_excludes<S: ToString>(mut self, strs: Vec<S>) -> Self {
        self.exclude = strs.iter().map(|e| e.to_string()).collect();
        self
    }
    /// Function to extract a file with bytes
    pub fn extract(self, bytes: Vec<u8>) -> Result<(), String> {
        let dir = tempdir().map_err(|e| e.to_string())?;
        let mut decoder = (self.decoder)(bytes).map_err(|e| e.to_string())?;
        decoder.extract(dir.path().to_path_buf())?;
        for entry in dir.path().read_dir().map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path().to_string_lossy().to_string();
            let path = path.replace(&dir.path().to_string_lossy().to_string(), "");
            if !self.exclude.contains(&path) {
                utils::rcopy(&entry.path(), &self.dst).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
    /// Function to extract a file with path
    pub fn extract_file<P: AsRef<Path>>(self, file: P) -> Result<(), String> {
        let bytes = fs::read(file).map_err(|e| e.to_string())?;
        self.extract(bytes)
    }
}

/// Define a downloadable file data
#[derive(Debug, Clone)]
pub struct File {
    /// Url of the asset
    pub url: String,
    /// Path to download
    pub path: PathBuf,
    /// Size of the file
    pub size: u64,
    /// Hash of the file
    hash: Option<crate::hash::Hash>,
    /// CAS store
    store: Option<Box<Arc<dyn crate::cas::Store + 'static>>>,
    /// Decompression config
    decompression: Option<Decompression>,
}
impl File {
    /// Create a new file
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            path: PathBuf::new(),
            size: 0,
            hash: None,
            store: None,
            decompression: None,
        }
    }
    /// Sets the file path
    pub fn with_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.path = path.as_ref().to_path_buf();
        self
    }
    /// Sets the file size
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }
    /// Sets the file hash
    pub fn with_hash(mut self, hash: crate::hash::Hash) -> Self {
        self.hash = Some(hash);
        self
    }
    /// Sets the cas store
    pub fn with_store<T: crate::cas::Store + 'static>(mut self, store: Arc<T>) -> Self {
        self.store = Some(Box::new(store));
        self
    }
    /// Sets the decompression config
    pub fn with_decompression(mut self, decompression: Decompression) -> Self {
        self.decompression = Some(decompression);
        self
    }
    pub(crate) fn download(
        self,
        agent: Arc<ureq::Agent>,
        indicator: &mut Box<dyn crate::indicator::Indicator + Send>,
    ) -> Result<(), String> {
        if self.path.eq(&PathBuf::new()) {
            error!("Please, define the path in the file: {}", self.url);
            return Err("Undefined Path".to_string());
        }

        // Use the agent to make a get request to the file url
        let request = agent.get(&self.url).call().map_err(|e| e.to_string())?;
        // If the response is not ok, comunicateit in the indicator
        if request.status() != 200 {
            let error = request.status_text().to_string();
            indicator.event(indicator::Event::Error(error.clone()));
            return Err(format!("HTTP ERROR: {}", error));
        }
        // Current downloaded bytes
        let mut current = 0u64;
        // Calculate the file path (if is a the file has a store, return temp file)
        let path = self.path();
        // Create the reader from the request and the writer in the file path
        let mut reader = request.into_reader();
        let mut writer = BufWriter::new(std::fs::File::create(&path).map_err(|e| e.to_string())?);
        // If the file does't has size, use the maximum to download all the request
        let size = if self.size == 0 { u64::MAX } else { self.size };
        while current < size {
            // Create the chunk buffer
            let mut buffer = [0u8; CHUNK_SIZE];
            // Read a chunk of the request in the buffer
            let size = reader.read(&mut buffer).map_err(|e| e.to_string())?;
            // If the reader does't has read nothing, the download finishes
            if size == 0 {
                break;
            }
            // Add the chunk bytes to the downloaded bytes
            current += size as u64;
            // Get the writen buffer data
            let buffer = &buffer[0..size];
            // Write the data in the file
            writer.write(buffer).map_err(|e| e.to_string())?;
            // Update the indicator
            indicator.event(indicator::Event::Update(current));
        }
        // Flush the writer to make sure that the data was entered correctly
        writer.flush().map_err(|e| e.to_string())?;

        // check the file hash
        if let Some(hash) = self.hash {
            let check = hash.check_file(&path).map_err(|e| e.to_string())?;
            if matches!(check, None) {
                return Err("Hashes don't matches".to_string());
            }
        }
        // process the store
        if let Some(store) = self.store {
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            fs::remove_file(&path).map_err(|e| e.to_string())?;
            store.create(bytes, self.path)?;
        }
        // decompress the file
        if let Some(decompression) = self.decompression {
            indicator.event(indicator::Event::Stage(String::from("Extracting...")));
            decompression.extract_file(&path)?;
        }
        indicator.event(indicator::Event::End);
        Ok(())
    }
    fn path(&self) -> PathBuf {
        if self.store.is_some() {
            let path = NamedTempFile::new().unwrap();
            return path.path().to_path_buf();
        }
        self.path.clone()
    }
}

pub struct Downloader {
    indicator: Box<dyn IndicatorFactory + Send + Sync>,
    files: Vec<File>,
    max_current_downloads: usize,
    agent: Arc<ureq::Agent>,
}
impl Downloader {
    pub fn new<T: IndicatorFactory + Sync + Send + 'static>(indicator: T) -> Self {
        Self {
            indicator: Box::new(indicator),
            files: Vec::new(),
            max_current_downloads: 5,
            agent: Arc::new(ureq::agent()),
        }
    }
    pub fn with_ureq_agent(mut self, agent: ureq::Agent) -> Self {
        self.agent = Arc::new(agent);
        self
    }
    pub fn with_max_current_downloads(mut self, max_current_downloads: usize) -> Self {
        self.max_current_downloads = max_current_downloads;
        self
    }
    pub fn with_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }
    pub fn with_files(mut self, files: Vec<File>) -> Self {
        self.files = files;
        self
    }
    pub fn with_indicator<T: IndicatorFactory + Send + Sync + 'static>(
        mut self,
        indicator: T,
    ) -> Self {
        self.indicator = Box::new(indicator);
        self
    }
    pub fn start(self) -> Result<(), String> {
        let mut handles = Vec::new();
        let semaphore = Arc::new(Semaphore::new(self.max_current_downloads));
        let factory = Arc::new(Mutex::new(self.indicator));
        let agent = self.agent;
        for file in self.files {
            let semaphore = semaphore.clone();
            let factory = factory.clone();
            let agent = agent.clone();
            handles.push(thread::spawn(move || {
                let factory = factory.clone();
                let semaphore = semaphore.clone();
                let agent = agent.clone();
                semaphore.acquire();
                let mut indicator = {
                    let mut fac = factory.lock().unwrap();
                    fac.create(
                        file.path.file_stem().unwrap().to_string_lossy().to_string(),
                        file.size as usize,
                    )
                };
                let err = file.download(agent, &mut indicator);
                if let Err(err) = err {
                    indicator.event(indicator::Event::Error(err));
                }
                semaphore.release();
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }
}
