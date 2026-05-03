#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/populated-raw-fixture-inventory"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

ORACLE_DIR="$OUT_DIR/oracle"
mkdir -p "$ORACLE_DIR"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/generate_native_raw_binary_oracle.sh" "$ORACLE_DIR" >/dev/null

current=(
    raw_empty_startup
    raw_reset_startup
    raw_after_one_cycle
)

required_future=(
    raw_social_heavy
    raw_immune_heavy
    raw_terrain_heavy
    raw_save_open_derived
)

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'oracle=%s\n' "$ORACLE_DIR"
    for scenario in "${current[@]}"; do
        artifact="$ORACLE_DIR/artifacts/$scenario.native"
        if [ ! -f "$artifact" ]; then
            echo "missing current raw artifact: $artifact" >&2
            exit 1
        fi
        printf 'fixture.%s.status=present\n' "$scenario"
        printf 'fixture.%s.bytes=%s\n' "$scenario" "$(wc -c < "$artifact" | tr -d ' ')"
    done
    for scenario in "${required_future[@]}"; do
        printf 'fixture.%s.status=needed\n' "$scenario"
    done
    printf 'covered=current-direct-raw-oracle-artifact-presence\n'
    printf 'open=social-heavy immune-heavy terrain-heavy save-open-derived populated raw byte fixtures\n'
} > "$OUT_DIR/populated_raw_fixture_inventory_manifest.txt"

echo "populated-raw-fixture-inventory=pass out=$OUT_DIR current=${#current[@]} needed=${#required_future[@]}"
