# Native Trace Oracle

Cycles 318-331 require C-generated traces for each native engine category.
Cycles 301-320 add the first command-visible native trace harness while the
direct in-engine emitters are still being promoted.

Current trace categories:

- land, tide, weather, terrain, and food
- population, movement, body, energy, and drives
- braincode, probes, social hooks, and speech
- social, episodic, territory, and family relationships
- lifecycle, pregnancy, birth, carrying, suckling, immune, and death
- save/load-visible state

The Rust trace formatter must match C field order, widths, signedness, and
sampling points before a trace category can be promoted into the absolute gate.

## Current Harness

- `golden/cli/sessions/trace_land_startup.commands` captures native `sim`,
  `step`, safe empty `run forever`, and `sim` output.
- `scripts/native_sim_transcript_to_trace.sh` converts command-visible native
  `sim` blocks into stable `TRACE` lines for map dimension, land seed,
  population, adult/juvenile counts, tide, spacetime, and running state.
- `scripts/generate_native_trace_oracles.sh` builds native C, captures the
  trace sessions, extracts trace files, and writes a manifest.
- `scripts/generate_c_engine_trace_probe.sh` links a direct native C probe
  against the engine and emits field-level traces for terrain, food, weather,
  population, body, braincode, social, episodic, territory, lifecycle/family,
  immune, movement, inventory, shout, and preference fields.
- `scripts/run_engine_trace_schema_gate.sh` compares C and Rust trace field
  order before value-diff closure is enabled.
- `scripts/run_engine_trace_value_gate.sh` compares the currently promoted
  C/Rust value subset and proves the comparison fails on a deliberate one-field
  mutation. The promoted subset covers startup scalar/empty-being values,
  invariant zero-valued runtime fields, first-cycle social/episodic/territory
  baseline fields, runtime-core clock/population fields, exact first-cycle
  terrain/weather/food values, and exact first-cycle selected-being native
  initialization/body/drive/brain-probe/immune values. Multi-day terrain and
  after-day selected-being movement, body, honor, brain, social, episodic, and
  immune runtime values remain open.
- `scripts/run_selected_minute_trace_inventory.sh` extracts the direct C/Rust
  `SELECTED-MINUTE` rows emitted hourly through the first day and every minute
  for the final 16 minutes. The gate requires schema and row-count parity and
  records the current value diff as inventory until movement, drive, brain,
  social, episodic, and immune runtime values are exact.
- `scripts/run_selected_minute_value_gate.sh` promotes the selected-being
  minute-60 core values with a deliberate mutation check. The promoted subset
  covers energy, location, facing, speed, honor, mass, awake/state, drives,
  episodic sample, immune sample, and preference. Minute-60 brain registers and
  social memory, plus later selected-minute rows, remain open inventory.

Command-visible traces are useful smoke oracles, but deep terrain, braincode,
social, lifecycle, immune, movement, and save/load closure still requires
direct native C emitters at simulation sampling points. The engine probe is the
first such emitter; exact value parity remains open by category beyond the
promoted subset.
