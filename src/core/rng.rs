//! Deterministic pseudo-random numbers for reproducible randomized algorithms.

/// Linear congruential generator (Knuth's MMIX constants). Not for cryptography; chosen
/// so that a seed reproduces a run exactly without depending on an RNG crate.
#[derive(Debug, Clone)]
pub(crate) struct LcgRng {
    state: u64,
}

impl LcgRng {
    pub(crate) fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Uniform in `[0, 1)`.
    pub(crate) fn next_f64(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let val = (self.state >> 11) as f64;
        val / (1u64 << 53) as f64
    }

    /// Uniform in `0..n`; `n` must be greater than zero.
    pub(crate) fn below(&mut self, n: usize) -> usize {
        ((self.next_f64() * n as f64) as usize).min(n - 1)
    }

    /// Fisher–Yates shuffle.
    pub(crate) fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            items.swap(i, self.below(i + 1));
        }
    }
}
