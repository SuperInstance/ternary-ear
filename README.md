# Ternary Ear

Listening and **pattern recognition across a ternary agent fleet**. Agents perceive each other's {-1, 0, +1} signal streams and learn from them — detecting recurring patterns, measuring frequency distributions, analyzing harmonic relationships, tracking rhythms, and improving recognition accuracy over time.

## Why It Matters

A fleet of ternary agents generates a continuous stream of ternary signals. Without listening infrastructure, each agent is deaf to others' behavior patterns. The Ear provides five layers of perception:

1. **Pattern matching**: Register specific ternary sequences and detect when they recur
2. **Frequency analysis**: Track the distribution of {-1, 0, +1} values over time
3. **Harmonic analysis**: Discover inverse, shifted, and correlated signal pairs
4. **Rhythm detection**: Find periodic patterns using autocorrelation
5. **Memory**: Store and retrieve recognized patterns with capacity-limited LRU eviction

Together, these transform raw signal streams into actionable fleet intelligence — detecting coordination, spotting anomalies, and predicting future behavior.

## How It Works

### Pattern Detection

The `Ear` maintains a ring buffer of incoming ternary values and a list of watched patterns. On each `listen(value)` call, it checks if the tail of the buffer matches any registered pattern:

$$\text{match}(P, B) \iff B[|B|-|P| \; \ldots \; |B|] = P$$

The `detect_recurring(min_obs)` function scans the entire buffer for all subsequences of length 2 to `max_pattern_len` that appear at least `min_obs` times. This uses a sliding-window enumeration:

$$\text{count}(s, B) = |\{i : B[i \ldots i+|s|] = s\}|$$

### Frequency Analysis

The `FrequencyDetector` maintains counts $[n_{-1}, n_0, n_{+1}]$ and computes:

$$f(v) = \frac{n_v}{n_{-1} + n_0 + n_{+1}}$$

The **dominant** value is $\arg\max_v n_v$. Uniformity is tested: all counts within 10% of $N/3$. Shannon entropy:

$$H = -\sum_v f(v) \log_2 f(v), \quad H_{\max} = \log_2 3 \approx 1.585$$

### Harmonic Analysis

The `HarmonicAnalyzer` discovers structural relationships between signal pairs:

**Inverse**: $\forall i: a_i = -b_i$ (perfect anti-correlation)

**Shifted by k**: $\forall i: a_i = b_{(i+k) \bmod n}$ (cyclic rotation)

**Correlation**: The ternary correlation coefficient:

$$\rho(a, b) = \frac{1}{n} \sum_{i=1}^{n} a_i \cdot b_i$$

Since $a_i, b_i \in \{-1, 0, +1\}$, the product $a_i \cdot b_i \in \{-1, 0, +1\}$, and $\rho \in [-1, +1]$. Perfect correlation: $\rho = +1$. Perfect anti-correlation: $\rho = -1$.

### Rhythm Tracking

The `RhythmTracker` detects periodic patterns via autocorrelation:

$$\hat{p} = \arg\max_{p \in [2, N/2]} \rho(B[0:N-p], B[p:N])$$

Only accepted if $\rho > 0.5$. Once a period is detected, prediction is:

$$\hat{x}_{t+1} = B[(t+1) \bmod \hat{p}]$$

### Complexity

| Operation | Time |
|-----------|------|
| `Ear::listen(v)` | O(P · L) — P patterns, L = max pattern length |
| `detect_recurring(min_obs)` | O(N · L²) — N = buffer length, L = max pattern |
| `FrequencyDetector::observe(v)` | O(1) |
| `HarmonicAnalyzer::correlation(a, b)` | O(N) |
| `RhythmTracker::detect_period()` | O(N²/2) — all candidate periods |
| `EarTraining::record(correct)` | O(1) |
| `EarMemory::store/retrieve` | O(C) — C = capacity |

## Quick Start

```rust
use ternary_ear::{Ear, Pattern, FrequencyDetector, HarmonicAnalyzer, RhythmTracker, EarMemory};

// Listen for patterns
let mut ear = Ear::new(3);
ear.watch(Pattern::new(vec![1, -1]));
ear.listen_all(&[1, -1, 0, 1, -1]);
assert!(ear.patterns[0].observations >= 2);

// Discover recurring patterns
let mut ear2 = Ear::new(3);
ear2.listen_all(&[1, 1, 0, 1, 1, 0, 1, 1]);
let found = ear2.detect_recurring(2);
assert!(found.iter().any(|p| p.values == vec![1, 1]));

// Frequency analysis
let mut fd = FrequencyDetector::new();
fd.observe(1); fd.observe(1); fd.observe(-1);
assert_eq!(fd.dominant(), 1);
let h = fd.entropy(); // Shannon entropy

// Harmonic analysis
let corr = HarmonicAnalyzer::correlation(&[1, 1, 1], &[1, 1, 1]);
assert!((corr - 1.0).abs() < 0.001);

// Rhythm detection
let mut rt = RhythmTracker::new();
for _ in 0..6 { rt.observe(1); rt.observe(-1); }
assert_eq!(rt.detect_period(), 2);
assert_eq!(rt.predict_next(), Some(1));

// Memory with LRU eviction
let mut mem = EarMemory::new(10);
mem.store(Pattern::new(vec![1, -1, 0]), 100);
```

## API

| Type | Description |
|------|-------------|
| `Ear` | Main pattern recognizer with buffer + watched patterns |
| `Pattern` | A ternary subsequence with observation count + last seen |
| `FrequencyDetector` | Per-value frequency tracking + entropy |
| `HarmonicAnalyzer` | Inverse/shifted/correlation analysis between sequences |
| `RhythmTracker` | Period detection + prediction via autocorrelation |
| `EarTraining` | Hit/miss accuracy tracking over time |
| `EarMemory` | Capacity-limited pattern storage with LRU eviction |

## Architecture Notes

The Ear system implements the **γ + η = C** conservation principle through information-theoretic invariants:

- **γ (structure)**: the fleet's signal generation architecture — which agents emit which signals
- **η (dynamics)**: the incoming signal stream — the observable perturbation that the Ear processes
- **C (conservation)**: the total information content — patterns detected + patterns stored + patterns predicted = constant information budget. The memory capacity constraint enforces this: storing new patterns evicts old ones.

The autocorrelation function used in rhythm detection is itself a conservation law: $\rho(\tau) \leq \rho(0) = 1$ for all $\tau$, meaning the signal's self-similarity is bounded by its energy. When $\rho(\hat{p}) > 0.5$, the period $\hat{p}$ captures more than half the signal's structure.

## References

- Shannon, C.E. (1948). *A Mathematical Theory of Communication*. Bell System Technical Journal.
| Box, G.E.P. & Jenkins, G.M. (1970). *Time Series Analysis: Forecasting and Control*. — Autocorrelation and period detection.
| Oppenheim, A.V. & Schafer, R.W. (2010). *Discrete-Time Signal Processing* (3rd ed.). Pearson.
| MacKay, D.J.C. (2003). *Information Theory, Inference, and Learning Algorithms*. Cambridge.

## License: MIT
