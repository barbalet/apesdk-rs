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
"$ROOT/scripts/run_selected_minute_trace_inventory.sh" "$OUT_DIR/selected-minute-trace" >/dev/null
"$ROOT/scripts/run_save_open_continuity_inventory.sh" "$OUT_DIR/save-open-continuity" >/dev/null
"$ROOT/scripts/run_populated_raw_fixture_inventory.sh" "$OUT_DIR/populated-raw-fixtures" >/dev/null
"$ROOT/scripts/run_corpus_promotion_inventory.sh" "$OUT_DIR/corpus-promotion" >/dev/null

selected_manifest="$OUT_DIR/selected-being-values/selected_being_value_inventory_manifest.txt"
minute_trace_manifest="$OUT_DIR/selected-minute-trace/selected_minute_trace_inventory_manifest.txt"
save_open_manifest="$OUT_DIR/save-open-continuity/save_open_continuity_inventory_manifest.txt"
raw_fixture_manifest="$OUT_DIR/populated-raw-fixtures/populated_raw_fixture_inventory_manifest.txt"
corpus_manifest="$OUT_DIR/corpus-promotion/corpus_promotion_inventory_manifest.txt"

selected_status="$(awk -F= '$1 == "selected_status" { print $2 }' "$selected_manifest")"
minute_status="$(awk -F= '$1 == "minute_status" { print $2 }' "$minute_trace_manifest")"
raw_diff_status="$(awk -F= '$1 == "raw_diff_status" { print $2 }' "$save_open_manifest")"
needed_raw_fixtures="$(awk -F= '$1 ~ /^fixture[.].*[.]status$/ && $2 == "needed" { count++ } END { print count + 0 }' "$raw_fixture_manifest")"
blocked_corpora="$(awk -F= '$1 == "blocked_sessions" { print $2 }' "$corpus_manifest")"
ready_corpora="$(awk -F= '$1 == "ready_sessions" { print $2 }' "$corpus_manifest")"
total_corpora="$(awk -F= '$1 == "total_sessions" { print $2 }' "$corpus_manifest")"

blockers=0
[ "$selected_status" = "exact" ] || blockers=$((blockers + 1))
[ "$minute_status" = "exact" ] || blockers=$((blockers + 1))
[ "$raw_diff_status" = "exact" ] || blockers=$((blockers + 1))
[ "$needed_raw_fixtures" = "0" ] || blockers=$((blockers + 1))
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
    printf 'selected_minute_trace_status=%s\n' "$minute_status"
    printf 'selected_minute_trace_manifest=%s\n' "$minute_trace_manifest"
    printf 'save_open_raw_diff_status=%s\n' "$raw_diff_status"
    printf 'save_open_manifest=%s\n' "$save_open_manifest"
    printf 'needed_populated_raw_fixtures=%s\n' "$needed_raw_fixtures"
    printf 'populated_raw_fixture_manifest=%s\n' "$raw_fixture_manifest"
    printf 'total_corpora=%s\n' "$total_corpora"
    printf 'ready_corpora=%s\n' "$ready_corpora"
    printf 'blocked_corpora=%s\n' "$blocked_corpora"
    printf 'corpus_manifest=%s\n' "$corpus_manifest"
    printf 'open=selected-after-day-runtime selected-minute-brain-social-and-later-values save-open-continuity broader-populated-raw-fixtures exact-corpus-promotion\n'
} > "$OUT_DIR/final_signoff_readiness_manifest.txt"

if [ "${APESDK_REQUIRE_ABSOLUTE_SIGNOFF:-0}" = "1" ] && [ "$status" != "exact" ]; then
    echo "final-signoff-readiness=blocked out=$OUT_DIR blockers=$blockers" >&2
    exit 1
fi

echo "final-signoff-readiness=pass out=$OUT_DIR status=$status blockers=$blockers"
