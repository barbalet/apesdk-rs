#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/absolute-parity-ci"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

mkdir -p "$OUT_DIR"

cargo fmt --manifest-path "$ROOT/Cargo.toml" --all --check
cargo test --manifest-path "$ROOT/Cargo.toml"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_raw_transcript_diff.sh" "$OUT_DIR/raw-transcripts" help help_errors command_edges
"$ROOT/scripts/trace_diff.sh" "$ROOT/golden/traces/final_gate_summary.trace" "$ROOT/golden/traces/final_gate_summary.trace"

echo "absolute-parity-ci=pass out=$OUT_DIR full_date=$FULL_DATE"
