//! # ternary-ear
//!
//! Listening and pattern recognition across a ternary agent fleet. Agents
//! perceive each other's behavior and learn from it using {-1, 0, +1}
//! signal streams.

#![forbid(unsafe_code)]

/// A recognized pattern in a ternary signal stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    /// The ternary values that form this pattern.
    pub values: Vec<i8>,
    /// How many times this pattern was observed.
    pub observations: u32,
    /// Last tick when this pattern was seen.
    pub last_seen: u64,
}

impl Pattern {
    pub fn new(values: Vec<i8>) -> Self {
        Self { values, observations: 0, last_seen: 0 }
    }

    /// Record an observation at the given tick.
    pub fn observe(&mut self, tick: u64) {
        self.observations += 1;
        self.last_seen = tick;
    }

    /// Length of the pattern.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Frequency score: observations relative to a baseline.
    pub fn frequency(&self, baseline: u32) -> f64 {
        if baseline == 0 { return 0.0; }
        self.observations as f64 / baseline as f64
    }
}

/// The ear — a pattern recognizer for ternary signal streams.
#[derive(Debug, Clone)]
pub struct Ear {
    /// Patterns recognized so far.
    pub patterns: Vec<Pattern>,
    /// Incoming signal buffer.
    pub buffer: Vec<i8>,
    /// Maximum pattern length to detect.
    pub max_pattern_len: usize,
    /// Current tick counter.
    pub tick: u64,
}

impl Ear {
    pub fn new(max_pattern_len: usize) -> Self {
        Self { patterns: Vec::new(), buffer: Vec::new(), max_pattern_len, tick: 0 }
    }

    /// Listen to one ternary value.
    pub fn listen(&mut self, value: i8) {
        let v = value.clamp(-1, 1);
        self.buffer.push(v);
        self.tick += 1;

        // Try to match existing patterns at the end of buffer
        for pattern in &mut self.patterns {
            let plen = pattern.values.len();
            if plen > self.buffer.len() { continue; }
            let start = self.buffer.len() - plen;
            if &self.buffer[start..] == &pattern.values[..] {
                pattern.observe(self.tick);
            }
        }
    }

    /// Listen to a slice of values.
    pub fn listen_all(&mut self, values: &[i8]) {
        for &v in values {
            self.listen(v);
        }
    }

    /// Register a pattern to watch for.
    pub fn watch(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);
    }

    /// Detect all recurring subsequences of lengths 2..=max_pattern_len.
    pub fn detect_recurring(&mut self, min_observations: u32) -> Vec<Pattern> {
        let mut found: Vec<Pattern> = Vec::new();
        for len in 2..=self.max_pattern_len.min(self.buffer.len()) {
            let mut i = 0;
            while i + len <= self.buffer.len() {
                let candidate: Vec<i8> = self.buffer[i..i + len].to_vec();
                let count = count_subsequence(&self.buffer, &candidate);
                if count >= min_observations {
                    if !found.iter().any(|p| p.values == candidate) {
                        let mut p = Pattern::new(candidate);
                        p.observations = count;
                        p.last_seen = self.tick;
                        found.push(p);
                    }
                }
                i += 1;
            }
        }
        found
    }

    /// Clear the buffer, keeping patterns.
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    /// Total ticks observed.
    pub fn total_ticks(&self) -> u64 {
        self.tick
    }
}

/// Detects the frequency of specific ternary values in a stream.
#[derive(Debug, Clone)]
pub struct FrequencyDetector {
    /// Counts for each ternary value.
    pub counts: [u64; 3], // [-1, 0, +1] mapped to indices 0, 1, 2
    /// Total values observed.
    pub total: u64,
}

impl FrequencyDetector {
    pub fn new() -> Self {
        Self { counts: [0, 0, 0], total: 0 }
    }

    fn idx(v: i8) -> usize {
        (v + 1) as usize
    }

    /// Observe one ternary value.
    pub fn observe(&mut self, value: i8) {
        let v = value.clamp(-1, 1);
        self.counts[Self::idx(v)] += 1;
        self.total += 1;
    }

    /// Frequency of a value (0.0..1.0).
    pub fn frequency(&self, value: i8) -> f64 {
        if self.total == 0 { return 0.0; }
        self.counts[Self::idx(value)] as f64 / self.total as f64
    }

    /// Dominant value.
    pub fn dominant(&self) -> i8 {
        let max_idx = self.counts.iter().enumerate()
            .max_by_key(|&(_, c)| c)
            .map(|(i, _)| i)
            .unwrap_or(1);
        max_idx as i8 - 1
    }

