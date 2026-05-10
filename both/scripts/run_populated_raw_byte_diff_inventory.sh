#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/populated-raw-byte-diff-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

RAW_VALUE_GATE_DIR="${APESDK_RAW_VALUE_GATE_DIR:-}"
if [ -z "$RAW_VALUE_GATE_DIR" ]; then
    RAW_VALUE_GATE_DIR="$OUT_DIR/native-raw-binary-values"
    "$ROOT/both/scripts/run_native_raw_binary_value_gate.sh" "$RAW_VALUE_GATE_DIR" >/dev/null
fi

case "$RAW_VALUE_GATE_DIR" in
    /*) ;;
    *) RAW_VALUE_GATE_DIR="$ROOT/$RAW_VALUE_GATE_DIR" ;;
esac

RAW_VALUE_MANIFEST="$RAW_VALUE_GATE_DIR/native_raw_binary_value_gate_manifest.txt"
if [ ! -f "$RAW_VALUE_MANIFEST" ]; then
    echo "missing native raw value gate manifest: $RAW_VALUE_MANIFEST" >&2
    exit 1
fi

ORACLE_DIR="$(awk -F= '$1 == "oracle" { print $2 }' "$RAW_VALUE_MANIFEST")"
ROUNDTRIP_DIR="$(awk -F= '$1 == "roundtrip" { print $2 }' "$RAW_VALUE_MANIFEST")"

POPULATED_SCENARIOS=(
    raw_after_one_cycle
    raw_social_heavy
    raw_immune_heavy
    raw_terrain_heavy
    raw_save_open_derived
)

section_for_offset() {
    local artifact="$1"
    local offset="$2"
    if [ -z "$offset" ] || [ "$offset" = "0" ]; then
        printf 'none'
        return
    fi
    local zero_based=$((offset - 1))
    local section
    section="$(LC_ALL=C grep -aboE 'simul[{]|landd[{]|topog[{]|weath[{]|being[{]|sosim[{]|episod[{]|terit[{]' "$artifact" \
        | awk -F: -v off="$zero_based" '$1 <= off { found=$1 ":" $2 } END { if (found != "") print found }')"
    if [ -z "$section" ]; then
        printf 'before-first-known-section'
    else
        printf '%s' "$section"
    fi
}

token_for_offset() {
    local artifact="$1"
    local offset="$2"
    if [ -z "$offset" ] || [ "$offset" = "0" ]; then
        printf 'none'
        return
    fi
    local zero_based=$((offset - 1))
    local token
    token="$(LC_ALL=C grep -aboE 'simul[{]|landd[{]|dated=|timed=|topog[{]|topby=|weath[{]|atmby=|litby=|being[{]|locat=|facin=|speed=|energ=|datob=|rando=|state=|brast=|heigt=|masss=|overr=|shout=|crowd=|postu=|inven=|paras=|honor=|conce=|atten=|genet=|fetag=|fathn=|sosim[{]|drive=|goals=|prefe=|genex=|genen=|chigx=|chign=|awako=|bname=|terit=|immun=|brreg=|brpro=|episod[{]|episo=' "$artifact" \
        | awk -F: -v off="$zero_based" '$1 <= off { found_offset=$1; found_token=$2 } END { if (found_token != "") printf "%s@%s+%s", found_token, found_offset, off - found_offset }')"
    if [ -z "$token" ]; then
        printf 'before-first-known-token'
    else
        printf '%s' "$token"
    fi
}

mkdir -p "$OUT_DIR/mutation"
mutation_source="$ORACLE_DIR/artifacts/raw_empty_startup.native"
mutation_target="$OUT_DIR/mutation/raw_empty_startup.mutated"
cp "$mutation_source" "$mutation_target"
printf 'X' | dd of="$mutation_target" bs=1 seek=0 count=1 conv=notrunc 2>/dev/null
if cmp -s "$mutation_source" "$mutation_target"; then
    echo "raw byte mutation was not detected" >&2
    exit 1
fi

pending_count=0
exact_count=0
missing=()
byte_modes=()

{
    printf 'raw_value_manifest=%s\n' "$RAW_VALUE_MANIFEST"
    printf 'oracle=%s\n' "$ORACLE_DIR"
    printf 'roundtrip=%s\n' "$ROUNDTRIP_DIR"
    printf 'mutation_check=pass\n'
    for scenario in "${POPULATED_SCENARIOS[@]}"; do
        artifact="$ORACLE_DIR/artifacts/$scenario.native"
        roundtrip="$ROUNDTRIP_DIR/$scenario.native"
        if [ ! -f "$artifact" ] || [ ! -f "$roundtrip" ]; then
            printf 'scenario.%s.byte_status=missing\n' "$scenario"
            missing+=("$scenario")
            pending_count=$((pending_count + 1))
            byte_modes+=("$scenario:missing")
            continue
        fi

        artifact_bytes="$(wc -c < "$artifact" | tr -d ' ')"
        roundtrip_bytes="$(wc -c < "$roundtrip" | tr -d ' ')"
        if cmp -s "$artifact" "$roundtrip"; then
            printf 'scenario.%s.byte_status=exact\n' "$scenario"
            printf 'scenario.%s.artifact_bytes=%s\n' "$scenario" "$artifact_bytes"
            printf 'scenario.%s.roundtrip_bytes=%s\n' "$scenario" "$roundtrip_bytes"
            exact_count=$((exact_count + 1))
            byte_modes+=("$scenario:exact")
            continue
        fi

        summary="$(
            (cmp -l "$artifact" "$roundtrip" 2>/dev/null || true) | awk '
                NR == 1 {
                    first_offset = $1
                    native_value = $2
                    rust_value = $3
                }
                { count++ }
                END {
                    printf "diff_count=%d first_offset=%s native_octal=%s rust_octal=%s",
                        count + 0, first_offset, native_value, rust_value
                }
            '
        )"
        diff_count="$(printf '%s\n' "$summary" | awk '{ for (idx = 1; idx <= NF; idx++) { split($idx, kv, "="); if (kv[1] == "diff_count") print kv[2] } }')"
        first_offset="$(printf '%s\n' "$summary" | awk '{ for (idx = 1; idx <= NF; idx++) { split($idx, kv, "="); if (kv[1] == "first_offset") print kv[2] } }')"
        native_octal="$(printf '%s\n' "$summary" | awk '{ for (idx = 1; idx <= NF; idx++) { split($idx, kv, "="); if (kv[1] == "native_octal") print kv[2] } }')"
        rust_octal="$(printf '%s\n' "$summary" | awk '{ for (idx = 1; idx <= NF; idx++) { split($idx, kv, "="); if (kv[1] == "rust_octal") print kv[2] } }')"
        section="$(section_for_offset "$artifact" "$first_offset")"
        token="$(token_for_offset "$artifact" "$first_offset")"

        printf 'scenario.%s.byte_status=value-exact-byte-pending\n' "$scenario"
        printf 'scenario.%s.artifact_bytes=%s\n' "$scenario" "$artifact_bytes"
        printf 'scenario.%s.roundtrip_bytes=%s\n' "$scenario" "$roundtrip_bytes"
        printf 'scenario.%s.diff_count=%s\n' "$scenario" "$diff_count"
        printf 'scenario.%s.first_diff_offset=%s\n' "$scenario" "$first_offset"
        printf 'scenario.%s.first_diff_native_octal=%s\n' "$scenario" "$native_octal"
        printf 'scenario.%s.first_diff_rust_octal=%s\n' "$scenario" "$rust_octal"
        printf 'scenario.%s.first_diff_section=%s\n' "$scenario" "$section"
        printf 'scenario.%s.first_diff_token=%s\n' "$scenario" "$token"
        pending_count=$((pending_count + 1))
        byte_modes+=("$scenario:value-exact-byte-pending")
    done
    printf 'populated_byte_modes=%s\n' "${byte_modes[*]}"
    printf 'exact_populated_raw_byte_scenarios=%s\n' "$exact_count"
    printf 'pending_populated_raw_byte_scenarios=%s\n' "$pending_count"
    printf 'missing_populated_scenarios=%s\n' "${missing[*]-}"
    if [ "$pending_count" = "0" ]; then
        printf 'populated_raw_byte_status=exact\n'
    else
        printf 'populated_raw_byte_status=inventory\n'
    fi
    printf 'covered=populated-native-raw-byte-first-diff-section-token-classification-and-mutation-check\n'
    printf 'open=close-populated-native-raw-byte-diffs-for-all-populated-scenarios\n'
} > "$OUT_DIR/populated_raw_byte_diff_inventory_manifest.txt"

status="$(awk -F= '$1 == "populated_raw_byte_status" { print $2 }' "$OUT_DIR/populated_raw_byte_diff_inventory_manifest.txt")"
echo "populated-raw-byte-diff-inventory=pass out=$OUT_DIR status=$status exact=$exact_count pending=$pending_count"
