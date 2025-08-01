use tracing::{debug, error, info, trace};

pub struct LogIndicator {
    filename: String,
    bytes: u64,
}
impl super::Indicator for LogIndicator {
    fn update(&mut self, bytes: u64) {
        trace!("[{}] {}/{}", self.filename, bytes, self.bytes);
    }
    fn stage(&mut self, stage: String) {
        debug!("[{}] {{STAGE}} -- {}", self.filename, stage);
    }
    fn error(&mut self, error: String) {
        error!("[{}] {}", self.filename, error);
    }
    fn end(&mut self) {
        info!("[{}] FINISHED", self.filename);
    }
}

pub struct LogFactory {}
impl super::IndicatorFactory for LogFactory {
    fn create(&mut self, filename: String, total_bytes: usize) -> Box<dyn super::Indicator + Send> {
        Box::new(LogIndicator {
            filename,
            bytes: total_bytes as u64,
        })
    }
}
impl LogFactory {
    pub fn new() -> Self {
        Self {}
    }
}
