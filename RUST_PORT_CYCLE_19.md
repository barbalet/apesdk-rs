# Rust Port Cycle 19: Empty Top and Epic Commands

## Objective

Port the C empty-population output for the `top` and `epic` command-line
surfaces.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_19.md
```

## Implemented

Command-line behavior:

```text
top
epic
```

For an empty simulation, `top` prints the C table header and separator:

```text
Honor Name                     Sex	Age
-----------------------------------------------------------------
```

For an empty simulation, `epic` emits no rows and no extra status text.

## Compatibility Notes

The ranking and episodic scoring logic still depends on porting beings and
episodic memory. This cycle only claims the current empty-shell behavior, which
is the only population state Rust can represent today.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
67 tests passed
```

The new transcript verifies the `top` header and quiet `epic` path.

## Next Cycle

Cycle 20 should port empty selected-ape navigation aliases so `next`,
`previous`, and `prev` no longer fall through to the generic unsupported error.
