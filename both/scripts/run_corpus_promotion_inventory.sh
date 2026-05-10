#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/corpus-promotion-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

PENDING_DIR="$OUT_DIR/pending"
if [ "${APESDK_RUN_CORPUS_RAW_DIFFS:-0}" = "1" ]; then
    APESDK_RUN_PENDING_CORPUS_DIFFS=1 "$ROOT/both/scripts/run_pending_corpus_inventory.sh" "$PENDING_DIR" >/dev/null
else
    "$ROOT/both/scripts/run_pending_corpus_inventory.sh" "$PENDING_DIR" >/dev/null
fi

INVENTORY="$PENDING_DIR/pending_corpus_inventory.txt"
if grep -Eq ' \| missing \| ' "$INVENTORY"; then
    echo "missing corpus command session" >&2
    cat "$INVENTORY" >&2
    exit 1
fi

total_sessions="$(awk -F' \\| ' 'NR > 1 && $1 !~ /raw-diff/ { count++ } END { print count + 0 }' "$INVENTORY")"
ready_sessions="$(awk -F' \\| ' '$3 == "ready-for-raw-diff" { count++ } END { print count + 0 }' "$INVENTORY")"
blocked_sessions="$(awk -F' \\| ' '$3 == "blocked-by-engine-values" { count++ } END { print count + 0 }' "$INVENTORY")"
raw_diff_passes="$(awk -F' \\| ' '$2 == "raw-diff-pass" { count++ } END { print count + 0 }' "$INVENTORY")"
raw_diff_blocked="$(awk -F' \\| ' '$2 == "raw-diff-blocked" { count++ } END { print count + 0 }' "$INVENTORY")"

{
    printf 'pending_inventory=%s\n' "$INVENTORY"
    printf 'total_sessions=%s\n' "$total_sessions"
    printf 'ready_sessions=%s\n' "$ready_sessions"
    printf 'blocked_sessions=%s\n' "$blocked_sessions"
    printf 'raw_diff_passes=%s\n' "$raw_diff_passes"
    printf 'raw_diff_blocked=%s\n' "$raw_diff_blocked"
    awk -F' \\| ' 'NR > 1 && $1 !~ /raw-diff/ { printf "session.%s.status=%s\n", $1, $3; printf "session.%s.blockers=%s\n", $1, $4 }' "$INVENTORY"
} > "$OUT_DIR/corpus_promotion_inventory_manifest.txt"

echo "corpus-promotion-inventory=pass out=$OUT_DIR total=$total_sessions ready=$ready_sessions blocked=$blocked_sessions raw_diff_passes=$raw_diff_passes raw_diff_blocked=$raw_diff_blocked"
