# Rust Port Cycle 17: Logging Toggle Command

## Objective

Port the C `logging` and `log` command aliases for enabling and disabling console
logging state.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_17.md
```

## Implemented

Console logging state:

```text
Console::logging_enabled
Console::logging_enabled()
```

Command-line behavior:

```text
logging off
logging on
log yes
log no
```

The parser follows C's `command_on_off` ordering: off values are checked before
on values.

## Compatibility Notes

Valid values emit the C-style messages:

```text
Logging turned off
Logging turned on
```

Missing or unrecognized values produce no output, matching the C command's
quiet `-1` path.

The actual image/data logging pipeline is not ported yet; this cycle preserves
the command state and transcript shape for later wiring.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
67 tests passed
```

The new logging transcript covers `logging off`, missing value silence, the
`log` alias, and final state restoration.

## Next Cycle

Cycle 18 should port the `event` command's current build behavior before the
episodic subsystem itself is moved to Rust.
