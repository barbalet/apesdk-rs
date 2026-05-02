# Rust Port Cycle 10: Rust simape Shell Skeleton

## Objective

Start the Rust command-line surface for `simape`, using the state and transfer
work from earlier cycles rather than waiting for the full simulation engine.

## Changed Files

```text
Cargo.toml
Cargo.lock
crates/simape/Cargo.toml
crates/simape/src/lib.rs
crates/simape/src/main.rs
RUST_PORT_CYCLE_10.md
```

## Implemented

New workspace crate:

```text
simape
```

The crate provides:

```text
Console
Console::startup_text
Console::execute_line
Console::run_script
simape binary entry point
```

Implemented command behavior:

```text
help
help (command)
sim / simulation
memory
ape / pwd
stop
quit / exit / close
unknown-command errors
```

The command table mirrors the C command table from `universe/universe.h`,
including aliases and hidden help entries. Commands that are present in C but
not ported yet remain visible in `help`, but dispatch to an explicit Rust-port
not-implemented error instead of silently pretending to work.

## C Compatibility Notes

The full `help` transcript now matches the C golden output exactly when using
the script harness with echoed input:

```text
golden/cli/transcripts/help.txt
```

The targeted help and unknown-command transcript also matches:

```text
golden/cli/transcripts/help_errors.txt
```

The `sim` command reports the empty startup state through the Rust `SimState`:

```text
Map dimension: 512
Land seed: 7633 53305
Population: 0
Adults: 0   Juveniles: 0
Tide level: 0
00:00 01/01/0 Simulation not running
```

The seed is deterministic for the Rust test fixture: `0x5261f726`, matching the
`FIXED_RANDOM_SIM` path captured in cycle 09.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'help run\nsim\nmemory\nape\nquit\n' | cargo run -q -p simape
```

Current result:

```text
42 tests passed
```

The new tests cover:

```text
full help transcript compatibility
targeted help and unknown-command compatibility
empty startup sim command output
memory and empty selected-ape smoke output
```

## Next Cycle

Cycle 11 should make the shell stateful beyond read-only output:

```text
implement save command against SimState transfer JSON
add file-write tests around the startup save shape
begin open/load compatibility by reproducing or replacing the current C JSON-load failure deliberately
```
