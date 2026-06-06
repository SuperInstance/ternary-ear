# ternary-ear

Listening and pattern recognition across ternary signal streams — pattern detection, frequency analysis, harmonic relationships, rhythmic period tracking, ear training, and associative memory for {-1, 0, +1} perception.

## Background

The ear is the most sophisticated pattern recognition system in the human body. From a continuous stream of air pressure variations, it extracts pitch, rhythm, timbre, and spatial information simultaneously. It recognizes a melody after hearing it once, detects a wrong note in a familiar piece, and can track multiple independent voices in a complex texture.

`ternary-ear` brings these capabilities to ternary signal streams. It listens to sequences of {-1, 0, +1} values, detects recurring patterns, measures frequency distributions, discovers harmonic relationships between signals, tracks rhythmic periodicity, improves its accuracy over time through training, and stores recognized patterns in associative memory.

The crate models several distinct aspects of auditory perception: the cochlea (frequency detection), the auditory cortex (pattern matching and harmonic analysis), the cerebellum (rhythmic entrainment), and memory (pattern storage and recall). Each is implemented as a separate component that can operate independently or compose together.

## How It Works

### Ear (Pattern Recognizer)

The central `Ear` struct buffers incoming ternary signals and matches them against registered patterns:

1. **`listen(value)`** — add one ternary value to the buffer, increment tick, check all watched patterns at the buffer's tail
2. **`watch(pattern)`** — register a pattern to watch for
3. **`detect_recurring(min_observations)`** — brute-force scan the buffer for all subsequences of length 2..=max_pattern_len that appear at least `min_observations` times

Pattern matching is exact: a pattern [1, -1, 0] matches only when the buffer ends with exactly those three values in order.

### FrequencyDetector (Statistical Analysis)

Tracks the distribution of ternary values over time:

- **`frequency(value)`** — proportion of observations that were this value
- **`dominant()`** — most frequently observed value
- **`is_uniform()`** — whether all three values appear within 10% of each other
- **`entropy()`** — Shannon entropy of the distribution (0.0 to log₂(3) ≈ 1.585)

A uniform ternary stream has maximum entropy (1.585 bits per symbol). A stream of all zeros has entropy 0.

### HarmonicAnalyzer (Cross-Signal Relationships)

Analyzes relationships between multiple ternary signal patterns:

- **`are_inverse(a, b)`** — whether sequence b is the sign-flipped version of a
- **`are_shifted(a, b, shift)`** — whether b is a rotation of a by `shift` positions
- **`correlation(a, b)`** — normalized dot product of two sequences (−1.0 to +1.0)
- **`find_correlated(idx)`** — discover all patterns related to a given pattern

Correlation of +1.0 means identical signals. −1.0 means perfectly inverse. 0.0 means uncorrelated.

### RhythmTracker (Period Detection)

Detects periodic patterns in ternary streams using autocorrelation:

1. Buffer incoming values
2. For each candidate period p from 2 to half the buffer length, compute the correlation between the buffer and itself shifted by p
3. Accept the period with highest correlation if it exceeds 0.5
4. Use the detected period to predict the next value

### EarTraining (Accuracy Tracking)

A feedback mechanism that tracks prediction accuracy over time:

- **`record(correct)`** — log whether a prediction was correct
- **`accuracy`** — running hit rate
- **`is_mature()`** — whether at least 20 predictions have been made (statistical significance threshold)

### EarMemory (Associative Storage)

A bounded-capacity associative memory for recognized patterns:

- **`store(pattern, tick)`** — add to memory, evicting oldest if at capacity
- **`retrieve(values)`** — lookup by exact value match
- **`find_frequent(min_observations)`** — retrieve all patterns seen at least N times

## Experimental Results

- **Pattern detection scales quadratically.** Scanning for all recurring subsequences of length up to k in a buffer of length n has O(n²×k) complexity. For real-time use, max_pattern_len should be kept small (3-5).
- **Autocorrelation reliably detects period-2 and period-3 patterns.** A stream alternating [+1, −1] correctly detects period 2. A stream cycling [+1, 0, −1] detects period 3. Period detection fails for aperiodic sequences (correlation stays below 0.5).
- **Frequency entropy distinguishes signal types.** A periodic ternary stream has entropy near 1.585 (using all three values equally). A biased stream (mostly +1) has lower entropy. A constant stream has entropy 0.
- **Inverse pattern detection reveals complementary structures.** In musical applications, a tension curve [+1, −1, +1] is the inverse of a resolution curve [−1, +1, −1]. The `are_inverse` check reveals these complementary relationships.
- **Memory capacity of 100 patterns is sufficient for most streams.** With max_pattern_len=4, there are at most 3⁴=81 possible patterns. A capacity of 100 covers the full space with room for metadata.

## Impact

`ternary-ear` demonstrates that sophisticated perceptual tasks — pattern recognition, frequency analysis, correlation detection, period estimation — can be performed on three-valued signals using simple algorithms. The crate provides a complete perception pipeline: sense (listen), recognize (pattern match), analyze (frequency, correlation), learn (training), and remember (memory).

The modular architecture — each component is independent and composable — allows users to assemble custom perception pipelines. A minimal pipeline (Ear + RhythmTracker) can run in embedded contexts; a full pipeline (all components) provides comprehensive signal analysis.

## Use Cases

1. **Agent behavior monitoring** — Watch for specific ternary behavior patterns across a fleet, receiving notifications when known patterns recur or novel patterns emerge.
2. **Musical pattern recognition** — Feed ternary representations of harmonic/rhythmic patterns to detect recurring motifs, track tempo, and build an associative memory of musical ideas.
3. **Anomaly detection via frequency analysis** — Monitor the entropy of ternary sensor streams; a sudden drop in entropy indicates a stuck or oscillating sensor.
4. **Cross-signal correlation** — Discover inverse, shifted, and correlated relationships between multiple ternary data streams using the HarmonicAnalyzer.

## Open Questions

1. **Approximate pattern matching.** The current implementation requires exact matches. Could Hamming distance or edit distance be used for fuzzy matching, allowing near-patterns to be detected?
2. **Online period detection.** The autocorrelation approach requires a full buffer. Could an online/streaming algorithm detect periods incrementally, without storing the entire history?
3. **Hierarchical pattern memory.** EarMemory currently stores flat patterns. Could a hierarchical memory (where patterns contain references to sub-patterns) improve storage efficiency and enable recognition of complex composite patterns?

## Connection to Oxide Stack

`ternary-ear` is the perception layer for the Oxide creative stack. It receives rhythmic patterns from `ternary-rhythm` and `ternary-polyrhythm`, harmonic sequences from `ternary-music`, and temporal patterns from `ternary-tidelight`. Its pattern detection feeds `ternary-tempo`'s BPM estimation. The harmonic analyzer complements `ternary-counterpoint`'s voice leading analysis. Ear training accuracy metrics can drive `ternary-compass`'s confidence adjustments.
