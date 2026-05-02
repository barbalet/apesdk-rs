# Rust Port Cycle Run 266-300

This 35-cycle pass moved the project from documented-difference closure toward
strict native C command-line parity.

## Implemented

- Default `simape save` now writes the same JSON transfer text for every
  extension, matching native C command-line behavior.
- Default `simape open` now follows native C `tranfer_in` behavior: native
  transfer text can load, JSON/framed binary compatibility inputs fail on the
  default CLI path, missing files quietly stop after `Simulation stopped`, and
  malformed inputs use C-matching error locations.
- Native C fixture builds can pin `FULL_DATE` through `APESDK_FULL_DATE`.
- The native build script compiles sources with `./folder/file.c` names so
  transcript error locations match Rust.
- The strict raw transcript gate captures native C and Rust sessions, applies
  only carriage-return transport cleanup, and diffs the promoted exact corpus.
- The absolute parity CI wrapper now runs formatting, tests, strict raw
  transcript smoke diffing, and trace diff smoke in one command.
- Current parity docs now treat remaining differences as open fixture tasks, not
  signoff exceptions.

## Validation

- `cargo fmt --all --check`
- `cargo test`
- `scripts/run_raw_transcript_diff.sh /private/tmp/apesdk_raw_diff_266_300 help help_errors command_edges`
- `scripts/run_absolute_parity_ci.sh /private/tmp/apesdk_absolute_parity_266_300`

## Current Exact Corpus

The promoted no-behavior-normalization transcript corpus is:

- `help`
- `help_errors`
- `command_edges`

## Open Work

Absolute native C parity is not signed off for every engine category yet.
Remaining fixture tasks are:

- native C raw binary artifact generation and byte layout mapping
- Rust raw binary reader/writer replacement once the C byte oracle exists
- C state trace emitters for terrain, food, braincode, social, lifecycle,
  immune, movement, and save/load state
- long seeded raw transcript matrices beyond the promoted smoke corpus
- fuzzed command/save/load mismatch capture and fixture promotion
- platform/profile/compiler expansion of the strict gate
