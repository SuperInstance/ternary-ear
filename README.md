# ternary-ear: Listening and pattern recognition across a ternary agent fleet

An ear that listens to ternary ({-1, 0, +1}) signal streams, detects recurring patterns, analyzes relationships between patterns, tracks temporal rhythms, and improves recognition over time. How agents perceive each other's behavior and learn from it.

## Why This Exists

Fleet agents broadcast ternary signals as they operate. Without an ear, those signals are noise. This crate gives agents the ability to detect patterns in what other agents are doing — recognizing recurring behaviors, finding correlations between agents, and learning to predict what comes next. It's the perceptual layer for fleet intelligence.

## Core Concepts

- **Ear**: The main pattern recognizer. Listens to a ternary stream, matches against known patterns, and can auto-detect recurring subsequences.
- **Pattern**: A recognized sequence of ternary values with observation count and last-seen timestamp.
- **FrequencyDetector**: Counts occurrences of each ternary value (-1, 0, +1) and computes distribution statistics including entropy.
- **HarmonicAnalyzer**: Finds relationships between patterns — inverse pairs, shifted versions, and correlation scores.
- **RhythmTracker**: Detects temporal periodicity using autocorrelation and predicts future values based on detected period.
- **EarTraining**: Tracks prediction accuracy (hits/misses) to measure and improve recognition over time.
- **EarMemory**: Persistent pattern storage with capacity limits, retrieval by value match, and frequency filtering.

## Quick Start

```toml
[dependencies]
ternary-ear = "0.1"
```

```rust
use ternary_ear::*;

let mut ear = Ear::new(4);
ear.watch(Pattern::new(vec![1, -1, 0]));

// Feed signal stream
ear.listen_all(&[1, -1, 0, 1, -1, 0, 1, 0, -1]);

// Check if watched pattern was found
println!("Observations: {}", ear.patterns[0].observations);

// Auto-detect recurring patterns
let recurring = ear.detect_recurring(2);
for p in &recurring {
    println!("Found: {:?} ({} times)", p.values, p.observations);
}
```

## API Overview

| Type | Description |
|------|-------------|
| `Pattern` | A recognized ternary subsequence with observation metadata |
| `Ear` | Main listener — buffers signal, matches patterns, detects recurring sequences |
| `FrequencyDetector` | Counts ternary value occurrences, computes entropy and dominant value |
| `HarmonicAnalyzer` | Finds inverse, shifted, and correlated relationships between sequences |
| `RhythmTracker` | Detects periodicity via autocorrelation and predicts next values |
| `EarTraining` | Tracks hit/miss rate for prediction accuracy measurement |
| `EarMemory` | Capacity-limited persistent pattern storage with query methods |

## How It Works

The Ear maintains a rolling buffer of observed ternary values. On each `listen()`, it checks the tail of the buffer against all registered patterns for matches. The `detect_recurring()` method brute-force scans all subsequences of length 2..=max_pattern_len, counting occurrences — effective for short patterns in moderate-length streams.

The FrequencyDetector maintains a simple 3-bin histogram for {-1, 0, +1} and computes Shannon entropy. The HarmonicAnalyzer compares sequences pairwise for inverse relationships (a[i] = -b[i]), shifted rotations, and Pearson-like correlation. The RhythmTracker uses a simplified autocorrelation approach: it tests each possible period length and picks the one with highest correlation between the sequence and its shifted self.

## Known Limitations

- Pattern detection is O(n * m) for n buffer length and m pattern length — not optimized for very long streams.
- `detect_recurring()` is O(n²) in the worst case — use sparingly on long buffers.
- RhythmTracker's autocorrelation requires at least 2 full periods to detect reliably.
- No statistical significance testing — patterns with 2 observations are treated the same as patterns with 200.
- EarMemory uses FIFO eviction — no recency or frequency weighting for retention decisions.

## Use Cases

- **Fleet behavior monitoring**: Watch for known malfunction patterns in agent decision streams.
- **Agent learning**: Build a perception layer so agents can predict each other's actions.
- **Anomaly detection**: Compare observed patterns against expected patterns to flag unexpected behavior.
- **Cross-agent correlation**: Use HarmonicAnalyzer to discover which agents are inversely correlated or phase-shifted.

## Ecosystem Context

Part of the SuperInstance ternary crate family. This is the perceptual layer — pairs with `ternary-jam` (coordination), `ternary-conduct` (orchestration), and `ternary-resonance` (sympathetic response). The ear provides the input that those systems act on.

## License

MIT
