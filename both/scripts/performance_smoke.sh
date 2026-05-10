#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/performance-smoke"}"
mkdir -p "$OUT_DIR"

cargo build --manifest-path "$ROOT/rust/Cargo.toml" -p simape >/dev/null

measure() {
    local name="$1"
    local commands="$2"
    local output="$OUT_DIR/$name.txt"
    local start end
    start="$(date +%s)"
    "$ROOT/rust/target/debug/simape" < "$commands" > "$output" || true
    end="$(date +%s)"
    echo "$name seconds=$((end - start)) output=$output"
}

measure help "$ROOT/both/golden/cli/sessions/help.commands"
measure populated_short "$ROOT/both/golden/cli/sessions/populated_short_matrix.commands"
measure save_open "$ROOT/both/golden/cli/sessions/save_open_matrix.commands"
