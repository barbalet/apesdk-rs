# Rust Port Cycle 03: API And Type Surface Inventory

## Objective

Inventory the public C API and core type surface that the Rust port must model
before behavior can be converted safely.

## Added Files

```text
rust_port/api_inventory.md
```

## Findings

The required Rust foundation is not just `longterm.c`; it starts with the C type
vocabulary in `toolkit/toolkit.h`, the simulation constants and land/time types
in `sim/sim.h`, and the large state model in `universe/universe.h`.

The most important compatibility rule is to keep `n_int` and `n_uint` as C
`long` and `unsigned long` equivalents at layout-sensitive boundaries. The file
format uses fixed-width fields such as `n_byte`, `n_byte2`, and `n_byte4`, but
many in-memory structures contain `n_int` or `n_uint`.

The C headers have a circular dependency between universe and entity concepts.
The Rust port should split shared state structs into a separate model module or
crate rather than reproducing that cycle.

## Port Order Established

The next implementation cycles should start with:

1. Primitive aliases and constants.
2. Toolkit memory, file, string, vector, math, random, and object/json helpers.
3. Sim land/time/map state.
4. Universe and entity state structs with layout tests.
5. Transfer and console command behavior.

## Validation

This cycle changed documentation only. No C source or build script behavior was
modified.

## Next Cycle

Cycle 04 should start creating Rust scaffolding for primitive aliases and
constants, with tests that lock down type sizes and the command-line constants
used by the golden transcripts.
