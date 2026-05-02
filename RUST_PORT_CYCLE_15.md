# Rust Port Cycle 15: Empty Run Intervals

## Objective

Build on the empty `step` support by adding a minimal `run` path for empty
populations: parse simple time intervals, print C-style run output, and advance
save-visible land time by the requested amount.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_15.md
```

## Implemented

Simulator time advancement:

```text
LandState::advance_minutes
SimState::step_empty_by
```

Command-line behavior:

```text
run (time format)
```

Supported interval forms include:

```text
run 1 minute
run 2 hours
run 1 day
run 1 M
run 1 y
run 2
```

The one-letter unit behavior follows C's command parser convention:

```text
m = minutes
M = months
h = hours
d = days
y = years
```

If no unit is supplied, the default is days.

## Compatibility Notes

The Rust command line now prints the C-style status line:

```text
Running for 1 mins
Running for 1 days
```

For empty populations, Rust advances the full requested interval rather than
stopping after the first empty cycle. This keeps the currently empty Rust shell
useful while the being cycle engine is still unported.

`run forever` remains explicit:

```text
ERROR: Run forever not implemented in Rust port yet
```

Missing or invalid run intervals report the C error text:

```text
ERROR: Time not specified, examples: run 2 days, run 6 hours @ ./universe/command.c 2211
```

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'run 1 minute\nsim\nrun 1 day\nsim\nrun forever\nquit\n' | cargo run -q -p simape
```

Current result:

```text
62 tests passed
```

The new tests cover:

```text
multi-day land minute advancement
SimState interval stepping
run interval parsing, including C one-letter unit names
run command advancing empty simulation time
run forever and missing interval boundaries
```

## Next Cycle

Cycle 16 should begin turning `run` from an empty-shell utility into a closer C
behavior:

```text
add interval command support for save_interval_steps
reuse interval settings in step/run output decisions
keep selected-being watch output explicit until beings are ported
```
