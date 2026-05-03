#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/command-fuzz-gate"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
SEED="${APESDK_FUZZ_SEED:-351360}"
COUNT="${APESDK_COMMAND_FUZZ_COUNT:-24}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

NATIVE_DIR="$OUT_DIR/native-build"
COMMAND_DIR="$OUT_DIR/commands"
RAW_DIR="$OUT_DIR/raw"
TRANSPORT_DIR="$OUT_DIR/transport"

mkdir -p "$COMMAND_DIR" "$RAW_DIR/native" "$RAW_DIR/rust" "$TRANSPORT_DIR/native" "$TRANSPORT_DIR/rust"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/build_native_simape.sh" "$NATIVE_DIR" >/dev/null

if ! command -v expect >/dev/null 2>&1; then
    echo "command fuzz gate requires expect for PTY command draining" >&2
    exit 69
fi

generate_commands() {
    local seed="$1"
    local output="$2"
    local state="$seed"
    local commands=(
        "help"
        "help run"
        "help nope"
        "unknown"
        "file xxxxx"
        "file landd"
        "interval"
        "interval 2 hours"
        "logging off"
        "logging"
        "event social"
        "event off"
        "watch"
        "monitor off"
        "friends"
        "social"
        "socialgraph Ada"
        "graph"
        "pathogen Ada"
        "braincode"
        "speech"
        "episodic"
        "stats"
        "appearance"
        "genome"
        "next"
        "previous"
        "run"
        "stop"
        "   help run   "
        "   unknown   "
    )

    : > "$output"
    for (( index = 0; index < COUNT; index++ )); do
        state=$(( (state * 1103515245 + 12345) & 0x7fffffff ))
        printf '%s\n' "${commands[$(( state % ${#commands[@]} ))]}" >> "$output"
    done
    printf 'quit\n' >> "$output"
}

run_expect() {
    local binary="$1"
    local commands="$2"
    local output="$3"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir"
    ( cd "$run_dir" && "$ROOT/golden/cli/run_cli_session.expect" "$binary" "$commands" "$output" )
}

generate_commands "$SEED" "$COMMAND_DIR/command_fuzz_a.commands"
generate_commands "$SEED" "$COMMAND_DIR/command_fuzz_b.commands"
cmp "$COMMAND_DIR/command_fuzz_a.commands" "$COMMAND_DIR/command_fuzz_b.commands"

run_expect "$NATIVE_DIR/simape" "$COMMAND_DIR/command_fuzz_a.commands" "$RAW_DIR/native/command_fuzz.txt"
run_expect "$ROOT/target/debug/simape" "$COMMAND_DIR/command_fuzz_a.commands" "$RAW_DIR/rust/command_fuzz.txt"
"$ROOT/scripts/transport_normalize_transcript.sh" "$RAW_DIR/native/command_fuzz.txt" > "$TRANSPORT_DIR/native/command_fuzz.txt"
"$ROOT/scripts/transport_normalize_transcript.sh" "$RAW_DIR/rust/command_fuzz.txt" > "$TRANSPORT_DIR/rust/command_fuzz.txt"
diff -u "$TRANSPORT_DIR/native/command_fuzz.txt" "$TRANSPORT_DIR/rust/command_fuzz.txt"

{
    printf 'seed=%s\n' "$SEED"
    printf 'count=%s\n' "$COUNT"
    printf 'sha256='
    shasum -a 256 "$COMMAND_DIR/command_fuzz_a.commands" | awk '{print $1}'
    printf 'native=%s\n' "$NATIVE_DIR/simape"
    printf 'rust=%s\n' "$ROOT/target/debug/simape"
} > "$OUT_DIR/command_fuzz_manifest.txt"

echo "command-fuzz-gate=pass out=$OUT_DIR seed=$SEED count=$COUNT full_date=$FULL_DATE"
