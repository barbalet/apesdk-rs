#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/seed-population-fuzz"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

COMMAND_DIR="$OUT_DIR/commands"
RAW_DIR="$OUT_DIR/raw"
TRANSPORT_DIR="$OUT_DIR/transport"

mkdir -p "$COMMAND_DIR" "$RAW_DIR/first" "$RAW_DIR/second" "$TRANSPORT_DIR/first" "$TRANSPORT_DIR/second"
APESDK_FULL_DATE="$FULL_DATE" cargo build --manifest-path "$ROOT/rust/Cargo.toml" -p simape >/dev/null

cat > "$COMMAND_DIR/reset_one_hour.commands" <<'COMMANDS'
reset
run 1 hour
sim
top
quit
COMMANDS

cat > "$COMMAND_DIR/reset_three_hours.commands" <<'COMMANDS'
reset
run 3 hours
sim
ape
social
friends
quit
COMMANDS

cat > "$COMMAND_DIR/interval_run.commands" <<'COMMANDS'
reset
interval 2 hours
run
sim
stop
quit
COMMANDS

run_pipe() {
    local commands="$1"
    local output="$2"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir"
    ( cd "$run_dir" && "$ROOT/rust/target/debug/simape" < "$commands" > "$output" ) || true
}

for commands in "$COMMAND_DIR"/*.commands; do
    name="$(basename "$commands" .commands)"
    run_pipe "$commands" "$RAW_DIR/first/$name.txt"
    run_pipe "$commands" "$RAW_DIR/second/$name.txt"
    "$ROOT/both/scripts/transport_normalize_transcript.sh" "$RAW_DIR/first/$name.txt" > "$TRANSPORT_DIR/first/$name.txt"
    "$ROOT/both/scripts/transport_normalize_transcript.sh" "$RAW_DIR/second/$name.txt" > "$TRANSPORT_DIR/second/$name.txt"
    diff -u "$TRANSPORT_DIR/first/$name.txt" "$TRANSPORT_DIR/second/$name.txt"
done

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'cases=reset_one_hour reset_three_hours interval_run\n'
    printf 'mode=rust-repeatability\n'
    printf 'note=C/Rust seeded population value diff remains pending direct engine trace value closure.\n'
} > "$OUT_DIR/seed_population_fuzz_manifest.txt"

echo "seed-population-fuzz=pass out=$OUT_DIR cases=3"
