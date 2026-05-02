# Rust Port Cycle 11: Save Command and JSON Load Failure

## Objective

Make the Rust `simape` shell stateful enough to write startup simulation output
to disk, while keeping the current C JSON-load behavior explicit until the real
loader is ported.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_11.md
```

## Implemented

New command behavior:

```text
save [file]
open [file]
load [file]
```

`save` now:

```text
stops the simulation
serializes the current SimState with tranfer_startup_out_json
writes the JSON bytes to the requested path
prints the C-style "Simulation file ... saved" line with the trailing blank line
```

`open` and `load` now:

```text
stop the simulation
read the requested file if it exists
report the current C JSON-load failure explicitly
```

The high-level failure is:

```text
ERROR: Signature not first in file @ ./universe/transfer.c 286
ERROR: Failed to read in file @ ./universe/command.c 2394
```

This preserves the current command-line compatibility target without pretending
that JSON load has been implemented.

## Compatibility Notes

The save output follows `universe/command.c`:

```text
Simulation stopped
Simulation file <path> saved

```

The saved startup JSON is generated from the Rust `SimState` fixture seed
`0x5261f726`:

```json
{"information":{"signature":20033,"version number":708,"copyright":"Copyright Tom Barbalet, 1996-2026.","date":"May  1 2026"},"land":{"date":0,"genetics":[7633,53305],"time":0}}
```

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'save /private/tmp/simape_cycle11_save.json\nopen /private/tmp/simape_cycle11_save.json\nquit\n' | cargo run -q -p simape
```

Current result:

```text
45 tests passed
```

The new tests cover:

```text
save writes exact startup transfer JSON
save without a filename no-ops like C
open/load on JSON reports the current C load failure
```

## Next Cycle

Cycle 12 should move from file output into a real input path:

```text
port enough JSON object reading to parse the Rust startup save
decide whether open should preserve C's current failure or intentionally migrate to working JSON load
add regression tests for whichever load behavior is chosen
```
