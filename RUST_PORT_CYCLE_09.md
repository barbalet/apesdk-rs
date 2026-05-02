# Rust Port Cycle 09: Startup Land State

## Objective

Move the startup transfer path from hand-fed land fixtures toward real simulator
state by porting the minimal `sim_init`/land seeding behavior needed for command
line startup saves.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_09.md
```

## Implemented

Minimal simulator state:

```text
AGE_OF_MATURITY
LandState
SimState
SimState::init
SimState::start_up
SimState::startup_transfer
SimState::tranfer_startup_out_json
SimState::prepare_land_for_first_cycle
```

`LandState` now owns the save-visible land values used by the command line
transfer path:

```text
date
time
tile 0 genetics
planet genetics
tide level
```

## C Mapping

The seed path follows `universe/sim.c` and `sim/tile.c`:

```text
sim_init:
  split randomise into two n_byte2 values
  math_random3(local_random)
  land_seed_genetics(local_random)

tile_land_random:
  generate each tile genetics pair from math_random low bytes
  math_random3(random) after each tile
  generate land-level genetics after tile genetics
```

`land_genetics()` in C returns `m_land.tiles[0].genetics`, so Rust transfer
snapshots intentionally use tile 0 genetics rather than the land-level genetics.

`LandState::clear` mirrors the save-visible part of `land_clear`: it erases land
state, preserves tile genetics, and for non-load startup kinds sets the first
cycle clock to `AGE_OF_MATURITY` and `5 * TIME_HOUR_MINUTES`.

## Deterministic Fixture

For `FIXED_RANDOM_SIM` style seed `0x5261f726`, Rust now verifies:

```text
tile 0 genetics: [7633, 53305]
land-level genetics: [2144, 24820]
remaining random seed: [46045, 62452]
startup transfer JSON length: 177 bytes
```

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
38 tests passed
```

The new tests cover:

```text
startup seed splitting and tile genetics generation
startup transfer JSON generated from SimState
first-cycle land clear date/time behavior
tile genetics preservation across land clear
```

## Next Cycle

Cycle 10 should continue from state into command-line behavior:

```text
add a small simape binary crate or command module
wire banner/version/startup state into the command shell surface
implement the first read-only commands against Rust state
compare output against golden CLI transcripts
```