    /// Is the distribution uniform (all within 10% of each other)?
    pub fn is_uniform(&self) -> bool {
        if self.total == 0 { return true; }
        let expected = self.total as f64 / 3.0;
        self.counts.iter().all(|&c| (c as f64 - expected).abs() / expected < 0.10)
    }

    /// Entropy of the distribution.
    pub fn entropy(&self) -> f64 {
        if self.total == 0 { return 0.0; }
        let mut ent = 0.0;
        for &c in &self.counts {
            if c > 0 {
                let p = c as f64 / self.total as f64;
                ent -= p * p.log2();
            }
        }
        ent
    }
}

/// Analyzes relationships between multiple ternary signal patterns.
#[derive(Debug, Clone)]
pub struct HarmonicAnalyzer {
    /// Pairs of pattern indices that are harmonically related.
    pub relationships: Vec<(usize, usize, i8)>, // (idx_a, idx_b, relationship: -1, 0, +1)
}

impl HarmonicAnalyzer {
    pub fn new() -> Self {
        Self { relationships: Vec::new() }
    }

    /// Check if two sequences are inverses of each other.
    pub fn are_inverse(a: &[i8], b: &[i8]) -> bool {
        if a.len() != b.len() { return false; }
        a.iter().zip(b.iter()).all(|(&x, &y)| x == -y)
    }

    /// Check if two sequences are shifted versions of each other.
    pub fn are_shifted(a: &[i8], b: &[i8], shift: usize) -> bool {
        if a.len() != b.len() || shift >= a.len() { return false; }
        for i in 0..a.len() {
            if a[i] != b[(i + shift) % b.len()] { return false; }
        }
        true
    }

    /// Compute correlation between two sequences (-1.0..1.0).
    pub fn correlation(a: &[i8], b: &[i8]) -> f64 {
        if a.len() != b.len() || a.is_empty() { return 0.0; }
        let n = a.len() as f64;
        let product: f64 = a.iter().zip(b.iter()).map(|(&x, &y)| (x * y) as f64).sum();
        product / n
    }

    /// Add a discovered relationship.
    pub fn add_relationship(&mut self, a: usize, b: usize, rel: i8) {
        self.relationships.push((a, b, rel));
    }

    /// Find all patterns correlated with a given index above threshold.
    pub fn find_correlated(&self, idx: usize) -> Vec<usize> {
        self.relationships.iter()
            .filter(|&&(a, b, _)| a == idx || b == idx)
            .map(|&(a, b, _)| if a == idx { b } else { a })
            .collect()
    }
}

/// Detects temporal patterns — rhythms in ternary streams.
#[derive(Debug, Clone)]
pub struct RhythmTracker {
    /// Buffer of observed values.
    pub buffer: Vec<i8>,
    /// Detected period (0 means none detected).
    pub period: usize,
}

impl RhythmTracker {
    pub fn new() -> Self {
        Self { buffer: Vec::new(), period: 0 }
    }

    /// Observe a value.
    pub fn observe(&mut self, value: i8) {
        let v = value.clamp(-1, 1);
        self.buffer.push(v);
    }

    /// Attempt to detect the period using autocorrelation.
    pub fn detect_period(&mut self) -> usize {
        if self.buffer.len() < 4 {
            self.period = 0;
            return 0;
        }

        let max_period = self.buffer.len() / 2;
        let mut best_period = 0;
        let mut best_corr = 0.0;

        for p in 2..=max_period {
            let a = &self.buffer[..self.buffer.len() - p];
            let b = &self.buffer[p..];
            let corr = HarmonicAnalyzer::correlation(a, b);
            if corr > best_corr {
                best_corr = corr;
                best_period = p;
            }
        }

        // Only accept if correlation is strong enough
        if best_corr > 0.5 {
            self.period = best_period;
        } else {
            self.period = 0;
        }
        self.period
    }

    /// Predict the next value based on detected period.
    pub fn predict_next(&self) -> Option<i8> {
        if self.period == 0 || self.buffer.len() < self.period {
            return None;
        }
        let idx = self.buffer.len() % self.period;
        Some(self.buffer[idx])
    }

    /// Number of observations.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

/// Improves recognition accuracy over time by tracking hit/miss rates.
#[derive(Debug, Clone)]
pub struct EarTraining {
    /// Correct predictions.
    pub hits: u64,
    /// Incorrect predictions.
    pub misses: u64,
    /// Current accuracy (0.0..1.0).
    pub accuracy: f64,
}

impl EarTraining {
    pub fn new() -> Self {
        Self { hits: 0, misses: 0, accuracy: 0.0 }
    }

    /// Record a prediction result.
    pub fn record(&mut self, correct: bool) {
        if correct { self.hits += 1; } else { self.misses += 1; }
        let total = self.hits + self.misses;
        if total > 0 {
            self.accuracy = self.hits as f64 / total as f64;
        }
    }

