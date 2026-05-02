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

## Parity Closure Extension: Cycles 206-265

The 106-205 plan completed the broad Rust implementation pass, but true
byte-for-byte / transcript-level native C parity still needs raw binary
save/load support, reproducible C fixture generation, long trace comparison,
and final gate hardening. These 60 cycles cover that remaining proof-and-parity
work.

### Cycle 206: Raw Binary Format Audit

Map the exact native C raw/binary save layout, including signatures, version
blocks, section order, struct padding, endian behavior, and array counts.

Validation: binary layout audit with offsets tied to C `sizeof` and
`offsetof` checks.

### Cycle 207: Binary Reader Framework

Add a byte-reader layer for little-endian native primitives and fixed-size
C-shaped structures without changing existing JSON/native-text loading.

Validation: primitive reader fixtures and malformed-length tests.

### Cycle 208: Binary Version And Land Load

Load raw binary signature, version, land date/time/genetics, terrain metadata,
and any stable land blocks.

Validation: C-generated land-only binary fixture loads into Rust.

### Cycle 209: Binary Being Delta Load

Load being delta fields from raw binary saves, including location, facing,
velocity, energy, state, body, goal, and social coordinates.

Validation: delta fixture comparison against C offsets.

### Cycle 210: Binary Being Constant Load

Load being constant fields, identity, birth date, generation range, and
genetics from raw binary saves.

Validation: constant fixture comparison.

### Cycle 211: Binary Events Load

Load raw binary social, episodic, and territory arrays into Rust C-shaped
state.

Validation: event array binary fixture round-trip into Rust state.

### Cycle 212: Binary Brain/Volatile Load

Load brain registers, probes, state, script overrides, attention, drives,
shout, inventory, preferences, pregnancy, and child-generation fields.

Validation: full-being binary fixture.

### Cycle 213: Binary Immune Load

Load immune antigens, antibodies, shapes, and random seeds exactly from raw
binary saves.

Validation: immune-heavy fixture comparison.

### Cycle 214: Binary Reader Edge Cases

Handle empty populations, maximum populations, truncated files, unknown
sections, and version mismatch failures with C-compatible behavior.

Validation: negative binary fixture suite.

### Cycle 215: Legacy Save Corpus Setup

Collect or generate representative older/native saves and classify which
versions must be supported or explicitly rejected.

Validation: corpus manifest and load expectation table.

### Cycle 216: Binary Writer Framework

Add a byte-writer layer for raw native structures while preserving JSON and
native-text transfer output.

Validation: writer primitive and section-header tests.

### Cycle 217: Binary Land Write

Write raw binary version and land blocks in native C order.

Validation: Rust-written land fixture loads in C.

### Cycle 218: Binary Being Write

Write being delta, constant, events, brain, volatile, and immune state in raw
C-compatible order.

Validation: C loader accepts simple Rust-written populated fixture.

### Cycle 219: Binary Empty/Maximum Write

Write empty, single-being, normal, and maximum-population raw binary fixtures.

Validation: C load behavior matches expected population counts and state.

### Cycle 220: Binary Cross-Load Round Trip

Round-trip C saves through Rust load/write and Rust saves through C load/write.

Validation: cross-load state comparison after one cycle.

### Cycle 221: Binary Byte Diff Gate

Compare stable Rust-written fixtures against C-written fixtures byte for byte
where no timestamps/randomized headers differ.

Validation: byte diff tests with documented normalizers if needed.

### Cycle 222: Binary CLI Integration

Wire raw binary save/open behavior into `simape` behind the expected file
extensions while keeping JSON/native-text compatibility.

Validation: CLI open/save binary transcript fixtures.

### Cycle 223: Binary Regression Lock

Add regression tests for every binary parsing/writing edge fixed so far.

Validation: repeated binary fixture suite.

### Cycle 224: Binary Compatibility Report

Document supported raw binary versions, unsupported versions, and exact drift
status.

Validation: binary compatibility report reviewed against fixture corpus.

### Cycle 225: C Build Harness

