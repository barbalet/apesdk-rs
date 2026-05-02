# Rust Port Cycles 266-300 Plan

These 35 cycles extend the completed 265-cycle plan to remove every accepted
difference from the native C oracle.

## Objective

Reach absolute native C parity for command-line `simape`:

- default Rust save/open behavior matches native C exactly
- Rust writes and reads raw native C save bytes, not a substitute frame
- transcript diffs require no behavior-hiding normalization
- C and Rust state traces match at native sampling points
- final docs contain no documentation-only caveats

## Cycle Range

Cycles 266-300 are now appended to
`RUST_PORT_DEVELOPMENT_PLAN_106_205.md`.

## Completion Target

Completion means the final absolute parity gate passes repeatedly from a clean
checkout and records zero differences against the native C command-line oracle.

## Run Summary

The cycle run added the strict transport-only diff gate, pinned native C/Rust
build metadata, moved default Rust command-line save/open behavior onto the C
path, and replaced documentation-only parity closure with fixture-task based
parity closure.

## Validation From The 35-Cycle Run

- `cargo fmt --all --check`
- `cargo test`
- `scripts/run_raw_transcript_diff.sh /private/tmp/apesdk_raw_diff_266_300 help help_errors command_edges`
- `scripts/run_absolute_parity_ci.sh /private/tmp/apesdk_absolute_parity_266_300`

## Remaining Fixture Tasks

The promoted exact command-line corpus now passes without behavior-hiding
normalization. Deeper native engine categories still need direct C oracle
promotion before they can be included in the same absolute signoff:

- raw native binary artifact generation and byte-map closure
- C state trace emitters for terrain, food, braincode, social, lifecycle,
  immune, movement, and save/load checkpoints
- long seeded raw transcript matrices beyond the promoted smoke corpus
- fuzzed command/save/load mismatch promotion
