# Parity Completion Report

## Cycle 161-205 Gate Status

The Rust command-line path now has a broader native-state surface for terrain,
food, awake/body/movement, immune cycling, social/episodic behavior, braincode
hooks, territory, preferences, pregnancy, family relationships, JSON transfer,
and native transfer text round trips.

The approved local validation gates currently pass with:

```sh
cargo test -p apesdk-sim --lib
cargo test -p simape --lib
```

The existing golden CLI sessions remain in `golden/cli`, and the Rust tests
continue to exercise help, error, runtime, open/save, populated social,
episodic, brain, probe, and transcript smoke behavior.

## Not Yet Proven

The final byte-for-byte native C parity gate is still blocked by missing raw
binary transfer compatibility and by the absence of generated C trace fixtures
for the new braincode, lifecycle, long-run state, and byte-save matrices. Rust
still rejects raw binary native saves by design; it reads and writes the native
transfer text sections used in the current port.

## Zero-Untriaged Drift List

Known observable drift categories that still need C fixture closure:

```text
raw binary native save/load byte layout
braincode attention-similarity trace details
exact child-genetics crossover and post-birth care timing
full social action/body inventory side effects
long seeded C-vs-Rust state and transcript matrices
debug-vs-release and cross-platform transcript proof
```

No additional untriaged drift categories were found during this cycle batch;
the listed items are the remaining explicit gates.

