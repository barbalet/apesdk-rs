#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/selected-minute-trace-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
"$ROOT/both/scripts/generate_c_engine_trace_probe.sh" "$OUT_DIR/c" >/dev/null
"$ROOT/both/scripts/generate_rust_engine_trace_probe.sh" "$OUT_DIR/rust" >/dev/null

extract_schema() {
    awk '
    /^SELECTED-MINUTE / {
        output = "SELECTED-MINUTE"
        for (field = 2; field <= NF; field++) {
            split($field, parts, "=")
            output = output " " parts[1] "="
        }
        print output
    }
    ' "$1"
}

grep '^SELECTED-MINUTE ' "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.selected-minute"
grep '^SELECTED-MINUTE ' "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.selected-minute"
extract_schema "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.selected-minute.schema"
extract_schema "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.selected-minute.schema"
diff -u "$OUT_DIR/c.selected-minute.schema" "$OUT_DIR/rust.selected-minute.schema"

c_count="$(wc -l < "$OUT_DIR/c.selected-minute" | tr -d ' ')"
rust_count="$(wc -l < "$OUT_DIR/rust.selected-minute" | tr -d ' ')"
if [ "$c_count" != "$rust_count" ]; then
    echo "selected minute trace count mismatch: c=$c_count rust=$rust_count" >&2
    exit 1
fi

minute_status=exact
if ! diff -u "$OUT_DIR/c.selected-minute" "$OUT_DIR/rust.selected-minute" > "$OUT_DIR/selected-minute.diff"; then
    minute_status=inventory
fi

minute_mismatches="$(awk '/^[+-]SELECTED-MINUTE / { count++ } END { print count + 0 }' "$OUT_DIR/selected-minute.diff")"
first_mismatch="$(awk '/^[+-]SELECTED-MINUTE / { print; exit }' "$OUT_DIR/selected-minute.diff")"

{
    printf 'c_trace=%s\n' "$OUT_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$OUT_DIR/rust/rust_engine_trace.trace"
    printf 'minute_status=%s\n' "$minute_status"
    printf 'minute_samples=%s\n' "$c_count"
    printf 'minute_mismatches=%s\n' "$minute_mismatches"
    printf 'first_mismatch=%s\n' "$first_mismatch"
    printf 'covered=selected-being-hourly-and-final-sixteen-minute-after-day-trace-schema-and-row-count\n'
    printf 'open=after-day-selected-being-drive-brain-social-movement-energy-honor-episodic-immune-values\n'
} > "$OUT_DIR/selected_minute_trace_inventory_manifest.txt"

echo "selected-minute-trace-inventory=pass out=$OUT_DIR minute_status=$minute_status samples=$c_count mismatches=$minute_mismatches"
