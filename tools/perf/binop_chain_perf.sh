#!/usr/bin/env bash
# binop_chain_perf.sh — microbench: chain of i64 const/binop(add) compiled via selected backend
# Usage: tools/perf/binop_chain_perf.sh <backend:crate|native|llvmlite> <size> <repeats>
# Notes:
# - Measures build (emit exe) and run time separately; prints one [perf] line per repeat.
# - Respects NYASH_LLVM_SKIP_BUILD (default 1) to avoid rebuilding every run.

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BACKEND="${1:-native}"
SIZE="${2:-2000}"
REPEATS="${3:-3}"
BIN_BUILDER="$ROOT/tools/ny_mir_builder.sh"

if [[ "$BACKEND" == "native" ]] && ! command -v llc >/dev/null 2>&1; then
  echo "[SKIP] perf (native) llc not found" >&2; exit 0
fi

NYASH_LLVM_SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-1}

gen_json() {
  local n="$1"
  local tmp="/tmp/perf_binop_$$.json"
  {
    echo '{"schema_version":1,"functions":[{"name":"ny_main","blocks":[{"id":0,"inst":['
    # seed const 0 -> v1
    echo '{"op":"const","dst":1,"ty":"i64","value":0},'
    # chain n adds: v(i+2) = add v(i+1) + const(1)
    local i=0 id=1
    while [[ $i -lt $n ]]; do
      id=$((id+1))
      echo '{"op":"const","dst":'$id',"ty":"i64","value":1},'
      id=$((id+1))
      echo '{"op":"binop","dst":'$id',"operation":"Add","lhs":'$((id-2))',"rhs":'$((id-1))'},'
      i=$((i+1))
    done
    # ret last id
    echo '{"op":"ret","value":'$id'}] }]}] }'
  } > "$tmp"
  echo "$tmp"
}

median_ms() {
  awk '{print $1}' | sort -n | awk 'NF{a[NR]=$1} END{ if (NR%2) print a[(NR+1)/2]; else printf "%.3f\n", (a[NR/2]+a[NR/2+1])/2 }'
}

TMP_JSON="$(gen_json "$SIZE")"
APP="/tmp/perf_app_$$"

build_times=()
run_times=()

rep=1
while [[ $rep -le $REPEATS ]]; do
  # build
  local_t0=$(date +%s%3N)
  if ! NYASH_LLVM_BACKEND="$BACKEND" NYASH_LLVM_SKIP_BUILD=$NYASH_LLVM_SKIP_BUILD bash "$BIN_BUILDER" --in "$TMP_JSON" --emit exe -o "$APP" >/dev/null 2>&1; then
    echo "[FAIL] perf build failed (backend=$BACKEND)" >&2; rm -f "$TMP_JSON" "$APP"; exit 1
  fi
  local_t1=$(date +%s%3N)
  # run
  run_t0=$(date +%s%3N)
  "$APP" >/dev/null 2>&1; rc=$?
  run_t1=$(date +%s%3N)
  build_ms=$((local_t1-local_t0))
  run_ms=$((run_t1-run_t0))
  echo "[perf] backend=$BACKEND size=$SIZE build_ms=$build_ms run_ms=$run_ms rc=$rc"
  build_times+=($build_ms)
  run_times+=($run_ms)
  rep=$((rep+1))
done

rm -f "$TMP_JSON" "$APP" 2>/dev/null || true

if [[ "${NYASH_LLVM_PERF:-0}" == "1" ]]; then
  printf '%s\n' "${build_times[@]}" | median_ms | awk -v b="$BACKEND" -v s="$SIZE" '{printf("[perf/median] backend=%s size=%s build_ms=%s\n", b,s,$1)}'
  printf '%s\n' "${run_times[@]}" | median_ms | awk -v b="$BACKEND" -v s="$SIZE" '{printf("[perf/median] backend=%s size=%s run_ms=%s\n", b,s,$1)}'
fi

exit 0

