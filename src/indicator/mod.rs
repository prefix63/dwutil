pub mod pacman;

pub enum Event {
    Update(usize),
    End,
    Error(String),
    Stage(String),
}

pub trait IndicatorFactory {
    fn create(&mut self, filename: String) -> impl Indicator;
}

pub trait Indicator {
    fn event(event: Event);
    fn update(bytes: usize);
    fn error(error: String);
    fn stage(stage: String);
}
