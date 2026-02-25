#!/usr/bin/env bash
# bench_ny_mir_builder.sh — Quick micro-bench for MIR(JSON) → {obj|exe}
# Usage: tools/perf/bench_ny_mir_builder.sh <mir.json> [rounds]
# Notes:
#  - Uses crate backend (ny-llvmc). Keeps defaults conservative (O0).
#  - Prints simple CSV: kind,round,ms

set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <mir.json> [rounds]" >&2
  exit 2
fi
IN="$1"; ROUNDS="${2:-3}"

BIN_BUILDER="tools/ny_mir_builder.sh"
if [[ ! -x "$BIN_BUILDER" ]]; then echo "error: $BIN_BUILDER not found/executable" >&2; exit 2; fi

measure() {
  local kind="$1"; shift
  local out_path="$PWD/target/aot_objects/__bench_${kind}_$$"
  [[ "$kind" == "exe" ]] && out_path+=".out" || out_path+=".o"
  local start end ms
  start=$(date +%s%3N)
  NYASH_LLVM_BACKEND=crate "$BIN_BUILDER" --in "$IN" --emit "$kind" -o "$out_path" --quiet || return 1
  end=$(date +%s%3N)
  ms=$((end - start))
  rm -f "$out_path" 2>/dev/null || true
  echo "$kind,$ms"
}

echo "kind,round,ms"
for k in obj exe; do
  for ((i=1; i<=ROUNDS; i++)); do
    line=$(measure "$k" || echo "$k,ERROR")
    echo "$k,$i,${line#*,}"
  done
done

