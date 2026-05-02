# Rust Port Cycle 13: Reset and Empty Population Commands

## Objective

Reduce the command-line gap around empty startup state by porting the simple
state commands that do not need the full being/cycle engine yet.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_13.md
```

## Implemented

Simulator reset seed derivation:

```text
SimState::reset_new_simulation_from_land_seed
```

This mirrors the C `command_reset` seed path:

```text
seed = land_genetics()
math_random3(seed)
randomise = (seed[0] << 16) | seed[1]
sim_init(KIND_NEW_SIMULATION, randomise, ...)
```

Command-line behavior:

```text
reset
clear
list
ls
dir
```

`reset` and `clear` now regenerate a new empty startup state from the current
land seed and print:

```text
Simulation reset
```

`list`, `ls`, and `dir` now report the C empty-population message:

```text
No apes present. Trying (re)running the Simulation
```

## Compatibility Notes

From the default Rust fixture seed `0x5261f726`:

```text
initial land seed: 7633 53305
after reset:       23809 53481
after clear:       50588 11145
```

`run` and `step` intentionally remain explicit Rust-port stubs this cycle:

```text
ERROR: Command not implemented in Rust port yet
```

Those commands require real `sim_cycle` behavior and selected-being watch output
to be useful, so cycle 13 keeps them visible but avoids pretending the engine is
ported.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'list\nreset\nsim\nstep\nquit\n' | cargo run -q -p simape
```

Current result:

```text
56 tests passed
```

The new tests cover:

```text
reset seed derivation in apesdk-sim
list/ls/dir empty-population output
reset/clear command output and regenerated land seeds
run/step explicit stub decision
```

## Next Cycle

Cycle 14 should start the minimum cycle/time support needed to make `step` less
of a placeholder:

```text
port the save-visible part of land_cycle/time advancement
add a minimal SimState step method for empty populations
decide the exact step/watch output for the empty simulation
```
