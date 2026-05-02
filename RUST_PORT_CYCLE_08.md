# Rust Port Cycle 08: Startup Transfer JSON

## Objective

Start translating the simulator-specific JSON transfer path from
`universe/transfer.c`, using the safe toolkit JSON layer added in cycle 07.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_08.md
```

## Implemented

Simulation transfer helpers:

```text
LandSnapshot
StartupTransfer
transfer_sim_obj
transfer_land_obj
transfer_startup_obj
tranfer_startup_out_json
```

The spelling `tranfer_startup_out_json` intentionally follows the C
`tranfer_out_json` spelling.

## Compatibility Notes

`transfer_sim_obj` preserves the C object field order:

```text
signature
version number
copyright
date
```

`transfer_land_obj` preserves the C object field order:

```text
date
genetics
time
```

`transfer_startup_obj` preserves the top-level C object order:

```text
information
land
beings, only when supplied
```

For the empty startup group, the Rust output matches the C save file shape from
the golden smoke transcript:

```text
{"information":{"signature":20033,"version number":708,"copyright":"Copyright Tom Barbalet, 1996-2026.","date":"May  1 2026"},"land":{"date":0,"genetics":[6443,36036],"time":0}}
```

That saved JSON is 177 bytes for the captured `[6443, 36036]` land genetics.
The later `String length : 170` line in the C open-failure transcript comes
from the loader after whitespace processing and parser mutation, so it is not
used as the saved JSON length.

`StartupTransfer` stores supplied being objects rather than just a count. This
avoids emitting an inaccurate `beings` array before the being serialization port
is ready.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
35 tests passed
```

The new tests cover:

```text
information object field order and values
land object field order and values
empty startup JSON exact byte output
empty group omits beings
supplied being objects serialize under beings
```

## Next Cycle

Cycle 09 should begin the simulation model/state bridge needed for real startup:

```text
minimal land state ownership in apesdk-sim
land_date, land_time, land_genetics equivalents
startup initialization from a seed
deterministic test fixture for transfer_startup_out_json
```

That will move the transfer path from fixture data toward actual simulator
state.
