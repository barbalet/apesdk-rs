#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/absolute-parity-failure-smoke"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

cp "$ROOT/golden/traces/final_gate_summary.trace" "$OUT_DIR/trace_expected.trace"
cp "$OUT_DIR/trace_expected.trace" "$OUT_DIR/trace_mutated.trace"
printf 'TRACE_MUTATION field=changed\n' >> "$OUT_DIR/trace_mutated.trace"
if "$ROOT/scripts/trace_diff.sh" "$OUT_DIR/trace_expected.trace" "$OUT_DIR/trace_mutated.trace" >/dev/null 2>&1; then
    echo "trace failure smoke did not fail on mutation" >&2
    exit 1
fi

printf 'line=one\n' > "$OUT_DIR/transcript_expected.txt"
printf 'line=two\n' > "$OUT_DIR/transcript_mutated.txt"
if diff -u "$OUT_DIR/transcript_expected.txt" "$OUT_DIR/transcript_mutated.txt" >/dev/null 2>&1; then
    echo "transcript failure smoke did not fail on mutation" >&2
    exit 1
fi

printf 'abc' > "$OUT_DIR/save_expected.bin"
cp "$OUT_DIR/save_expected.bin" "$OUT_DIR/save_mutated.bin"
printf 'z' | dd of="$OUT_DIR/save_mutated.bin" bs=1 seek=1 conv=notrunc status=none
if cmp -s "$OUT_DIR/save_expected.bin" "$OUT_DIR/save_mutated.bin"; then
    echo "save-byte failure smoke did not fail on mutation" >&2
    exit 1
fi

{
    printf 'trace_mutation=failed_as_expected\n'
    printf 'transcript_mutation=failed_as_expected\n'
    printf 'save_byte_mutation=failed_as_expected\n'
} > "$OUT_DIR/failure_smoke_manifest.txt"

echo "absolute-parity-failure-smoke=pass out=$OUT_DIR"
