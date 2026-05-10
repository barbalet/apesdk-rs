#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/pending-corpus-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
MANIFEST="$OUT_DIR/pending_corpus_inventory.txt"

sessions=(
    long_seeded_command_corpus
    multi_day_runtime_matrix
    multi_month_runtime_matrix
    save_open_runtime_continuity
    exhaustive_command_surface
)

classify_blockers() {
    local commands="$1"
    local blockers=()
    if grep -Eq '^[[:space:]]*run[[:space:]]+[0-9]+[[:space:]]+(day|days|month|months)' "$commands"; then
        blockers+=("day-one-movement-energy-honor-runtime")
        blockers+=("brain-social-episodic-immune-runtime")
    fi
    if grep -Eq '^[[:space:]]*(save|open)([[:space:]]|$)' "$commands"; then
        blockers+=("save-open-post-load-day-continuity")
        blockers+=("save-open-raw-transcript")
    fi
    if grep -Eq '^[[:space:]]*(stats|appearance|genome|friends|pathogen|graph|braincode|speech|episodic|probes)([[:space:]]|$)' "$commands"; then
        blockers+=("selected-minute-brain-social-and-detail-values")
    fi
    if grep -Eq '^[[:space:]]*(speak|alpha)[[:space:]]+' "$commands"; then
        blockers+=("file-producing-command-order")
    fi

    if [ "${#blockers[@]}" -eq 0 ]; then
        printf 'none'
    else
        local joined="${blockers[*]}"
        printf '%s' "${joined// /,}"
    fi
}

{
    printf '# session | commands | status | blockers\n'
    for session in "${sessions[@]}"; do
        commands="$ROOT/both/golden/cli/sessions/$session.commands"
        if [ ! -f "$commands" ]; then
            printf '%s | 0 | missing | missing-file\n' "$session"
            continue
        fi
        command_count="$(grep -Evc '^[[:space:]]*($|#)' "$commands")"
        blockers="$(classify_blockers "$commands")"
        if [ "$blockers" = "none" ]; then
            status="ready-for-raw-diff"
        else
            status="blocked-by-engine-values"
        fi
        printf '%s | %s | %s | %s\n' "$session" "$command_count" "$status" "$blockers"
    done
} > "$MANIFEST"

if [ "${APESDK_RUN_PENDING_CORPUS_DIFFS:-0}" = "1" ]; then
    DIFF_DIR="$OUT_DIR/raw-diffs"
    mkdir -p "$DIFF_DIR"
    for session in "${sessions[@]}"; do
        if "$ROOT/both/scripts/run_raw_transcript_diff.sh" "$DIFF_DIR/$session" "$session" > "$DIFF_DIR/$session.log" 2>&1; then
            printf '%s | raw-diff-pass\n' "$session" >> "$MANIFEST"
        else
            printf '%s | raw-diff-blocked | %s.log\n' "$session" "$DIFF_DIR/$session" >> "$MANIFEST"
        fi
    done
fi

echo "pending-corpus-inventory=pass out=$OUT_DIR sessions=${#sessions[@]}"
