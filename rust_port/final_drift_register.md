# Final Drift Register

This register closes cycles 263-265 for the current approved fixture corpus.

## Status

There are no untriaged drift items in the checked-in Rust fixture corpus. Each
known difference is either covered by a regression fixture or explicitly
accepted below.

## Accepted Differences

- Native C startup save/open behavior: the current C transcript captures a
  saved startup JSON file failing to reopen with `Signature not first in file`.
  Rust intentionally preserves JSON reopen support and records the difference
  through the save/open matrix.
- Binary artifact shape: Rust currently writes and reads the framed `NAB1`
  transfer format represented by the binary compatibility report. No raw C
  byte-for-byte save artifact is checked in as a final external oracle.
- Build metadata: native C and Rust banners/build details may differ by local
  build date or compiler. Transcript normalization masks only documented
  volatile paths and lengths, not state values.

## Covered Drift Categories

- Pregnancy, birth, carrying, suckling, immune, movement/body, terrain/food,
  social, territory, family, braincode, and save/open behavior have regression
  coverage through Rust unit tests and manifest trace fixtures.
- Empty, populated short-run, save/open, command-edge, and runtime parity
  sessions are captured through `scripts/run_cli_transcripts.sh`.
- Release/debug transcript determinism is gated by
  `scripts/run_release_debug_gate.sh`.
