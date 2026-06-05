# ternary-ear

**Ear training for ternary agents (and the humans who build them).**

Ear training is what musicians do to connect the abstract (intervals, chords, scales) to the experiential (what they sound like). You hear a minor third, you *know* it's a minor third, because you've trained your ear to recognize it. It's pattern recognition for sound.

This crate implements ear training for ternary systems: interval recognition, rhythm identification, frequency discrimination, and pattern matching. It's for agents that need to *listen* — to recognize what they're hearing and respond appropriately. It's also for humans who want to train their ears on the ternary sound world.

## What's Inside

- **`EarTrainer`** — manages training sessions with scoring and progression
- **`interval_quiz(ticks)`** — generate a random interval, ask the listener to identify it
- **`rhythm_quiz(ticks)`** — play a rhythm pattern, ask the listener to identify the meter
- **`frequency_discrimination(base_freq, ticks)`** — is the second tone higher, lower, or the same?
- **`pattern_match(heard, vocabulary)`** — which pattern from the vocabulary was just played?
- **`relative_pitch(reference, target)`** — what's the interval between these two tones?
- **`TrainingScore`** — tracks correct/incorrect/streak for each category

## Quick Example

```rust
use ternary_ear::*;

let mut trainer = EarTrainer::new();

// Generate an interval quiz: two tones, listener identifies the relationship
let quiz = trainer.interval_quiz(16);
// Returns two ternary signals at different frequencies

// Check the answer
let correct = trainer.check_answer(Interval::Fifth);
// Was it a fifth? trainer knows.

// Rhythm identification
let rhythm = trainer.rhythm_quiz(8);
// Play a rhythm, identify if it's 4/4, 3/4, 6/8, etc.

// Pattern matching against a vocabulary
let vocab = vec![
    vec![1, 0, -1, 0],
    vec![1, 1, -1, -1],
    vec![-1, 0, 1, 0],
];
let heard = vec![1, 0, -1, 0];
let best = pattern_match(&heard, &vocab);
// Should match index 0

// Check training progress
let score = trainer.score();
println!("Correct: {}/{}, streak: {}", score.correct, score.total, score.streak);
```

## The Deeper Truth

**An agent that can't hear can't improvise.** In a jam session (ternary-jam), agents need to recognize what other agents are playing — is that a rhythm or a melody? Is it ascending or descending? Is it repeating? Ear training gives agents the perceptual vocabulary to participate in musical conversation.

The ternary ear is different from the continuous ear. In continuous audio, you hear absolute pitch (440Hz = A). In ternary, there IS no absolute pitch — only relative relationships. Every interval is a ratio, not a frequency. This means ternary ear training is purely about *proportions* — the relationship between things, not the things themselves. It's musical relativity.

**Use cases:**
- **Agent training** — teach agents to recognize musical patterns
- **Music education** — ear training with the simplest possible sound world
- **Pattern recognition** — identify recurring patterns in ternary signals
- **Interactive music** — let AI identify what the player is doing musically
- **Accessibility** — ternary patterns are easier to distinguish than continuous audio for some listeners

## See Also

- **ternary-jam** — jam sessions where ear-trained agents perform better
- **ternary-harmonic** — the intervals and chords being identified
- **ternary-rhythm** — rhythm patterns for rhythm quizzes
- **ternary-phase** — phase recognition (another ear training skill)
- **ternary-muse** — creative generation (the complement to ear training's perception)

## Install

```bash
cargo add ternary-ear
```

## License

MIT
