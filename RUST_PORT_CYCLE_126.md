# Rust Port Cycle 126: Events Binary Load

## Objective

Load social and episodic event sections from native transfer files.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_126.md
```

## Implemented

The native transfer reader now accepts `sgcia{...};` and `episo{...};`
sections after a `being{...};` block and attaches them to that being's events.
Social sections load location, time, date, remembered name, attraction,
friend/foe, belief, familiarity, relationship, entity type, and optional local
braincode. Episodic sections load spacetime, names, event, food, affect, and
arg.

## Validation

Added a native fixture that loads social and episodic sections and verifies the
restored event arrays.
