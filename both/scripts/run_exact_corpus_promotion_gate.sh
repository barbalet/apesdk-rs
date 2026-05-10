#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/exact-corpus-promotion-gate"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

if [ "${APESDK_RUN_EXACT_CORPUS_RAW_DIFFS:-0}" = "1" ]; then
    APESDK_RUN_CORPUS_RAW_DIFFS=1 "$ROOT/both/scripts/run_corpus_promotion_inventory.sh" "$OUT_DIR/corpus" >/dev/null
else
    "$ROOT/both/scripts/run_corpus_promotion_inventory.sh" "$OUT_DIR/corpus" >/dev/null
fi

MANIFEST="$OUT_DIR/corpus/corpus_promotion_inventory_manifest.txt"
total_sessions="$(awk -F= '$1 == "total_sessions" { print $2 }' "$MANIFEST")"
ready_sessions="$(awk -F= '$1 == "ready_sessions" { print $2 }' "$MANIFEST")"
blocked_sessions="$(awk -F= '$1 == "blocked_sessions" { print $2 }' "$MANIFEST")"
raw_diff_blocked="$(awk -F= '$1 == "raw_diff_blocked" { print $2 }' "$MANIFEST")"

status="exact"
if [ "$total_sessions" = "0" ] || [ "$ready_sessions" != "$total_sessions" ] || [ "$blocked_sessions" != "0" ] || [ "$raw_diff_blocked" != "0" ]; then
    status="blocked"
fi

{
    printf 'corpus_manifest=%s\n' "$MANIFEST"
    printf 'total_sessions=%s\n' "$total_sessions"
    printf 'ready_sessions=%s\n' "$ready_sessions"
    printf 'blocked_sessions=%s\n' "$blocked_sessions"
    printf 'raw_diff_blocked=%s\n' "$raw_diff_blocked"
    printf 'status=%s\n' "$status"
    printf 'required=all-promoted-corpus-sessions-ready-and-no-raw-diff-blockers\n'
} > "$OUT_DIR/exact_corpus_promotion_gate_manifest.txt"

if [ "$status" != "exact" ]; then
    echo "exact-corpus-promotion-gate=blocked out=$OUT_DIR total=$total_sessions ready=$ready_sessions blocked=$blocked_sessions raw_diff_blocked=$raw_diff_blocked" >&2
    exit 1
fi

echo "exact-corpus-promotion-gate=pass out=$OUT_DIR total=$total_sessions"
