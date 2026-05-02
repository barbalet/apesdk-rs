# Rust Port Development Plan: Cycles 56-85

## Objective

Continue the ApeSDK Rust port past the summary-backed CLI milestone and move
the command-line `simape` toward native engine parity with the C version.

This 30-cycle plan focuses on:

```text
awake-state calculation
universal energy and immune cycling
movement and body logic
social and episodic behavior
closer C parity for save/load/runtime behavior
```

## Cycle Plan

### Cycle 56: Native Cycle Audit

Inventory the exact C call graph for one simulated minute:
`sim_cycle`, awake evaluation, universal cycle, awake cycle, drives, brain,
social, removal, and speed advance. Add a Rust tracking document that maps C
functions to current Rust equivalents or missing work.

Validation: documentation review plus `cargo test`.

### Cycle 57: Awake State Constants And Descriptions

Port the C being state bit flags, state descriptions, and relationship/body
description helpers needed by `stats` and watch output. Replace summary-only
status wording with C-shaped descriptions.

Validation: unit tests for state descriptions and unchanged empty CLI
transcripts.

### Cycle 58: Energy Accessors And Saturating Semantics

Port `being_energy`, `being_energy_delta`, `being_energy_less_than`, and the
C clamp behavior around `BEING_DEAD`, `BEING_HUNGRY`, and `BEING_FULL`.

Validation: table tests against C boundary cases.

### Cycle 59: Awake-State Calculation

Port `being_awake` and wire it into Rust minute advancement before per-being
cycling. Ensure dead, hungry, resting, and awake states are represented.

Validation: tests for awake/asleep/dead transitions and `stats` status output.

### Cycle 60: Universal Cycle Skeleton

Create a Rust `being_cycle_universal` equivalent that runs for every being each
minute. Initially include awake update, energy clamp, macro-state maintenance,
and deterministic hooks for immune and body work.

Validation: populated `step` and `run` tests prove universal cycle execution.

### Cycle 61: Immune Data Model Completion

Extend Rust state to carry the native immune arrays:
antigens, shape antigens, antibodies, shape antibodies, and random seed.

Validation: save/open round-trip tests for all immune arrays.

### Cycle 62: Immune Initialization

Port `immune_init` so reset beings get C-shaped immune state rather than only a
seed summary.

Validation: deterministic immune initialization tests from fixed seeds.

### Cycle 63: Immune Response Cycle

Port the core `immune_response` behavior: pathogen mutation/depletion,
antibody response, and energy cost calculation.

Validation: unit tests for seeded immune response and energy-cost boundaries.

### Cycle 64: Universal Energy And Immune Integration

Wire immune response into `being_cycle_universal`, including energy reduction
and dead-state handling.

Validation: multi-minute simulation tests showing immune effects persist through
save/open.

### Cycle 65: Drive Accessors And Drive Cycle

Port drive constants/accessors and the core `drives_cycle` behavior for hunger,
social, fatigue, and sex drives.

Validation: seeded drive evolution tests and populated `stats` drive output.

### Cycle 66: Facing And Motion Vector Parity

Port `being_facing_vector`, `being_facing_towards`, `being_wander`, and related
angle wrapping behavior.

Validation: table tests for sixteen wind/facing/vector cases.

### Cycle 67: Speed And Speed Advance

Port `being_speed`, `being_set_speed`, `being_speed_advance`, and C speed
clamping/decay semantics.

Validation: tests for speed boundaries and post-cycle speed decay.

### Cycle 68: Movement Energy

Port `being_move_energy` and connect movement cost to universal/awake cycling.

Validation: energy delta tests for stationary, walking, and fast movement.

### Cycle 69: Awake Movement Loop

Replace the placeholder summary movement with a native-shaped awake movement
step using facing, speed, move energy, and Ape-space wrapping.

Validation: deterministic movement snapshots from fixed seeds.

### Cycle 70: Body Constants And Body State

Port body constants, body inventory descriptions, attention body fields, and
native height/mass/body-accessor behavior needed by command output.

Validation: tests for body descriptions and `appearance` output.

