# Rust Port Development Plans

Consolidated numbered Rust port plans. Each section begins with the original numbered filename so plan ranges remain visible without keeping one file per range.

Included files: 8.

## Index

- [RUST_PORT_DEVELOPMENT_PLAN_56_85.md](#rust-port-development-plan-56-85md)
- [RUST_PORT_DEVELOPMENT_PLAN_106_205.md](#rust-port-development-plan-106-205md)
- [RUST_PORT_CYCLE_266_300_PLAN.md](#rust-port-cycle-266-300-planmd)
- [RUST_PORT_CYCLE_301_400_PLAN.md](#rust-port-cycle-301-400-planmd)
- [RUST_PORT_CYCLE_366_400_PLAN.md](#rust-port-cycle-366-400-planmd)
- [RUST_PORT_CYCLE_411_500_PLAN.md](#rust-port-cycle-411-500-planmd)
- [RUST_PORT_CYCLE_551_620_PLAN.md](#rust-port-cycle-551-620-planmd)
- [RUST_PORT_CYCLE_621_680_PLAN.md](#rust-port-cycle-621-680-planmd)

---

<a id="rust-port-development-plan-56-85md"></a>

## RUST_PORT_DEVELOPMENT_PLAN_56_85.md

Original file: `RUST_PORT_DEVELOPMENT_PLAN_56_85.md`

# Rust Port Development Plan: Cycles 56-105

## Objective

Continue the ApeSDK Rust port past the summary-backed CLI milestone and move
the command-line `simape` toward native engine parity with the C version.

This 50-cycle plan focuses on:

```text
awake-state calculation
universal energy and immune cycling
movement and body logic
social and episodic behavior
closer C parity for save/load/runtime behavior
terrain and food parity
deeper social behavior
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

### Cycle 86: Terrain Operator Audit

Inventory the C terrain operator stack used by movement and food:
`land_operator_interpolated`, `land_vect2`, `land_location_vect`,
`spacetime_convert_to_map`, `WATER_TEST`, tide use, and biology operators.
Map each C dependency to current Rust coverage or a missing implementation.

Validation: documentation table plus `cargo test`.

### Cycle 87: Terrain Height And Slope Sampling

Port the minimum land height and slope sampling needed by
`being_temporary_speed` and `being_move_energy`. Add Rust helpers that return
height, slope vector, and map-space position from ape-space coordinates.

Validation: seeded terrain fixtures for height/slope stability and bounded
coordinate conversion.

### Cycle 88: Tide And Water State Parity

Port tide-level handling and water tests used by awake movement. Ensure beings
can distinguish land, water, and intertidal zones in a C-shaped way.

Validation: table tests for water state at fixed height/tide combinations and
state-bit tests for swimming/no-swimming behavior.

### Cycle 89: Temporary Speed From Terrain

Replace reduced random target speed with native-shaped `being_temporary_speed`
using terrain slope, facing vector, look-ahead position, and water detection.

Validation: deterministic speed snapshots for flat, uphill, downhill, and
water-facing terrain fixtures.

### Cycle 90: Movement Energy Terrain Cost

Expand `being_move_energy` from flat-land cost to C-shaped terrain cost:
slope dot product, uphill penalty, mass contribution, swimming cost, and body
fat insulation.

Validation: energy-cost tests for stationary, uphill, downhill, swimming, and
high-fat/low-fat cases.

### Cycle 91: Terrain-Aware Awake Movement

Wire terrain speed and terrain movement energy into `being_cycle_awake`.
Preserve current save/load behavior while replacing reduced movement decisions
with terrain-driven movement.

Validation: populated `step`/`run` tests proving movement, state bits, and
energy updates remain deterministic through save/open.

### Cycle 92: Biology Operator And Food Value Port

Port the C biology operator values used by food lookup: grass, bush, tree,
seaweed, rockpool, and beach. Add terrain fixtures that can force each food
source to dominate.

Validation: direct tests for biology operator interpolation and dominant food
selection.

### Cycle 93: Land Food Eating Parity

Port `food_eat_land` and land food selection among grass, bush, and fruit.
Connect land food type and max energy into Rust food absorption.

Validation: controlled land fixtures for vegetation, bush, and fruit food
types, including energy caps.

### Cycle 94: Intertidal Food Eating Parity

Port `food_intertidal` and intertidal food selection among seaweed,
shellfish/rockpool, and beach fallback.

Validation: controlled intertidal fixtures for seaweed and shellfish selection
plus no-food fallback.

### Cycle 95: Food Absorption And Pathogen Ingestion

Complete `food_absorption` parity across vegetable, fruit, shellfish, seaweed,
bird eggs, and lizard eggs. Wire food pathogen ingestion into the existing
immune arrays.

Validation: gene-driven absorption tests, 320-energy cap tests, and pathogen
ingestion mutation tests.

### Cycle 96: Eating Events And Runtime Food Transcript

Make eating behavior produce C-shaped state, energy, hunger-drive, growth, and
episodic output across terrain food types. Add a CLI transcript covering
hungry ape eating on controlled terrain.

Validation: golden transcript plus save/open continuity for food/episodic
state.

### Cycle 97: Social Attraction Audit

Audit C social attraction and prejudice calculations:
pheromone, pigmentation, height, frame, hair, mate preferences, stereotypes,
and episodic celebrity effects. Map each to Rust state and missing gene
helpers.

Validation: documentation table plus `cargo test`.

### Cycle 98: Social Feature And Stereotype State

Port enough feature-set and classification behavior for social stereotypes to
be stored, loaded, and updated from met beings. Use native
`simulated_featureset` fields instead of placeholder classification summaries.

Validation: social classification save/load tests and deterministic stereotype
update fixtures.

### Cycle 99: Social Attraction Calculations

Implement the C-shaped attraction/prejudice scoring used when a being first
meets another being. Include pigmentation, height, frame, hair, and pheromone
hooks where data is available.

Validation: pairwise attraction table tests for controlled genetics and body
values.

### Cycle 100: Familiarity And Friend/Foe Dynamics

Deepen social memory maintenance: familiarity scaling, respect mean,
friend/foe drift, and relationship maintenance across repeated meetings and
non-meet cycles.

Validation: multi-being seeded social evolution tests over hundreds of minutes.

### Cycle 101: Grooming Probability And Parasite Parity

Replace deterministic reduced grooming with C-shaped grooming probability,
grooming body-location selection, parasite removal, touch pathogen
transmission, and honor adjustment.

Validation: seeded grooming tests for probability thresholds, body attention,
parasites, immune transmission, and episodic memories.

### Cycle 102: Squabble And Show-Force Parity

Replace reduced squabble/show-force hooks with native-shaped aggression,
victor/vanquished selection, energy costs, honor swap, wound marking, facing,
and flee speed.

Validation: deterministic fight fixtures for show-force, attack, wound,
honor, facing, and flee behavior.

### Cycle 103: Mate-Seeking And Pair Social Behavior

Port mate-seeking social decisions from drives and episodic/social memory:
goal setting, attraction checks, mate memories, and pair behavior hooks without
yet requiring full pregnancy parity.

Validation: mature male/female fixtures for seek-mate goals and episodic
records, plus non-mature and same-sex rejection cases.

### Cycle 104: Chat And Anecdote Exchange

Port chat probability and `episodic_anecdote` enough for apes to exchange
episodic memories, mutate anecdote event/affect according to preferences, and
update social preferences.

Validation: paired chat fixtures proving copied memories, mutation bounds, and
preference changes.

### Cycle 105: Terrain/Food/Social Stabilization

Run longer seeded simulations with terrain-aware movement, food, and deeper
social behavior. Compare Rust CLI transcripts against C where available and
document remaining gaps before binary transfer parity resumes.

Validation: long smoke test, full `cargo test`, updated transcript fixtures,
and revised gap list.

## Completion Criteria

By the end of cycle 105, the Rust command-line `simape` should:

```text
cycle populated beings through native-shaped awake/universal logic
apply energy, immune, drive, movement, and body updates
maintain useful social and episodic memory
save and load substantially fuller native C-shaped JSON
have golden transcript coverage for key runtime workflows
document any remaining binary-format or deep-brain parity gaps
drive movement and eating from C-shaped terrain and food surfaces
apply deeper social attraction, grooming, squabble, mate-seeking, and chat behavior
```

## Validation Standard

Every cycle should end with:

```sh
cargo fmt --all --check
cargo test
```

Cycles that affect CLI behavior should also include a focused `cargo run -p
simape` smoke script and, where practical, a transcript/golden update.

---

<a id="rust-port-development-plan-106-205md"></a>

## RUST_PORT_DEVELOPMENT_PLAN_106_205.md

Original file: `RUST_PORT_DEVELOPMENT_PLAN_106_205.md`

# Rust Port Development Plan: Cycles 106-205

## Objective

Move the Rust command-line `simape` from strong functional parity toward
byte-for-byte and transcript-level native C parity.

The original 100-cycle block plus later parity extensions now cover the
requested remaining work:

```text
15 cycles: native terrain tile-map parity, food depletion/regrowth, C fixture comparisons
12 cycles: binary save/load compatibility
12 cycles: braincode execution and social braincode hooks
10 cycles: territory, family relationships, pregnancy/conception
6 cycles: preference learning and remaining social polish
5 cycles: long seeded C-vs-Rust transcript hardening and command edge cases
40 cycles: deterministic parity closure, fixture generation, drift elimination, and release hardening
35 cycles: no-triage closure for stdin, malformed loaders, raw binary bytes, engine values, and pending corpora
```

## Cycle Plan

### Cycle 106: Terrain Tile Format Audit

Inventory C terrain structures, tile generation paths, high-definition maps,
`tiles_topography`, land genetics, and reset/load ownership.

Validation: terrain data model map plus `cargo test`.

### Cycle 107: Rust Terrain Tile Data Model

Add Rust storage for native map/high-definition tile data without changing CLI
behavior yet.

Validation: layout and default-state tests.

### Cycle 108: Tile Generation Seed Parity

Port the C tile generation seed flow so a Rust startup land seed can reproduce
the same base tile data.

Validation: fixed-seed tile checks against C fixture dumps.

### Cycle 109: Topography Map Generation

Port `tiles_topography_map` and related interpolation/preparation paths.

Validation: map-byte fixture comparisons for selected seeds.

### Cycle 110: High-Definition Topography

Port high-definition topography generation used by rendering and terrain
sampling side effects.

Validation: high-definition tile fixture comparisons.

### Cycle 111: Replace Procedural Height Source

Switch Rust `LandState::height_at` and slope sampling from deterministic
procedural terrain to native tile-backed terrain.

Validation: existing terrain/movement tests plus C location fixtures.

### Cycle 112: Land Operator Exactness

Compare Rust `land_operator` and `land_operator_interpolated` against C over
fixed map and ape-space samples; remove approximations.

Validation: operator fixture table.

### Cycle 113: Weather And Moving Sun Inputs

Port or fixture the weather inputs required by moving-sun biology operators.

Validation: biology operator fixtures across day/night/weather cases.

### Cycle 114: Tide And Terrain Integration

Verify tide, water, high-water mark, and walk-on-water behavior against C with
tile-backed heights.

Validation: water/intertidal fixture matrix.

### Cycle 115: Food Quantity State

Add native-shaped food availability state where C decrements or gates food
sources.

Validation: repeated eating fixtures.

### Cycle 116: Food Depletion

Port depletion behavior for land and intertidal foods during eating.

Validation: eat-twice tests showing changed availability.

### Cycle 117: Food Regrowth Cycle

Port time-based food regrowth and any terrain/season/tide dependencies.

Validation: long fixture showing depletion and recovery.

### Cycle 118: Animal And Egg Food Hooks

Connect bird eggs, lizard eggs, fish/nut hooks, and existing inventory foods
where the C command-line runtime can reach them.

Validation: food type and absorption fixture table.

### Cycle 119: Terrain/Food C Fixture Harness

Build a small C fixture runner or checked-in fixture set for terrain, biology,
and food values.

Validation: Rust tests consume fixture outputs.

### Cycle 120: Terrain/Food Parity Freeze

Run broad fixed-seed terrain/food comparisons and document remaining drift, if
any.

Validation: fixture suite plus CLI smoke.

### Cycle 121: Binary Format Audit

Inventory native save sections, signatures, packing, endian assumptions,
structure ordering, and legacy compatibility rules.

Validation: binary field map document plus `cargo test`.

### Cycle 122: Binary Reader Skeleton

Add a native binary reader that detects signatures and reports precise
unsupported sections without affecting JSON loading.

Validation: binary rejection and signature tests.

### Cycle 123: Version And Section Parsing

Parse native version blocks, file section headers, and size checks.

Validation: C save header fixtures.

### Cycle 124: Land Binary Load

Load land date, time, genetics, terrain tiles, tide state, and any terrain
arrays from binary saves.

Validation: C binary land fixture round-trip into Rust state.

### Cycle 125: Being Constant/Delta Binary Load

Load native being identity, genetics, birth/generation, location, energy,
velocity, macro-state, and physical fields.

Validation: binary being fixture tests.

### Cycle 126: Events Binary Load

Load social, episodic, and territory/event arrays from native binary saves.

Validation: event array fixture tests.

### Cycle 127: Volatile/Brain/Immune Binary Load

Load changes, brain data, inventory, preferences, immune arrays, and random
seeds.

Validation: full-being binary fixture tests.

### Cycle 128: Binary Writer Skeleton

Write native signatures, version blocks, and section boundaries from Rust.

Validation: structural comparison to C writer.

### Cycle 129: Land And Population Binary Write

Write land and beings in native order with byte-level layout discipline.

Validation: Rust-written save loads in C for simple fixtures.

### Cycle 130: Full Binary Round Trip

Round-trip C binary saves through Rust load/write and Rust saves through C
load/write.

Validation: C/Rust mutual load fixtures.

### Cycle 131: Binary Compatibility Edge Cases

Handle empty populations, maximum population, older versions, malformed sizes,
and partial files.

Validation: negative and boundary binary tests.

### Cycle 132: Binary Save/Load Freeze

Promote binary loading/writing into the CLI `open`/`save` flow where native C
expects it.

Validation: command transcript fixtures for JSON and binary saves.

### Cycle 133: Braincode Audit

Inventory C braincode instruction format, sensors, actuators, probes,
registers, frequency, and scheduling.

Validation: braincode opcode map.

### Cycle 134: Braincode VM Skeleton

Add Rust braincode execution scaffolding with instruction decode and register
state.

Validation: decode tests.

### Cycle 135: Braincode Arithmetic Operators

Port arithmetic, comparison, data, and control-flow operators.

Validation: single-instruction VM fixtures.

### Cycle 136: Braincode Sensors

Port native sensors for body, drives, terrain, social, episodic, and random
inputs.

Validation: sensor fixture table.

### Cycle 137: Braincode Actuators

Port native actuators for movement, attention, drives, speech/shout, social,
and inventory hooks.

Validation: actuator side-effect tests.

### Cycle 138: Probe Scheduling

Implement probe frequency, address, state, and history behavior.

Validation: probe execution timing tests.

### Cycle 139: Brain State Persistence

Ensure braincode registers, states, probes, and script overrides round-trip via
JSON and binary formats.

Validation: save/load brain fixtures.

### Cycle 140: Braincode Runtime Integration

Wire braincode execution into the per-minute cycle at the same point as C.

Validation: seeded one-minute brain fixture.

### Cycle 141: Social Braincode Initialization

Port `being_init_braincode` behavior for newly met beings and social graph
entries.

Validation: social entry braincode fixtures.

### Cycle 142: Braincode Speech And Idea Parity

Align CLI `braincode`, `probes`, `speech`, and `idea` commands with native
brain state.

Validation: CLI transcript updates.

### Cycle 143: Braincode C Fixture Harness

Generate C fixture traces for representative braincode programs.

Validation: Rust VM matches C traces.

### Cycle 144: Braincode Freeze

Run longer seeded simulations with braincode enabled and fix remaining VM
drift.

Validation: long braincode fixture plus full tests.

### Cycle 145: Territory Data Audit

Inventory C territory memory, naming, familiarity, indexing, and serialization.

Validation: territory field map.

### Cycle 146: Territory Memory Model

Add full Rust territory state and native-shaped save/load support.

Validation: territory state round-trip tests.

### Cycle 147: Territory Familiarity Cycle

Port territory familiarity updates, rescaling, and per-minute indexing.

Validation: movement-over-territory fixtures.

### Cycle 148: Territory Naming

Port chat-based territory naming and agreement behavior.

Validation: pair chat territory fixtures.

### Cycle 149: Family Relationship Audit

Map native relationship constants, mother/father/child/grandparent/sibling
inference, and relationship storage rules.

Validation: relationship audit document.

### Cycle 150: Family Relationship Inference

Port relationship inference and social graph relationship updates.

Validation: controlled family tree fixtures.

### Cycle 151: Pregnancy State Audit

Inventory conception dates, fetal genetics, father/mother fields, gestation,
inhibition, carrying, suckling, and birth paths.

Validation: pregnancy field map.

### Cycle 152: Conception And Fetal Genetics

Port conception eligibility, fetal genetics creation, father/mother identity,
and date tracking.

Validation: deterministic conception tests.

### Cycle 153: Gestation And Birth

Port gestation, birth, child initialization, generation range, and population
capacity behavior.

Validation: birth fixture tests.

### Cycle 154: Suckling, Carrying, And Weaning

Port post-birth care hooks and their energy/social side effects.

Validation: mother/child lifecycle fixtures.

### Cycle 155: Preference Learning Audit

Inventory learned preference changes in social, mate, grooming, chat, and
episodic anecdote paths.

Validation: preference map document.

### Cycle 156: Mate Preference Learning

Port pigmentation, hair, height, and frame preference updates from mate/social
events.

Validation: controlled preference update tests.

### Cycle 157: Grooming Preference Learning

Port male/female grooming preference changes and honor/status effects.

Validation: grooming preference fixtures.

### Cycle 158: Chat And Anecdote Preference Learning

Port chat likelihood and anecdote event/affect mutation preference feedback.

Validation: chat/anecdote learning tests.

### Cycle 159: Stereotype Learning Polish

Deepen feature-set stereotype matching, observations, frequency normalization,
and prejudice transfer.

Validation: stereotype fixture tests.

### Cycle 160: Social Polish Freeze

Run dense social simulations and compare friend/foe, familiarity, attraction,
and preference traces.

Validation: seeded social trace fixtures.

### Cycle 161: Long Transcript Harness

Build a repeatable C-vs-Rust command transcript runner with fixed seeds,
timeouts, normalization, and diff output.

Validation: harness runs existing golden sessions.

### Cycle 162: Empty And Startup Transcript Matrix

Compare empty startup, help, memory, file, save/open, and run commands.

Validation: C/Rust transcript parity for empty sessions.

### Cycle 163: Populated Short-Run Transcript Matrix

Compare reset, run, stats, top, social, pathogen, episodic, brain, and speech
commands over short populated runs.

Validation: C/Rust transcript parity for short sessions.

### Cycle 164: Long Seeded Transcript Matrix

Compare long multi-day and multi-month seeded simulations, including save/open
continuity.

Validation: long transcript parity fixtures.

### Cycle 165: Command Edge Case Sweep

Harden parsing, error messages, aliases, missing arguments, malformed files,
and command ordering against C.

Validation: command edge-case golden suite.

### Cycle 166: Deterministic Random Audit

Audit every Rust random draw introduced since cycle 56 against C call order.

Validation: random-count trace comparisons.

### Cycle 167: Per-Minute Call Order Trace

Create a one-minute engine trace from C and Rust for land, beings, brain,
social, tidy, and removal loops.

Validation: trace diff fixture.

### Cycle 168: Energy Drift Pass

Eliminate drift in energy use/gain across movement, sleep, immune, food,
social, pregnancy, and braincode.

Validation: multi-being energy trace parity.

### Cycle 169: Position And Facing Drift Pass

Eliminate drift in movement, facing, wrapping, slope, swimming, and flee
behavior.

Validation: position/facing trace parity.

### Cycle 170: State Bit Drift Pass

Eliminate drift in macro-state and awake state bit transitions.

Validation: state bit trace parity.

### Cycle 171: Body Drift Pass

Eliminate drift in mass, height, body fat, posture, parasites, wounds, and
inventory.

Validation: body trace parity.

### Cycle 172: Immune Drift Pass

Eliminate drift in antigen/antibody mutation, transmission, severity, and
energy costs.

Validation: immune trace parity.

### Cycle 173: Food Drift Pass

Eliminate drift in food selection, absorption, depletion, regrowth, and
pathogen ingestion.

Validation: food trace parity.

### Cycle 174: Terrain Drift Pass

Eliminate terrain, tide, operator, and weather drift across representative
map-space and ape-space samples.

Validation: terrain trace parity.

### Cycle 175: Social Drift Pass

Eliminate drift in meeting, attraction, grooming, squabble, mate, chat, and
stereotype behavior.

Validation: social trace parity.

### Cycle 176: Braincode Drift Pass

Eliminate VM instruction, sensor, actuator, scheduling, and probe drift.

Validation: braincode trace parity.

### Cycle 177: Birth/Lifecycle Drift Pass

Eliminate drift in conception, fetal state, birth, suckling, carrying, aging,
and death.

Validation: lifecycle trace parity.

### Cycle 178: Save Byte Diff Pass

Compare Rust-written native binary saves against C byte-for-byte for stable
fixtures.

Validation: byte diff tests.

### Cycle 179: Load Behavior Diff Pass

Compare C-loaded Rust saves and Rust-loaded C saves after one or more cycles.

Validation: cross-load behavioral diff tests.

### Cycle 180: JSON Compatibility Pass

Ensure JSON transfer remains stable while binary parity is added.

Validation: JSON golden and backward compatibility tests.

### Cycle 181: CLI Formatting Pass

Match C spacing, punctuation, ordering, elapsed-time wording, and numeric
formatting for all command output.

Validation: broad transcript diff suite.

### Cycle 182: Error Message Pass

Match C error text, file/line references, and failure ordering.

Validation: malformed command/file transcript suite.

### Cycle 183: Platform Width Audit

Verify integer widths, casts, signed shifts, overflows, and alignment across
Rust/C boundaries.

Validation: layout and arithmetic boundary tests.

### Cycle 184: Overflow Semantics Pass

Match C wrapping, saturation, truncation, and signed/unsigned conversion where
observable.

Validation: boundary fixture tests.

### Cycle 185: Performance Baseline

Measure Rust runtime against C for common CLI runs and identify parity blockers
from slow paths.

Validation: benchmark notes, no functional regressions.

### Cycle 186: Performance Corrections

Optimize hot terrain, social, braincode, and save/load paths without changing
trace output.

Validation: trace parity plus benchmark comparison.

### Cycle 187: Fixture Corpus Expansion

Add fixture saves and transcripts for empty, small, normal, maximum, old,
injured, pregnant, immune-heavy, and social-heavy populations.

Validation: corpus runs in CI.

### Cycle 188: Fuzz Malformed Inputs

Fuzz command input, JSON, and binary saves for safe C-compatible failure.

Validation: fuzz regression fixtures.

### Cycle 189: C Harness CI Integration

Add optional CI jobs or scripts that build C and Rust and compare generated
fixtures locally.

Validation: documented reproducible harness run.

### Cycle 190: Golden Update Discipline

Document and enforce how C goldens are generated, reviewed, normalized, and
updated.

Validation: golden README and script checks.

### Cycle 191: Full Day Parity Run

Run a full simulated day from fixed seeds and fix remaining trace/transcript
drift.

Validation: one-day parity report.

### Cycle 192: Full Month Parity Run

Run a full simulated month from fixed seeds and fix remaining drift.

Validation: one-month parity report.

### Cycle 193: Population Stress Parity

Run maximum or near-maximum populations and compare state, command output, and
save/load behavior.

Validation: stress parity report.

### Cycle 194: Legacy Save Corpus

Collect or create older native save examples and ensure Rust handles expected
version compatibility.

Validation: legacy load tests.

### Cycle 195: Release-Mode Determinism

Verify debug and release builds produce identical observable simulation and
transcript output.

Validation: debug/release transcript diff.

### Cycle 196: Cross-Platform Determinism

Audit macOS/Linux integer, filesystem, newline, and path behavior for parity.

Validation: platform notes and normalized transcript tests.

### Cycle 197: Public API Cleanup

Clean up Rust APIs exposed during the port while preserving CLI behavior and
fixture stability.

Validation: tests and crate documentation check.

### Cycle 198: Documentation Finalization

Update architecture, parity status, command behavior, save/load, and fixture
documentation.

Validation: documentation review.

### Cycle 199: Known Drift Zero List

Produce a final list of known drift; each item must be fixed, explicitly
accepted, or proven unobservable.

Validation: zero untriaged drift items.

### Cycle 200: Final Binary Parity Gate

Require byte-for-byte binary parity for approved fixture corpus.

Validation: binary parity gate passes.

### Cycle 201: Final Transcript Parity Gate

Require transcript-level parity for approved CLI corpus.

Validation: transcript parity gate passes.

### Cycle 202: Final State Trace Parity Gate

Require per-minute state trace parity for approved seed corpus.

Validation: state trace parity gate passes.

### Cycle 203: Regression Lockdown

Add regression tests for all fixed drift categories and ensure no flaky
fixtures remain.

Validation: repeated full suite runs.

### Cycle 204: Release Candidate Smoke

Run final C/Rust smoke scenarios, save/load loops, long runs, and command
matrix.

Validation: release candidate report.

### Cycle 205: Byte-For-Byte Parity Signoff

Freeze the Rust command-line `simape` as byte-for-byte save compatible and
transcript-level native C compatible for the approved parity corpus.

Validation: all gates pass; final parity report documents scope and any
explicitly accepted non-observable differences.

## Completion Criteria

By the end of cycle 205, Rust `simape` should:

```text
load and write native C binary saves
match C terrain, food, braincode, social, territory, family, pregnancy, and preference behavior
produce matching CLI transcripts for the approved command corpus
produce matching per-minute state traces for the approved seed corpus
preserve JSON transfer compatibility
document fixture generation and parity gates
have zero untriaged observable drift from native C
```

## Validation Standard

Every cycle should end with:

```sh
cargo fmt --all --check
cargo test
```

Cycles that affect native parity should also run the relevant C fixture or
transcript comparison. Cycles that affect CLI behavior should update or add
golden transcripts only from the documented C harness.

## Parity Closure Extension: Cycles 206-265

The 106-205 plan completed the broad Rust implementation pass, but true
byte-for-byte / transcript-level native C parity still needs raw binary
save/load support, reproducible C fixture generation, long trace comparison,
and final gate hardening. These 60 cycles cover that remaining proof-and-parity
work.

### Cycle 206: Raw Binary Format Audit - Complete

Map the exact native C raw/binary save layout, including signatures, version
blocks, section order, struct padding, endian behavior, and array counts.

Validation: binary layout audit with offsets tied to C `sizeof` and
`offsetof` checks.

### Cycle 207: Binary Reader Framework - Complete

Add a byte-reader layer for little-endian native primitives and fixed-size
C-shaped structures without changing existing JSON/native-text loading.

Validation: primitive reader fixtures and malformed-length tests.

### Cycle 208: Binary Version And Land Load - Complete

Load raw binary signature, version, land date/time/genetics, terrain metadata,
and any stable land blocks.

Validation: C-generated land-only binary fixture loads into Rust.

### Cycle 209: Binary Being Delta Load - Complete

Load being delta fields from raw binary saves, including location, facing,
velocity, energy, state, body, goal, and social coordinates.

Validation: delta fixture comparison against C offsets.

### Cycle 210: Binary Being Constant Load - Complete

Load being constant fields, identity, birth date, generation range, and
genetics from raw binary saves.

Validation: constant fixture comparison.

### Cycle 211: Binary Events Load - Complete

Load raw binary social, episodic, and territory arrays into Rust C-shaped
state.

Validation: event array binary fixture round-trip into Rust state.

### Cycle 212: Binary Brain/Volatile Load - Complete

Load brain registers, probes, state, script overrides, attention, drives,
shout, inventory, preferences, pregnancy, and child-generation fields.

Validation: full-being binary fixture.

### Cycle 213: Binary Immune Load - Complete

Load immune antigens, antibodies, shapes, and random seeds exactly from raw
binary saves.

Validation: immune-heavy fixture comparison.

### Cycle 214: Binary Reader Edge Cases - Complete

Handle empty populations, maximum populations, truncated files, unknown
sections, and version mismatch failures with C-compatible behavior.

Validation: negative binary fixture suite.

### Cycle 215: Legacy Save Corpus Setup - Complete

Collect or generate representative older/native saves and classify which
versions must be supported or explicitly rejected.

Validation: corpus manifest and load expectation table.

### Cycle 216: Binary Writer Framework - Complete

Add a byte-writer layer for raw native structures while preserving JSON and
native-text transfer output.

Validation: writer primitive and section-header tests.

### Cycle 217: Binary Land Write - Complete

Write raw binary version and land blocks in native C order.

Validation: Rust-written land fixture loads in C.

### Cycle 218: Binary Being Write - Complete

Write being delta, constant, events, brain, volatile, and immune state in raw
C-compatible order.

Validation: C loader accepts simple Rust-written populated fixture.

### Cycle 219: Binary Empty/Maximum Write - Complete

Write empty, single-being, normal, and maximum-population raw binary fixtures.

Validation: C load behavior matches expected population counts and state.

### Cycle 220: Binary Cross-Load Round Trip - Complete

Round-trip C saves through Rust load/write and Rust saves through C load/write.

Validation: cross-load state comparison after one cycle.

### Cycle 221: Binary Byte Diff Gate - Complete

Compare stable Rust-written fixtures against C-written fixtures byte for byte
where no timestamps/randomized headers differ.

Validation: byte diff tests with documented normalizers if needed.

### Cycle 222: Binary CLI Integration - Complete

Wire raw binary save/open behavior into `simape` behind the expected file
extensions while keeping JSON/native-text compatibility.

Validation: CLI open/save binary transcript fixtures.

### Cycle 223: Binary Regression Lock - Complete

Add regression tests for every binary parsing/writing edge fixed so far.

Validation: repeated binary fixture suite.

### Cycle 224: Binary Compatibility Report - Complete

Document supported raw binary versions, unsupported versions, and exact drift
status.

Validation: binary compatibility report reviewed against fixture corpus.

### Cycle 225: C Build Harness - Complete

Create a reproducible local script that builds the native C command-line
`simape` and records compiler/version details.

Validation: harness builds C and Rust from a clean checkout.

### Cycle 226: C Transcript Runner - Complete

Extend the existing CLI golden runner to execute both C and Rust sessions with
timeouts, output capture, and normalized line endings.

Validation: help/session transcripts compare through the harness.

### Cycle 227: Transcript Normalization - Complete

Normalize expected volatile fields such as dates, temporary paths, random save
sizes, and platform-specific line endings without masking behavioral drift.

Validation: normalization unit tests and sample diff.

### Cycle 228: C State Trace Hooks - Complete

Add optional trace points for C and Rust covering land, population, selected
being, energy, position, drives, brain, social, territory, and lifecycle state.

Validation: one-minute trace files generated for both implementations.

### Cycle 229: Trace Diff Tool - Complete

Build a structured diff tool for state traces with clear mismatch categories
and first-difference reporting.

Validation: intentional mismatch fixture produces readable output.

### Cycle 230: Fixture Manifest - Complete

Create a manifest describing every save, command script, seed, expected gate,
and normalization rule.

Validation: manifest parser test and fixture existence check.

### Cycle 231: Harness CI Script - Complete

Add a local CI-style script that runs Rust tests, C build, C/Rust transcripts,
binary fixtures, and trace fixtures in a stable order.

Validation: script runs locally and reports pass/fail summary.

### Cycle 232: Harness Documentation - Complete

Document how to generate, review, normalize, and update C-derived goldens.

Validation: README walkthrough from clean checkout.

### Cycle 233: Braincode C Trace Fixtures - Complete

Generate C traces for representative braincode programs covering data,
arithmetic, control flow, sensors, actuators, probes, and anecdotes.

Validation: fixture files committed with manifest entries.

### Cycle 234: Braincode Decode Trace Parity - Complete

Compare Rust decode, address wrapping, constant flags, and program-counter
movement against C traces.

Validation: decode trace parity tests.

### Cycle 235: Braincode Arithmetic Trace Parity - Complete

Close drift in `DAT`, arithmetic, byte mutation, register, and control
operators.

Validation: arithmetic trace parity tests.

### Cycle 236: Braincode Sensor Trace Parity - Complete

Close drift in body, drive, social, episodic, terrain, weather, immune, and
territory sensor values.

Validation: sensor trace parity table.

### Cycle 237: Braincode Actuator Trace Parity - Complete

Close drift in action, goal, friend/foe, attraction, familiarity, probes,
shout, posture, preferences, intentions, and anecdotes.

Validation: actuator trace parity tests.

### Cycle 238: Braincode Scheduling Trace Parity - Complete

Match C dialogue iteration counts, internal/external scheduling, probe
frequency, offset, and state update timing.

Validation: scheduling trace parity tests.

### Cycle 239: Braincode Social Hook Parity - Complete

Match social braincode initialization and per-social-entry braincode behavior
for newly met beings and chat interactions.

Validation: social braincode fixture traces.

### Cycle 240: Braincode Long-Run Freeze - Complete

Run long seeded braincode-enabled simulations and fix remaining VM drift.

Validation: long braincode trace parity.

### Cycle 241: Social Action Drift Pass - Complete

Close drift in social actions, body inventory actions, giving, pickup/drop,
brandish, drag, bash, chew, fish, and shout-visible states.

Validation: social/body action trace fixtures.

### Cycle 242: Groom/Squabble Drift Pass - Complete

Close drift in grooming selection, wounds, parasites, honor, squabble force,
attack, flee, and episodic side effects.

Validation: grooming/squabble trace parity.

### Cycle 243: Mate/Preference Drift Pass - Complete

Close drift in attraction, mate bond, mate preference learning, sex drive, mate
goals, and conception trigger behavior.

Validation: mate/preference fixture parity.

### Cycle 244: Territory Drift Pass - Complete

Close drift in territory familiarity indexing, rescaling, naming, chat-based
agreement, and territory-focused attention.

Validation: territory trace parity.

### Cycle 245: Family Relationship Drift Pass - Complete

Close drift in mother/father/child/grandparent/sibling relationship inference
and social graph storage rules.

Validation: controlled family-tree fixtures.

### Cycle 246: Pregnancy And Birth Drift Pass - Complete

Close drift in conception date, fetal genetics, gestation, birth creation,
population capacity, and child initialization.

Validation: pregnancy/birth trace fixtures.

### Cycle 247: Carrying/Suckling Drift Pass - Complete

Close drift in carrying, suckling, weaning, mother/child energy transfer,
immunity seeding, and post-birth episodic events.

Validation: mother/child lifecycle trace fixtures.

### Cycle 248: Immune Transmission Drift Pass - Complete

Close drift in air, touch, sex, and food pathogen transmission, antibody
mutation, antigen severity, and energy cost.

Validation: immune trace parity.

### Cycle 249: Movement/Body Drift Pass - Complete

Close drift in walking, swimming, water avoidance, slope energy, mass, height,
fat, posture, fatigue, and velocity history.

Validation: movement/body trace parity.

### Cycle 250: Food/Terrain Drift Pass - Complete

Close drift in terrain sampling, tide, weather, food choice, depletion,
regrowth, absorption, and pathogen ingestion.

Validation: terrain/food trace parity.

### Cycle 251: Empty Startup Transcript Matrix - Complete

Compare C and Rust for empty startup, help, memory, file, save/open, errors,
and no-population detail commands.

Validation: empty transcript matrix passes.

### Cycle 252: Populated Short Transcript Matrix - Complete

Compare reset, run, stats, top, social, pathogen, episodic, braincode, probes,
speech, idea, and navigation commands after short runs.

Validation: populated short transcript matrix passes.

### Cycle 253: Save/Open Continuity Matrix - Complete

Compare save/open loops across JSON, native text, raw binary, malformed files,
and cross-loaded files.

Validation: save/open transcript and state matrix passes.

### Cycle 254: Multi-Day Seeded Matrix - Complete

Run multi-day seeded simulations with trace and transcript comparison.

Validation: multi-day C/Rust parity report.

### Cycle 255: Multi-Month Seeded Matrix - Complete

Run multi-month seeded simulations with save/open continuity and drift
triage.

Validation: multi-month C/Rust parity report.

### Cycle 256: Population Stress Matrix - Complete

Run near-maximum and maximum population scenarios through command, trace, and
save/load gates.

Validation: stress parity report.

### Cycle 257: Command Edge Case Sweep - Complete

Match parsing, aliases, missing arguments, malformed commands, command ordering,
and C error text/file-line behavior.

Validation: command edge-case golden suite.

### Cycle 258: Release/Debug Determinism - Complete

Verify debug and release Rust builds produce identical observable transcripts,
traces, and saves for approved fixtures.

Validation: debug/release diff gate.

### Cycle 259: Cross-Platform Determinism - Complete

Audit macOS/Linux newline, path, integer, filesystem, and locale behavior for
stable fixture output.

Validation: platform notes and normalized transcript tests.

### Cycle 260: Performance Baseline And Corrections - Complete

Measure Rust versus C on common runs, optimize hot paths only where trace
output remains unchanged, and document any accepted performance gap.

Validation: benchmark report plus parity tests.

### Cycle 261: Public API Cleanup - Complete

Clean up internal Rust APIs introduced during parity work while preserving CLI,
fixture, and transfer behavior.

Validation: full tests and docs check.

### Cycle 262: Final Documentation Pass - Complete

Finalize architecture, command behavior, save/load compatibility, fixture
generation, drift categories, and release notes.

Validation: documentation review against implemented behavior.

### Cycle 263: Known Drift Zero Gate - Complete

Ensure every known drift item is fixed, fixture-proven unobservable, or
explicitly accepted with rationale.

Validation: zero untriaged drift list.

### Cycle 264: Final Parity Gates - Complete

Run final binary byte diff, transcript diff, state trace diff, release/debug,
and stress gates together.

Validation: all final gates pass repeatedly without flakes.

### Cycle 265: Native C Parity Signoff - Complete

Freeze Rust `simape` as byte-for-byte save compatible and transcript-level
native C compatible for the approved corpus.

Validation: signed final parity report with fixture manifest, gate outputs,
and any explicitly accepted non-observable differences.

## Absolute Native C Parity Extension: Cycles 266-300

The 246-265 signoff closed the approved Rust fixture corpus, but it still
contained documented differences from a stricter native C oracle. These 35
cycles remove every accepted/documented difference and replace the current
approved-corpus signoff with absolute native C parity: no transcript
normalization beyond platform-invisible transport cleanup, no Rust-only framed
binary substitute, no JSON save/open behavior drift, and no undocumented
fixture gaps.

### Cycle 266: Accepted Drift Triage Freeze

Convert every accepted item in `rust_port/final_drift_register.md` into a
blocking work item with an owner, fixture, expected native C behavior, and
removal condition.

Validation: drift register contains no "accepted" category, only open blocking
items.

### Cycle 267: C Save/Open Oracle Capture

Capture native C save/open behavior for startup, populated, JSON extension,
native-text extension, binary extension, malformed, missing, and cross-loaded
files without Rust normalization.

Validation: raw C transcript and artifact corpus checked into a reviewed
golden directory.

### Cycle 268: Rust Save/Open Behavior Match

Change Rust `simape` save/open behavior to match native C exactly for every
captured extension and file-content case, including native C's JSON reopen
failure if that remains the observed C command behavior.

Validation: C/Rust save/open transcript diff is empty without behavioral
normalization.

### Cycle 269: Save/Open Compatibility Escape Hatch

If Rust still needs non-C compatibility for developer workflows, move it behind
an explicit non-default command or feature so default `simape` remains native
C-identical.

Validation: default CLI transcripts match C; compatibility mode has separate
tests and is excluded from native parity gates.

### Cycle 270: Raw C Binary Artifact Generator

Add a native C harness target that writes raw binary saves for empty, startup,
single-being, normal, maximum, old-version, social-heavy, immune-heavy,
terrain-heavy, and pregnant/lifecycle states.

Validation: generated artifacts are reproducible and content-addressed.

### Cycle 271: Raw Binary Layout Byte Map

Derive the exact C byte layout for every raw save block from generated
artifacts plus C `sizeof`, `offsetof`, endian, padding, and array-count probes.

Validation: byte map explains every byte in the approved raw binary corpus.

### Cycle 272: Rust Raw Binary Reader Replacement

Replace or bypass the framed Rust binary transfer reader with the exact raw C
reader on default CLI paths.

Validation: every C raw binary artifact loads into Rust with matching state
trace.

### Cycle 273: Rust Raw Binary Writer Replacement

Write default Rust binary saves in native C raw byte order instead of the Rust
framed transfer substitute.

Validation: Rust-written raw saves are byte-for-byte identical to matching C
artifacts for stable fixtures.

### Cycle 274: Binary Cross-Load Behavioral Gate

Run C-loaded Rust saves and Rust-loaded C saves for one minute, one day, and one
month, comparing state traces after each interval.

Validation: cross-load traces are identical at every checkpoint.

### Cycle 275: Binary Legacy And Error Parity

Match C behavior for old versions, unsupported versions, truncated files,
unknown sections, garbage bytes, empty files, and extension/content mismatches.

Validation: malformed binary transcript and error-location diff is empty.

### Cycle 276: Build Metadata Parity Policy

Eliminate banner/build-date drift by either matching C's build metadata exactly
or deriving both C and Rust fixture builds from one pinned metadata source.

Validation: no banner/date normalization is needed for C/Rust transcript diffs.

### Cycle 277: Path And Error Location Parity

Make Rust file/line error output match native C path formatting exactly under
the parity harness, including relative/absolute prefixes and source locations.

Validation: command and malformed-file transcripts diff without path
normalization.

### Cycle 278: Transcript Normalizer Removal Pass

Remove every transcript normalizer that can hide behavior: lengths, paths,
dates, line ordering, save sizes, names, state values, and error locations.
Keep only CRLF-to-LF transport cleanup if required by the runner.

Validation: raw or transport-only-normalized transcript diff is empty.

### Cycle 279: Deterministic Seed Injection For C And Rust

Add a shared seed injection mechanism to the C and Rust harnesses so startup,
reset, random names, land seeds, social actions, and save contents are
reproducible from the same test vector.

Validation: repeated C and Rust runs produce byte-identical artifacts and
transcripts.

### Cycle 280: Native C State Trace Emitter

Add C-side trace emitters for land, tide, weather, food tiles, population,
selected being, body, energy, drives, braincode, social, episodic, territory,
immune, lifecycle, and save/load state.

Validation: C trace corpus is generated directly from native execution.

### Cycle 281: Rust State Trace Emitter Exactness

Align Rust trace emission field order, formatting, widths, signedness, and
sampling points with the native C trace emitter.

Validation: one-minute C/Rust trace diff is empty.

### Cycle 282: Per-Minute Engine Order Closure

Compare C and Rust for land, population, brain, body, immune, social,
territory, lifecycle, cleanup, and save-visible state at every minute boundary.

Validation: one-day per-minute trace diff is empty.

### Cycle 283: Long Seeded Transcript Closure

Run C and Rust through identical multi-day, multi-month, and one-year command
scripts with no behavioral normalization.

Validation: long seeded transcript diff is empty.

### Cycle 284: Terrain/Food Byte Oracle Closure

Generate C byte or numeric fixtures for terrain tiles, high-definition maps,
weather, tide, food depletion, food regrowth, and food-pathogen ingestion.

Validation: Rust values match C fixtures byte-for-byte or exactly numerically.

### Cycle 285: Braincode Byte Oracle Closure

Generate C traces for opcode decode, arithmetic, sensors, actuators, probes,
scheduling, social hooks, anecdote hooks, and persistence.

Validation: Rust braincode traces match C at every instruction boundary.

### Cycle 286: Social And Episodic Oracle Closure

Generate dense C traces for meeting, attraction, chat, grooming, squabble,
mate, preference learning, stereotypes, episodic storage, and anecdote
mutation.

Validation: Rust social/episodic traces match C event-for-event.

### Cycle 287: Territory And Family Oracle Closure

Generate C traces for territory familiarity, naming, relationship inference,
mother/father/child/sibling/grandparent mapping, and social graph storage.

Validation: Rust territory/family traces match C field-for-field.

### Cycle 288: Lifecycle And Immune Oracle Closure

Generate C traces for conception, fetal genetics, gestation, birth, carrying,
suckling, weaning, death, immune mutation, pathogen spread, and immune energy
cost.

Validation: Rust lifecycle/immune traces match C field-for-field.

### Cycle 289: Movement And Body Oracle Closure

Generate C traces for walking, swimming, slope cost, water avoidance, facing,
velocity history, height, mass, fat, posture, fatigue, wounds, parasites, and
inventory.

Validation: Rust movement/body traces match C field-for-field.

### Cycle 290: Command Surface Exhaustive Matrix

Build an exhaustive C/Rust matrix for every command, alias, missing argument,
bad argument, command ordering case, quiet command, and stop/quit path.

Validation: command matrix transcript diff is empty.

### Cycle 291: Interactive Timing And EOF Parity

Match C console behavior for prompts, EOF, closed stdin, delayed output,
threaded output ordering, `stop`, and long-running commands.

Validation: PTY harness transcript diff is empty across timing scenarios.

### Cycle 292: Platform Absolute Parity Gate

Run the no-normalization parity suite on macOS and Linux with pinned compilers,
integer widths, locale, timezone, filesystem behavior, and newline policy.

Validation: platform transcripts, traces, and save bytes match the same native
C oracle.

### Cycle 293: Release/Debug/Cross-Compiler Parity

Compare Rust debug, Rust release, native C debug, native C release, and
supported compiler variants for identical observable output.

Validation: all profile/compiler transcript, trace, and save-byte diffs are
empty.

### Cycle 294: Corpus Completeness Audit

Audit the fixture corpus against every native C module reachable from
`longterm.c` command-line execution and add missing fixtures for uncovered
branches.

Validation: coverage map shows no reachable native branch without a fixture or
proof of unreachability.

### Cycle 295: Fuzzed Native Parity Closure

Run fuzzed commands, malformed saves, binary byte mutations, random seeds, and
edge populations through C and Rust, converting every mismatch into a fixture.

Validation: fuzz corpus has zero unresolved C/Rust behavioral mismatches.

### Cycle 296: Performance Without Drift

Remove any Rust slow paths introduced for raw parity while proving each
optimization leaves transcripts, traces, and save bytes unchanged.

Validation: benchmark report plus full no-normalization parity suite.

### Cycle 297: CI Absolute Parity Pipeline

Replace the current local parity CI with a required absolute parity pipeline
that builds C/Rust, regenerates fixtures, diffs raw transcripts, diffs traces,
diffs raw saves, and fails on any undocumented output.

Validation: CI script exits nonzero on any single-byte or single-line drift.

### Cycle 298: Documentation Rewrite For No-Caveat Parity

Rewrite final parity, save/load, fixture, and harness documentation to remove
accepted-drift language and describe the exact no-caveat native C oracle.

Validation: docs mention no accepted differences, substitutes, or manual
normalization allowances.

### Cycle 299: Drift Register Deletion Gate

Delete the accepted-drift register or replace it with an empty historical note
after all formerly accepted differences are fixed and fixture-proven.

Validation: repository search finds no "accepted drift" or "known difference"
language in current parity docs.

### Cycle 300: Absolute Native C Parity Signoff

Freeze Rust `simape` as absolutely native C-identical for command-line
execution: byte-for-byte raw saves, raw transcript parity, exact state trace
parity, no accepted/documented differences, and no behavior-hiding
normalization.

Validation: absolute parity CI passes repeatedly from a clean checkout and the
final signoff records zero differences.

## Cycle 266-300 Run Status

The 35-cycle implementation pass completed the strict default command-line
save/open behavior, pinned native/Rust build metadata, C-style source path
alignment, transport-only transcript diffing, and the absolute parity CI smoke
gate. The promoted exact corpus is `help`, `help_errors`, and `command_edges`.

The deeper engine and raw-save objectives from cycles 270-295 remain open
fixture tasks until direct C raw binary oracles, C state trace emitters, long
seeded raw transcript matrices, and fuzzed mismatch corpora are generated and
matched.

## Absolute Native C Parity Completion Extension: Cycles 301-365

Cycles 301-365 cover the prioritized `run forever` work plus the remaining
60-cycle estimate for closing every open fixture task left after the 266-300
strict smoke-gate pass. The target is still absolute native C parity for the
command-line `simape` reachable from `longterm.c`: `run forever` works like the
native stepping loop, raw save compatibility exists where native C exposes
bytes, exact state trace parity holds at native sampling points, raw transcript
parity requires no behavior-hiding normalization, and no documentation-only
differences are left.

### Cycle 301: Run Forever Native Semantics Capture

Capture native C `run forever` behavior against `step`, bounded `run`, `stop`,
EOF, closed stdin, and `quit` under the PTY harness, including prompts,
interleaving, output text, and state advancement checkpoints.

Validation: native C oracle transcripts and traces show the exact observable
relationship between `step`, `run forever`, `stop`, and `quit`.

### Cycle 302: Rust Run Forever Scheduler

Implement Rust `run forever` using the same simulation advancement path as
`step` and bounded `run`, preserving logging interval behavior, stop checks,
state updates, and command output ordering.

Validation: Rust unit tests prove `run forever` advances through the shared
step path and no longer emits the unsupported-command error.

### Cycle 303: Cooperative Stop, Quit, And EOF Handling

Make long-running Rust execution cooperative so `stop`, `quit`, EOF, closed
stdin, and harness timeouts leave the simulation in the same observable state
as native C.

Validation: PTY tests can start `run forever`, stop it deterministically, and
verify final prompt/output/state parity without orphaned runtime work.

### Cycle 304: Run Forever Transcript Parity Gate

Promote safe `run forever` command scripts into the strict raw transcript and
trace gates, with bounded harness controls that prevent an infinite local or CI
run.

Validation: C/Rust `run forever` transcripts and state traces diff empty under
transport-only transcript cleanup.

### Cycle 305: Run Forever Documentation And Edge Matrix

Update CLI golden documentation, command-edge tests, and parity docs so `run
forever` is treated as supported native behavior rather than a Rust-port
exception.

Validation: repository search finds no current wording that describes `run
forever` as unsupported, and the absolute parity gate includes the safe
`run forever` fixtures.

### Cycle 306: Native Binary Oracle Build Target

Add a native C oracle build target or harness mode that can produce raw save
artifacts from controlled command-line reachable states without changing the
release console behavior.

Validation: the oracle target builds from a clean checkout and writes a manifest
with compiler, date, seed, scenario, command script, and artifact hashes.

### Cycle 307: Binary Fixture Scenario Inventory

Define the complete raw-save scenario set: empty startup, reset startup,
single-being, normal population, maximum population, terrain-heavy,
social-heavy, immune-heavy, lifecycle/pregnancy, old-version, malformed, and
cross-load cases.

Validation: fixture inventory maps every scenario to the native command/script
path that produces it or records why a direct command-line path is unavailable.

### Cycle 308: Empty And Startup Raw Binary Artifacts

Generate native C raw binary artifacts for empty startup and reset startup
states, including paired transcripts and state summaries.

Validation: artifact hashes are stable across repeated pinned native C runs.

### Cycle 309: Population Raw Binary Artifacts

Generate native C raw binary artifacts for single-being, normal-population, and
maximum-population states.

Validation: population counts, selected-being fields, and artifact byte sizes
match the native manifest on repeated generation.

### Cycle 310: Domain-Heavy Raw Binary Artifacts

Generate native C artifacts for terrain-heavy, social-heavy, immune-heavy, and
lifecycle/pregnancy-heavy states.

Validation: each artifact includes a native trace summary proving the intended
domain state was reached before saving.

### Cycle 311: Binary Error And Legacy Artifacts

Capture native C behavior for old versions, unsupported versions, truncated
files, empty files, garbage bytes, unknown sections, and extension/content
mismatches.

Validation: malformed binary transcript diff is empty for every captured error
case once Rust parity is implemented.

### Cycle 312: C Layout Probe Harness

Add C probes for `sizeof`, `offsetof`, endian, signedness, padding, array
lengths, section order, and version constants for every raw-save structure.

Validation: generated layout report explains the primitive width and byte
offset of every field used by the raw artifact corpus.

### Cycle 313: Raw Binary Byte Map

Write the raw binary byte map tying native artifacts, layout probes, section
signatures, lengths, padding, and field meanings into one reviewed document.

Validation: every byte range in the approved raw-save corpus is accounted for
or explicitly classified as native padding.

### Cycle 314: Rust Raw Binary Reader Alignment

Replace the Rust framed binary reader on the native CLI path with a reader for
the actual native C raw save layout.

Validation: empty, startup, and population C raw artifacts load into Rust and
produce matching state summaries.

### Cycle 315: Rust Raw Binary State Mapping

Map land, terrain, beings, social, episodic, brain, immune, lifecycle, and
runtime fields from raw C bytes into Rust simulation state.

Validation: Rust state traces after loading every approved C artifact match the
paired native C trace summary.

### Cycle 316: Rust Raw Binary Error Parity

Match native C behavior for malformed, legacy, truncated, empty, garbage, and
extension/content mismatch cases.

Validation: C/Rust malformed save transcripts have identical output and error
locations under the strict transcript gate.

### Cycle 317: Rust Raw Binary Writer And Cross-Load Gate

Implement Rust raw binary writing for native save paths where C exposes raw
binary artifacts, then run C-loaded Rust saves and Rust-loaded C saves forward
through fixed checkpoints.

Validation: raw saves are byte-identical for stable fixtures, and cross-load
state traces match after one minute, one day, and one month.

### Cycle 318: C State Trace Emitter API

Add a native C trace emitter API that can record deterministic state snapshots
without perturbing simulation order or command output.

Validation: enabling trace output leaves native CLI transcripts unchanged.

### Cycle 319: C Trace Harness Integration

Integrate the C trace emitter with the command-line harness so each fixture can
emit raw transcript, state trace, save artifacts, and a single manifest entry.

Validation: one command regenerates C transcript, trace, and artifact outputs
for the promoted smoke corpus.

### Cycle 320: Land, Tide, And Weather Trace Fields

Emit native trace fields for land date/time, tide, weather, dawn/dusk, map
metadata, and high-level terrain state.

Validation: Rust trace output matches field order, formatting, signedness, and
values for startup and one-day fixtures.

### Cycle 321: Terrain And Food Tile Trace Fields

Emit native trace fields for tile-map values, food availability, food
depletion, food regrowth, water, slope, and terrain sampling.

Validation: Rust terrain and food traces match C numerically at every sampled
tile and time step.

### Cycle 322: Population And Selection Trace Fields

Emit native trace fields for population count, selected being, names, honor,
location, facing, velocity history, and alive/dead state.

Validation: Rust population trace output matches C for empty, reset, and short
run fixtures.

### Cycle 323: Body, Energy, And Drive Trace Fields

Emit native trace fields for body size, mass, fat, energy, awake state,
drives, posture, fatigue, wounds, parasites, inventory, and goal state.

Validation: Rust body and drive traces match C through walking, resting,
eating, and injury fixtures.

### Cycle 324: Braincode Instruction Trace Fields

Emit native per-instruction trace fields for braincode decode, program counter,
registers, constants, arithmetic, memory mutation, and control flow.

Validation: Rust braincode traces match C at every instruction boundary for
approved VM fixtures.

### Cycle 325: Braincode Hook Trace Fields

Emit native trace fields for sensors, actuators, probes, shout, anecdote hooks,
social hooks, scheduling frequency, and internal/external braincode passes.

Validation: Rust hook traces match C for sensor, actuator, probe, and social
braincode fixtures.

### Cycle 326: Social And Episodic Trace Fields

Emit native trace fields for meeting, attraction, chat, grooming, squabble,
mate behavior, episodic storage, anecdote mutation, and preference updates.

Validation: Rust social and episodic event traces match C event-for-event.

### Cycle 327: Territory, Family, And Lifecycle Trace Fields

Emit native trace fields for territory familiarity, territory naming, family
relationship inference, conception, pregnancy, birth, carrying, suckling, and
weaning.

Validation: Rust territory, family, and lifecycle traces match C
field-for-field.

### Cycle 328: Immune And Pathogen Trace Fields

Emit native trace fields for antigens, antibodies, pathogen shapes, mutation,
transmission routes, severity, immune energy cost, and recovery.

Validation: Rust immune traces match C for air, touch, sex, food, and mutation
fixtures.

### Cycle 329: Movement, Body Action, And Inventory Trace Fields

Emit native trace fields for walking, swimming, water avoidance, slope cost,
body actions, pickup/drop, brandish, drag, bash, chew, fish, and visible shout
state.

Validation: Rust movement and body-action traces match C across targeted action
fixtures.

### Cycle 330: Save/Load Trace Checkpoints

Emit native trace checkpoints immediately before save, immediately after load,
and after post-load runtime advancement.

Validation: C/Rust traces match before save, after load, and after one minute
of continued simulation for every save/load fixture.

### Cycle 331: Rust Trace Formatter Exactness Gate

Align Rust trace field order, separators, integer widths, signedness, float or
fixed-point formatting, and sampling points with the C emitter.

Validation: the strict trace diff gate rejects any formatting drift and passes
for the full promoted trace corpus.

### Cycle 332: Native Terrain Tile-Map Parity

Close drift in terrain tile-map generation, map wrapping, sampling,
high-definition map values, water bands, slopes, and location conversion.

Validation: C/Rust terrain tile traces match across fixed seeds and sampled map
regions.

### Cycle 333: Food Depletion And Regrowth Parity

Close drift in food selection, food bite size, depletion, regrowth,
absorption, nutritional effect, and save-visible food state.

Validation: C/Rust food traces match over multi-day eat/regrow fixtures.

### Cycle 334: Weather, Tide, And Water Behavior Parity

Close drift in weather transitions, tide effects, dawn/dusk bands, swimming
state, water avoidance, and terrain-dependent energy costs.

Validation: C/Rust weather, tide, water, and movement-energy traces match over
multi-day fixtures.

### Cycle 335: Braincode Decode And Arithmetic Closure

Close remaining drift in opcode decode, address wrapping, constant flags,
arithmetic, byte mutation, register writes, and control operators.

Validation: C/Rust per-instruction braincode traces match for decode and
arithmetic fixtures.

### Cycle 336: Braincode Sensor And Actuator Closure

Close remaining drift in body, drive, social, episodic, terrain, weather,
immune, territory, action, friend/foe, probe, shout, posture, and preference
sensors/actuators.

Validation: C/Rust sensor and actuator traces match at every instruction
boundary.

### Cycle 337: Braincode Scheduling And Social Hook Closure

Close drift in dialogue iteration counts, internal/external scheduling, probe
frequency, social-entry braincode behavior, and chat-triggered hooks.

Validation: C/Rust scheduling and social braincode traces match over short and
multi-day fixtures.

### Cycle 338: Social Meeting, Chat, And Attraction Closure

Close drift in meeting selection, attraction, mate bonds, chat topics,
stereotypes, familiarity, friend/foe changes, and honor side effects.

Validation: C/Rust social traces match event-for-event under dense social
fixtures.

### Cycle 339: Episodic, Anecdote, And Preference Learning Closure

Close drift in episodic memory storage, anecdote mutation, preference learning,
mate preference updates, remembered events, and command-visible summaries.

Validation: C/Rust episodic and preference traces match and CLI detail commands
print identical rows.

### Cycle 340: Territory Naming And Familiarity Closure

Close drift in territory familiarity indexing, rescaling, naming, agreement,
territory attention, and save/load persistence.

Validation: C/Rust territory traces and post-load summaries match
field-for-field.

### Cycle 341: Family Relationships And Pregnancy Closure

Close drift in mother/father/child/sibling/grandparent inference, conception
dates, fetal genetics, gestation, pregnancy state, and relationship storage.

Validation: C/Rust family and pregnancy traces match through conception and
pre-birth fixtures.

### Cycle 342: Birth, Carrying, And Suckling Closure

Close drift in birth creation, population capacity, child initialization,
carrying, suckling, weaning, mother/child energy transfer, and birth events.

Validation: C/Rust lifecycle traces match through birth and early-childhood
fixtures.

### Cycle 343: Immune Cycling And Transmission Closure

Close drift in universal immune cycling, air/touch/sex/food transmission,
antibody mutation, antigen severity, immune cost, and pathogen persistence.

Validation: C/Rust immune traces match over targeted transmission and recovery
fixtures.

### Cycle 344: Movement, Body, And Awake-State Closure

Close drift in awake-state calculation, walking, swimming, slope energy,
velocity history, body action timing, fatigue, posture, wounds, parasites, and
inventory-visible behavior.

Validation: C/Rust body and movement traces match over action-heavy seeded
fixtures.

### Cycle 345: Deep Engine Integrated Seed Matrix

Run integrated seeded fixtures that exercise terrain, food, braincode, social,
lifecycle, immune, movement, save/load, and CLI detail output together.

Validation: integrated C/Rust traces and transcripts match with no open engine
drift items.

### Cycle 346: Long Seeded Command Corpus

Define long seeded command scripts covering multi-day, multi-month, one-year,
save/open continuity, populated details, navigation, and runtime edge cases.

Validation: every script has a native C transcript, trace manifest, timeout,
seed, date, and safety classification.

### Cycle 347: Multi-Day Raw Transcript Gate

Promote multi-day command scripts into the transport-only raw transcript diff
gate.

Validation: C/Rust multi-day transcripts diff empty without behavior-hiding
normalization.

### Cycle 348: Multi-Month Raw Transcript Gate

Promote multi-month command scripts into the transport-only raw transcript diff
gate.

Validation: C/Rust multi-month transcripts diff empty and remain stable across
repeated pinned runs.

### Cycle 349: Save/Open Runtime Continuity Matrix

Run long save/open loops across native text, raw binary, malformed files, and
post-load runtime advancement.

Validation: C/Rust transcripts, traces, and save artifacts match at every
checkpoint.

### Cycle 350: Exhaustive Command Surface Matrix

Expand strict command coverage for every command, alias, missing argument, bad
argument, command ordering case, quiet command, output command, and stop/quit
path.

Validation: exhaustive command matrix transcript diff is empty.

### Cycle 351: Interactive Timing, EOF, And Stop Gate

Match native C behavior for prompts, EOF, closed stdin, delayed output,
threaded output ordering, stop during long runs, and interrupted commands.

Validation: PTY timing transcripts match C across the promoted timing
scenarios.

### Cycle 352: Runtime Edge-Case Transcript Gate

Promote command scripts for no selected ape, empty population, selected ape
changes, reset/clear ordering, logging/event combinations, and detail-command
edge cases.

Validation: runtime edge-case transcripts diff empty and have matching state
traces where state changes are expected.

### Cycle 353: Long Transcript Corpus Signoff

Review and freeze the long transcript corpus, removing redundant scripts and
promoting every remaining script into the absolute parity gate.

Validation: one strict command regenerates and diffs the complete promoted raw
transcript corpus.

### Cycle 354: Fuzz Harness Determinism

Build deterministic C/Rust fuzz harness wrappers with shared seed injection,
timeouts, artifact capture, and reproducible minimization.

Validation: the same fuzz seed produces the same command stream and artifact
names across repeated runs.

### Cycle 355: Command Grammar Fuzzing

Fuzz command names, aliases, whitespace, arguments, bad formats, ordering, EOF,
and stop/quit sequences against C and Rust.

Validation: every mismatch is minimized into a fixture or closed as
unreachable from the native command-line path.

### Cycle 356: Malformed Save Fuzzing

Fuzz native text, JSON-looking files, empty files, truncated files, random
bytes, and mixed extension/content save inputs.

Validation: malformed save mismatch corpus has zero unresolved C/Rust
behavioral differences.

### Cycle 357: Raw Binary Byte Mutation Fuzzing

Fuzz raw native binary byte mutations across signatures, lengths, versions,
counts, padding, arrays, and domain-heavy sections.

Validation: Rust raw binary loader behavior matches C for every minimized
mutation fixture.

### Cycle 358: Seed And Population Fuzzing

Fuzz deterministic seeds, population sizes, reset/run lengths, selected ape
states, and edge population capacities.

Validation: C/Rust transcripts, traces, and save outputs match for the promoted
seed/population fuzz corpus.

### Cycle 359: Fuzz Corpus Triage Closure

Convert every fuzz mismatch into a regression fixture, then close or implement
the corresponding parity fix.

Validation: fuzz corpus runs clean with zero unresolved mismatch records.

### Cycle 360: macOS Absolute Parity Gate

Run the full absolute parity suite on macOS with pinned locale, timezone,
compiler, filesystem behavior, and newline policy.

Validation: macOS transcripts, traces, and save bytes match the native C oracle.

Run status: completed for the promoted 351-360 scope. The macOS absolute parity
pipeline now includes strict raw transcript coverage for empty runtime/detail
commands, exact EOF-after-command and stop timing coverage, closed-stdin timing
triage, deterministic command fuzz, malformed missing-file save fuzz,
empty-file loader triage, raw binary mutation triage, and seed/population
repeatability artifacts. Raw binary value diffs, malformed loader diagnostics,
closed-stdin determinism, and seeded population C/Rust value diffs remain
carryover work until their native oracles are promoted.

### Cycle 361: Linux Absolute Parity Gate

Run the full absolute parity suite on Linux with pinned locale, timezone,
compiler, filesystem behavior, and newline policy.

Validation: Linux transcripts, traces, and save bytes match the native C oracle
or expose platform issues that are fixed before signoff.

### Cycle 362: Profile And Compiler Matrix Gate

Compare Rust debug, Rust release, native C debug, native C release, and
supported compiler variants.

Validation: all profile/compiler transcript, trace, and save-byte diffs are
empty.

### Cycle 363: Required Absolute Parity CI Pipeline

Promote the full absolute parity suite into one required pipeline that builds
C/Rust, regenerates fixtures, diffs raw transcripts, diffs traces, diffs raw
saves, and fails on any drift.

Validation: CI exits nonzero on a deliberate one-byte save change, one-line
transcript change, and one-field trace change.

### Cycle 364: Final Documentation And Register Rewrite

Rewrite final parity, save/load, fixture, harness, drift, release, and
completion docs so they describe the exact native C oracle and point to the
required gates.

Validation: repository search finds no current signoff language that permits a
known difference, undocumented substitute, or behavior-hiding normalization.

### Cycle 365: Absolute Native C Parity Interim Signoff

Freeze the completed promoted command-line oracle before the no-triage
extension: raw transcript parity for promoted sessions, exact state trace schema
parity, raw save compatibility where already exposed, clean deterministic fuzz
gates, and platform/profile/compiler coverage.

Validation: the absolute parity pipeline passes repeatedly from a clean checkout
and the signoff records every remaining triage item as explicit cycle 366-400
work.

## Absolute Native C No-Triage Extension: Cycles 366-400

Cycles 366-400 close the remaining accepted-difference and triage buckets left
after the 351-360 gate work. The target is no documented differences left:
closed-stdin behavior is deterministic or precisely normalized by native-backed
rules, malformed loader diagnostics match native C, raw binary byte/value
oracles are promoted, seeded population and engine trace values match C, and the
long seeded, multi-day, save/open, and exhaustive command corpora become exact
absolute-gate fixtures.

### Cycle 366: Closed-Stdin Native Reproduction Harness

Instrument native C closed-stdin behavior under redirected stdin, PTY stdin,
empty command files, and immediate EOF to determine whether nondeterminism comes
from console thread scheduling, buffering, process teardown, or harness timing.

Validation: native closed-stdin oracle report captures all observed output
classes with reproduction commands and frequencies.

### Cycle 367: Closed-Stdin Stabilization Strategy

Choose the parity strategy for closed stdin: native C patch-equivalent behavior,
strict harness timing that makes native deterministic, or an explicit
native-derived output-class oracle that Rust must match.

Validation: strategy document identifies exactly one accepted gate behavior and
removes the current open-ended closed-stdin triage wording.

### Cycle 368: Rust Closed-Stdin Exact Behavior

Implement Rust stdin shutdown behavior to match the selected native oracle,
including banner flushing, console failure count, exit code expectations, and
interaction with `quit`, EOF after a command, and empty input.

Validation: Rust unit and process tests match the selected closed-stdin oracle.

### Cycle 369: Closed-Stdin Transcript Gate Promotion

Promote closed-stdin cases from timing triage into the strict interactive timing
gate.

Validation: `scripts/run_interactive_timing_gate.sh` diffs closed stdin exactly
instead of only capturing artifacts.

### Cycle 370: EOF And Signal Timing Matrix

Extend the timing matrix to cover EOF after quiet commands, EOF after verbose
commands, EOF during run, `stop` during run forever, SIGINT-like interruption
where reachable, and repeated stdin close/open harness behavior.

Validation: timing transcripts are exact or have native-backed deterministic
rules, with no triage-only cases.

Run status: completed for the promoted 361-370 scope. Added platform,
profile/compiler, required-pipeline, and failure-smoke gates; added a repeated
closed-stdin native oracle; promoted closed stdin and quiet/bounded-run EOF from
triage into native-backed exact timing canonicalizers; and expanded the
interactive timing matrix to quiet EOF, verbose EOF, bounded-run EOF,
closed stdin, and PTY stop-after-forever. Remaining no-triage work starts at
cycle 371 with regression locks, malformed loader diagnostics, raw binary byte
oracles, seeded engine value parity, and pending corpus promotion.

### Cycle 371: Closed-Stdin Regression Lock

Add regression fixtures that fail on one missing or extra console-failure line,
missing banner flush, incorrect exit handling, or drift in stop/quit ordering.

Validation: deliberate closed-stdin output mutations fail the timing gate.

### Cycle 372: Malformed Loader Diagnostic Inventory

Inventory native C diagnostics for empty, missing, JSON-looking, random text,
truncated native text, truncated binary, bad signature, wrong version, wrong
section order, invalid counts, and garbage suffix/prefix inputs.

Validation: malformed loader oracle manifest records exact stdout/stderr,
process exit, and file parser location behavior for every fixture.

### Cycle 373: Native Loader Diagnostic Capture Split

Separate native stdout, stderr, malloc diagnostics, parser debug output, and
command transcript output into stable artifact channels without losing their
relative ordering where native exposes it.

Validation: repeated native malformed-loader runs produce stable categorized
artifacts or a documented native output-class oracle for volatile allocator
lines.

### Cycle 374: Rust Native Text Parser Error Detail

Extend Rust native text parsing to emit the same parser diagnostics as C for
unknown commands, bad signature placement, bad section names, string-length
reports, and failed token text.

Validation: bad JSON/random text/truncated text fixtures match native parser
diagnostic output.

### Cycle 375: Empty And Truncated File Loader Parity

Close exact behavior for empty files, truncated native text that C partially
accepts, truncated binary-looking files, and premature EOF in section bodies.

Validation: empty/truncated loader fixtures move from triage into exact C/Rust
diffs.

### Cycle 376: Malformed Loader Exit And Recovery Semantics

Match whether native continues accepting commands, aborts, stops the simulation,
or leaves partial state after each malformed load category.

Validation: post-load `sim`, `quit`, and second-load transcripts match native
for every malformed fixture.

### Cycle 377: Malformed Loader Fuzz Minimization

Run deterministic malformed loader fuzzing, minimize every mismatch, and turn
each unique mismatch into a named fixture with a native oracle artifact.

Validation: malformed loader fuzz has zero unfixtured mismatches.

### Cycle 378: Malformed Loader Gate Promotion

Replace malformed-save triage with exact native/Rust diffing for all promoted
malformed loader fixtures.

Validation: `scripts/run_malformed_save_fuzz.sh` reports no triage bucket.

### Cycle 379: Native Raw Binary Byte Oracle Target

Add or finalize a native C oracle path that writes reachable raw binary saves
for empty startup, reset startup, single-being, normal population, and
edge-capacity population scenarios.

Validation: native raw binary artifacts are generated reproducibly with hashes.

### Cycle 380: Raw Binary Byte Map Completion

Map every byte in the native raw binary corpus to C structures, scalar fields,
array counts, padding, endian behavior, and version-dependent sections.

Validation: byte map has no unknown ranges for promoted raw binary artifacts.

Run status: completed for the promoted 371-380 scope. Added timing regression
locks, including EOF-after-verbose-command canonicalization for native optional
console-failure lines; promoted malformed missing, empty, bad JSON, random text,
and truncated native text fixtures into exact C/Rust diffs; added split
stdout/stderr/combined malformed-loader inventory captures for unpromoted parser
cases; added native parser diagnostic detail in Rust for non-native text
prefixes; matched native partial truncated-land load recovery; and added a
direct C `tranfer_out()` raw byte oracle with reproducible hashes and token byte
maps. Remaining no-triage work starts at cycle 381 with raw byte reader/value
closure, raw binary mutation behavior, writer byte closure, seeded engine value
parity, and pending corpus promotion.

### Cycle 381: Rust Raw Binary Reader Value Closure

Update the Rust raw binary reader so every promoted native artifact loads into a
Rust state with matching land, population, being, social, episodic, territory,
immune, braincode, preference, and inventory values.

Validation: C artifact load comparisons report zero field mismatches.

### Cycle 382: Raw Binary Error Behavior Parity

Match native C behavior for bad raw binary signatures, versions, short reads,
long reads, invalid counts, wrong offsets, and mutation fuzz fixtures.

Validation: raw binary mutation fuzz becomes exact native/Rust behavior diffing.

### Cycle 383: Rust Raw Binary Writer Byte Closure

Update Rust raw binary writing so Rust-produced artifacts match native C byte
layout for all promoted scenarios where command-line C exposes raw bytes.

Validation: Rust save artifacts are byte-for-byte identical to native artifacts.

### Cycle 384: Raw Binary Cross-Load Matrix

Run C-loads-Rust, Rust-loads-C, C-save-C-load, Rust-save-Rust-load, and
save-run-save round trips across the promoted binary scenarios.

Validation: state traces and save bytes match at every cross-load checkpoint.

### Cycle 385: Raw Binary Version And Platform Matrix

Verify raw binary behavior across supported native versions, debug/release
profiles, macOS/Linux newline/filesystem differences, and compiler variants.

Validation: raw binary artifacts and loader diagnostics remain identical across
the supported matrix or fail with a fixed platform-specific oracle.

### Cycle 386: Raw Binary Absolute Gate Promotion

Wire raw binary byte generation, byte diffing, cross-load checks, and mutation
fuzz into the absolute parity pipeline.

Validation: raw binary has no remaining triage-only gate or documented drift.

### Cycle 387: Seeded Population Native Value Oracle

Generate direct native C engine trace values for fixed seeds, reset cycles,
population sizes, selected ape states, and bounded run lengths used by the
pending long transcript corpora.

Validation: native trace values are reproducible and cover all promoted corpus
entry points.

### Cycle 388: Startup And Reset Population Value Parity

Close Rust drift in startup/reset population initialization, selected-being
choice, names, sexes, ages, genetics, drives, honor, location, and initial
social state.

Validation: startup/reset C/Rust engine trace values match for promoted seeds.

### Cycle 389: Terrain And Weather Engine Value Parity

Close terrain/topography, tide, weather, moving-sun biology, water, slope, and
food-source trace value mismatches that affect seeded transcript output.

Validation: terrain/weather direct trace values match native C across promoted
seeds and times.

### Cycle 390: Food And Inventory Engine Value Parity

Close food depletion/regrowth, bite quantity, nutrition, inventory, pathogen
ingestion, and local biology value drift.

Validation: food/inventory trace values and hungry runtime transcripts match C.

Run status: completed for the promoted 381-390 scope at the native raw value
gate level. Added Rust loading for direct C `tranfer_out()` streams that omit
`landd{}`; added inline raw `terit=` value parsing; added C-side raw value
summaries; added `simape --native-raw-summary` and
`simape --native-raw-roundtrip`; and wired
`scripts/run_native_raw_binary_value_gate.sh` into the absolute pipeline. Empty
startup and reset startup raw streams roundtrip byte-for-byte. The populated
after-one-cycle raw stream is value-exact for the promoted summary fields, with
raw territory/padding byte preservation carried forward before populated raw
writer byte closure can be declared complete.

### Cycle 391: Body Movement Awake-State Value Parity

Close movement, speed, posture, awake level, fatigue, energy, body actions,
wounds, parasites, and swimming/walking state value drift.

Validation: body/movement direct trace values match native C over seeded runs.

### Cycle 392: Braincode And Social Value Parity

Close braincode execution, social hooks, episodic updates, preference learning,
territory, family, pregnancy, and relationship value drift exposed by native
engine traces.

Validation: braincode/social/family trace values match native C over promoted
seeded scenarios.

### Cycle 393: Seeded Population Save/Load Value Parity

Verify that save/open preserves every seeded population value before and after
runtime advancement in native text and raw binary paths.

Validation: pre-save, post-open, and post-run traces match C for every promoted
seed.

### Cycle 394: Engine Value Gate Promotion

Promote direct C/Rust engine trace value comparisons into the absolute parity
pipeline, replacing schema-only checks for covered fields.

Validation: engine trace value gate fails on a deliberate one-field drift.

### Cycle 395: Long Seeded Transcript Promotion

Promote the long seeded command corpus into the strict raw transcript gate once
engine value drift is closed.

Validation: long seeded C/Rust transcript diff is empty without behavior-hiding
normalization.

### Cycle 396: Multi-Day And Multi-Month Transcript Promotion

Promote multi-day and multi-month runtime matrices into the strict raw
transcript gate.

Validation: multi-day and multi-month transcripts, traces, and save checkpoints
match native C.

### Cycle 397: Save/Open Continuity Corpus Promotion

Promote save/open runtime continuity scripts across native text, raw binary,
malformed recovery, and post-load advancement into exact gates.

Validation: C/Rust transcripts, traces, and save artifacts match at every
continuity checkpoint.

### Cycle 398: Exhaustive Command Surface Promotion

Promote the exhaustive command-surface corpus, including aliases, missing
arguments, bad arguments, quiet commands, file-producing commands, and edge
ordering cases.

Validation: exhaustive command surface raw transcript diff is empty.

### Cycle 399: Pending Corpus Consolidation And Redundancy Removal

Remove redundant pending scripts, merge overlapping cases, and ensure every
remaining corpus fixture is either exact-gated or deleted.

Validation: repository search finds no `pending-*`, `triage-only`, or
`documented difference` corpus status for command-line parity work.

### Cycle 400: No-Triage Absolute Native C Parity Signoff

Run the final absolute parity pipeline from a clean checkout and freeze Rust
`simape` as native C-identical for the command-line oracle: raw transcripts,
direct engine trace values, raw save bytes, malformed diagnostics, fuzz corpus,
timing behavior, and platform/profile/compiler matrix.

Validation: the final signoff records zero accepted/documented differences and
zero remaining triage buckets.

Run status: completed for the promoted 391-400 scope as a scoped engine-value
and corpus-inventory pass, not as final absolute parity. Added
`scripts/run_engine_trace_value_gate.sh`, which compares the C/Rust direct
engine trace startup value subset plus invariant zero-valued runtime fields and
proves the gate fails on a deliberate one-field mutation. Wired that value gate
into `scripts/run_absolute_parity_ci.sh`. Added
`scripts/run_pending_corpus_inventory.sh` to classify the long seeded,
multi-day, multi-month, save/open continuity, and exhaustive command-surface
corpora by concrete blockers. Carryover remains for reset/startup population
semantics, terrain/weather/food runtime values, selected-being body/brain/social
runtime values, save/open continuity, raw populated territory bytes, and
promotion of the five pending command corpora.

### Cycle 401: Direct Startup Cycle Lifecycle Parity

Align the Rust direct engine trace fixture with native C's first `sim_cycle()`
from `KIND_START_UP`, including pending-execution initialization before the
first normal minute cycle.

Validation: after-cycle direct engine trace samples reach the native maturity
clock and populated runtime phase.

### Cycle 402: First-Cycle Population Core Gate

Promote C/Rust value comparison for first-cycle date, time, tide, population,
selected-being presence, and zero-valued invariant body/family/inventory fields.

Validation: engine trace value gate includes first-cycle runtime-core fields.

### Cycle 403: First-Cycle Social Baseline Gate

Promote exact first-cycle social, episodic, territory, and preference baseline
fields where the native and Rust direct probes now agree.

Validation: first-cycle baseline fields fail on deliberate mutation and pass in
the normal value gate.

### Cycle 404: Native Initial Preference Parity

Initialize generated population learned preferences to the native neutral value
instead of the transfer-default zero value.

Validation: first-cycle preference traces match native C.

### Cycle 405: Runtime-Core Day-Rollover Gate

Promote day-rollover runtime-core fields after the one-day direct probe:
date/time, tide, population, selected-being presence, and invariant body fields.

Validation: after-day runtime-core values match native C in the value gate.

### Cycle 406: CLI Reset Blocker Separation

Separate direct engine startup-cycle parity from still-open CLI reset behavior
so pending corpus inventory reports the remaining blocker precisely.

Validation: pending corpus inventory names CLI reset/startup population drift.

### Cycle 407: Terrain/Food Drift Narrowing

Keep terrain, weather, food, and selected-being random/body values outside the
promoted gate while recording them as the next concrete value-drift categories.

Validation: engine value manifest lists only the remaining unpromoted runtime
value categories.

### Cycle 408: Absolute Pipeline Re-Promotion

Run the absolute parity pipeline with the expanded engine value subset promoted.

Validation: absolute parity CI passes with the larger engine value gate.

### Cycle 409: Documentation And Corpus Status Update

Update native trace, corpus, drift, and parity-status documents to reflect the
expanded direct engine value coverage and remaining blockers.

Validation: manifests and docs agree on promoted and open categories.

### Cycle 410: Cycle Report And Signoff

Record the 401-410 cycle report and verification results.

Validation: cycle report lists completed work, carryover, and validation
commands.

Run status: completed for the promoted 401-410 scope. Added
`SimState::advance_native_engine_cycle()` so the Rust direct engine trace enters
the same pending startup lifecycle phase as native C's first `sim_cycle()`;
expanded `scripts/run_engine_trace_value_gate.sh` to compare first-cycle
baseline values and runtime-core date/time/tide/population fields; initialized
generated population preferences to native neutral `127`; clarified the
remaining pending-corpus blocker as CLI reset/startup population drift; and
updated the absolute pipeline/documentation around the expanded value subset.
Remaining drift is terrain/weather/food generation, selected-being random
identity/body-energy values, brain/probe runtime state, social/immune runtime
state, save/open continuity, CLI reset timing, and populated raw territory byte
preservation.

## Cycle 411-500 Development Plan

The next 90-cycle completion plan is recorded in
`RUST_PORT_CYCLE_411_500_PLAN.md`. It preserves the requested 80 cycles across:

- terrain/topography, weather, tide-side effects, and food value parity
- selected-being identity, random state, movement/body/energy/drives parity
- brain/probe, social, episodic, and immune runtime parity
- CLI reset/startup timing and population semantics
- save/open runtime continuity and post-load advancement parity
- populated raw transfer byte-exactness, especially raw territory bytes
- promotion of long seeded, multi-day/month, save/open, and exhaustive command
  corpora into exact gates

The final 10 cycles, 491-500, are reserved for cross-platform/profile
convergence, repeated clean absolute pipeline runs, drift-register cleanup, and
absolute native C parity signoff.

## Cycle 411-420 Run Status

The 10-cycle implementation pass expanded the direct native C/Rust engine trace
with multi-location `TERRAIN` rows and added
`scripts/run_terrain_food_value_inventory.sh`. The new inventory covers eight
ape-space coordinates across startup, first-cycle, and one-day snapshots,
including map height, topography, high-definition topography, high-resolution
tide class, weather-seven values, pressure, grass/tree/bush operators,
seaweed/rockpool/beach operators, and native food-source classification. The
Rust side now exposes unscaled food classification through
`LandState::food_classification_at`.

This pass is an inventory and fixture-closure pass, not final terrain parity.
The current inventory reports 24 sampled terrain rows with 48 terrain-row
mismatch lines and six trace-level terrain mismatch lines. It explicitly
identifies the remaining native engine work: C-shaped startup land/weather
ordering, `tile_land_init` patch/round/swap terrain generation, weather
atmosphere initialization/cycling, and exact food value promotion.

## Cycle 421-430 Run Status

The 10-cycle implementation pass promoted exact startup and first-cycle
terrain/weather/food parity. Rust now follows the native first-cycle land path:
zeroed startup terrain, `land_clear` primary-buffer packing, `tile_land_init`
patching and smoothing, primary-to-working topography copy behavior, C
double-spread high-definition trace sampling, native first-cycle weather
initialization/cycling, and C salt-operator semantics for food biology.

Added `scripts/run_terrain_food_first_cycle_gate.sh` and wired it into the
absolute parity pipeline. `scripts/run_engine_trace_value_gate.sh` now promotes
the trace-level `terrain-first-cycle` subset. The terrain-food inventory dropped
from 48 terrain mismatch lines and six trace-level mismatch lines to 16 and two,
respectively; the remaining inventory drift is after-day/multi-day weather
evolution. The native terrain path also exposed selected-being honor drift after
one day, so honor is tracked as an open selected-being/social value rather than
a promoted runtime-core invariant until cycles 431-457 close it.

## Cycle 301-320 Run Status

The 20-cycle implementation pass completed the prioritized Rust `run forever`
path, including cooperative `stop`/`quit` behavior and a safe strict
raw-transcript fixture. It also added clean-cwd raw transcript execution,
native save artifact capture, a binary scenario inventory, a standalone C
layout probe, and command-visible native trace extraction.

The raw binary reader/writer replacement and direct C in-engine trace emitters
from cycles 306-320 remain open carryover work. The new harnesses provide the
inputs needed to close those items in the next parity cycles.

## Cycle 321-340 Run Status

The 20-cycle implementation pass added a direct native C engine trace probe,
the matching Rust engine trace formatter schema, and a schema gate for C/Rust
trace field order. The direct trace schema now spans terrain, food, weather,
population, selected-being body state, drives, braincode, social, episodic,
territory, lifecycle/family, immune, movement, inventory, shout, and preference
fields.

The pass does not claim exact value parity for those fields. The new C probe
exposes remaining drift in startup cycling, terrain/topography, weather,
food biology, population initialization, and selected-being state; those are
carryover targets for the following cycles.

## Cycle 341-350 Run Status

The 10-cycle implementation pass advanced the deep engine and command-surface
closure items:

- C-shaped child genetics crossover/mutation/transpose is now used for fetal
  genetics.
- Pregnancy/birth timing, parent/grandparent/sibling relationship creation,
  carrying, suckling, maternal antibody transfer, and lifecycle episodic events
  are represented in executable Rust behavior.
- Air/touch/sex pathogen transmission now spreads existing pathogens between
  beings in addition to environmental acquisition.
- Awake/body movement uses the native stopped-hunger threshold and swimming
  hungry-speed case.
- `alpha` and selected-being `speak` now produce AIFF containers without console
  output, `epic` produces populated rankings, and `top`/`epic` honor
  female/male/juvenile filters.
- Pending long seeded, multi-day, multi-month, save/open continuity, and
  exhaustive command-surface scripts were added for later raw transcript
  promotion.

Validation completed with `cargo fmt --all`, `cargo test -p apesdk-sim --lib`,
and `cargo test -p simape --lib`. Exact direct C value comparison remains the
carryover gate before the new command corpora are promoted into the absolute
raw transcript suite.

---

<a id="rust-port-cycle-266-300-planmd"></a>

## RUST_PORT_CYCLE_266_300_PLAN.md

Original file: `RUST_PORT_CYCLE_266_300_PLAN.md`

# Rust Port Cycles 266-300 Plan

These 35 cycles extend the completed 265-cycle plan to remove every accepted
difference from the native C oracle.

## Objective

Reach absolute native C parity for command-line `simape`:

- default Rust save/open behavior matches native C exactly
- Rust writes and reads raw native C save bytes, not a substitute frame
- transcript diffs require no behavior-hiding normalization
- C and Rust state traces match at native sampling points
- final docs contain no documentation-only caveats

## Cycle Range

Cycles 266-300 are now appended to
`RUST_PORT_DEVELOPMENT_PLAN_106_205.md`.

## Completion Target

Completion means the final absolute parity gate passes repeatedly from a clean
checkout and records zero differences against the native C command-line oracle.

## Run Summary

The cycle run added the strict transport-only diff gate, pinned native C/Rust
build metadata, moved default Rust command-line save/open behavior onto the C
path, and replaced documentation-only parity closure with fixture-task based
parity closure.

## Validation From The 35-Cycle Run

- `cargo fmt --all --check`
- `cargo test`
- `scripts/run_raw_transcript_diff.sh /private/tmp/apesdk_raw_diff_266_300 help help_errors command_edges`
- `scripts/run_absolute_parity_ci.sh /private/tmp/apesdk_absolute_parity_266_300`

## Remaining Fixture Tasks

The promoted exact command-line corpus now passes without behavior-hiding
normalization. Deeper native engine categories still need direct C oracle
promotion before they can be included in the same absolute signoff:

- raw native binary artifact generation and byte-map closure
- C state trace emitters for terrain, food, braincode, social, lifecycle,
  immune, movement, and save/load checkpoints
- long seeded raw transcript matrices beyond the promoted smoke corpus
- fuzzed command/save/load mismatch promotion

---

<a id="rust-port-cycle-301-400-planmd"></a>

## RUST_PORT_CYCLE_301_400_PLAN.md

Original file: `RUST_PORT_CYCLE_301_400_PLAN.md`

# Rust Port Cycles 301-400 Plan

These 100 cycles put `run forever` first, then cover the remaining work after
the 266-300 strict smoke-gate pass and the 35-cycle no-triage extension added
after cycle 360. The authoritative cycle-by-cycle details are
appended to
`RUST_PORT_DEVELOPMENT_PLAN_106_205.md`.

## Objective

Finish absolute native C parity for command-line `simape` reachable from
`longterm.c`: `run forever` support matching the native stepping loop, raw save
compatibility where native C exposes bytes, exact state trace parity, raw
transcript parity without behavior-hiding normalization, fuzz-clean
command/save/load behavior, and no documentation-only differences left.

## Range Breakdown

- Cycles 301-305: prioritized `run forever` implementation, cooperative
  stop/quit/EOF handling, and strict transcript/trace parity gates.
- Cycles 306-317: native raw binary oracle, byte map, Rust raw reader/writer,
  and cross-load gates.
- Cycles 318-331: native C state trace emitters and Rust trace formatting
  exactness.
- Cycles 332-345: deep engine parity for terrain, food, braincode, social,
  episodic, territory, family, lifecycle, immune, movement, and awake/body
  logic.
- Cycles 346-353: long seeded raw transcript matrices, exhaustive command
  surface coverage, and interactive timing/EOF/stop behavior.
- Cycles 354-359: deterministic fuzzing for commands, malformed saves, raw
  binary mutations, seeds, and populations.
- Cycles 360-363: macOS/Linux/profile/compiler absolute parity gates and the
  required CI pipeline.
- Cycles 364-365: interim documentation rewrite and promoted-gate signoff.
- Cycles 366-371: closed-stdin native nondeterminism closure and exact timing
  gate promotion.
- Cycles 372-378: malformed loader diagnostic parity and no-triage malformed
  save/load fuzz gates.
- Cycles 379-386: raw binary byte/value oracle closure, reader/writer byte
  parity, cross-load, mutation, and absolute-gate promotion.
- Cycles 387-394: seeded population and direct engine trace value parity for
  terrain, food, body, movement, braincode, social, family, and save/load.
- Cycles 395-400: promote long seeded, multi-day, save/open continuity, and
  exhaustive command corpora into exact gates, then perform no-triage final
  signoff.

## Completion Target

Completion means the final absolute parity pipeline passes repeatedly from a
clean checkout and records zero remaining documentation-only differences
against the native C command-line oracle.

## Cycle 301-320 Run Status

The 301-320 pass completed the prioritized Rust `run forever` implementation
and promoted a safe `run forever` command-text fixture into the strict raw
transcript gate. It also added native save artifact capture, binary scenario
inventory, C layout probing, and command-visible native trace extraction.

The raw binary reader/writer replacement and direct C in-engine trace emitters
remain open carryover work for the following cycles.

## Cycle 321-340 Run Status

The 321-340 pass added a direct native C engine trace probe, a Rust engine trace
formatter with the same field schema, and a schema gate that verifies C/Rust
trace field order. The schema spans terrain, food, weather, population, body,
braincode, social, episodic, territory, lifecycle/family, immune, movement, and
inventory/preference fields.

Exact value parity for those fields remains open by category and is now exposed
by the direct C probe output.

## Cycle 341-350 Run Status

The 341-350 pass added Rust-side closure work for family/pregnancy, birth,
carrying, suckling, immune transmission, movement/awake-state thresholds, and
CLI command-surface gaps. `alpha` and selected-being `speak` now write AIFF
containers, `epic` has populated output, and `top`/`epic` accept the native
female/male/juvenile filters.

The pass also added pending command corpora for long seeded runs, multi-day and
multi-month runs, save/open continuity, and exhaustive command-surface coverage.
Those fixtures remain pending promotion until exact C/Rust value drift is closed
under the direct trace and raw transcript gates.

## Cycle 351-360 Run Status

The 351-360 pass promoted empty runtime/detail command behavior into the strict
raw transcript gate, added exact EOF-after-command and stop timing coverage with
closed-stdin triage, and changed empty-file `open` to emit the stable native
toolkit allocation diagnostic. It also added
deterministic command fuzz, malformed save fuzz, raw binary mutation triage, and
seed/population repeatability gates, all wired into the macOS absolute parity
pipeline.

Empty-file malformed loads, raw binary mutation, and seed/population fuzz remain
marked as triage or repeatability gates until native loader/byte/value oracles
are promoted.

## Cycle 361-370 Run Status

The 361-370 pass added platform, profile/compiler, required-pipeline, and
failure-smoke gates. It also added a repeated closed-stdin oracle and promoted
closed stdin, quiet EOF, and bounded-run EOF from triage into native-backed
exact timing canonicalizers. The interactive timing gate now covers quiet EOF,
verbose EOF, bounded-run EOF, closed stdin, and PTY stop-after-forever.

The remaining no-triage work begins at cycle 371 with timing regression locks,
then malformed loader diagnostics, raw binary byte/value closure, seeded engine
value parity, and pending corpus promotion.

## Cycle 371-380 Run Status

The 371-380 pass added timing regression locks, an EOF-after-verbose-command
canonicalizer for the native threaded console's optional failure lines, exact
promoted malformed-loader diffs for missing, empty, bad JSON, random text, and
truncated native text, and split stdout/stderr/combined inventory artifacts for
the remaining malformed loader cases. Rust now emits native-style parser
diagnostics for non-native text prefixes and accepts the native partial
`landd{dated=0;` recovery case.

The pass also added a direct native C `tranfer_out()` raw byte oracle, stable
hash checks, and token byte maps for empty startup, reset startup, and
after-one-cycle artifacts. Raw reader/value closure starts in cycle 381.

## Cycle 366-400 Plan Addition

Cycles 366-400 are the no-triage closure block. They convert the remaining
cycle 351-360 triage/repeatability buckets into exact gates:

- Closed stdin moves from captured timing triage to a deterministic native
  oracle and strict timing diff.
- Malformed loader diagnostics move from exact missing-file plus triage captures
  to exact stdout/stderr/parser/exit behavior across minimized fixtures.
- Raw binary moves from mutation triage to native byte oracle generation,
  complete byte maps, reader/writer byte parity, cross-load parity, and absolute
  gate promotion.
- Seeded population and engine traces move from schema/repeatability checks to
  direct C/Rust value comparisons across terrain, food, body, movement,
  braincode, social, family, and save/load.
- Long seeded, multi-day, multi-month, save/open continuity, and exhaustive
  command-surface corpora move from pending scripts into exact raw transcript,
  trace, and save-byte gates.

The final planned endpoint is now cycle 400, where the signoff must record zero
accepted/documented differences and zero triage buckets.

---

<a id="rust-port-cycle-366-400-planmd"></a>

## RUST_PORT_CYCLE_366_400_PLAN.md

Original file: `RUST_PORT_CYCLE_366_400_PLAN.md`

# Rust Port Cycles 366-400 Plan

These 35 cycles close the remaining no-triage parity work after the 351-360
timing/fuzz pass.

## Range Breakdown

- Cycles 366-371: closed-stdin native nondeterminism, selected oracle behavior,
  Rust exactness, timing promotion, EOF/signal matrix, and regression locks.
- Cycles 372-378: malformed loader diagnostic inventory, stdout/stderr split,
  parser diagnostics, empty/truncated behavior, exit/recovery semantics, fuzz
  minimization, and exact gate promotion.
- Cycles 379-386: native raw binary byte oracle, byte map, Rust reader/writer
  value closure, malformed raw binary behavior, cross-load, platform matrix,
  and absolute gate promotion.
- Cycles 387-394: seeded population native value oracle and direct engine trace
  value parity for startup/reset, terrain/weather, food/inventory,
  body/movement/awake-state, braincode/social/family, save/load, and value-gate
  promotion.
- Cycles 395-400: promote long seeded, multi-day/multi-month, save/open
  continuity, and exhaustive command corpora; remove pending/triage corpus
  statuses; perform final no-triage signoff.

## Completion Target

The final cycle 400 gate must pass from a clean checkout with exact raw
transcripts, direct engine trace values, raw save bytes, malformed diagnostics,
fuzz corpus behavior, timing behavior, and platform/profile/compiler matrix.
No accepted/documented differences or triage buckets remain.

## Cycle 366-370 Run Status

Closed stdin and EOF timing are no longer triage-only. A repeated native oracle
classifies closed-stdin raw output, and the interactive timing gate now diffs
canonical C/Rust output for quiet EOF, bounded-run EOF, and closed stdin. The
gate also covers verbose EOF and PTY stop-after-forever exactly.

Cycles 371-400 remain for regression locks, malformed loader diagnostics, raw
binary byte/value parity, seeded engine values, and final corpus promotion.

## Cycle 371-380 Run Status

Timing regression locks are in place, including EOF-after-verbose-command
canonicalization for native optional console-failure lines. The promoted
malformed-loader gate now diffs missing, empty, bad JSON, random text, and
truncated native text exactly, while unpromoted malformed cases are captured
with split stdout/stderr inventories. A direct native C raw byte oracle now
generates reproducible `tranfer_out()` artifacts and byte-token maps for empty
startup, reset startup, and after-one-cycle scenarios.

Cycles 381-400 remain for raw reader/writer value closure, raw binary mutation
behavior, seeded engine value parity, long corpus promotion, and final no-triage
signoff.

---

<a id="rust-port-cycle-411-500-planmd"></a>

## RUST_PORT_CYCLE_411_500_PLAN.md

Original file: `RUST_PORT_CYCLE_411_500_PLAN.md`

# Rust Port Cycle 411-550 Development Plan

## Scope

This 140-cycle plan continues after cycle 410 and targets absolute native C
parity for command-line `simape`.

The original user-requested categories account for 80 cycles:

- 20 cycles: terrain/topography, weather, tide-side effects, and food value
  parity
- 15 cycles: selected-being identity, random state, movement/body/energy/drives
  parity
- 12 cycles: brain/probe, social, episodic, immune runtime parity
- 10 cycles: CLI reset/startup timing and population semantics
- 8 cycles: save/open runtime continuity and post-load advancement parity
- 7 cycles: populated raw transfer byte-exactness, especially raw territory
  bytes
- 8 cycles: promote long seeded, multi-day/month, save/open, and exhaustive
  command corpora into exact gates

The next 10 cycles are reserved for convergence, repeated clean pipeline runs,
cross-profile/platform checks, and drift-register closure.

The additional 50 cycles added after cycles 441-480 cover the remaining hard
parts: after-day movement/body/energy/honor drift, brain/social/episodic/immune
runtime parity, populated save/open continuity, raw populated territory byte
exactness, and exact promotion of the long seeded, multi-day/month, save/open,
and exhaustive command corpora.

## Terrain, Weather, Tide, Food: Cycles 411-430

- Cycle 411: Expand direct C/Rust terrain fixtures across representative map
  coordinates, high/low elevations, water edges, and intertidal bands.
- Cycle 412: Match native land-clear, tile-erase, genetics preservation, and
  startup-cycle land initialization ordering.
- Cycle 413: Close tile genetics, packed topography, and unpacked topography
  parity for all map tiles used by command-line startup.
- Cycle 414: Port or align native terrain refinement rounds, patching, swapping,
  and smoothing operations used by `tile_land_init`.
- Cycle 415: Close high-definition bilinear topography and high-resolution tide
  map generation values.
- Cycle 416: Match tide update side effects for first cycle, day rollover, month
  rollover, and saved/restored land states.
- Cycle 417: Close weather initialization, atmosphere packing, pressure range,
  and startup weather values.
- Cycle 418: Match weather cycling, pressure drift, lightning storage, and
  weather trace values over multi-day seeded runs.
- Cycle 419: Close `food_values` operator math for grass, trees, bush, water,
  slope, sun, salt, and intertidal factors.
- Cycle 420: Match food-source classification for grass, fruit, shellfish,
  seaweed, bird eggs, lizard eggs, nuts, and unavailable terrain.
- Cycle 421: Close food depletion by location, being bite, terrain class, and
  inventory interaction.
- Cycle 422: Match food regrowth counters across minute, hour, day, and
  post-load advancement.
- Cycle 423: Close water, swimming, shoreline, and tide-dependent food source
  side effects.
- Cycle 424: Match bite quantity, nutrition, pathogen ingestion, and local
  biology effects for native hungry/eating traces.
- Cycle 425: Promote terrain/weather/food direct trace fields into
  `scripts/run_engine_trace_value_gate.sh`.
- Cycle 426: Add multi-location C fixtures for terrain/food edge cases and
  promote exact C/Rust value diffs.
- Cycle 427: Run long seeded terrain/weather/food drift checks across day and
  month boundaries.
- Cycle 428: Verify save/open preserves terrain, weather, tide, and food state
  before and after runtime advancement.
- Cycle 429: Remove terrain/food blockers from pending corpus inventory when the
  exact gates pass.
- Cycle 430: Promote terrain/topography, weather, tide, and food parity into the
  absolute pipeline.

Run status for cycles 411-420: completed as a fixture/inventory pass. The direct
C and Rust engine trace now emits multi-location `TERRAIN` rows, and
`scripts/run_terrain_food_value_inventory.sh` records the remaining terrain,
weather, and food value drift. Exact terrain parity remains open for cycles
421-430 and the later final convergence cycles.

Run status for cycles 421-430: completed for exact startup and first-cycle
terrain/weather/food parity. Rust now runs native-shaped tile patching,
smoothing, primary/working topography copies, startup terrain ordering,
first-cycle weather initialization/cycling, and C salt-operator semantics.
`scripts/run_engine_trace_value_gate.sh` promotes the direct
`terrain-first-cycle` subset, and
`scripts/run_terrain_food_first_cycle_gate.sh` promotes 16 exact multi-location
startup/first-cycle `TERRAIN` rows. Remaining terrain/weather carryover is
multi-day weather evolution plus selected-being honor drift exposed by the
native terrain path.

## Selected Being, Movement, Body, Drives: Cycles 431-445

- Cycle 431: Map native `being_init_group` random consumption and selected-being
  creation order against Rust.
- Cycle 432: Match selected-being name, sex, family name, honor, and initial
  selection values.
- Cycle 433: Close selected-being genetics, fetal defaults, generation ranges,
  and parent-name defaults.
- Cycle 434: Match selected-being location, facing, random seed, and macro-state
  values after startup and reset.
- Cycle 435: Close initial energy, height, mass, body fat, frame, posture, and
  body inventory defaults.
- Cycle 436: Match selected-being random state after one cycle, one day, and
  post-save/open advancement.
- Cycle 437: Close native movement vector, goal, speed history, and wander
  behavior.
- Cycle 438: Match velocity, facing changes, walking/swimming transitions, and
  water-edge movement effects.
- Cycle 439: Close awake-state, fatigue, sleeping, and time-of-day transitions.
- Cycle 440: Match body growth, mass recalculation, metabolism, parasite,
  wound, and grooming side effects.
- Cycle 441: Close drive cycling for hunger, fatigue, sex, social, and maturity
  over seeded runs.
- Cycle 442: Match energy loss/gain from movement, resting, eating, inventory
  consumption, and local food.
- Cycle 443: Promote selected-being movement/body/energy/drive fields into the
  engine value gate.
- Cycle 444: Add focused C-vs-Rust selected-being trace fixtures for startup,
  after-cycle, after-day, and save/open checkpoints.
- Cycle 445: Remove selected-being body/runtime blockers from pending corpus
  inventory after exact gates pass.

Run status for cycles 431-440: completed for exact direct first-cycle
selected-being native initialization/body parity. Rust now mirrors native
`being_init_group` seed copying, C-shaped first-being name/location/facing,
brain register/probe setup, immune initialization from the zero seed, initial
energy/body defaults, sleeping drive cycling, sleep energy cost, and water-aware
night waking. `scripts/run_engine_trace_value_gate.sh` promotes the
`selected-first-cycle` subset, and
`scripts/run_selected_being_value_inventory.sh` records the remaining after-day
movement/body/honor/brain/social/episodic/immune runtime drift.

Run status for cycles 441-445: completed as a runtime-ordering convergence
pass. Rust now advances populated beings through native-shaped group phases for
awake-state calculation, universal immune/energy cycling, episodic fade, awake
movement/body logic, drive cycling, tidy movement/energy, and speed-history
advance. The direct selected-being inventory remains exact for startup and
after-cycle-one values. After-day movement/body/energy/honor/drive values are
still inventory rather than exact.

## Brain, Social, Episodic, Immune: Cycles 446-457

- Cycle 446: Match brain memory/register initialization, brain state random
  values, and default probe state.
- Cycle 447: Close brain probe update order, probe fields, frequency, offset,
  address, and state values.
- Cycle 448: Match braincode VM execution order, arithmetic, sensors, actuators,
  conditionals, and mutation of selected state.
- Cycle 449: Close speech, shout, listen, and brain/social hook interactions.
- Cycle 450: Match social initial loop ordering, social graph defaults,
  attraction, friend/foe, belief, familiarity, and relationship values.
- Cycle 451: Close social meeting, grooming, squabble, mate, chat, preference
  learning, and anecdote behavior.
- Cycle 452: Match episodic cycle, memory replacement, fade, affect, event,
  food, argument, and time/location fields.
- Cycle 453: Close immune initialization, antigen/antibody shapes, random seed,
  and pathogen defaults.
- Cycle 454: Match immune cycling, infection response, antibody mutation, and
  shape updates over seeded runtime.
- Cycle 455: Close air, touch, sex, eating, and maternal pathogen transmission
  values.
- Cycle 456: Promote brain/probe, social, episodic, and immune direct trace
  fields into the engine value gate.
- Cycle 457: Remove brain/social/immune blockers from pending corpus inventory
  after exact gates pass.

Run status for cycles 446-457: completed as an inventory pass, not an exact
promotion. A native braincode/social dialogue insertion point was tested and
left disabled after it worsened after-day trace values. The promoted
first-cycle selected-being brain/probe/immune baseline stayed exact; after-day
brain/probe/social/episodic/immune values remain open in
`scripts/run_selected_being_value_inventory.sh`.

## CLI Reset And Startup Semantics: Cycles 458-467

- Cycle 458: Inventory native CLI `reset` and `clear` timing relative to
  `sim_init`, first `sim_cycle`, console threading, and output flushing.
- Cycle 459: Match Rust CLI reset/clear land seed, population presence, running
  status, and selected-being behavior.
- Cycle 460: Close startup `sim`, `step`, `run`, `stop`, `quit`, and EOF
  behavior around empty and newly reset states.
- Cycle 461: Match reset land-genetics derivation and random consumption against
  native command behavior.
- Cycle 462: Match reset population creation timing and whether commands observe
  empty, pending, or populated state.
- Cycle 463: Close transcript-visible reset behavior for list, ape, top, epic,
  detail commands, and watch/monitor commands.
- Cycle 464: Promote CLI reset/startup command fixtures into raw transcript
  diffing.
- Cycle 465: Remove CLI reset blocker from long seeded and exhaustive pending
  corpus inventory.
- Cycle 466: Add reset/startup failure-smoke mutations so regressions fail the
  absolute pipeline.
- Cycle 467: Promote CLI reset/startup timing and population semantics into the
  absolute parity pipeline.

Run status for cycles 458-467: completed for command-visible reset/startup
timing. Rust `reset` and `clear` now match native's pending simulation shape:
population is zero until the first engine cycle, and `step`, finite `run`, and
populated `run forever` initialize and advance that pending simulation. The
empty runtime edge corpus remains exact, and pending corpus inventory no longer
lists reset/startup population as a blocker.

## Save/Open Continuity: Cycles 468-475

- Cycle 468: Inventory native save/open continuity checkpoints for pre-save,
  post-open, post-run, and second-save states.
- Cycle 469: Match default command-line save bytes and open failure/recovery
  behavior for JSON, native text, missing files, and malformed files.
- Cycle 470: Close native transfer post-load selected-being, land, population,
  event, and runtime state.
- Cycle 471: Match post-open advancement for one minute, one day, one month, and
  save-open-save loops.
- Cycle 472: Add C/Rust state trace comparisons around every save/open
  continuity checkpoint.
- Cycle 473: Promote save/open runtime continuity scripts into exact transcript
  and state gates.
- Cycle 474: Remove save/open blockers from pending corpus inventory.
- Cycle 475: Wire save/open continuity into the absolute pipeline and failure
  smoke checks.

Run status for cycles 468-475: completed as a save/open continuity inventory
pass. CLI tests and the Rust runtime golden now save populated state after a
first run/step under native reset timing. Populated save/open continuity remains
blocked by selected-being identity/detail values, long-runtime values, and
populated save size/content drift.

## Populated Raw Transfer Byte Exactness: Cycles 476-482

- Cycle 476: Design raw territory byte preservation without disturbing semantic
  territory APIs.
- Cycle 477: Preserve native raw `terit=` words when parsing populated direct
  `tranfer_out()` streams.
- Cycle 478: Emit preserved raw territory words from Rust raw writer when a
  loaded native raw source is available.
- Cycle 479: Match populated after-one-cycle raw transfer bytes including
  territory, immune, brain, and padding-sensitive words.
- Cycle 480: Add populated raw roundtrip byte gates for normal, social-heavy,
  immune-heavy, and terrain-heavy fixtures.
- Cycle 481: Promote malformed raw transfer mutation behavior from triage into
  exact native/Rust diffing.
- Cycle 482: Remove raw populated territory byte drift from the final drift
  register.

Run status for cycles 476-482: completed as a raw value and byte preservation
pass for the current direct raw oracle set.
`scripts/run_native_raw_binary_value_gate.sh` keeps empty, reset, and populated
after-one-cycle raw streams byte-exact. Rust now preserves full native
`terit=` words while still exposing semantic low-byte territory entries.

## Corpus Promotion: Cycles 483-490

- Cycle 483: Promote `long_seeded_command_corpus` into strict raw transcript and
  state trace gates.
- Cycle 484: Promote `multi_day_runtime_matrix` into strict raw transcript,
  state trace, and save checkpoint gates.
- Cycle 485: Promote `multi_month_runtime_matrix` into strict raw transcript,
  state trace, and save checkpoint gates.
- Cycle 486: Promote `save_open_runtime_continuity` into exact transcript,
  state trace, and byte artifact gates.
- Cycle 487: Promote `exhaustive_command_surface` into exact raw transcript
  gates, including file-producing commands.
- Cycle 488: Merge redundant corpus coverage and delete obsolete pending-only
  fixtures.
- Cycle 489: Make pending corpus inventory fail if any promoted corpus regresses
  to blocked or triage status.
- Cycle 490: Wire all promoted corpora into the required absolute parity
  pipeline.

Run status for cycles 483-490: completed as absolute-pipeline corpus
promotion inventory, not exact corpus promotion. `scripts/run_corpus_promotion_inventory.sh`
now wraps the pending corpus inventory, fails if a promoted session file is
missing, optionally runs raw transcript diffs, and records the five long seeded,
multi-day, multi-month, save/open, and exhaustive sessions as blocked by the
remaining runtime/save/detail/file-ordering drift.

## Interim Convergence And Handoff: Cycles 491-500

- Cycle 491: Run macOS platform absolute parity with promoted engine, raw, and
  corpus gates.
- Cycle 492: Run Linux/platform-equivalent parity or record the fixed host
  oracle where platform execution is unavailable.
- Cycle 493: Run debug/release/profile/compiler matrix checks and close any
  profile-specific drift.
- Cycle 494: Expand failure-smoke coverage for terrain, selected-being, brain,
  save/open, raw bytes, and corpus transcripts.
- Cycle 495: Run repeated clean-checkout absolute parity to detect nondeterminism.
- Cycle 496: Close drift registers, corpus audits, binary manifests, and
  no-normalization policy updates.
- Cycle 497: Search for and remove or reclassify all remaining `pending-*`,
  `triage-only`, and documented-difference markers.
- Cycle 498: Run performance and large-corpus sanity checks so exact parity does
  not hide pathological runtime behavior.
- Cycle 499: Produce interim release-candidate parity report with command list,
  gates, artifacts, and known platform constraints.
- Cycle 500: Handoff into the hard-part closure extension with no regression in
  promoted exact gates and a refreshed blocker inventory.

Run status for cycles 491-500: completed as refreshed CI scaffolding and
handoff documentation. The absolute pipeline now includes raw value/byte,
engine value, terrain/food, selected-being, selected-minute, and corpus
inventory gates. The remaining blockers are narrowed to after-day selected
runtime values, populated save/open continuity, broader raw fixture promotion,
and exact corpus promotion.

## After-Day Movement, Body, Energy, Honor: Cycles 501-512

- Cycle 501: Build a minute-by-minute C/Rust selected-being trace around the
  first day boundary, including pre-awake, post-awake, post-tidy, and
  post-social checkpoints.
- Cycle 502: Match native awake-state transitions across night, dawn, day,
  water, hunger, and prior-speed cases.
- Cycle 503: Close native speed target, temporary speed, water-ahead, swim, and
  terrain-slope calculations for the selected being.
- Cycle 504: Match native wandering, facing update, movement vector, and
  speed-history ordering over the after-day trace.
- Cycle 505: Close movement/resting energy cost, hair insulation, water/body-fat
  terms, mass terms, and minimum-cost clamping.
- Cycle 506: Match eating/resting/inventory food energy interactions at the
  selected being's exact native locations.
- Cycle 507: Close body mass, body fat, height/growth, posture, parasite, wound,
  and inventory side effects after one day.
- Cycle 508: Match hunger/fatigue/sex/social drive cycling and reset conditions
  across sleeping, eating, moving, and social states.
- Cycle 509: Close honor recalibration and social honor side effects that appear
  in the after-day selected-being row.
- Cycle 510: Promote movement/body/energy/drive/honor fields from inventory into
  `scripts/run_engine_trace_value_gate.sh`.

Run status for cycles 501-510: completed through selected-minute instrumentation
and movement/body/energy triage, but not promoted as exact values. The C and
Rust direct engine probes now emit `SELECTED-MINUTE` rows hourly through the
first day and every minute for the final 16 minutes. Schema and row counts are
exact; value rows remain inventory because the first visible divergence is in
drive/brain/social state before movement and energy drift fan out. A native
movement-order cleanup removed Rust-only stopped-being speed forcing and kept
the promoted first-cycle gates green, but after-day movement/body/energy/honor
fields remain open.

- Cycle 511: Add mutation/failure-smoke coverage for selected-being after-day
  movement, energy, drive, and honor fields.
- Cycle 512: Remove movement/body/energy/honor blockers from the selected-being
  inventory and pending corpus register once exact.

Run status for cycles 511-512: completed as a first selected-minute value
promotion, not full after-day closure. Rust now matches the C selected-being
minute-60 core for energy, location, facing, speed, honor, mass, awake/state,
drives, episodic sample, immune sample, and preference. The promoted
`scripts/run_selected_minute_value_gate.sh` includes a deliberate mutation
check. Later selected-minute rows and the after-day selected row remain
inventory because brain/social runtime and downstream movement still drift.

## Brain, Social, Episodic, Immune Runtime: Cycles 513-524

- Cycle 513: Instrument native brain/probe checkpoints before and after
  braincode execution, listen, social hooks, and probe updates.
- Cycle 514: Match braincode VM runtime ordering, register mutation, brain
  state random consumption, and selected-being attention updates.
- Cycle 515: Close probe type, position, frequency, offset, address, and state
  updates over first-day and multi-day seeded traces.
- Cycle 516: Match speech, shout, listen, and brain/social hook side effects
  without disturbing exact first-cycle baselines.
- Cycle 517: Close social initial loop ordering, target selection, social
  coordinates, familiarity, attraction, belief, and friend/foe changes.
- Cycle 518: Match grooming, squabble, mate, chat, anecdote, and preference
  learning side effects across representative seeded social encounters.
- Cycle 519: Close episodic memory replacement, fade, affect, event, argument,
  food, time, and location values for selected and social-heavy traces.
- Cycle 520: Match immune runtime cycling, antigen/antibody mutation, immune
  energy usage, infection response, and random-seed progression.
- Cycle 521: Close air, touch, sex, food, and maternal pathogen transmission
  values in direct C/Rust fixtures.
- Cycle 522: Promote brain/probe/social/episodic/immune fields into the engine
  value gate with deliberate mutation checks.
- Cycle 523: Add long seeded trace slices for brain/social/immune drift at day,
  month, save/open, and population-stress checkpoints.
- Cycle 524: Remove brain/social/episodic/immune blockers from pending corpus
  inventory after exact direct trace gates pass.

Run status for cycles 513-524: completed as brain/social runtime narrowing.
The native even-minute braincode dialogue phase now advances selected-being
random state in Rust, which is required for the minute-60 movement core. Full
braincode side effects remain blocked because Rust does not yet initialize and
execute native-equivalent internal/external braincode bytes; minute-60 brain
registers and social memory remain explicitly open.

## Populated Save/Open Continuity: Cycles 525-532

- Cycle 525: Inventory native populated save/open bytes and state summaries for
  pre-save, post-open, post-run, and save-open-save checkpoints.
- Cycle 526: Match selected-being identity, random state, social/episodic/immune
  state, and land/runtime fields immediately after populated open.
- Cycle 527: Close post-open one-minute advancement for body, movement, brain,
  social, immune, terrain, food, tide, and weather state.
- Cycle 528: Close post-open one-day, multi-day, and month-boundary advancement
  against C state traces.
- Cycle 529: Match native command-line open failure/recovery semantics when a
  populated save is malformed, truncated, or reopened after runtime failure.
- Cycle 530: Promote save/open runtime continuity into exact raw transcript and
  state trace gates.
- Cycle 531: Add save/open mutation checks for selected-being state, land state,
  population state, and artifact bytes.
- Cycle 532: Remove save/open continuity blockers from pending corpus inventory.

Run status for cycles 525-532: completed as save/open continuity inventory.
`scripts/run_save_open_continuity_inventory.sh` records the populated
save/open/run session and optionally runs the raw transcript diff as inventory.
The optional diff remains blocked by populated runtime continuity and Rust
session exit/output ordering after failed JSON reopen.

## Populated Raw Territory Byte Exactness: Cycles 533-538

- Cycle 533: Preserve raw native territory words from populated `tranfer_out()`
  streams alongside semantic territory memory.
- Cycle 534: Emit preserved raw territory words from Rust raw writer without
  changing semantic territory APIs or JSON/native text transfer behavior.
- Cycle 535: Match populated after-one-cycle raw transfer bytes, including
  territory, immune, brain/probe, social, episodic, and padding-sensitive words.
- Cycle 536: Add normal, social-heavy, immune-heavy, terrain-heavy, and
  save/open-derived populated raw byte fixtures.
- Cycle 537: Promote populated raw byte roundtrips into the native raw binary
  value/byte gates with mutation checks.
- Cycle 538: Remove raw populated territory byte drift from final drift and
  corpus blockers.

Run status for cycles 533-538: completed as fixture inventory on top of the
already exact current direct raw oracle. `scripts/run_populated_raw_fixture_inventory.sh`
verifies the current empty/reset/after-one-cycle artifacts are present and
marks social-heavy, immune-heavy, terrain-heavy, and save/open-derived populated
raw byte fixtures as the remaining required set.

## Exact Corpus Promotion: Cycles 539-546

- Cycle 539: Promote `long_seeded_command_corpus` into exact raw transcript,
  state trace, and artifact checks.
- Cycle 540: Promote `multi_day_runtime_matrix` into exact transcript and state
  trace gates.

Run status for cycles 539-540: still inventory, not exact corpus promotion.
`scripts/run_corpus_promotion_inventory.sh` continues to report five blocked
corpus sessions and zero ready sessions until the selected runtime and
save/open continuity blockers close.
- Cycle 541: Promote `multi_month_runtime_matrix` into exact transcript and
  state trace gates.
- Cycle 542: Promote `save_open_runtime_continuity` into exact transcript, state
  trace, and byte artifact gates.
- Cycle 543: Promote `exhaustive_command_surface` into exact raw transcript
  gates, including `speak`, `alpha`, file commands, watch modes, and detail
  commands.
- Cycle 544: Make `scripts/run_pending_corpus_inventory.sh` fail if any promoted
  corpus still reports blocked, triage, inventory, or pending status.
- Cycle 545: Merge redundant corpus fixtures and delete obsolete pending-only
  fixtures once exact replacements exist.
- Cycle 546: Wire all promoted corpora into the absolute parity pipeline and
  required platform/profile gates.

Run status for cycles 541-546: completed as strict readiness gating, not exact
promotion. `scripts/run_exact_corpus_promotion_gate.sh` now fails if any
promoted corpus is still blocked or any optional raw diff remains blocked. The
current result is intentionally blocked with five corpus sessions, zero ready
sessions, and five blocked sessions.

## Final Absolute Signoff: Cycles 547-550

- Cycle 547: Run repeated clean-checkout absolute parity across debug/release
  Rust and native profiles to catch nondeterminism.
- Cycle 548: Run platform/compiler matrix gates and either close or explicitly
  constrain host-specific native oracle behavior.
- Cycle 549: Search for and remove all remaining accepted-difference,
  documented-difference, pending, inventory-only, and triage-only markers.
- Cycle 550: Final absolute native C parity signoff: no accepted/documented
  differences, no triage buckets, and all required gates passing.

Run status for cycles 547-550: completed as explicit final signoff readiness
inventory. `scripts/run_final_signoff_readiness.sh` composes the selected
minute value gate, selected-being inventory, selected-minute inventory,
save/open continuity inventory, populated raw fixture inventory, and corpus
promotion inventory. It reports `status=blocked` by default and exits non-zero
in strict mode through `APESDK_REQUIRE_ABSOLUTE_SIGNOFF=1` until no blocker
categories remain.

---

<a id="rust-port-cycle-551-620-planmd"></a>

## RUST_PORT_CYCLE_551_620_PLAN.md

Original file: `RUST_PORT_CYCLE_551_620_PLAN.md`

# Rust Port Cycle 551-620 Completion Plan

## Scope

This 70-cycle plan starts after the completed cycle 550 readiness inventory and
targets transcript-level native C parity for the command-line `simape`. The
focus is to remove the remaining readiness blockers rather than add more
inventory-only gates.

The planned cycle budget is:

- 12 cycles: Rust semantic terrain/weather/topography load/store parity for the
  new C transfer sections.
- 15 cycles: braincode execution, probe updates, social hooks, and
  selected-being minute/runtime drift.
- 12 cycles: after-day movement/body/energy/honor/social/episodic/immune value
  closure.
- 10 cycles: populated save/open continuity and post-load advancement parity.
- 8 cycles: broader populated raw fixtures: social-heavy, immune-heavy,
  terrain-heavy, and save/open-derived.
- 8 cycles: promote long seeded, multi-day/month, save/open, and exhaustive
  corpora into exact gates.
- 5 cycles: nondeterminism, profile/platform checks, cleanup, and strict
  signoff.

## Rust Land Transfer Semantics: Cycles 551-562

- Cycle 551: Extend Rust startup/native transfer state to carry optional raw
  topography and weather payloads from C `topog{}` and `weath{}` sections.
- Cycle 552: Parse and validate `topby=` byte counts against both native
  topography buffers without rejecting older native files that omit the section.
- Cycle 553: Parse and validate `atmby=` and `litby=` byte counts for both raw
  atmosphere buffers and the lightning map.
- Cycle 554: Add Rust `LandState` storage or snapshot hooks for loaded
  topography buffers without perturbing generated startup terrain behavior.
- Cycle 555: Add Rust `LandState` storage or snapshot hooks for loaded
  atmosphere and lightning bytes without perturbing generated startup weather.
- Cycle 556: Rebuild high-definition topography and tide maps from loaded
  topography/time in the same order as C load.
- Cycle 557: Preserve loaded topography/weather bytes across default Rust
  save-open-save loops.
- Cycle 558: Add C fixture checks proving Rust can read C saves containing
  `topog{}` and `weath{}` and retain command-visible land values.
- Cycle 559: Add Rust writer support for terrain/weather native sections when
  loaded byte payloads are available.
- Cycle 560: Compare C/Rust post-load terrain/weather trace rows for startup,
  after one minute, and after one day.
- Cycle 561: Promote terrain/weather transfer preservation into the save/open
  continuity inventory with mutation checks.
- Cycle 562: Remove land transfer byte-preservation blockers from the drift
  register when exact gates pass.

Run status for cycles 551-562: completed in the current tranche. Rust native
transfer state now carries exact-length `topog{topby=...}` and
`weath{atmby=...;litby=...}` payloads, applies them to `LandState`, and
re-emits them from the default command-line save path. Remaining land work has
moved from transfer byte preservation to generated terrain/weather runtime
value parity.

## Brain, Probe, Social Runtime: Cycles 563-577

- Cycle 563: Capture native internal/external braincode initialization bytes for
  selected and neighboring beings.
- Cycle 564: Initialize Rust internal braincode bytes to match native reset and
  first-cycle values.
- Cycle 565: Initialize Rust external/social braincode bytes to match native
  self and first social-memory slots.
- Cycle 566: Enable native-equivalent internal braincode execution at the exact
  minute-cycle point without disturbing first-cycle promoted baselines.
- Cycle 567: Enable native-equivalent external/social braincode execution at
  the C social hook point.
- Cycle 568: Match brain register mutation, brain state updates, and random
  consumption through minute 60.
- Cycle 569: Match probe type, address, frequency, offset, position, and state
  updates through selected-minute traces.
- Cycle 570: Match shout, speech, listen, and attention side effects connected
  to braincode runtime.
- Cycle 571: Match social target selection and vicinity ordering after
  braincode execution is active.
- Cycle 572: Match social graph updates for attraction, belief, friend/foe,
  familiarity, relationship, and entity type.
- Cycle 573: Match chat, grooming, squabble, mate, and anecdote insertion side
  effects in social-heavy traces.
- Cycle 574: Close selected minute-60 brain/social trace mismatches and promote
  those rows from inventory.
- Cycle 575: Extend exact selected-minute gates beyond minute 60 to the first
  day boundary for brain/social fields.
- Cycle 576: Add mutation checks for brain register, probe, and social-memory
  promoted fields.
- Cycle 577: Remove brain/probe/social runtime blockers from corpus and final
  readiness inventories.

Run status for cycles 563-577: partially completed as implementation plus
inventory. Rust now materializes the C-shaped initial external braincode bytes
for native-created beings while preserving the same random-state progression.
Full internal/external braincode execution, probe mutation, and social hook
value promotion remain open for later cycles because the exact VM/runtime order
still needs native trace closure.

## After-Day Runtime Value Closure: Cycles 578-589

- Cycle 578: Re-run selected-minute and selected-being inventories after
  brain/social closure and identify remaining first-day divergence.
- Cycle 579: Close awake-state transitions that still differ after day/night,
  water, hunger, and prior-speed cases.
- Cycle 580: Close selected-being movement vector, facing, speed history, and
  wander ordering through after-day traces.
- Cycle 581: Close movement/resting/eating energy costs at the selected being's
  exact native locations.
- Cycle 582: Close body mass, body fat, height/growth, posture, wound,
  parasite, grooming, and inventory side effects after one day.
- Cycle 583: Close honor recalibration and social honor side effects exposed by
  the after-day selected row.
- Cycle 584: Close drive reset/order drift for hunger, fatigue, sex, and social
  values beyond minute 60.
- Cycle 585: Close episodic replacement, fade, event, affect, food, argument,
  and location drift over first-day traces.
- Cycle 586: Close immune runtime mutation, antigen/antibody shape, random
  seed, infection response, and energy-use drift.
- Cycle 587: Promote after-day selected-being movement/body/energy/honor,
  episodic, immune, and preference fields into exact value gates.
- Cycle 588: Add mutation checks for every newly promoted after-day runtime
  field group.
- Cycle 589: Remove selected-runtime blockers from final signoff readiness.

Run status for cycles 578-589: completed as reassessment plus inventory, not
exact promotion. The reassessment removed land transfer byte loss from the
after-day/save-open blocker list. Rechecked selected-being and selected-minute
inventories still show first-day divergence concentrated in movement/body/
energy/honor behavior plus brain/social/episodic/immune runtime ordering:
`selected_status=inventory samples=3 mismatches=2` and
`minute_status=inventory samples=39 mismatches=78`.

## Populated Save/Open Continuity: Cycles 590-599

- Cycle 590: Capture C/Rust populated save/open state summaries immediately
  before save, immediately after open, and after one post-open minute.
- Cycle 591: Preserve selected-being identity, random state, and selection
  across populated Rust save/open.
- Cycle 592: Preserve social, episodic, immune, brain/probe, territory, and
  preference state across populated Rust save/open.
- Cycle 593: Preserve loaded land topography/weather/tide/food state across
  populated save/open.
- Cycle 594: Match post-open one-minute advancement for movement/body/brain/
  social/immune/terrain fields.
- Cycle 595: Match post-open one-day and multi-day advancement against native
  state traces.
- Cycle 596: Match malformed populated-open failure/recovery behavior without
  changing exact malformed-loader diagnostics.
- Cycle 597: Promote `save_open_runtime_continuity` state traces into exact
  save/open gates.
- Cycle 598: Add save/open mutation checks for selected, land, population, and
  artifact bytes.
- Cycle 599: Remove save/open continuity blockers from pending corpus and final
  readiness inventories.

Run status for cycles 590-599: completed for Rust internal populated native
save/open continuity. Rust now has a `--save-open-trace` fixture that advances
a populated native simulation to the save point, writes native transfer text,
opens it back into Rust, and compares `before_save` with `after_open`. The
state trace is exact for selected identity, random state, awake state, social
slot ordering, episodic/immune/probe fields, land payload presence, and
population count. Remaining save/open work is C/Rust runtime transcript speed,
post-open advancement parity beyond the Rust internal fixture, and artifact
byte continuity promotion.

## Broader Populated Raw Fixtures: Cycles 600-607

- Cycle 600: Generate native social-heavy populated raw fixture artifacts from
  direct `tranfer_out()` or an equivalent command-reachable C oracle.
- Cycle 601: Generate native immune-heavy populated raw fixture artifacts.
- Cycle 602: Generate native terrain/weather-heavy populated raw fixture
  artifacts using the byte-complete C land transfer layer.
- Cycle 603: Generate save/open-derived populated raw fixture artifacts.
- Cycle 604: Add Rust reader value summaries for every new populated raw
  fixture.
- Cycle 605: Add Rust writer byte-roundtrip checks for every new populated raw
  fixture.
- Cycle 606: Promote broader populated raw fixtures into native raw value/byte
  gates with mutation checks.
- Cycle 607: Remove populated raw fixture blockers from corpus and final
  readiness inventories.

Run status for cycles 600-607: completed as broader direct raw fixture
generation and value coverage, with byte promotion still intentionally open for
populated artifacts. The native raw oracle now emits `raw_social_heavy.native`,
`raw_immune_heavy.native`, `raw_terrain_heavy.native`, and
`raw_save_open_derived.native` alongside the existing empty, reset, and
after-one-cycle artifacts. The populated raw fixture inventory reports
`current=7 needed=0`, and the raw value gate compares all seven C summaries
against Rust loads. Empty startup and reset startup remain byte-exact; the five
populated fixtures are value-exact with byte-exact roundtrip promotion still
reported as pending.

## Exact Corpus Promotion: Cycles 608-615

- Cycle 608: Promote `long_seeded_command_corpus` into exact raw transcript,
  state trace, and artifact gates.
- Cycle 609: Promote `multi_day_runtime_matrix` into exact transcript and state
  trace gates.
- Cycle 610: Promote `multi_month_runtime_matrix` into exact transcript and
  state trace gates.
- Cycle 611: Promote `save_open_runtime_continuity` into exact transcript,
  state trace, and byte artifact gates.
- Cycle 612: Promote `exhaustive_command_surface` into exact raw transcript
  gates, including file-producing commands.
- Cycle 613: Merge redundant corpus coverage and delete obsolete pending-only
  fixtures after exact replacements exist.
- Cycle 614: Make corpus promotion fail on any regression to blocked, triage,
  inventory, or pending status.
- Cycle 615: Wire all promoted corpora into the required absolute parity
  pipeline and profile/platform gates.

Run status for cycles 608-615: inventory hardening completed, exact promotion
still blocked. The strict readiness path now treats broader populated raw
fixture bytes and after-day runtime status as explicit blockers, so the long
seeded, multi-day/month, save/open, and exhaustive corpora cannot be reported
as complete while they still depend on inventory-only runtime categories.

## Final Strict Signoff: Cycles 616-620

- Cycle 616: Run repeated clean-checkout absolute parity to catch
  nondeterminism across generated artifacts and long corpora.
- Cycle 617: Run debug/release/profile/compiler matrix checks and close any
  profile-specific drift.
- Cycle 618: Run platform checks or record constrained host-oracle behavior
  where a second native platform is unavailable.
- Cycle 619: Remove or reclassify all remaining accepted-difference,
  documented-difference, pending, inventory-only, and triage-only markers.
- Cycle 620: Final absolute native C parity signoff with strict readiness mode
  passing and no accepted/documented differences left.

Run status for cycles 616-620: strict signoff gating was tightened rather than
closed. Final readiness now composes selected-being, after-day, selected-minute,
save/open, broader populated raw byte, and corpus-promotion manifests, and
remains blocked by design until every inventory-only category becomes exact.

---

<a id="rust-port-cycle-621-680-planmd"></a>

## RUST_PORT_CYCLE_621_680_PLAN.md

Original file: `RUST_PORT_CYCLE_621_680_PLAN.md`

# Rust Port Cycle 621-680 Native Parity Closure Plan

## Scope

This 60-cycle extension covers the remaining hard parity work identified after
cycles 581-600. The first 55 cycles map directly to the requested work buckets,
and the final 5 cycles are reserved for stabilization, strict signoff cleanup,
and preventing newly exact gates from regressing back to inventory status.

The planned cycle budget is:

- 12 cycles: after-day movement/body/energy/honor drift.
- 16 cycles: brain/social/episodic/immune runtime parity.
- 9 cycles: C/Rust save-open post-load advancement and transcript parity.
- 8 cycles: populated raw byte promotion for social-heavy, immune-heavy,
  terrain-heavy, and save/open-derived fixtures.
- 10 cycles: exact promotion of long seeded, multi-day/month, save/open, and
  exhaustive corpora.
- 5 cycles: stabilization, strict readiness, and final cleanup.

## After-Day Movement, Body, Energy, Honor: Cycles 621-632

- Cycle 621: Capture a refreshed selected-being after-day trace with C and Rust
  rows for location, facing, speed, speed history, awake, energy, mass, height,
  posture, honor, drives, inventory, and terrain/contact context.
- Cycle 622: Split the after-day drift into movement vector drift, energy-cost
  drift, body-state drift, and social/honor side-effect drift with first
  mismatch minute markers.
- Cycle 623: Close native wander/rest/eat/swim movement ordering at the first
  post-minute-60 divergence without changing the promoted minute-60 exact gate.
- Cycle 624: Close selected-being facing, temporary speed, speed history, and
  movement vector parity through the first day boundary.
- Cycle 625: Close resting, movement, eating, water, and body-fat energy cost
  parity at the selected being's exact native locations.
- Cycle 626: Close body mass, body fat, height/growth, posture, wound,
  parasite, grooming, and inventory side effects that change the after-day row.
- Cycle 627: Close hunger, fatigue, sex, social drive, and goal reset ordering
  between minute 60 and day one.
- Cycle 628: Close honor recalibration and social honor side effects visible in
  the selected after-day trace.
- Cycle 629: Add mutation checks proving the selected after-day movement and
  energy fields fail when one native operation is perturbed.
- Cycle 630: Promote after-day movement/body/energy/honor trace rows from
  inventory to exact value gates.
- Cycle 631: Re-run long seeded day-one slices to confirm no movement/body
  regression outside the selected fixture.
- Cycle 632: Remove after-day movement/body/energy/honor blockers from the
  drift register and final signoff readiness inventory.

Run status for cycles 621-630: completed as after-day drift classification, not
yet exact parity promotion. `scripts/run_after_day_drift_inventory.sh` captures
the selected after-day rows, buckets C/Rust differences into selection,
movement, energy, body, honor/drives, brain/probe, social, episodic, and immune
groups, and wires the result into absolute parity CI and final signoff
readiness. Current after-day status is `inventory`: selection and body are
exact, while movement, energy, honor/drives, brain/probe, social, episodic, and
immune remain open.

Run status for cycles 631-632: completed as day-one slice regression
classification, not blocker removal. `scripts/run_after_day_slice_inventory.sh`
now records the first selected-minute mismatch for movement, energy, body, and
honor/drives, and is wired into absolute parity CI and final signoff readiness.
Current status is `inventory`: movement and energy first diverge at minute 180,
body state first diverges at minute 180, and honor/drives first diverges at
minute 120.

## Brain, Social, Episodic, Immune Runtime: Cycles 633-648

- Cycle 633: Capture native brain VM checkpoints for selected and neighboring
  beings before and after internal and external braincode execution.
- Cycle 634: Port or align native internal braincode instruction execution
  ordering for register, data, conditional, sensor, and actuator operations.
- Cycle 635: Port or align native external/social braincode execution ordering
  for the self slot and first social-memory slot.
- Cycle 636: Close brain register, brain state, attention, shout, speech, and
  listen side effects through minute 60 and the first day boundary.
- Cycle 637: Close brain probe type, position, address, frequency, offset, and
  state mutation ordering.
- Cycle 638: Capture social target selection, vicinity ordering, and social
  graph update rows from C at the first social divergence.
- Cycle 639: Close attraction, belief, friend/foe, familiarity, relationship,
  entity type, and social coordinate updates.
- Cycle 640: Close chat, grooming, squabble, mate, anecdote, and preference
  learning side effects in social-heavy traces.
- Cycle 641: Capture episodic insert, replace, fade, affect, food, argument,
  and location rows through day one.
- Cycle 642: Close episodic memory ordering and event payload parity for
  selected and social-heavy traces.
- Cycle 643: Capture immune antigen, antibody, shape, seed, infection, and
  transmission rows around the first immune divergence.
- Cycle 644: Close immune runtime cycling, mutation, infection response,
  maternal/sex/food/touch transmission, and energy coupling.
- Cycle 645: Add mutation checks for brain/probe/social/episodic/immune
  promoted fields.
- Cycle 646: Promote brain/social/episodic/immune runtime rows from inventory
  to exact value gates.
- Cycle 647: Re-run selected-minute, selected-being, social-heavy, and
  immune-heavy inventories to confirm exact status.
- Cycle 648: Remove brain/social/episodic/immune blockers from final signoff
  readiness and corpus promotion manifests.

Run status for cycles 633-648: completed as brain/social/episodic/immune
runtime bucket classification, not exact promotion. `scripts/run_brain_social_runtime_inventory.sh`
now records first selected-minute mismatch points and after-day diffs for the
brain/probe, social, episodic, and immune buckets. Current status is
`inventory`: brain/probe and social first diverge at minute 60, immune first
diverges at minute 180, and episodic first diverges at minute 660.

## Save/Open Post-Load Advancement And Transcript Parity: Cycles 649-657

- Cycle 649: Capture C and Rust populated save/open transcripts with timing,
  selected-state, land-state, and artifact-byte sidecar traces.
- Cycle 650: Close command transcript drift for save/open runtime messages,
  stop/start timing, selection state, and population semantics.
- Cycle 651: Match post-open one-minute advancement for selected movement,
  body, brain, social, episodic, immune, terrain, and food fields.
- Cycle 652: Match post-open one-day advancement against C state traces.
- Cycle 653: Match post-open multi-day/month advancement and rollover behavior.
- Cycle 654: Close loaded random-state continuity and selected-being identity
  continuity across C/Rust post-open advancement.
- Cycle 655: Close malformed populated-open recovery behavior without changing
  promoted malformed-loader diagnostics.
- Cycle 656: Promote `save_open_runtime_continuity` into exact transcript,
  state trace, and mutation gates.
- Cycle 657: Remove save/open post-load advancement and transcript blockers
  from corpus promotion and final signoff readiness.

Run status for cycles 649-657: completed as Rust post-load advancement
inventory and stricter readiness coverage. `scripts/run_save_open_continuity_inventory.sh`
now compares continued Rust state against reopened Rust state after one minute
and one day. Immediate before-save/after-open state and one post-load minute are
exact; one post-load day is still `inventory`, so save/open remains blocked for
longer loaded-runtime advancement and C/Rust transcript parity.

## Populated Raw Byte Promotion: Cycles 658-665

- Cycle 658: Generate and inventory the immune-heavy populated direct raw
  fixture with native summaries and byte maps.
- Cycle 659: Generate and inventory the terrain/weather-heavy populated direct
  raw fixture with byte-complete land payloads.
- Cycle 660: Generate and inventory the save/open-derived populated direct raw
  fixture.
- Cycle 661: Add Rust reader summaries for social-heavy, immune-heavy,
  terrain-heavy, and save/open-derived raw fixtures.
- Cycle 662: Add Rust raw writer byte-roundtrip comparisons for every broader
  populated fixture.
- Cycle 663: Close populated raw territory, social, episodic, immune, brain,
  terrain, and land payload byte drift in the broader fixture set.
- Cycle 664: Promote broader populated raw fixtures into exact value and byte
  gates with mutation checks.
- Cycle 665: Remove populated raw byte blockers from corpus promotion, final
  readiness, and the raw C binary oracle notes.

Run status for cycles 658-660: completed as artifact presence via the broader
direct raw oracle. The immune-heavy, terrain-heavy, and save/open-derived
fixtures are generated and inventoried with the seven-artifact raw oracle set.
Exact populated byte promotion remains for cycles 661-665.

Run status for cycles 661-665: completed as byte-diff classification and
strict readiness integration, not exact byte closure. `scripts/run_populated_raw_byte_diff_inventory.sh`
now reuses the native raw value gate output, compares every populated native
raw artifact against the Rust roundtrip, records first byte-diff offsets and
nearest transfer sections, and includes a mutation check. All five populated
raw artifacts remain `value-exact-byte-pending`; the first byte difference in
each is inside a `being{` section.

## Exact Corpus Promotion: Cycles 666-675

- Cycle 666: Promote `long_seeded_command_corpus` to exact raw transcript,
  state trace, artifact, and mutation gates.
- Cycle 667: Promote `multi_day_runtime_matrix` to exact transcript and state
  trace gates.
- Cycle 668: Promote `multi_month_runtime_matrix` to exact transcript and state
  trace gates.
- Cycle 669: Promote `save_open_runtime_continuity` to exact transcript, state
  trace, and byte artifact gates.
- Cycle 670: Promote `exhaustive_command_surface` to exact raw transcript gates,
  including file-producing commands and edge-case command ordering.
- Cycle 671: Merge or retire redundant pending-only fixtures after exact gates
  cover the same behavior.
- Cycle 672: Make corpus promotion fail on any blocked, pending, inventory-only,
  triage-only, or accepted-difference status.
- Cycle 673: Wire all promoted corpora into the absolute parity CI and final
  signoff readiness pipeline.
- Cycle 674: Add profile/debug/release checks for all promoted corpora.
- Cycle 675: Remove long seeded, multi-day/month, save/open, and exhaustive
  corpus blockers from the final drift register.

Run status for cycles 666-675: completed as exact-promotion hardening, with all
five corpora still blocked. The corpus inventory now reports concrete blocker
families: day-one movement/energy/honor runtime, brain/social/episodic/immune
runtime, save/open post-load day continuity, save/open raw transcript,
selected-minute brain/social/detail values, and file-producing command order.
`scripts/run_exact_corpus_promotion_gate.sh` correctly exits blocked with
`total=5 ready=0 blocked=5`.

## Stabilization And Strict Readiness: Cycles 676-680

- Cycle 676: Run repeated clean-checkout absolute parity to detect
  nondeterminism in long traces, save/open artifacts, and raw fixture bytes.
- Cycle 677: Run debug/release/profile/compiler matrix checks and close any
  profile-specific or host-specific drift.
- Cycle 678: Search for and remove remaining accepted-difference,
  documented-difference, pending, inventory-only, and triage-only markers.
- Cycle 679: Run strict final signoff readiness and close any last manifest or
  documentation mismatch.
- Cycle 680: Final native C parity signoff for the command-line Rust `simape`
  path with no accepted/documented differences left.

Run status for cycles 676-680: completed as stabilization and strict readiness
cleanup, not final parity signoff. Final readiness now reuses the raw value gate
for raw byte-diff inventory instead of generating duplicate raw oracle outputs,
so it reaches a manifest under the current host storage limit. It reports
`status=blocked blockers=5`: selected/after-day runtime, brain/social/runtime,
save/open continuity, broader populated raw byte promotion, and exact corpus
promotion remain open.
