# Rust Port Cycle 14: Empty Step Land Time

## Objective

Make `step` do the first useful piece of simulation work without pretending the
being engine has been ported: advance save-visible land time for empty
simulations.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_14.md
```

## Implemented

Land time cycling:

```text
LandState::cycle
SimState::step_empty
```

`LandState::cycle` mirrors the save-visible part of C `land_cycle`:

```text
time += 1
if time == TIME_DAY_MINUTES:
  time = 0
  date += 1
```

The C tide recalculation is not ported yet; the saved tide level remains `0`.

Command-line behavior:

```text
step
```

For the current empty-population Rust shell, `step` now:

```text
marks the simulation briefly running
advances land time by one minute
marks the simulation stopped because population is zero
emits no watch output, matching the no-selected-being path
```

`run` remains an explicit stub until repeated stepping and watch output have
more of the simulation engine underneath them.

## Compatibility Notes

From the default fixture state:

```text
before step: 00:00 01/01/0
after step:  00:01 01/01/0
```

Saving after `step` now writes:

```json
{"information":{"signature":20033,"version number":708,"copyright":"Copyright Tom Barbalet, 1996-2026.","date":"May  1 2026"},"land":{"date":0,"genetics":[7633,53305],"time":1}}
```

Opening that save restores the advanced time.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'step\nsim\nsave /private/tmp/simape_cycle14_save.json\nopen /private/tmp/simape_cycle14_save.json\nsim\nquit\n' | cargo run -q -p simape
```

Current result:

```text
59 tests passed
```

The new tests cover:

```text
land time rollover at TIME_DAY_MINUTES
empty step state transition and saved land snapshot
step command advancing visible sim time
save/open preserving time advanced by step
run remaining an explicit stub
```

## Next Cycle

Cycle 15 should build on this by adding a minimal `run` path for empty
populations:

```text
parse simple run intervals such as "run 1 minute" and "run 1 day"
advance empty land time by the requested number of steps
keep forever and being watch behavior explicit until the cycle engine exists
```
