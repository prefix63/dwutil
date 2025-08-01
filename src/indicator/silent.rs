pub struct SilentIndicator {}
impl super::Indicator for SilentIndicator {
    fn end(&mut self) {}
    fn event(&mut self, _: super::Event) {}
    fn error(&mut self, _: String) {}
    fn stage(&mut self, _: String) {}
    fn update(&mut self, _: u64) {}
}

pub struct SilentFactory {}
impl super::IndicatorFactory for SilentFactory {
    fn create(&mut self, _: String, _: usize) -> Box<dyn super::Indicator + Send> {
        Box::new(SilentIndicator {})
    }
}
impl SilentFactory {
    pub fn new() -> Self {
        Self {}
    }
}