### Cycle 71: Body Growth And Mass Calculation

Port `being_mass_calculation` and height/mass updates from the awake cycle.

Validation: age/height/mass tests across juvenile and adult dates.

### Cycle 72: Food And Eating Surface

Port the minimum food/terrain lookup behavior needed by `being_cycle_awake`:
food type, food energy, eating state, and hunger reset.

Validation: seeded eating tests with controlled terrain/food fixtures.

### Cycle 73: Awake Cycle Integration

Create a Rust `being_cycle_awake` equivalent that coordinates movement, food,
height/mass, state transitions, and energy changes.

Validation: one-minute and multi-minute populated runs without summary
movement fallback.

### Cycle 74: Social Memory Native State

Extend Rust being state and transfer handling for native social memory entries,
including names, relationship, familiarity, attraction, friend/foe, belief, and
classification summary.

Validation: save/open round-trip tests for social arrays.

### Cycle 75: Social Graph Output Parity

Replace placeholder friends/enemies output with data-backed social graph
formatting closer to `command_show_friends`.

Validation: CLI transcript tests for friend/enemy rows.

### Cycle 76: Social Initial And Secondary Loops

Port `social_initial_loop` and `social_secondary_loop_no_sim` enough to update
familiarity, self links, and relationship maintenance.

Validation: seeded social memory evolution tests.

### Cycle 77: Social Interaction Actions

Port core social actions used by awake cycling: grooming, aggression,
show-force, mate-seeking hooks, and honor adjustment.

Validation: deterministic paired-being interaction tests.

### Cycle 78: Episodic Memory Native State

Extend Rust state and transfer handling for native episodic memory entries:
spacetime, names, event, food, affect, and argument.

Validation: save/open round-trip tests for episodic arrays.

### Cycle 79: Episodic Event Recording

Port the event recording helpers for self and social events. Wire event modes
so `event`, `event social`, and `event off` affect output behavior.

Validation: event-mode tests plus episodic memory updates after actions.

### Cycle 80: Episodic Output Parity

Replace the populated episodic placeholder with C-shaped episodic memory
formatting for selected beings.

Validation: CLI transcript tests for episodic rows.

### Cycle 81: Native Save Format Expansion

Expand JSON save output to include native social, episodic, brain probe,
attention, inventory, preferences, and immune fields that are still omitted.

Validation: populated save JSON structure tests against C field names.

### Cycle 82: Native Load Format Expansion

Expand JSON load parsing for the same native fields added in cycle 81. Keep
backward compatibility with earlier Rust summary saves.

Validation: load fixtures for legacy summary JSON, current native JSON, and
partially populated C-shaped JSON.

### Cycle 83: Binary Save/Load Parity Investigation

Audit C binary `tranfer_out`/`tranfer_in` support and decide whether the Rust
CLI should support binary save/load immediately or explicitly remain JSON-only
until a later tranche.

Validation: documented decision and tests for current accepted/rejected file
types.

### Cycle 84: Runtime Transcript Parity Pass

Generate new C and Rust golden transcripts covering reset, step, bounded run,
stats, social, episodic, save/open, and event modes. Close all low-risk wording
and ordering gaps.

Validation: golden transcript tests.

### Cycle 85: Native Engine Stabilization

Run longer seeded simulations through the Rust CLI, verify deterministic
save/open continuity, and document the remaining gaps against the C runtime.

Validation: longer smoke test, full `cargo test`, and updated next-cycle gap
list.

## Completion Criteria

By the end of cycle 85, the Rust command-line `simape` should:

```text
cycle populated beings through native-shaped awake/universal logic
apply energy, immune, drive, movement, and body updates
maintain useful social and episodic memory
save and load substantially fuller native C-shaped JSON
have golden transcript coverage for key runtime workflows
document any remaining binary-format or deep-brain parity gaps
```

## Validation Standard

Every cycle should end with:

```sh
cargo fmt --all --check
cargo test
```

Cycles that affect CLI behavior should also include a focused `cargo run -p
simape` smoke script and, where practical, a transcript/golden update.
