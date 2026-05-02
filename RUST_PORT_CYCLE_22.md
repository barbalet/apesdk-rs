# Rust Port Cycle 22: Empty Social Detail Commands

## Objective

Port empty-population error behavior for social graph and pathogen detail
commands that route through C `command_duplicate`.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_22.md
```

## Implemented

Command-line behavior:

```text
friends
social
socialgraph
graph
pathogen
```

With no argument and no selected being, Rust now prints:

```text
ERROR: No being was specified @ ./universe/command.c 1311
```

With a named being that does not exist, Rust now prints:

```text
ERROR: Being not found @ ./universe/command.c 1300
```

## Compatibility Notes

The social graph and pathogen renderers are not ported yet. This cycle captures
the command-line failure surface for the current empty Rust simulation state.

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

Cycle 23 should apply the same empty `command_duplicate` behavior to brain,
speech, episodic, and probe detail commands.
