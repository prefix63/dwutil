#[cfg(feature = "indicatif")]
pub mod indicatif;

/// Logs all the events in tracing
///   Event - Level
/// - Update: Trace
/// - Stage: Debug
/// - Error: Error
/// - End: Info
pub mod log;
/// Don't print anything in the console
pub mod silent;

/// Event that update the indicator
pub enum Event {
    /// Update the position of the progress
    Update(u64),
    /// Finalize the download
    End,
    /// Error in the download
    Error(String),
    /// Change the stage of the download
    Stage(String),
}

/// Factory that creates an indicator
/// Implements: [Send]
pub trait IndicatorFactory: Send {
    /// Create a new indicator from the filename and total size
    fn create(&mut self, filename: String, total_bytes: usize) -> Box<dyn Indicator + Send>;
}

/// Indicator that process the events
pub trait Indicator {
    /// Main entrypoint of the events
    fn event(&mut self, event: Event) {
        match event {
            Event::Update(bytes) => self.update(bytes),
            Event::End => self.end(),
            Event::Error(error) => self.error(error),
            Event::Stage(stage) => self.stage(stage),
        }
    }
    /// Process the update event
    fn update(&mut self, bytes: u64);
    /// Process the error event
    fn error(&mut self, error: String);
    /// Process the stage event
    fn stage(&mut self, stage: String);
    /// Process the end event
    fn end(&mut self);
}
