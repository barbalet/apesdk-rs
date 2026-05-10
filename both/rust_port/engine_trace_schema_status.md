# Engine Trace Schema Status

Cycles 321-340 promote a direct native C engine trace probe and a matching Rust
trace formatter schema.

## Exact Today

- C and Rust both emit three `TRACE` snapshots: `startup`, `after_cycle_1`, and
  `after_day`.
- Field order is identical under `scripts/run_engine_trace_schema_gate.sh`.
- The schema covers terrain, food, weather, population, selected-being body
  state, drives, braincode registers/probe zero, social entry zero, episodic
  entry zero, territory entry zero, conception/family fields, immune fields,
  inventory, shout, and preference state.
- The selected-minute runtime schema additionally carries selected random seed,
  goal, social coordinates, brain state, attention, script override, expanded
  shout bytes, expanded social/episodic/territory samples, terrain/food
  sidecars, body posture/parasites/crowding/inventory, and full first
  immune antigen/antibody/shape/seed values.
- Focused inventory gates now split the selected-minute schema into day-one
  movement/body/honor, brain/social runtime, and immune/episodic runtime
  buckets so later value promotion can close one bucket without weakening the
  broader trace schema.

## Not Yet Exact

The schema gate intentionally does not compare values yet. The direct C probe
exposes remaining value drift in startup cycling, terrain/topography generation,
weather/pressure, food biology, initial population creation, and selected-being
state. Those values are the closure target for the following deep parity cycles.

Cycle 341-350 moved several selected-being categories from placeholders toward
native-shaped Rust behavior: child genetics, pregnancy/birth timing, family
fan-out, carrying/suckling, maternal antibody transfer, pathogen transmission,
and stopped/swimming movement thresholds. These are still value-parity targets
until direct C/Rust trace comparisons are promoted beyond schema order.

## Gates

- `scripts/generate_c_engine_trace_probe.sh`
- `scripts/generate_rust_engine_trace_probe.sh`
- `scripts/run_engine_trace_schema_gate.sh`
- `scripts/run_after_day_slice_inventory.sh`
- `scripts/run_brain_social_runtime_inventory.sh`
- `scripts/run_immune_episodic_runtime_inventory.sh`
