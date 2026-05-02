#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/transcripts"}"
NATIVE_BIN="${NATIVE_SIMAPE:-"$ROOT/target/native/simape"}"
RUST_BIN="${RUST_SIMAPE:-"$ROOT/target/debug/simape"}"

mkdir -p "$OUT_DIR/native" "$OUT_DIR/rust" "$OUT_DIR/normalized"

if [ ! -x "$RUST_BIN" ]; then
    cargo build --manifest-path "$ROOT/Cargo.toml" -p simape
fi

run_one() {
    local binary="$1"
    local commands="$2"
    local output="$3"
    if command -v expect >/dev/null 2>&1; then
        "$ROOT/golden/cli/run_cli_session.expect" "$binary" "$commands" "$output"
    else
        "$binary" < "$commands" > "$output" || true
    fi
}

for commands in "$ROOT"/golden/cli/sessions/*.commands; do
    name="$(basename "$commands" .commands)"
    if [ -x "$NATIVE_BIN" ]; then
        if grep -Eiq '^[[:space:]]*run[[:space:]]+forever([[:space:]]|$)' "$commands"; then
            echo "skip native unsafe session: $name" >&2
        else
            run_one "$NATIVE_BIN" "$commands" "$OUT_DIR/native/$name.txt"
            "$ROOT/scripts/normalize_transcript.sh" "$OUT_DIR/native/$name.txt" > "$OUT_DIR/normalized/native_$name.txt"
        fi
    fi
    run_one "$RUST_BIN" "$commands" "$OUT_DIR/rust/$name.txt"
    "$ROOT/scripts/normalize_transcript.sh" "$OUT_DIR/rust/$name.txt" > "$OUT_DIR/normalized/rust_$name.txt"
done

echo "transcripts=$OUT_DIR"
