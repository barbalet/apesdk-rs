# Rust Port Cycle 79: Episodic Event Recording

## Objective

Port the core event-recording helpers used by eating and social actions.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_79.md
```

## Implemented

Added reduced equivalents of food and interaction event storage. Eating records
`EVENT_EAT`; grooming, being groomed, squabble, and mate seeking record their
corresponding social events. Episodic affect now fades back toward
`EPISODIC_AFFECT_ZERO` while beings are awake.

Event-mode CLI commands continue to control the public mode state; full C-style
event logging callbacks are documented as remaining work.

## Validation

Added tests for recording, affect fading, and social-action memory creation.

