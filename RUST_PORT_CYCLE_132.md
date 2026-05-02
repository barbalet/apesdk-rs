# Rust Port Cycle 132: Binary Save/Load Freeze

## Objective

Expose native transfer loading and writing through CLI workflows without
breaking JSON saves.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_132.md
```

## Implemented

`open` and `load` already try JSON first and native transfer second. `save`
continues to write JSON by default, and writes native transfer text when the
filename contains `.native` or ends in `.ape`.

## Validation

Added a CLI test that saves to a native filename, verifies native transfer
tokens, and opens the saved file back into a populated simulation.
