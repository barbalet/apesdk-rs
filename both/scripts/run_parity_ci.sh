#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/parity-ci"}"
NATIVE_DIR="$OUT_DIR/native"
TRANSCRIPT_DIR="$OUT_DIR/transcripts"

mkdir -p "$OUT_DIR"

cargo fmt --manifest-path "$ROOT/rust/Cargo.toml" --all --check
cargo test --manifest-path "$ROOT/rust/Cargo.toml"
"$ROOT/both/scripts/build_native_simape.sh" "$NATIVE_DIR"
NATIVE_SIMAPE="$NATIVE_DIR/simape" RUST_SIMAPE="$ROOT/rust/target/debug/simape" "$ROOT/both/scripts/run_cli_transcripts.sh" "$TRANSCRIPT_DIR"

echo "parity-ci=pass out=$OUT_DIR"
