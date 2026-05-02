# Rust Port Cycle 16: Logging Interval Command

## Objective

Port the C `interval` command surface so the Rust console can store and report
the simulation logging interval used by step/run watch output.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_16.md
```

## Implemented

Console interval state:

```text
Console::save_interval_steps
```

Command-line behavior:

```text
interval
interval 2 hours
interval 30 minutes
interval 1 day
```

The default interval is `60` minutes, matching C's static
`save_interval_steps = 60`.

## Compatibility Notes

With no valid argument, Rust now reports the current interval using the same C
thresholds:

```text
Current time interval is 30 min(s)
Current time interval is 2 hour(s)
Current time interval is 1 day(s)
```

With a valid argument, Rust prints the C-style set message:

```text
Logging interval set to 2 hours
Logging interval set to 1 days
```

`interval 0 minutes` is treated as not set and reports the current interval.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
67 tests passed
```

The new interval transcript covers default reporting, hour/minute/day updates,
and invalid zero input.

## Next Cycle

Cycle 17 should port the C `logging`/`log` command toggle while keeping image and
data logging backends explicit until those systems exist in Rust.
