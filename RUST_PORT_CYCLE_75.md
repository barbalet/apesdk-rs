# Rust Port Cycle 75: Social Graph Output Parity

## Objective

Replace placeholder friends/enemies output with data-backed social graph rows.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_75.md
```

## Implemented

The CLI `friends`, `social`, `socialgraph`, and `graph` commands now iterate
native social memory entries. Rows are split into friends and enemies using
`SOCIAL_RESPECT_NORMAL`, skip empty native entries, include familiarity,
relationship text, and attraction status.

Initial reset output remains empty like the C command because only the self
slot is populated at reset.

## Validation

Added a CLI fixture test that loads social memory and verifies friend and enemy
rows are printed from data.

Run:

```sh
cargo fmt
cargo test
```

Current result:

```text
105 tests passed
```

## Next Cycle

Cycle 76 should port the social initial and secondary loops so familiarity,
self links, and relationship maintenance evolve during runtime.

