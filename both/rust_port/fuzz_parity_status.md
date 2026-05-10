# Fuzz Parity Status

Fuzz closure for absolute native C parity uses this promotion rule:

1. Run the same generated command/save input against native C and Rust.
2. If both fail, compare exact error behavior.
3. If either output differs, minimize the input.
4. Commit the minimized fixture before changing Rust behavior.
5. Promote the fixture into `golden/absolute/oracle_manifest.txt` only after
   exact diff passes.

No fuzz mismatch is accepted by documentation alone.
