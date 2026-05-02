# Rust Port Cycle 134: Braincode VM Skeleton

## Objective

Add Rust braincode decode and VM scaffolding.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_134.md
```

## Implemented

Added native opcode constants, `BraincodeInstruction`, `braincode_decode`, and
`BraincodeVm` with local/remote memory, P-space registers, program counter,
and one-step execution.

## Validation

Added decode tests that verify constant flags and wraparound argument reads.
