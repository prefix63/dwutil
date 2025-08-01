use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use super::{Indicator, IndicatorFactory};

pub struct IndicatifFactory {
    multiprogress: MultiProgress,
    style: ProgressStyle,
}
impl IndicatifFactory {
    pub fn new() -> Self {
        let mut tick_frames = [
            "c o o o", "c o o o", "c o o o", "c o o o", "c o o o", "c o o o", "-c o o o",
            "-c o o o", "-c o o o", "-c o o o", "-c o o o", "-c o o o", "--Co o o", "--Co o o",
            "--Co o o", "--Co o o", "--Co o o", "--Co o o", "---c o o", "---c o o", "---c o o",
            "---c o o", "---c o o", "---c o o", "----Co o", "----Co o", "----Co o", "----Co o",
            "----Co o", "----Co o", "-----c o", "-----c o", "-----c o", "-----c o", "-----c o",
            "-----c o", "-----c o", "-----c o", "-----c o", "------Co", "------Co", "------Co",
            "------Co", "------Co", "------Co", "------Co", "------Co", "------Co", "-------c",
            "-------c", "-------c", "-------c", "-------c", "-------c",
        ];
        tick_frames.reverse();

        let style = ProgressStyle::with_template("{prefix:.30} {bytes:>8} {binary_bytes_per_sec:>10} {eta:>5} {wide_bar} {spinner} {percent:>3}%")
            .unwrap()
            .progress_chars("=> ")
            .tick_strings(&tick_frames);

        Self::with_style(style)
    }
    pub fn with_style(style: ProgressStyle) -> Self {
        IndicatifFactory {
            multiprogress: MultiProgress::new(),
            style,
        }
    }
}
impl IndicatorFactory for IndicatifFactory {
    fn create(&mut self, filename: String, total: usize) -> Box<dyn Indicator + Send> {
        let pb = ProgressBar::new(total as u64);
        let pb = self.multiprogress.add(pb);
        pb.set_style(self.style.clone());
        pb.set_prefix(filename);

        Box::new(SimpleIndicator { bar: pb })
    }
}

pub struct SimpleIndicator {
    bar: ProgressBar,
}
impl Indicator for SimpleIndicator {
    fn end(&mut self) {
        self.bar.finish();
    }
    fn error(&mut self, error: String) {
        self.bar.abandon_with_message(error);
    }
    fn stage(&mut self, stage: String) {
        self.bar.set_message(stage);
    }
    fn update(&mut self, bytes: u64) {
        self.bar.set_position(bytes);
    }
}
