# Rust Port Cycle 131: Binary Compatibility Edge Cases

## Objective

Handle malformed native transfer boundaries and unsupported sections.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_131.md
```

## Implemented

Native loading now rejects raw byte streams, missing first signature sections,
newer versions, weather sections that are not yet ported, and event sections
that appear before any being.

## Validation

Negative native transfer tests cover raw bytes, missing `simul`, newer version,
unsupported `weath`, and social-before-being ordering.
