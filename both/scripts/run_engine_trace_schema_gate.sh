#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/engine-trace-schema-gate"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
"$ROOT/both/scripts/generate_c_engine_trace_probe.sh" "$OUT_DIR/c" >/dev/null
"$ROOT/both/scripts/generate_rust_engine_trace_probe.sh" "$OUT_DIR/rust" >/dev/null

extract_schema() {
    awk '
    /^TRACE / {
        output = "TRACE"
        for (field = 2; field <= NF; field++) {
            split($field, parts, "=")
            output = output " " parts[1] "="
        }
        print output
    }
    ' "$1"
}

extract_schema "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.schema"
extract_schema "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.schema"
diff -u "$OUT_DIR/c.schema" "$OUT_DIR/rust.schema"

echo "engine-trace-schema-gate=pass out=$OUT_DIR"
