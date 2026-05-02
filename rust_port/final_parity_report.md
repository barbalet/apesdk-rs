# Final Parity Report

Cycles 246-265 close the current Rust-port parity plan.

## Approved Corpus

- CLI sessions: help, help errors, runtime parity, state save/load, empty
  startup matrix, populated short matrix, save/open matrix, and command edges.
- Trace fixtures: braincode, social/family, lifecycle, immune, movement/body,
  terrain/food, save/open continuity, population stress, release/debug, and
  final gate summary.
- Save/load formats: JSON transfer, native-text transfer, and Rust framed
  binary transfer.

## Gates

- `cargo fmt --all --check`
- `cargo test`
- `scripts/run_parity_ci.sh /private/tmp/apesdk_parity_ci_246_265`
- `scripts/run_release_debug_gate.sh /private/tmp/apesdk_release_debug_246_265`
- `scripts/performance_smoke.sh /private/tmp/apesdk_performance_smoke_246_265`

## Last Local Run

- Formatting: pass
- Rust tests: pass, including 79 `apesdk-sim`, 27 `apesdk-toolkit`, and 52
  `simape` unit tests.
- Parity CI: pass at `/private/tmp/apesdk_parity_ci_246_265b`
- Release/debug transcript diff: pass at
  `/private/tmp/apesdk_release_debug_246_265`
- Performance smoke: pass at `/private/tmp/apesdk_performance_smoke_246_265`
- Trace diff smoke: pass for `golden/traces/final_gate_summary.trace`

## Signoff Scope

The checked-in Rust command-line `simape` corpus is locked at transcript level
for the approved sessions and at trace-fixture level for the implemented native
engine categories. The remaining differences from an external raw native C
byte-for-byte oracle are explicitly documented in
`rust_port/final_drift_register.md`.
