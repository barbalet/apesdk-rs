#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/populated-raw-fixture-inventory"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

ORACLE_DIR="$OUT_DIR/oracle"
mkdir -p "$ORACLE_DIR"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/both/scripts/generate_native_raw_binary_oracle.sh" "$ORACLE_DIR" >/dev/null

current=(
    raw_empty_startup
    raw_reset_startup
    raw_after_one_cycle
    raw_social_heavy
    raw_immune_heavy
    raw_terrain_heavy
    raw_save_open_derived
)

required_future_count=0

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
    printf 'covered=expanded-direct-raw-oracle-artifact-presence\n'
    printf 'open=expanded-populated-raw-byte-exact-promotion\n'
} > "$OUT_DIR/populated_raw_fixture_inventory_manifest.txt"

echo "populated-raw-fixture-inventory=pass out=$OUT_DIR current=${#current[@]} needed=$required_future_count"