    /// Total predictions made.
    pub fn total(&self) -> u64 {
        self.hits + self.misses
    }

    /// Is the training mature enough (at least 20 predictions)?
    pub fn is_mature(&self) -> bool {
        self.total() >= 20
    }
}

/// Stores recognized patterns for later retrieval.
#[derive(Debug, Clone)]
pub struct EarMemory {
    /// Stored patterns with timestamps.
    pub memories: Vec<(Pattern, u64)>, // (pattern, stored_at_tick)
    /// Maximum memories to retain.
    pub capacity: usize,
}

impl EarMemory {
    pub fn new(capacity: usize) -> Self {
        Self { memories: Vec::new(), capacity }
    }

    /// Store a pattern.
    pub fn store(&mut self, pattern: Pattern, tick: u64) {
        if self.memories.len() >= self.capacity {
            // Remove oldest
            self.memories.remove(0);
        }
        self.memories.push((pattern, tick));
    }

    /// Retrieve a pattern by exact value match.
    pub fn retrieve(&self, values: &[i8]) -> Option<&Pattern> {
        self.memories.iter().find(|(p, _)| p.values == values).map(|(p, _)| p)
    }

    /// Find patterns observed at least N times.
    pub fn find_frequent(&self, min_observations: u32) -> Vec<&Pattern> {
        self.memories.iter()
            .filter(|(p, _)| p.observations >= min_observations)
            .map(|(p, _)| p)
            .collect()
    }

