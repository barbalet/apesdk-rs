#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/performance-smoke"}"
mkdir -p "$OUT_DIR"

cargo build --manifest-path "$ROOT/Cargo.toml" -p simape >/dev/null

measure() {
    local name="$1"
    local commands="$2"
    local output="$OUT_DIR/$name.txt"
    local start end
    start="$(date +%s)"
    "$ROOT/target/debug/simape" < "$commands" > "$output" || true
    end="$(date +%s)"
    echo "$name seconds=$((end - start)) output=$output"
}

measure help "$ROOT/golden/cli/sessions/help.commands"
measure populated_short "$ROOT/golden/cli/sessions/populated_short_matrix.commands"
measure save_open "$ROOT/golden/cli/sessions/save_open_matrix.commands"
