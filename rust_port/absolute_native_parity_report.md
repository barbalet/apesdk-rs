# Absolute Native C Parity Report

Cycles 266-300 replace documentation-only signoff with strict native
command-line gates.

## Completed Changes

- Native C `save` now writes native transfer text for every filename extension;
  Rust `save` still needs to be retargeted from the older JSON-compatible
  behavior to mirror this completed C path.
- Default Rust `open` now mirrors native C by reading native transfer text on
  the command-line path and rejecting JSON/framed binary input.
- Native C harness builds can pin `FULL_DATE` and `FIXED_RANDOM_SIM` so exact
  transcript comparisons do not need date or seed normalization.
- Native C is built with relative source paths so error locations match Rust's
  `./folder/file.c line` format.
- A transport-only transcript diff gate replaces behavior-hiding normalization
  for the absolute smoke corpus.

## Gates

- `cargo fmt --all --check`
- `cargo test`
- `scripts/run_raw_transcript_diff.sh /private/tmp/apesdk_raw_diff help help_errors command_edges`
- `scripts/run_absolute_parity_ci.sh /private/tmp/apesdk_absolute_parity`

## Signoff Rule

No documentation-only difference is allowed in the final signoff. A category is
either exact, blocked by an open fixture task, or excluded from the absolute
signoff until a native C oracle is generated and matched.
