#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/native-raw-binary-value-gate"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

ORACLE_DIR="$OUT_DIR/oracle"
RUST_DIR="$OUT_DIR/rust"
ROUNDTRIP_DIR="$OUT_DIR/roundtrip"
mkdir -p "$ORACLE_DIR" "$RUST_DIR" "$ROUNDTRIP_DIR"

APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/generate_native_raw_binary_oracle.sh" "$ORACLE_DIR" >/dev/null
APESDK_FULL_DATE="$FULL_DATE" cargo build --manifest-path "$ROOT/Cargo.toml" -p simape >/dev/null

NATIVE_VALUES="$ORACLE_DIR/artifacts/native_raw_binary_values.trace"
SCENARIOS=(
    raw_empty_startup
    raw_reset_startup
    raw_after_one_cycle
    raw_social_heavy
    raw_immune_heavy
    raw_terrain_heavy
    raw_save_open_derived
)

for scenario in "${SCENARIOS[@]}"; do
    artifact="$ORACLE_DIR/artifacts/$scenario.native"
    native_line="$RUST_DIR/$scenario.native.trace"
    rust_line="$RUST_DIR/$scenario.rust.trace"
    grep "^RAW scenario=$scenario " "$NATIVE_VALUES" > "$native_line"
    "$ROOT/target/debug/simape" --native-raw-summary "$scenario" "$artifact" > "$rust_line"
    diff -u "$native_line" "$rust_line"
done

EXACT_BYTE_SCENARIOS=(
    raw_empty_startup
    raw_reset_startup
)

for scenario in "${EXACT_BYTE_SCENARIOS[@]}"; do
    artifact="$ORACLE_DIR/artifacts/$scenario.native"
    roundtrip="$ROUNDTRIP_DIR/$scenario.native"
    "$ROOT/target/debug/simape" --native-raw-roundtrip "$artifact" "$roundtrip"
    diff -u "$artifact" "$roundtrip"
done

POPULATED_SCENARIOS=(
    raw_after_one_cycle
    raw_social_heavy
    raw_immune_heavy
    raw_terrain_heavy
    raw_save_open_derived
)

populated_byte_modes=()
for scenario in "${POPULATED_SCENARIOS[@]}"; do
    artifact="$ORACLE_DIR/artifacts/$scenario.native"
    roundtrip="$ROUNDTRIP_DIR/$scenario.native"
    "$ROOT/target/debug/simape" --native-raw-roundtrip "$artifact" "$roundtrip"
    "$ROOT/target/debug/simape" --native-raw-summary "$scenario" "$roundtrip" > "$RUST_DIR/$scenario.roundtrip.trace"
    diff -u "$RUST_DIR/$scenario.native.trace" "$RUST_DIR/$scenario.roundtrip.trace"
    if cmp -s "$artifact" "$roundtrip"; then
        populated_byte_modes+=("$scenario:exact")
    else
        populated_byte_modes+=("$scenario:value-exact-byte-pending")
    fi
done

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'oracle=%s\n' "$ORACLE_DIR"
    printf 'rust=%s\n' "$RUST_DIR"
    printf 'roundtrip=%s\n' "$ROUNDTRIP_DIR"
    printf 'value_scenarios=%s\n' "${SCENARIOS[*]}"
    printf 'exact_byte_scenarios=%s\n' "${EXACT_BYTE_SCENARIOS[*]}"
    printf 'populated_scenarios=%s\n' "${POPULATED_SCENARIOS[*]}"
    printf 'populated_byte_modes=%s\n' "${populated_byte_modes[*]}"
} > "$OUT_DIR/native_raw_binary_value_gate_manifest.txt"

echo "native-raw-binary-value-gate=pass out=$OUT_DIR full_date=$FULL_DATE exact_bytes=${EXACT_BYTE_SCENARIOS[*]} populated=${populated_byte_modes[*]}"