    /// Number of stored memories.
    pub fn len(&self) -> usize {
        self.memories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.memories.is_empty()
    }
}

/// Count how many times a subsequence appears in a buffer (overlapping allowed).
fn count_subsequence(buffer: &[i8], sub: &[i8]) -> u32 {
    if sub.is_empty() || sub.len() > buffer.len() { return 0; }
    let mut count = 0u32;
    for i in 0..=buffer.len() - sub.len() {
        if &buffer[i..i + sub.len()] == sub {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_observe() {
        let mut p = Pattern::new(vec![1, 0, -1]);
        p.observe(10);
        p.observe(20);
        assert_eq!(p.observations, 2);
        assert_eq!(p.last_seen, 20);
    }

    #[test]
    fn pattern_frequency() {
        let p = Pattern { values: vec![1], observations: 5, last_seen: 0 };
        assert!((p.frequency(10) - 0.5).abs() < 0.001);
    }

    #[test]
    fn ear_listen_and_match() {
        let mut ear = Ear::new(3);
        ear.watch(Pattern::new(vec![1, -1]));
        ear.listen_all(&[1, -1, 0, 1, -1]);
        // Pattern [1, -1] should have been observed at least twice
        assert!(ear.patterns[0].observations >= 2);
    }

    #[test]
    fn ear_detect_recurring() {
        let mut ear = Ear::new(3);
        ear.listen_all(&[1, 1, 0, 1, 1, 0, 1, 1]);
        let found = ear.detect_recurring(2);
        assert!(!found.is_empty());
        // [1, 1] should be detected
        assert!(found.iter().any(|p| p.values == vec![1, 1]));
    }

    #[test]
    fn ear_total_ticks() {
        let mut ear = Ear::new(2);
        ear.listen_all(&[1, 0, -1]);
        assert_eq!(ear.total_ticks(), 3);
    }

    #[test]
    fn frequency_detector_basic() {
        let mut fd = FrequencyDetector::new();
        fd.observe(1);
        fd.observe(1);
        fd.observe(-1);
        assert_eq!(fd.total, 3);
        assert!((fd.frequency(1) - 0.6667).abs() < 0.01);
        assert_eq!(fd.dominant(), 1);
    }

    #[test]
    fn frequency_detector_uniform() {
        let mut fd = FrequencyDetector::new();
        for _ in 0..10 { fd.observe(-1); fd.observe(0); fd.observe(1); }
        assert!(fd.is_uniform());
    }

    #[test]
    fn frequency_detector_entropy() {
        let mut fd = FrequencyDetector::new();
        for _ in 0..33 { fd.observe(-1); fd.observe(0); fd.observe(1); }
        let e = fd.entropy();
        assert!(e > 1.5); // near max entropy for 3 values = log2(3) ≈ 1.585
    }

    #[test]
    fn harmonic_analyzer_inverse() {
        assert!(HarmonicAnalyzer::are_inverse(&[1, 0, -1], &[-1, 0, 1]));
        assert!(!HarmonicAnalyzer::are_inverse(&[1, 0, -1], &[1, 0, -1]));
    }

    #[test]
    fn harmonic_analyzer_shifted() {
        // [1, 0, -1] shifted by 1 => compare [1, 0, -1][i] with [0, -1, 1][(i+1)%3]
        // i=0: 1 vs b[1]=-1 FAIL
        // So this is NOT shifted by 1. Let's test with a correct case:
        // [1, 0, -1] shifted by 0 means a[i] == b[(i+0)%3] => must be identical
        assert!(HarmonicAnalyzer::are_shifted(&[1, 0, -1], &[1, 0, -1], 0));
        // [1, 0, -1] shifted by 2: a[i] == b[(i+2)%3]
        // i=0: 1 vs b[2]=-1 FAIL => not shifted by 2
        // Try: [1, 0, -1] rotated by 1 = [0, -1, 1]
        // shift=1 means a[i] == b[(i+1)%3]: a[0]=1 vs b[1]=-1 FAIL
        // Actually shift means b is a rotated by `shift` positions
        // Let me fix the test to match the actual implementation:
        // are_shifted checks a[i] == b[(i+shift)%len]
        // So for shift=0, a==b
        // For [1,0,-1] and [-1,1,0] with shift=1:
        //   i=0: a[0]=1 vs b[1]=1 OK
        //   i=1: a[1]=0 vs b[2]=0 OK  
        //   i=2: a[2]=-1 vs b[0]=-1 OK
        assert!(HarmonicAnalyzer::are_shifted(&[1, 0, -1], &[-1, 1, 0], 1));
        assert!(!HarmonicAnalyzer::are_shifted(&[1, 0, -1], &[0, 0, 0], 1));
    }

    #[test]
    fn harmonic_analyzer_correlation() {
        let c = HarmonicAnalyzer::correlation(&[1, 1, 1], &[1, 1, 1]);
        assert!((c - 1.0).abs() < 0.001);
        let c2 = HarmonicAnalyzer::correlation(&[1, 1, 1], &[-1, -1, -1]);
        assert!((c2 - (-1.0)).abs() < 0.001);
        let c3 = HarmonicAnalyzer::correlation(&[1, -1], &[-1, 1]);
        assert!((c3 - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn rhythm_tracker_period() {
        let mut rt = RhythmTracker::new();
        // Perfect period-2 pattern
        for _ in 0..6 { rt.observe(1); rt.observe(-1); }
        let p = rt.detect_period();
        assert_eq!(p, 2);
    }

    #[test]
    fn rhythm_tracker_predict() {
        let mut rt = RhythmTracker::new();
        for _ in 0..6 { rt.observe(1); rt.observe(-1); }
        rt.detect_period();
        assert_eq!(rt.predict_next(), Some(1));
    }

    #[test]
    fn ear_training_accuracy() {
        let mut et = EarTraining::new();
        for _ in 0..8 { et.record(true); }
        for _ in 0..2 { et.record(false); }
        assert_eq!(et.total(), 10);
        assert!((et.accuracy - 0.8).abs() < 0.001);
    }

    #[test]
    fn ear_training_maturity() {
        let mut et = EarTraining::new();
        assert!(!et.is_mature());
        for _ in 0..20 { et.record(true); }
        assert!(et.is_mature());
    }

    #[test]
    fn ear_memory_store_and_retrieve() {
        let mut mem = EarMemory::new(10);
        let p = Pattern { values: vec![1, -1, 0], observations: 3, last_seen: 100 };
        mem.store(p, 100);
        let retrieved = mem.retrieve(&[1, -1, 0]);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().observations, 3);
    }

    #[test]
    fn ear_memory_capacity() {
        let mut mem = EarMemory::new(2);
        mem.store(Pattern::new(vec![1]), 1);
        mem.store(Pattern::new(vec![0]), 2);
        mem.store(Pattern::new(vec![-1]), 3); // should evict oldest
        assert_eq!(mem.len(), 2);
        assert!(mem.retrieve(&[1]).is_none()); // evicted
    }

    #[test]
    fn ear_memory_frequent() {
        let mut mem = EarMemory::new(10);
        mem.store(Pattern { values: vec![1], observations: 5, last_seen: 0 }, 0);
        mem.store(Pattern { values: vec![0], observations: 1, last_seen: 0 }, 0);
        let freq = mem.find_frequent(3);
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn harmonic_analyzer_find_correlated() {
        let mut ha = HarmonicAnalyzer::new();
        ha.add_relationship(0, 1, 1);
        ha.add_relationship(0, 2, -1);
        let correlated = ha.find_correlated(0);
        assert_eq!(correlated.len(), 2);
    }

    #[test]
    fn ear_clamps_values() {
        let mut ear = Ear::new(2);
        ear.listen(5);
        ear.listen(-3);
        assert_eq!(ear.buffer, vec![1, -1]);
    }
}
