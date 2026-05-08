#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/final-signoff-readiness"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

"$ROOT/scripts/run_selected_minute_value_gate.sh" "$OUT_DIR/selected-minute-values" >/dev/null
"$ROOT/scripts/run_selected_being_value_inventory.sh" "$OUT_DIR/selected-being-values" >/dev/null
"$ROOT/scripts/run_after_day_drift_inventory.sh" "$OUT_DIR/after-day-drift" >/dev/null
"$ROOT/scripts/run_after_day_slice_inventory.sh" "$OUT_DIR/after-day-slices" >/dev/null
"$ROOT/scripts/run_brain_social_runtime_inventory.sh" "$OUT_DIR/brain-social-runtime" >/dev/null
"$ROOT/scripts/run_selected_minute_trace_inventory.sh" "$OUT_DIR/selected-minute-trace" >/dev/null
"$ROOT/scripts/run_save_open_continuity_inventory.sh" "$OUT_DIR/save-open-continuity" >/dev/null
"$ROOT/scripts/run_populated_raw_fixture_inventory.sh" "$OUT_DIR/populated-raw-fixtures" >/dev/null
"$ROOT/scripts/run_native_raw_binary_value_gate.sh" "$OUT_DIR/native-raw-binary-values" >/dev/null
"$ROOT/scripts/run_corpus_promotion_inventory.sh" "$OUT_DIR/corpus-promotion" >/dev/null

selected_manifest="$OUT_DIR/selected-being-values/selected_being_value_inventory_manifest.txt"
after_day_manifest="$OUT_DIR/after-day-drift/after_day_drift_inventory_manifest.txt"
after_day_slice_manifest="$OUT_DIR/after-day-slices/after_day_slice_inventory_manifest.txt"
brain_social_manifest="$OUT_DIR/brain-social-runtime/brain_social_runtime_inventory_manifest.txt"
minute_trace_manifest="$OUT_DIR/selected-minute-trace/selected_minute_trace_inventory_manifest.txt"
save_open_manifest="$OUT_DIR/save-open-continuity/save_open_continuity_inventory_manifest.txt"
raw_fixture_manifest="$OUT_DIR/populated-raw-fixtures/populated_raw_fixture_inventory_manifest.txt"
raw_value_manifest="$OUT_DIR/native-raw-binary-values/native_raw_binary_value_gate_manifest.txt"
corpus_manifest="$OUT_DIR/corpus-promotion/corpus_promotion_inventory_manifest.txt"

selected_status="$(awk -F= '$1 == "selected_status" { print $2 }' "$selected_manifest")"
after_day_status="$(awk -F= '$1 == "after_day_status" { print $2 }' "$after_day_manifest")"
after_day_open_buckets="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$after_day_manifest")"
after_day_slice_status="$(awk -F= '$1 == "after_day_slice_status" { print $2 }' "$after_day_slice_manifest")"
after_day_slice_open_buckets="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$after_day_slice_manifest")"
brain_social_runtime_status="$(awk -F= '$1 == "brain_social_runtime_status" { print $2 }' "$brain_social_manifest")"
brain_social_open_buckets="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$brain_social_manifest")"
minute_status="$(awk -F= '$1 == "minute_status" { print $2 }' "$minute_trace_manifest")"
raw_diff_status="$(awk -F= '$1 == "raw_diff_status" { print $2 }' "$save_open_manifest")"
post_load_minute_status="$(awk -F= '$1 == "post_load_minute_status" { print $2 }' "$save_open_manifest")"
post_load_day_status="$(awk -F= '$1 == "post_load_day_status" { print $2 }' "$save_open_manifest")"
needed_raw_fixtures="$(awk -F= '$1 ~ /^fixture[.].*[.]status$/ && $2 == "needed" { count++ } END { print count + 0 }' "$raw_fixture_manifest")"
pending_raw_byte_scenarios="$(awk -F= '$1 == "populated_byte_modes" { print gsub(/value-exact-byte-pending/, "&") }' "$raw_value_manifest")"
blocked_corpora="$(awk -F= '$1 == "blocked_sessions" { print $2 }' "$corpus_manifest")"
ready_corpora="$(awk -F= '$1 == "ready_sessions" { print $2 }' "$corpus_manifest")"
total_corpora="$(awk -F= '$1 == "total_sessions" { print $2 }' "$corpus_manifest")"

