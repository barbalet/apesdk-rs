# Release/Debug Determinism

Cycle 258 adds a local gate comparing normalized CLI transcripts from debug and
release Rust `simape` builds.

Run:

```sh
scripts/run_release_debug_gate.sh /private/tmp/apesdk_release_debug_gate
```

The gate builds both profiles, runs every session under `golden/cli/sessions`,
normalizes the resulting Rust transcripts, and fails on any diff.
