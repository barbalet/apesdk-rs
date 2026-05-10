#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/release-debug-gate"}"
DEBUG_DIR="$OUT_DIR/debug"
RELEASE_DIR="$OUT_DIR/release"

mkdir -p "$OUT_DIR"

cargo build --manifest-path "$ROOT/rust/Cargo.toml" -p simape
cargo build --manifest-path "$ROOT/rust/Cargo.toml" --release -p simape

NATIVE_SIMAPE="$OUT_DIR/no-native-simape" \
RUST_SIMAPE="$ROOT/rust/target/debug/simape" \
"$ROOT/both/scripts/run_cli_transcripts.sh" "$DEBUG_DIR"

NATIVE_SIMAPE="$OUT_DIR/no-native-simape" \
RUST_SIMAPE="$ROOT/rust/target/release/simape" \
"$ROOT/both/scripts/run_cli_transcripts.sh" "$RELEASE_DIR"

shopt -s nullglob
for debug_file in "$DEBUG_DIR"/normalized/rust_*.txt; do
    name="$(basename "$debug_file")"
    release_file="$RELEASE_DIR/normalized/$name"
    if [ ! -f "$release_file" ]; then
        echo "missing release transcript: $name" >&2
        exit 1
    fi
    diff -u "$debug_file" "$release_file"
done

echo "release-debug=pass out=$OUT_DIR"
