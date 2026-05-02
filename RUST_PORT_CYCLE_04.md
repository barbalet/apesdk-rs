# Rust Port Cycle 04: Primitive Aliases And Constants

## Objective

Start the Rust implementation with C-compatible primitive aliases and simulator
constants that match the current command-line C build.

## Added Files

```text
Cargo.toml
Cargo.lock
crates/apesdk-toolkit/Cargo.toml
crates/apesdk-toolkit/src/lib.rs
crates/apesdk-sim/Cargo.toml
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_04.md
```

Updated `.gitignore` to ignore Cargo's generated `/target/` directory while
preserving the existing Xcode/editor ignore entries.

## Implemented

`apesdk-toolkit` now defines the foundational C-compatible aliases:

```text
n_double, n_char, n_string, n_constant_string
n_byte, n_byte2, n_byte4, n_c_int, n_int, n_uint, n_audio
n_string_block
```

It also starts the layout surface for common toolkit structs:

```text
n_vect2, n_area2, n_quad, n_line, n_vect3
n_rgba, n_rgba32, n_spacetime, n_file
simulated_console_command
```

`apesdk-sim` now defines the command-line constants used by the current C
baseline:

```text
SHORT_VERSION_NAME, FULL_DATE, VERSION_NUMBER
SIMULATED_APE_SIGNATURE, SIMULATED_WAR_SIGNATURE
MAP_BITS, MAP_DIMENSION, MAP_AREA
TERRITORY_DIMENSION, TERRITORY_AREA
OFFSCREENSIZE, LARGE_SIM
BRAINCODE_* constants
TIME_* constants
```

## Compatibility Notes

`n_int` and `n_uint` intentionally use `std::os::raw::c_long` and
`std::os::raw::c_ulong`, matching the current C command-line target.

`FULL_DATE` defaults to `May  1 2026` to match the current golden transcripts,
but can be overridden at Rust compile time with `APESDK_FULL_DATE`. This keeps
the initial tests stable while leaving room to reproduce C `__DATE__` behavior
when Rust build integration lands.

## Validation

Run:

```sh
cargo test
```

The tests lock down primitive type sizes, initial `#[repr(C)]` layout sizes, and
the command-line constants used by the golden CLI transcripts.

## Next Cycle

Cycle 05 should port the first toolkit behavior: memory helpers, string length
and find helpers, and basic vector operations, with tests based on the C API.
