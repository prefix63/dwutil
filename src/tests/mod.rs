use std::sync::Once;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg(test)]
mod hash;

#[cfg(test)]
mod cas;

static INIT: Once = Once::new();

fn init_tracing() {
    INIT.call_once(|| {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE) // o "info", etc.
            .with_test_writer() // para que se vea en output de cargo test
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");
    });
}
