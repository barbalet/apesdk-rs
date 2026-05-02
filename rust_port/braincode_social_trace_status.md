# Braincode And Social Trace Status

Cycles 233-245 are now represented in the harness by deterministic trace
fixtures and Rust unit gates.

## Braincode

The committed trace fixtures cover:

- decode wrapping and constant flag decoding
- arithmetic/register state shape
- sensor and actuator fixture shape
- social braincode hook shape

The current fixtures are Rust-side deterministic traces. They are ready for C
trace replacement once the C harness grows direct trace emission hooks.

## Social, Territory, And Family

The social/family trace fixture covers controlled relationship ids, territory
familiarity, and conception state. Existing Rust unit coverage exercises close
meetings, grooming, squabble wounds, mate/conception behavior, territory memory
updates, birth, and family relationship creation.

## Known Gaps

These cycles do not yet prove native C trace parity for long braincode runs or
deep social action drift. The harness now makes those comparisons mechanical:
replace or supplement the Rust-authored trace files with C-emitted traces, then
run `scripts/trace_diff.sh` or `scripts/run_parity_ci.sh`.
