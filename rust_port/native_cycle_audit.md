# Native Cycle Audit

## Purpose

This audit maps the C command-line one-minute simulation flow to the current
Rust port. It was added in cycle 56 to guide the native engine tranche.

## C One-Minute Call Graph

The C runtime enters the populated minute path through `sim_cycle` in
`universe/sim.c`.

```text
sim_cycle
  land_cycle
  sim_being_awake_loop_no_sim
    being_awake
  being_cycle_universal
    immune_response
    being_energy_delta
    being_set_state(BEING_STATE_ASLEEP)
    being_reset_drive(DRIVE_FATIGUE)
  sim_being_cycle
    being_cycle_awake
  drives_cycle
    drives_hunger
    drives_sociability
    drives_sex
    drives_fatigue
  brain/social loops
  being_tidy_loop_no_sim
  social_initial_loop
  social_secondary_loop_no_sim
  being_remove loops
  being_speed_advance
  sim_time
```

## Rust Mapping After Cycle 65

```text
SimState::advance_minutes
  LandState::advance_minutes
  PopulationState::advance_minute
  BeingSummary::advance_minute
    awake_level_for_time
    cycle_universal
      immune_response
      energy_delta
      reset_drive(DRIVE_FATIGUE)
    movement scaffold
    cycle_drives
```

## Ported In This Tranche

```text
C awake levels: FULLY_ASLEEP, SLIGHTLY_AWAKE, FULLY_AWAKE
C state flags and state descriptions
C drive constants and drive descriptions
being_energy / being_energy_delta / being_energy_less_than semantics
being_awake, without the terrain-water nighttime exception
being_cycle_universal skeleton
native immune arrays and transfer
immune_init
immune_response core
drives_hunger, reduced sociability, reduced sex drive, drives_fatigue
```

## Remaining Gaps

```text
nighttime water test in being_awake
full land/food terrain interaction in being_cycle_awake
native speed decay through being_speed_advance
full social proximity loop and social memory updates
brain and braincode loops
birth/death removal and selection preservation
episodic event generation
full native save/load expansion for social, episodic, probes, inventory, preferences
```

## Validation

Cycle 65 finished with:

```text
cargo fmt --all --check
cargo test
99 tests passed
```
