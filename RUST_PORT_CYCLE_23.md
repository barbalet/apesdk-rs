# Rust Port Cycle 23: Empty Brain and Speech Detail Commands

## Objective

Port empty-population error behavior for commands that would normally display
braincode, speech, episodic memory, and brain probes.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_23.md
```

## Implemented

Command-line behavior:

```text
braincode
speech
episodic
probes
```

The commands now share the C `command_duplicate` empty-state messages:

```text
ERROR: No being was specified @ ./universe/command.c 1311
ERROR: Being not found @ ./universe/command.c 1300
```

## Compatibility Notes

The deeper braincode, speech, episodic, and probe data models remain future
work. The Rust CLI now behaves like the C console when those commands are used
against an empty group.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
72 tests passed
```

## Next Cycle

Cycle 24 should port the same empty detail-command behavior for stats and
appearance aliases.
