# Rust Port Cycles 266-300 Plan

These 35 cycles extend the completed 265-cycle plan to remove every accepted
difference from the native C oracle.

## Objective

Reach absolute native C parity for command-line `simape`:

- default Rust save/open behavior matches native C exactly
- Rust writes and reads raw native C save bytes, not a substitute frame
- transcript diffs require no behavior-hiding normalization
- C and Rust state traces match at native sampling points
- final docs contain no accepted-drift caveats

## Cycle Range

Cycles 266-300 are now appended to
`RUST_PORT_DEVELOPMENT_PLAN_106_205.md`.

## Completion Target

Completion means the final absolute parity gate passes repeatedly from a clean
checkout and records zero differences against the native C command-line oracle.
