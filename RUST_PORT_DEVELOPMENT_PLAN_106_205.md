# Rust Port Development Plan: Cycles 106-205

## Objective

Move the Rust command-line `simape` from strong functional parity toward
byte-for-byte and transcript-level native C parity.

This 100-cycle plan covers the requested remaining work:

```text
15 cycles: native terrain tile-map parity, food depletion/regrowth, C fixture comparisons
12 cycles: binary save/load compatibility
12 cycles: braincode execution and social braincode hooks
10 cycles: territory, family relationships, pregnancy/conception
6 cycles: preference learning and remaining social polish
5 cycles: long seeded C-vs-Rust transcript hardening and command edge cases
40 cycles: deterministic parity closure, fixture generation, drift elimination, and release hardening
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