blockers=0
[ "$selected_status" = "exact" ] && [ "$after_day_status" = "exact" ] && [ "$after_day_slice_status" = "exact" ] || blockers=$((blockers + 1))
[ "$minute_status" = "exact" ] && [ "$brain_social_runtime_status" = "exact" ] || blockers=$((blockers + 1))
[ "$raw_diff_status" = "exact" ] && [ "$post_load_minute_status" = "exact" ] && [ "$post_load_day_status" = "exact" ] || blockers=$((blockers + 1))
[ "$needed_raw_fixtures" = "0" ] && [ "$pending_raw_byte_scenarios" = "0" ] || blockers=$((blockers + 1))
[ "$blocked_corpora" = "0" ] && [ "$ready_corpora" = "$total_corpora" ] || blockers=$((blockers + 1))

status="exact"
if [ "$blockers" != "0" ]; then
    status="blocked"
fi

{
    printf 'status=%s\n' "$status"
    printf 'blocker_categories=%s\n' "$blockers"
    printf 'selected_status=%s\n' "$selected_status"
    printf 'selected_manifest=%s\n' "$selected_manifest"
    printf 'after_day_status=%s\n' "$after_day_status"
    printf 'after_day_open_buckets=%s\n' "$after_day_open_buckets"
    printf 'after_day_manifest=%s\n' "$after_day_manifest"
    printf 'after_day_slice_status=%s\n' "$after_day_slice_status"
    printf 'after_day_slice_open_buckets=%s\n' "$after_day_slice_open_buckets"
    printf 'after_day_slice_manifest=%s\n' "$after_day_slice_manifest"
    printf 'brain_social_runtime_status=%s\n' "$brain_social_runtime_status"
    printf 'brain_social_open_buckets=%s\n' "$brain_social_open_buckets"
    printf 'brain_social_manifest=%s\n' "$brain_social_manifest"
    printf 'selected_minute_trace_status=%s\n' "$minute_status"
    printf 'selected_minute_trace_manifest=%s\n' "$minute_trace_manifest"
    printf 'save_open_raw_diff_status=%s\n' "$raw_diff_status"
    printf 'save_open_post_load_minute_status=%s\n' "$post_load_minute_status"
    printf 'save_open_post_load_day_status=%s\n' "$post_load_day_status"
    printf 'save_open_manifest=%s\n' "$save_open_manifest"
    printf 'needed_populated_raw_fixtures=%s\n' "$needed_raw_fixtures"
    printf 'populated_raw_fixture_manifest=%s\n' "$raw_fixture_manifest"
    printf 'pending_populated_raw_byte_scenarios=%s\n' "$pending_raw_byte_scenarios"
    printf 'native_raw_value_manifest=%s\n' "$raw_value_manifest"
    printf 'total_corpora=%s\n' "$total_corpora"
    printf 'ready_corpora=%s\n' "$ready_corpora"
    printf 'blocked_corpora=%s\n' "$blocked_corpora"
    printf 'corpus_manifest=%s\n' "$corpus_manifest"
    printf 'open=selected-after-day-runtime day-one-movement-energy-honor-runtime brain-social-episodic-immune-runtime save-open-continuity broader-populated-raw-byte-promotion exact-corpus-promotion\n'
} > "$OUT_DIR/final_signoff_readiness_manifest.txt"

if [ "${APESDK_REQUIRE_ABSOLUTE_SIGNOFF:-0}" = "1" ] && [ "$status" != "exact" ]; then
    echo "final-signoff-readiness=blocked out=$OUT_DIR blockers=$blockers" >&2
    exit 1
fi

echo "final-signoff-readiness=pass out=$OUT_DIR status=$status blockers=$blockers"
