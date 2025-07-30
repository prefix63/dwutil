#[cfg(feature = "indicatif")]
pub mod indicatif;

pub enum Event {
    Update(usize),
    End,
    Error(String),
    Stage(String),
}

pub trait IndicatorFactory {
    fn create(&mut self, filename: String, total_bytes: usize) -> Box<dyn Indicator + Send>;
}

pub trait Indicator {
    fn event(&mut self, event: Event) {
        match event {
            Event::Update(bytes) => self.update(bytes),
            Event::End => self.end(),
            Event::Error(error) => self.error(error),
            Event::Stage(stage) => self.stage(stage),
        }
    }
    fn update(&mut self, bytes: usize);
    fn error(&mut self, error: String);
    fn stage(&mut self, stage: String);
    fn end(&mut self);
}
