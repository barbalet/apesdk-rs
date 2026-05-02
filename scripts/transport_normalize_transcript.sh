#!/usr/bin/env bash
set -euo pipefail

input="${1:-/dev/stdin}"
tr -d '\r' < "$input"
