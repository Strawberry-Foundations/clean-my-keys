use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(enabled: bool) {
    VERBOSE.store(enabled, Ordering::Relaxed);
}

#[must_use]
pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

pub fn log(message: impl Display) {
    if is_verbose() {
        eprintln!("{message}");
    }
}