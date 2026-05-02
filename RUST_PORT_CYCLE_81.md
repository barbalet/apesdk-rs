# Rust Port Cycle 81: Native Save Format Expansion

## Objective

Expand JSON saves with native fields that were still omitted.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_81.md
```

## Implemented

JSON save output now includes:

```text
events.social
events.episodic
braindata.braincode_register
braindata.brainprobe
braindata.brain_state
braindata.script_overrides
braindata.attention
changes.drives
changes.shout
changes.inventory
changes.learned_preference
changes.date_of_conception
changes.fetal_genetics
changes.father_name
changes.mother_name
changes.child_generation_range
immune_system
```

## Validation

Added expanded save-structure assertions.

