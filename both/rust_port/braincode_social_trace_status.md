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

Cycle 681-710 update: the direct C/Rust selected-minute probe now emits
native-side brain registers, brain state, attention, probe zero, script
override, and shout bytes at the first-day sampling points. The refreshed
runtime inventory places the first brain/probe mismatch at minute 60.

Cycle 711-740 update: populated raw byte-diff token classification moved the
first raw mismatch from awake-state serialization to `brpro=` for all five
populated raw scenarios. Brain-probe runtime and brain-probe raw serialization
are now the same next target from two independent oracles.

## Social, Territory, And Family

The social/family trace fixture covers controlled relationship ids, territory
familiarity, and conception state. Existing Rust unit coverage exercises close
meetings, grooming, squabble wounds, mate/conception behavior, territory memory
updates, birth, and family relationship creation.

Cycle 681-710 update: selected-minute rows now include expanded social slot
zero, territory slot zero, goal/social coordinate fields, conception/family
fields, and episodic payload values. The refreshed runtime inventory places the
first social and episodic sidecar mismatches at minute 60.

Cycle 711-740 update: the focused immune/episodic runtime inventory separates
episodic payload drift from the broader brain/social bucket. The earliest
episodic sidecar mismatch remains minute 60, with a later target row at minute
660 for replacement/payload behavior.

## Known Gaps

These cycles do not yet prove native C trace parity for long braincode runs or
deep social action drift. The harness now makes those comparisons mechanical:
replace or supplement the Rust-authored trace files with C-emitted traces, then
run `scripts/trace_diff.sh` or `scripts/run_parity_ci.sh`.
