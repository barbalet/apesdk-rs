#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/malformed-save-fuzz"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

NATIVE_DIR="$OUT_DIR/native-build"
FIXTURE_DIR="$OUT_DIR/fixtures"
COMMAND_DIR="$OUT_DIR/commands"
RAW_DIR="$OUT_DIR/raw"
TRANSPORT_DIR="$OUT_DIR/transport"
INVENTORY_DIR="$OUT_DIR/inventory"

mkdir -p "$FIXTURE_DIR" "$COMMAND_DIR" "$RAW_DIR/native" "$RAW_DIR/rust" "$TRANSPORT_DIR/native" "$TRANSPORT_DIR/rust" "$INVENTORY_DIR"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/build_native_simape.sh" "$NATIVE_DIR" >/dev/null

MISSING_FILE="$FIXTURE_DIR/missing.native"
EMPTY_FILE="$FIXTURE_DIR/empty.native"
BAD_JSON="$FIXTURE_DIR/bad_json.native"
TRUNCATED_NATIVE="$FIXTURE_DIR/truncated.native"
RANDOM_TEXT="$FIXTURE_DIR/random_text.native"

: > "$EMPTY_FILE"
rm -f "$MISSING_FILE"
printf '{"information":true}\n' > "$BAD_JSON"
printf 'simul{signa=20033;verio=708;};landd{dated=0;' > "$TRUNCATED_NATIVE"
printf 'not a native transfer file\nwith extra lines\n' > "$RANDOM_TEXT"

run_expect() {
    local binary="$1"
    local commands="$2"
    local output="$3"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir"
    if command -v expect >/dev/null 2>&1; then
        ( cd "$run_dir" && "$ROOT/golden/cli/run_cli_session.expect" "$binary" "$commands" "$output" )
    else
        ( cd "$run_dir" && "$binary" < "$commands" > "$output" ) || true
    fi
}

exact_cases=(
    "missing:$MISSING_FILE"
    "empty:$EMPTY_FILE"
    "bad_json:$BAD_JSON"
    "truncated_native:$TRUNCATED_NATIVE"
    "random_text:$RANDOM_TEXT"
)

for exact_case in "${exact_cases[@]}"; do
    name="${exact_case%%:*}"
    fixture="${exact_case#*:}"
    commands="$COMMAND_DIR/malformed_exact_$name.commands"
    {
        printf 'open %s\n' "$fixture"
        printf 'quit\n'
    } > "$commands"
    run_expect "$NATIVE_DIR/simape" "$commands" "$RAW_DIR/native/$name.txt"
    run_expect "$ROOT/target/debug/simape" "$commands" "$RAW_DIR/rust/$name.txt"
    "$ROOT/scripts/canonicalize_malformed_loader_transcript.sh" "$RAW_DIR/native/$name.txt" > "$TRANSPORT_DIR/native/$name.txt"
    "$ROOT/scripts/canonicalize_malformed_loader_transcript.sh" "$RAW_DIR/rust/$name.txt" > "$TRANSPORT_DIR/rust/$name.txt"
    diff -u "$TRANSPORT_DIR/native/$name.txt" "$TRANSPORT_DIR/rust/$name.txt"
done

APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_malformed_loader_oracle.sh" "$INVENTORY_DIR/oracle" >/dev/null

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'exact=missing empty bad_json truncated_native random_text\n'
    printf 'inventory=%s\n' "$INVENTORY_DIR/oracle/malformed_loader_oracle_manifest.txt"
    printf 'note=unpromoted malformed cases are inventoried with split stdout/stderr artifacts for later exact gate promotion.\n'
    printf 'native=%s\n' "$NATIVE_DIR/simape"
    printf 'rust=%s\n' "$ROOT/target/debug/simape"
} > "$OUT_DIR/malformed_save_fuzz_manifest.txt"

echo "malformed-save-fuzz=pass out=$OUT_DIR exact=missing,empty,bad_json,truncated_native,random_text inventory=$INVENTORY_DIR/oracle"
