# No-Normalization Policy

Absolute native C parity does not allow behavior-hiding transcript
normalization.

Allowed transport cleanup:

- remove carriage returns from PTY captures

Disallowed cleanup:

- dates
- source paths
- file sizes
- string lengths
- names
- random seeds
- state values
- command ordering
- error text or file-line locations

The strict gate is `scripts/run_raw_transcript_diff.sh`. The older
`scripts/normalize_transcript.sh` remains only for historical approved-corpus
review and is excluded from absolute parity signoff.
