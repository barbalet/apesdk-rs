#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/malformed-loader-oracle"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

NATIVE_DIR="$OUT_DIR/native-build"
FIXTURE_DIR="$OUT_DIR/fixtures"
COMMAND_DIR="$OUT_DIR/commands"
SPLIT_DIR="$OUT_DIR/split"
CANONICAL_DIR="$OUT_DIR/canonical"

mkdir -p "$FIXTURE_DIR" "$COMMAND_DIR" "$SPLIT_DIR/native" "$SPLIT_DIR/rust" "$CANONICAL_DIR/native" "$CANONICAL_DIR/rust"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/build_native_simape.sh" "$NATIVE_DIR" >/dev/null

create_fixtures() {
    : > "$FIXTURE_DIR/empty.native"
    rm -f "$FIXTURE_DIR/missing.native"
    printf '{"information":true}\n' > "$FIXTURE_DIR/bad_json.native"
    printf 'not a native transfer file\nwith extra lines\n' > "$FIXTURE_DIR/random_text.native"
    printf 'simul{signa=20033;verio=708;};landd{dated=0;' > "$FIXTURE_DIR/truncated_native.native"
    printf 'NAB1\000\000\002\304\377\377' > "$FIXTURE_DIR/truncated_binary.bin"
    printf 'simul{signa=1;verio=708;};landd{dated=0;timed=0;landg=1,2;};' > "$FIXTURE_DIR/bad_signature.native"
    printf 'simul{signa=20033;verio=9999;};landd{dated=0;timed=0;landg=1,2;};' > "$FIXTURE_DIR/wrong_version.native"
    printf 'landd{dated=0;timed=0;landg=1,2;};simul{signa=20033;verio=708;};' > "$FIXTURE_DIR/wrong_section_order.native"
    printf 'simul{signa=20033;verio=708;};landd{dated=0;timed=0;landg=1,two;};' > "$FIXTURE_DIR/invalid_counts.native"
    printf 'garbagesimul{signa=20033;verio=708;};landd{dated=0;timed=0;landg=1,2;};' > "$FIXTURE_DIR/garbage_prefix.native"
    printf 'simul{signa=20033;verio=708;};landd{dated=0;timed=0;landg=1,2;};garbage' > "$FIXTURE_DIR/garbage_suffix.native"
}

run_split() {
    local profile="$1"
    local binary="$2"
    local case_name="$3"
    local fixture="$4"
    local commands="$COMMAND_DIR/$case_name.commands"
    local out_dir="$SPLIT_DIR/$profile/$case_name"
    mkdir -p "$out_dir"
    {
        printf 'open %s\n' "$fixture"
        printf 'sim\n'
        printf 'quit\n'
    } > "$commands"
    ( cd "$out_dir" && "$binary" < "$commands" > stdout.txt 2> stderr.txt ) || true
    cat "$out_dir/stdout.txt" "$out_dir/stderr.txt" > "$out_dir/combined.txt"
    "$ROOT/scripts/canonicalize_malformed_loader_transcript.sh" "$out_dir/combined.txt" > "$CANONICAL_DIR/$profile/$case_name.txt"
}

classify_case() {
    local canonical="$1"
    if grep -q 'Simulation file .* open' "$canonical"; then
        printf 'opens'
    elif grep -q 'File data could not be allocated' "$canonical"; then
        printf 'allocation-error'
    elif grep -q 'Signature not first in file' "$canonical"; then
        printf 'signature-not-first'
    elif grep -q 'Failed to read in file' "$canonical"; then
        printf 'read-error'
    else
        printf 'other'
    fi
}

create_fixtures

cases=(
    missing.native
    empty.native
    bad_json.native
    random_text.native
    truncated_native.native
    truncated_binary.bin
    bad_signature.native
    wrong_version.native
    wrong_section_order.native
    invalid_counts.native
    garbage_prefix.native
    garbage_suffix.native
)

manifest="$OUT_DIR/malformed_loader_oracle_manifest.txt"
{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'native=%s\n' "$NATIVE_DIR/simape"
    printf 'rust=%s\n' "$ROOT/target/debug/simape"
    printf 'fixtures=%s\n' "${cases[*]}"
} > "$manifest"

for case_file in "${cases[@]}"; do
    case_name="${case_file%.*}"
    fixture="$FIXTURE_DIR/$case_file"
    run_split native "$NATIVE_DIR/simape" "$case_name" "$fixture"
    run_split rust "$ROOT/target/debug/simape" "$case_name" "$fixture"
    printf '%s | native=%s | rust=%s\n' \
        "$case_name" \
        "$(classify_case "$CANONICAL_DIR/native/$case_name.txt")" \
        "$(classify_case "$CANONICAL_DIR/rust/$case_name.txt")" >> "$manifest"
done

echo "malformed-loader-oracle=pass out=$OUT_DIR full_date=$FULL_DATE cases=${#cases[@]}"
