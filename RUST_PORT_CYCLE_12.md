# Rust Port Cycle 12: Startup JSON Load

## Objective

Move the Rust command line from write-only saves to a real startup JSON load path
for files produced by the Rust `save` command.

## Changed Files

```text
crates/apesdk-toolkit/src/lib.rs
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_12.md
```

## Implemented

Toolkit JSON reader:

```text
object_parse_json
JsonParser
object parsing
array parsing
string parsing with basic escapes
integer number parsing
boolean/null parsing
trailing-data rejection
```

Simulator startup load:

```text
LandState::from_snapshot
SimState::from_startup_transfer
SimState::load_startup_json
startup_transfer_from_json_bytes
```

Command-line behavior:

```text
open [file]
load [file]
```

`open` and `load` now parse Rust startup save JSON, update the current
`Console` state, and print:

```text
Simulation stopped
Simulation file <path> open

```

Malformed JSON or unsupported startup content reports:

```text
ERROR: Failed to read in file @ ./universe/command.c 2394
```

## Compatibility Decision

Cycle 11 preserved the current C behavior where opening a JSON startup save
failed. Cycle 12 intentionally migrates the Rust command line to a working JSON
load for the Rust save format, because the 30-cycle port needs the command line
to become useful before the old binary loader is rebuilt.

The C-compatible failure text is still used for malformed or unsupported input,
but valid Rust startup JSON is now accepted.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'save /private/tmp/simape_cycle12_save.json\nopen /private/tmp/simape_cycle12_save.json\nsim\nquit\n' | cargo run -q -p simape
```

Current result:

```text
52 tests passed
```

The new tests cover:

```text
toolkit JSON parsing for transfer-shaped objects
whitespace, booleans, escapes, negative integers
trailing-data and decimal-number rejection
startup transfer parsing and validation
SimState restoration from startup JSON
open/load success path and malformed-load failure path
```

## Next Cycle

Cycle 13 should start reducing the gap between an empty startup shell and the C
command surface:

```text
port list/ls/dir empty-population behavior
port reset/clear startup-state regeneration
decide whether run/step should remain explicit stubs or begin advancing minimal land time
```
