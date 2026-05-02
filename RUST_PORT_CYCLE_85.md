# Rust Port Cycle 85: Native Engine Stabilization

## Objective

Run the stabilized Rust runtime through longer smoke coverage and document the
remaining native-engine gaps.

## Changed Files

```text
rust_port/native_engine_stabilization.md
RUST_PORT_CYCLE_85.md
```

## Implemented

Documented the native-shaped systems now covered and the remaining gaps:
terrain/water cost, full food lookup, social probability models, braincode
execution, territory memory, event logging callbacks, binary transfer, and
long-duration C-vs-Rust transcript diffs.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'reset\nrun 20 minutes\nsave /private/tmp/simape_cycle85.json\nopen /private/tmp/simape_cycle85.json\nstats\nepisodic\nquit\n' | cargo run -q -p simape
```

Current result:

```text
111 tests passed
```

## Next Tranche

Start with terrain/food parity and deeper social behavior before binary
transfer parity.
