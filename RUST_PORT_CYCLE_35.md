# Rust Port Cycle 35: Population Transfer Round Trip

## Objective

Extend the Rust startup JSON transfer path so named population summaries survive
save/open.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_35.md
```

## Implemented

Population summary transfer fields:

```text
name
gender name
family name
date of birth
generation min
generation max
genetics
```

`SimState::startup_transfer` now emits beings when the Rust population is not
empty, and `SimState::load_startup_json` restores valid being summaries and
selects the first restored ape.

## Compatibility Notes

Empty saves keep the earlier compact C-shaped startup JSON with no `beings`
array. Populated Rust saves include the summary-level `beings` array as an
intermediate format until the full native transfer model is ported.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'reset\nsim\nape\nnext\nape\nlist\nsave /private/tmp/simape_cycle35_population.json\nopen /private/tmp/simape_cycle35_population.json\nsim\nape\nquit\n' | cargo run -q -p simape
```

Current result:

```text
83 tests passed
```

The smoke run confirmed reset population creation, selection movement, three
column listing, populated save, populated open, and selected-name restoration.

## Next Cycle

Cycle 36 should start replacing the summary transfer with fuller native being
transfer fields and begin aligning Rust save/open with C's native file data.