Create a reproducible local script that builds the native C command-line
`simape` and records compiler/version details.

Validation: harness builds C and Rust from a clean checkout.

### Cycle 226: C Transcript Runner

Extend the existing CLI golden runner to execute both C and Rust sessions with
timeouts, output capture, and normalized line endings.

Validation: help/session transcripts compare through the harness.

### Cycle 227: Transcript Normalization

Normalize expected volatile fields such as dates, temporary paths, random save
sizes, and platform-specific line endings without masking behavioral drift.

Validation: normalization unit tests and sample diff.

### Cycle 228: C State Trace Hooks

Add optional trace points for C and Rust covering land, population, selected
being, energy, position, drives, brain, social, territory, and lifecycle state.

Validation: one-minute trace files generated for both implementations.

### Cycle 229: Trace Diff Tool

Build a structured diff tool for state traces with clear mismatch categories
and first-difference reporting.

Validation: intentional mismatch fixture produces readable output.

### Cycle 230: Fixture Manifest

Create a manifest describing every save, command script, seed, expected gate,
and normalization rule.

Validation: manifest parser test and fixture existence check.

### Cycle 231: Harness CI Script

Add a local CI-style script that runs Rust tests, C build, C/Rust transcripts,
binary fixtures, and trace fixtures in a stable order.

Validation: script runs locally and reports pass/fail summary.

### Cycle 232: Harness Documentation

Document how to generate, review, normalize, and update C-derived goldens.

Validation: README walkthrough from clean checkout.

### Cycle 233: Braincode C Trace Fixtures

Generate C traces for representative braincode programs covering data,
arithmetic, control flow, sensors, actuators, probes, and anecdotes.

Validation: fixture files committed with manifest entries.

### Cycle 234: Braincode Decode Trace Parity

Compare Rust decode, address wrapping, constant flags, and program-counter
movement against C traces.

Validation: decode trace parity tests.

### Cycle 235: Braincode Arithmetic Trace Parity

Close drift in `DAT`, arithmetic, byte mutation, register, and control
operators.

Validation: arithmetic trace parity tests.

### Cycle 236: Braincode Sensor Trace Parity

Close drift in body, drive, social, episodic, terrain, weather, immune, and
territory sensor values.

Validation: sensor trace parity table.

### Cycle 237: Braincode Actuator Trace Parity

Close drift in action, goal, friend/foe, attraction, familiarity, probes,
shout, posture, preferences, intentions, and anecdotes.

Validation: actuator trace parity tests.

### Cycle 238: Braincode Scheduling Trace Parity

Match C dialogue iteration counts, internal/external scheduling, probe
frequency, offset, and state update timing.

Validation: scheduling trace parity tests.

### Cycle 239: Braincode Social Hook Parity

Match social braincode initialization and per-social-entry braincode behavior
for newly met beings and chat interactions.

Validation: social braincode fixture traces.

### Cycle 240: Braincode Long-Run Freeze

Run long seeded braincode-enabled simulations and fix remaining VM drift.

Validation: long braincode trace parity.

### Cycle 241: Social Action Drift Pass

Close drift in social actions, body inventory actions, giving, pickup/drop,
brandish, drag, bash, chew, fish, and shout-visible states.

Validation: social/body action trace fixtures.

### Cycle 242: Groom/Squabble Drift Pass

Close drift in grooming selection, wounds, parasites, honor, squabble force,
attack, flee, and episodic side effects.

Validation: grooming/squabble trace parity.

### Cycle 243: Mate/Preference Drift Pass

Close drift in attraction, mate bond, mate preference learning, sex drive, mate
goals, and conception trigger behavior.

Validation: mate/preference fixture parity.

### Cycle 244: Territory Drift Pass

Close drift in territory familiarity indexing, rescaling, naming, chat-based
agreement, and territory-focused attention.

Validation: territory trace parity.

### Cycle 245: Family Relationship Drift Pass

Close drift in mother/father/child/grandparent/sibling relationship inference
and social graph storage rules.

