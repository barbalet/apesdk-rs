# Absolute Native C Oracle Corpus

This directory tracks the stricter no-caveat native C parity gates introduced
for cycles 266-300.

`oracle_manifest.txt` lists the currently promoted exact-diff corpus. The
transcript gate allows only transport cleanup, specifically CRLF-to-LF
conversion, before diffing C and Rust output.

Generate local C oracle artifacts with:

```sh
scripts/generate_c_oracle_artifacts.sh /private/tmp/apesdk_c_oracles
```

Run the strict smoke gate with:

```sh
scripts/run_absolute_parity_ci.sh /private/tmp/apesdk_absolute_parity
```
