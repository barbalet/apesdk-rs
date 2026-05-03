#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/absolute-parity-ci"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

mkdir -p "$OUT_DIR"

cargo fmt --manifest-path "$ROOT/Cargo.toml" --all --check
cargo test --manifest-path "$ROOT/Cargo.toml"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_raw_transcript_diff.sh" "$OUT_DIR/raw-transcripts" help help_errors command_edges run_forever_empty runtime_edges_empty
"$ROOT/scripts/run_interactive_timing_gate.sh" "$OUT_DIR/interactive-timing"
"$ROOT/scripts/run_timing_regression_lock.sh" "$OUT_DIR/timing-regression-lock"
"$ROOT/scripts/run_fuzz_parity_ci.sh" "$OUT_DIR/fuzz"
"$ROOT/scripts/run_absolute_parity_failure_smoke.sh" "$OUT_DIR/failure-smoke"
"$ROOT/scripts/run_native_raw_binary_oracle_gate.sh" "$OUT_DIR/native-raw-binary"
"$ROOT/scripts/run_native_raw_binary_value_gate.sh" "$OUT_DIR/native-raw-binary-values"
"$ROOT/scripts/run_populated_raw_fixture_inventory.sh" "$OUT_DIR/populated-raw-fixtures"
"$ROOT/scripts/trace_diff.sh" "$ROOT/golden/traces/final_gate_summary.trace" "$ROOT/golden/traces/final_gate_summary.trace"
"$ROOT/scripts/run_engine_trace_schema_gate.sh" "$OUT_DIR/engine-trace-schema"
"$ROOT/scripts/run_engine_trace_value_gate.sh" "$OUT_DIR/engine-trace-values"
"$ROOT/scripts/run_terrain_food_first_cycle_gate.sh" "$OUT_DIR/terrain-food-first-cycle"
"$ROOT/scripts/run_terrain_food_value_inventory.sh" "$OUT_DIR/terrain-food-values"
"$ROOT/scripts/run_selected_being_value_inventory.sh" "$OUT_DIR/selected-being-values"
"$ROOT/scripts/run_selected_minute_value_gate.sh" "$OUT_DIR/selected-minute-values"
"$ROOT/scripts/run_selected_minute_trace_inventory.sh" "$OUT_DIR/selected-minute-trace"
"$ROOT/scripts/run_save_open_continuity_inventory.sh" "$OUT_DIR/save-open-continuity"
"$ROOT/scripts/run_corpus_promotion_inventory.sh" "$OUT_DIR/corpus-promotion"

echo "absolute-parity-ci=pass out=$OUT_DIR full_date=$FULL_DATE"