Validation: controlled family-tree fixtures.

### Cycle 246: Pregnancy And Birth Drift Pass

Close drift in conception date, fetal genetics, gestation, birth creation,
population capacity, and child initialization.

Validation: pregnancy/birth trace fixtures.

### Cycle 247: Carrying/Suckling Drift Pass

Close drift in carrying, suckling, weaning, mother/child energy transfer,
immunity seeding, and post-birth episodic events.

Validation: mother/child lifecycle trace fixtures.

### Cycle 248: Immune Transmission Drift Pass

Close drift in air, touch, sex, and food pathogen transmission, antibody
mutation, antigen severity, and energy cost.

Validation: immune trace parity.

### Cycle 249: Movement/Body Drift Pass

Close drift in walking, swimming, water avoidance, slope energy, mass, height,
fat, posture, fatigue, and velocity history.

Validation: movement/body trace parity.

### Cycle 250: Food/Terrain Drift Pass

Close drift in terrain sampling, tide, weather, food choice, depletion,
regrowth, absorption, and pathogen ingestion.

Validation: terrain/food trace parity.

### Cycle 251: Empty Startup Transcript Matrix

Compare C and Rust for empty startup, help, memory, file, save/open, errors,
and no-population detail commands.

Validation: empty transcript matrix passes.

### Cycle 252: Populated Short Transcript Matrix

Compare reset, run, stats, top, social, pathogen, episodic, braincode, probes,
speech, idea, and navigation commands after short runs.

Validation: populated short transcript matrix passes.

### Cycle 253: Save/Open Continuity Matrix

Compare save/open loops across JSON, native text, raw binary, malformed files,
and cross-loaded files.

Validation: save/open transcript and state matrix passes.

### Cycle 254: Multi-Day Seeded Matrix

Run multi-day seeded simulations with trace and transcript comparison.

Validation: multi-day C/Rust parity report.

### Cycle 255: Multi-Month Seeded Matrix

Run multi-month seeded simulations with save/open continuity and drift
triage.

Validation: multi-month C/Rust parity report.

### Cycle 256: Population Stress Matrix

Run near-maximum and maximum population scenarios through command, trace, and
save/load gates.

Validation: stress parity report.

### Cycle 257: Command Edge Case Sweep

Match parsing, aliases, missing arguments, malformed commands, command ordering,
and C error text/file-line behavior.

Validation: command edge-case golden suite.

### Cycle 258: Release/Debug Determinism

Verify debug and release Rust builds produce identical observable transcripts,
traces, and saves for approved fixtures.

Validation: debug/release diff gate.

### Cycle 259: Cross-Platform Determinism

Audit macOS/Linux newline, path, integer, filesystem, and locale behavior for
stable fixture output.

Validation: platform notes and normalized transcript tests.

### Cycle 260: Performance Baseline And Corrections

Measure Rust versus C on common runs, optimize hot paths only where trace
output remains unchanged, and document any accepted performance gap.

Validation: benchmark report plus parity tests.

### Cycle 261: Public API Cleanup

Clean up internal Rust APIs introduced during parity work while preserving CLI,
fixture, and transfer behavior.

Validation: full tests and docs check.

### Cycle 262: Final Documentation Pass

Finalize architecture, command behavior, save/load compatibility, fixture
generation, drift categories, and release notes.

Validation: documentation review against implemented behavior.

### Cycle 263: Known Drift Zero Gate

Ensure every known drift item is fixed, fixture-proven unobservable, or
explicitly accepted with rationale.

Validation: zero untriaged drift list.

### Cycle 264: Final Parity Gates

Run final binary byte diff, transcript diff, state trace diff, release/debug,
and stress gates together.

Validation: all final gates pass repeatedly without flakes.

### Cycle 265: Native C Parity Signoff

Freeze Rust `simape` as byte-for-byte save compatible and transcript-level
native C compatible for the approved corpus.

Validation: signed final parity report with fixture manifest, gate outputs,
and any explicitly accepted non-observable differences.
