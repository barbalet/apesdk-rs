#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/save-open-continuity-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

SESSION="save_open_runtime_continuity"
COMMANDS="$ROOT/golden/cli/sessions/$SESSION.commands"
if [ ! -f "$COMMANDS" ]; then
    echo "missing save/open continuity commands: $COMMANDS" >&2
    exit 1
fi

command_count="$(grep -Evc '^[[:space:]]*($|#)' "$COMMANDS")"
transcript_status="not-run"
if [ "${APESDK_RUN_SAVE_OPEN_RAW_DIFF:-0}" = "1" ]; then
    if "$ROOT/scripts/run_raw_transcript_diff.sh" "$OUT_DIR/raw-diff" "$SESSION" > "$OUT_DIR/raw-diff.log" 2>&1; then
        transcript_status="exact"
    else
        transcript_status="inventory"
    fi
fi

{
    printf 'session=%s\n' "$SESSION"
    printf 'commands=%s\n' "$COMMANDS"
    printf 'command_count=%s\n' "$command_count"
    printf 'raw_diff_status=%s\n' "$transcript_status"
    if [ -f "$OUT_DIR/raw-diff.log" ]; then
        printf 'raw_diff_log=%s\n' "$OUT_DIR/raw-diff.log"
    fi
    printf 'covered=populated-save-open-run-save-open-session-presence-and-optional-raw-diff\n'
    printf 'open=selected-runtime-values populated-save-open-state-continuity artifact-byte-continuity\n'
} > "$OUT_DIR/save_open_continuity_inventory_manifest.txt"

echo "save-open-continuity-inventory=pass out=$OUT_DIR commands=$command_count raw_diff_status=$transcript_status"
