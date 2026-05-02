# Corpus Completeness Audit

The command-line runtime reachable from `longterm.c` uses these native folders:

- `toolkit`
- `script`
- `render`
- `sim`
- `entity`
- `universe`
- `longterm.c`

Absolute parity coverage map:

- CLI command surface: promoted smoke coverage for help, targeted help, unknown
  commands, command edges, file-format lookup, logging, event toggles, watch,
  and interval.
- Save/open: default JSON save and native-text open behavior now follows C.
- Binary: default CLI no longer uses the Rust framed-binary substitute.
- Engine traces: category manifest exists; direct C trace emitters are still the
  promotion path for deeper categories.
- Fuzzing: fuzzed mismatch fixtures should be added only after both C and Rust
  runs are deterministic under the pinned seed/date harness.
