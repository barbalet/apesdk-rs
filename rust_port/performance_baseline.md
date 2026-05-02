# Performance Baseline

Cycle 260 records a lightweight command-line smoke baseline rather than a
microbenchmark suite. The goal is to catch severe regressions while preserving
trace output.

Run from the repository root:

```sh
scripts/performance_smoke.sh /private/tmp/apesdk_performance_smoke
```

The script builds debug Rust `simape` and measures wall-clock seconds for:

- `help.commands`
- `populated_short_matrix.commands`
- `save_open_matrix.commands`

Optimization work should only land with unchanged transcript and trace gates.
