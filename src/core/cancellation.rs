//! Search cancellation tokens for interrupting running algorithms asynchronously.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Thread-safe cancellation handle allowing external interruption of running search/optimization
/// algorithms. Cloning shares the same underlying flag — any clone observes a cancellation
/// triggered through any other clone.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Creates a new inactive cancellation token.
    ///
    /// Time & Space: O(1).
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Triggers cancellation, signaling all algorithms holding this token to abort search.
    ///
    /// Time complexity: O(1).
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Returns `true` if cancellation has been requested.
    ///
    /// Time complexity: O(1).
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}
