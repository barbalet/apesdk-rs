#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/raw-binary-mutation-fuzz"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
SEED="${APESDK_BINARY_FUZZ_SEED:-357360}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

FIXTURE_DIR="$OUT_DIR/fixtures"
COMMAND_DIR="$OUT_DIR/commands"
RAW_DIR="$OUT_DIR/raw"

mkdir -p "$FIXTURE_DIR" "$COMMAND_DIR" "$RAW_DIR/rust"
APESDK_FULL_DATE="$FULL_DATE" cargo build --manifest-path "$ROOT/rust/Cargo.toml" -p simape >/dev/null

printf 'NAB1' > "$FIXTURE_DIR/nab1_short.bin"
printf 'NAB1\000\000\002\304\377\377' > "$FIXTURE_DIR/nab1_mutated_header.bin"

RANDOM_FILE="$FIXTURE_DIR/random64.bin"
: > "$RANDOM_FILE"
state="$SEED"
for (( index = 0; index < 64; index++ )); do
    state=$(( (state * 1664525 + 1013904223) & 0xffffffff ))
    byte=$(( (state >> 16) & 0xff ))
    printf -v octal '%03o' "$byte"
    printf "\\$octal" >> "$RANDOM_FILE"
done

for fixture in "$FIXTURE_DIR"/*.bin; do
    name="$(basename "$fixture" .bin)"
    commands="$COMMAND_DIR/$name.commands"
    output="$RAW_DIR/rust/$name.txt"
    {
        printf 'open %s\n' "$fixture"
        printf 'quit\n'
    } > "$commands"
    "$ROOT/rust/target/debug/simape" < "$commands" > "$output" || true
    grep -q 'ERROR: Failed to read in file @ ./universe/command.c 2394' "$output"
done

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'seed=%s\n' "$SEED"
    printf 'fixtures=nab1_short nab1_mutated_header random64\n'
    printf 'mode=rust-loader-rejects\n'
    printf 'note=native raw binary value diff remains pending until a command-line C byte oracle is promoted.\n'
} > "$OUT_DIR/raw_binary_mutation_fuzz_manifest.txt"

echo "raw-binary-mutation-fuzz=triaged out=$OUT_DIR seed=$SEED fixtures=3"
