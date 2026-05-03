#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/terrain-food-value-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
"$ROOT/scripts/generate_c_engine_trace_probe.sh" "$OUT_DIR/c" >/dev/null
"$ROOT/scripts/generate_rust_engine_trace_probe.sh" "$OUT_DIR/rust" >/dev/null

extract_trace_terrain_values() {
    awk '
    function reset_fields(   key) {
        for (key in fields) {
            delete fields[key]
        }
    }

    function read_fields(   field, parts) {
        reset_fields()
        for (field = 2; field <= NF; field++) {
            split($field, parts, "=")
            fields[parts[1]] = parts[2]
        }
    }

    /^TRACE / {
        read_fields()
        printf "TRACE-TERRAIN snapshot=%s genetics=%s tide=%s topo=%s high=%s hightide=%s weather=%s pressure=%s food=%s\n",
            fields["snapshot"],
            fields["genetics"],
            fields["tide"],
            fields["topo"],
            fields["high"],
            fields["hightide"],
            fields["weather"],
            fields["pressure"],
            fields["food"]
    }
    ' "$1"
}

awk '/^TERRAIN / { print }' "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.terrain"
awk '/^TERRAIN / { print }' "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.terrain"
extract_trace_terrain_values "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.trace-terrain"
extract_trace_terrain_values "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.trace-terrain"

terrain_status=exact
if ! diff -u "$OUT_DIR/c.terrain" "$OUT_DIR/rust.terrain" > "$OUT_DIR/terrain.diff"; then
    terrain_status=inventory
fi

trace_status=exact
if ! diff -u "$OUT_DIR/c.trace-terrain" "$OUT_DIR/rust.trace-terrain" > "$OUT_DIR/trace-terrain.diff"; then
    trace_status=inventory
fi

terrain_mismatches="$(awk '/^[+-]TERRAIN / { count++ } END { print count + 0 }' "$OUT_DIR/terrain.diff")"
trace_mismatches="$(awk '/^[+-]TRACE-TERRAIN / { count++ } END { print count + 0 }' "$OUT_DIR/trace-terrain.diff")"
sample_count="$(wc -l < "$OUT_DIR/c.terrain" | tr -d ' ')"

{
    printf 'c_trace=%s\n' "$OUT_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$OUT_DIR/rust/rust_engine_trace.trace"
    printf 'terrain_status=%s\n' "$terrain_status"
    printf 'trace_status=%s\n' "$trace_status"
    printf 'terrain_samples=%s\n' "$sample_count"
    printf 'terrain_mismatches=%s\n' "$terrain_mismatches"
    printf 'trace_mismatches=%s\n' "$trace_mismatches"
    printf 'covered=multi-location-height-topography-highdef-hightide-weather-pressure-food-operators-food-classification\n'
    printf 'open=multi-day-weather-evolution-selected-being-honor\n'
} > "$OUT_DIR/terrain_food_value_inventory_manifest.txt"

echo "terrain-food-value-inventory=pass out=$OUT_DIR terrain_status=$terrain_status trace_status=$trace_status samples=$sample_count mismatches=$terrain_mismatches/$trace_mismatches"
