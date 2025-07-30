use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use dwutil::indicator::{Indicator, IndicatorFactory, indicatif::IndicatifFactory};

use std::sync::Condvar;

struct Semaphore {
    max: usize,
    state: Mutex<usize>,
    cond: Condvar,
}

impl Semaphore {
    fn new(max: usize) -> Self {
        Self {
            max,
            state: Mutex::new(0),
            cond: Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut count = self.state.lock().unwrap();
        while *count >= self.max {
            count = self.cond.wait(count).unwrap();
        }
        *count += 1;
    }

    fn release(&self) {
        let mut count = self.state.lock().unwrap();
        *count -= 1;
        self.cond.notify_one();
    }
}

fn main() {
    let factory = IndicatifFactory::new();
    let factory = Arc::new(Mutex::new(factory));

    let semaphore = Arc::new(Semaphore::new(5));
    let mut handles = Vec::new();

    for i in (0..100).rev() {
        let size = 1024 * i;
        let factory = factory.clone();
        let semaphore = semaphore.clone();
        let handle = thread::spawn(move || {
            semaphore.acquire();
            let mut indicator = {
                let mut factory = factory.lock().unwrap();
                factory.create(format!("file-x{}.tar", i), size)
            };
            for i in 0..size {
                sleep(Duration::from_micros(1));
                indicator.event(dwutil::indicator::Event::Update(i));
            }
            semaphore.release();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
