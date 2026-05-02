# Rust Port Cycle 33: Names And Selection

## Objective

Wire population names and selection into `SimState` so command handlers can
query and move the selected ape.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_33.md
```

## Implemented

Simulator APIs:

```text
SimState::population
SimState::max_population
SimState::beings
SimState::selected_being
SimState::selected_name
SimState::select_by_name
SimState::select_next
SimState::select_previous
```

CLI behavior:

```text
ape
pwd
next
previous
prev
list
```

For populated groups, `ape` now prints the selected name, navigation moves the
selected index, and `list` prints names in C-style three-column rows.

## Compatibility Notes

Empty startup behavior is unchanged: `ape` still reports all apes dead and
`list` still asks the user to rerun the simulation when the population is empty.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
83 tests passed
```

## Next Cycle

Cycle 34 should create an initial named population from `reset`/new simulation.
