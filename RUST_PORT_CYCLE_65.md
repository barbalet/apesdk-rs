# Rust Port Cycle 65: Drive Accessors And Drive Cycle

## Objective

Port C drive constants, accessors, and the first drive-cycle behavior into Rust.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_65.md
```

## Implemented

Added drive constants and accessors:

```text
DRIVE_HUNGER
DRIVE_SOCIAL
DRIVE_FATIGUE
DRIVE_SEX
BeingSummary::drive
BeingSummary::inc_drive
BeingSummary::dec_drive
BeingSummary::reset_drive
```

Added `BeingSummary::cycle_drives` with native-shaped hunger, reduced
sociability, sex-drive, and fatigue updates.

## Compatibility Notes

The sociability and sex-drive paths are reduced until full social proximity,
mate goals, pregnancy, and episodic memory are ported. The public behavior is
now structured like the C drive cycle and ready for deeper social integration.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'reset\nrun 400 minutes\nstats\nsave /private/tmp/simape_cycle65.json\nopen /private/tmp/simape_cycle65.json\nstats\nquit\n' | cargo run -q -p simape
```

Current result:

```text
99 tests passed
```

## Next Cycle

Cycle 66 should port facing and motion vector parity: `being_facing_vector`,
`being_facing_towards`, `being_wander`, and angle wrapping behavior.
