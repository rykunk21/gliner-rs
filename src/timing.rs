//! Lightweight timing utilities built on `tracing` spans.
//!
//! These helpers wrap `tracing::info_span!` and emit duration events
//! when the span exits, making it easy to add hierarchical timing to
//! the GLiNER2 pipeline without manual `Instant` bookkeeping.

use std::time::Instant;

/// A small RAII guard that logs elapsed time for a block of work.
pub struct TimedBlock {
    name: &'static str,
    start: Instant,
}

impl TimedBlock {
    #[inline]
    pub fn new(name: &'static str) -> Self {
        let start = Instant::now();
        tracing::info!(target: "gliner_rs::timing", "{name}: start");
        Self { name, start }
    }
}

impl Drop for TimedBlock {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        tracing::info!(
            target: "gliner_rs::timing",
            "{}: done in {:.2?}",
            self.name,
            elapsed
        );
    }
}

/// Shorthand to create a `TimedBlock`.
#[inline]
pub fn timed_block(name: &'static str) -> TimedBlock {
    TimedBlock::new(name)
}
