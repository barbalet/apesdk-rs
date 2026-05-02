# Rust Port Cycle 45: Summary Field Round-Trip Coverage

## Objective

Lock down the richer population summary data so future native-transfer work can
replace internals without regressing command behavior.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_45.md
```

## Implemented

Added regression coverage for extended populated save/open fields:

```text
location
facing
energy
speed
honor
height
mass
awake
drives
braincode register
immune seed
```

The CLI test coverage now exercises populated detail commands, watch selection,
top ranking, event toggles, social/pathogen summaries, brain/speech summaries,
and idea output.

## Compatibility Notes

The Rust command line is still not cycling living apes minute by minute. These
cycles reduce that future work by establishing a data-backed populated command
surface and transfer contract.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
89 tests passed
```

## Next Cycle

The next cycle should start moving from summary-backed command output toward
native `simulated_being` transfer construction and C-compatible save/open data.
