# Rust Port Cycle 77: Social Interaction Actions

## Objective

Add the first runtime social actions used by awake cycling.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_77.md
```

## Implemented

Added reduced grooming, squabble/show-force, and mate-seeking hooks. Grooming
raises friend/foe, adjusts honor/body attention, reduces parasites, and records
grooming episodic events. Squabble lowers friend/foe, spends energy, sets
show-force state, and records hit/hit-by events. Mature opposite-sex beings
with high sex drive can set mate-seeking goals and memories.

## Compatibility Notes

The C probability model, stereotypes, attraction calculations, and mating
biology remain deeper follow-up work.

## Validation

Added deterministic paired-being tests covering grooming state and episodic
recording.

