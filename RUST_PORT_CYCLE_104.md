# Rust Port Cycle 104: Chat And Anecdote Exchange

## Objective

Port chat and anecdote exchange enough for social memories to carry stories.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_104.md
```

## Implemented

Added respect-gated chat, speaking state, drive reset, chat episodic records,
and copied anecdote memories with event/affect mutation hooks from learned
preferences.

## Validation

Added a chat fixture proving chat state and copied food anecdote memory.
