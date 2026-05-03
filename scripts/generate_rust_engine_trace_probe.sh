#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/rust-engine-trace-probe"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
APESDK_FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}" cargo run --manifest-path "$ROOT/Cargo.toml" -p simape -- --engine-trace > "$OUT_DIR/rust_engine_trace.trace"

{
    printf 'trace=%s\n' "$OUT_DIR/rust_engine_trace.trace"
    rustc --version
    cargo --version
    shasum -a 256 "$OUT_DIR/rust_engine_trace.trace"
} > "$OUT_DIR/rust_engine_trace_manifest.txt"

echo "rust-engine-trace-probe=$OUT_DIR"
