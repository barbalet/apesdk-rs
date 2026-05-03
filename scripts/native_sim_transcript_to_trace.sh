#!/usr/bin/env bash
set -euo pipefail

INPUT="${1:?usage: native_sim_transcript_to_trace.sh transcript [label-prefix]}"
LABEL_PREFIX="${2:-native_sim}"

awk -v prefix="$LABEL_PREFIX" '
BEGIN { trace_index = 0; in_block = 0 }
{
    gsub(/\r/, "")
}
/^Map dimension:/ {
    in_block = 1
    map = $3
    next
}
in_block && /^Land seed:/ {
    seed0 = $3
    seed1 = $4
    next
}
in_block && /^Population:/ {
    population = $2
    next
}
in_block && /^Adults:/ {
    adults = $2
    juveniles = $4
    next
}
in_block && /^Tide level:/ {
    tide = $3
    next
}
in_block && /^[0-9][0-9]:[0-9][0-9] / {
    running = "unknown"
    if (index($0, "Simulation running") > 0) {
        running = "running"
    } else if (index($0, "Simulation not running") > 0) {
        running = "not-running"
    }
    printf "TRACE label=%s_%d map=%s seed=%s:%s population=%s adults=%s juveniles=%s tide=%s spacetime=%s_%s running=%s\n", prefix, trace_index, map, seed0, seed1, population, adults, juveniles, tide, $1, $2, running
    trace_index++
    in_block = 0
    next
}
' "$INPUT"
