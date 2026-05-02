# Rust Port Cycle 44: Idea Summary Output

## Objective

Give the `idea` command a populated response so the command surface matches the
C console shape before shared braincode analysis is fully ported.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_44.md
```

## Implemented

`idea` remains quiet for an empty population and prints a deterministic
zero-match table when at least two apes are present.

The table covers block sizes three through eight, matching the C command's
summary categories.

## Compatibility Notes

This is a scaffold for the future shared-braincode comparison. It preserves the
interactive command shape without claiming computed matches yet.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
89 tests passed
```

## Next Cycle

Cycle 45 should harden the extended summary fields with round-trip coverage and
finish this populated-command tranche.
