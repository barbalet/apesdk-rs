#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/terrain-food-first-cycle-gate"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
"$ROOT/scripts/generate_c_engine_trace_probe.sh" "$OUT_DIR/c" >/dev/null
"$ROOT/scripts/generate_rust_engine_trace_probe.sh" "$OUT_DIR/rust" >/dev/null

extract_promoted_rows() {
    awk '
    /^TERRAIN / && ($2 == "snapshot=startup" || $2 == "snapshot=after_cycle_1") {
        print
    }
    ' "$1"
}

extract_promoted_rows "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.promoted-terrain"
extract_promoted_rows "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.promoted-terrain"
diff -u "$OUT_DIR/c.promoted-terrain" "$OUT_DIR/rust.promoted-terrain"

awk '
NR == 9 {
    sub(/height=/, "height=999:")
}
{ print }
' "$OUT_DIR/c.promoted-terrain" > "$OUT_DIR/c.promoted-terrain.mutated"

if diff -u "$OUT_DIR/c.promoted-terrain.mutated" "$OUT_DIR/rust.promoted-terrain" > "$OUT_DIR/mutated.diff"; then
    echo "terrain food first-cycle mutation unexpectedly passed" >&2
    exit 1
fi

{
    printf 'c_trace=%s\n' "$OUT_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$OUT_DIR/rust/rust_engine_trace.trace"
    printf 'covered=startup-and-first-cycle-multi-location-terrain-weather-food\n'
    printf 'samples=16\n'
    printf 'negative_check=one-field-mutation-fails\n'
    printf 'open=multi-day-weather-evolution\n'
} > "$OUT_DIR/terrain_food_first_cycle_gate_manifest.txt"

echo "terrain-food-first-cycle-gate=pass out=$OUT_DIR samples=16"
