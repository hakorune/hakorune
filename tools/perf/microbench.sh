#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BIN="$ROOT/target/release/hakorune"

usage() { echo "Usage: $0 --case {loop|strlen|box|branch|call|stringchain|arraymap|chip8|kilo|sieve|matmul|matmul_core|linidx|maplin} [--n N] [--runs R] [--backend {llvm|vm}] [--exe] [--budget-ms B]"; }

CASE="loop"; N=5000000; RUNS=5; BACKEND="llvm"; EXE_MODE=0; BUDGET_MS=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --case) CASE="$2"; shift 2;;
    --n) N="$2"; shift 2;;
    --runs) RUNS="$2"; shift 2;;
    --backend) BACKEND="$2"; shift 2;;
    --exe) EXE_MODE=1; shift 1;;
    --budget-ms) BUDGET_MS="$2"; shift 2;;
    --help|-h) usage; exit 0;;
    *) echo "Unknown arg: $1"; usage; exit 2;;
  esac
done

if [[ ! -x "$BIN" ]]; then echo "[FAIL] hakorune not built: $BIN" >&2; exit 2; fi

# Helpers: build once, then reuse
ensure_llvmc() {
  if [[ ! -x "$ROOT/target/release/ny-llvmc" ]]; then
    (cargo build -q --release -p nyash-llvm-compiler >/dev/null 2>&1) || true
  fi
}
ensure_nyrt() {
  # Accept either .a or .rlib as presence of built runtime
  if [[ ! -f "$ROOT/target/release/libnyash_kernel.a" && ! -f "$ROOT/target/release/libnyash_kernel.rlib" ]]; then
    (cd "$ROOT/crates/nyash_kernel" && cargo build -q --release >/dev/null 2>&1) || true
  fi
}

bench_hako() {
  local file="$1"; local backend="$2"; shift 2
  local start end
  start=$(date +%s%N)
  if [[ "$backend" = "llvm" ]]; then
    # Ensure ny-llvmc exists; build if missing
    if [[ ! -x "$ROOT/target/release/ny-llvmc" ]]; then
      (cargo build -q --release -p nyash-llvm-compiler >/dev/null 2>&1) || true
    fi
    # Boundary is the default; explicit =1 keeps the frozen llvmlite oracle.
    PYTHONPATH="${PYTHONPATH:-$ROOT}" \
    NYASH_AOT_COLLECTIONS_HOT=1 NYASH_LLVM_FAST=1 NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_MAP_KEY_MODE=auto \
    NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 HAKO_USING_RESOLVER_FIRST=1 \
    NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
    NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
    NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-0}" "$BIN" --backend llvm "$file" >/dev/null 2>&1
  else
    "$BIN" --backend vm "$file" >/dev/null 2>&1
  fi
  end=$(date +%s%N)
  echo $(( (end - start)/1000000 ))
}

bench_c() {
  local csrc="$1"; local exe="$2"
  cc -O3 -march=native -o "$exe" "$csrc"
  local start end
  start=$(date +%s%N)
  "$exe" >/dev/null 2>&1
  end=$(date +%s%N)
  echo $(( (end - start)/1000000 ))
}

# Build once and time executable runs (ms)
time_exe_run() {
  local exe="$1"
  local start end
  start=$(date +%s%N)
  "$exe" >/dev/null 2>&1
  end=$(date +%s%N)
  echo $(( (end - start)/1000000 ))
}

mktemp_hako() { mktemp --suffix .hako; }
mktemp_c() { mktemp --suffix .c; }

# Fallback diagnostics for EXE flow: check MIR JSON for externcall/boxcall/jsonfrag
diag_mir_json() {
  local json="$1"
  local rewrites; rewrites=$(rg -c '"op":"externcall"' "$json" 2>/dev/null || echo 0)
  local arrays; arrays=$(rg -c 'nyash\.array\.' "$json" 2>/dev/null || echo 0)
  local maps;   maps=$(rg -c 'nyash\.map\.'   "$json" 2>/dev/null || echo 0)
  local boxcalls; boxcalls=$(rg -c '"op":"boxcall"' "$json" 2>/dev/null || echo 0)
  local jsonfrag; jsonfrag=$(rg -c '\[emit/jsonfrag\]' "$json" 2>/dev/null || echo 0)
  echo "[diag] externcall=${rewrites} (array=${arrays}, map=${maps}), boxcall_left=${boxcalls}, jsonfrag=${jsonfrag}" >&2
}

# Case emitters that are large enough to hide the benchmark harness live under
# microbench_cases. They are sourced functions and intentionally keep using the
# existing globals (N/BACKEND/EXE_MODE/HAKO_FILE/C_FILE).
. "$SCRIPT_DIR/microbench_cases/kilo.sh"

. "$SCRIPT_DIR/microbench_cases/default.sh"
emit_case "$CASE"

echo "[perf] case=$CASE n=$N runs=$RUNS backend=$BACKEND" >&2
sum_c=0; sum_h=0

if [[ "$EXE_MODE" = "1" ]]; then
  # Build C exe once
  C_EXE=$(mktemp --suffix .out)
  cc -O3 -march=native -o "$C_EXE" "$C_FILE"
  # Build Nyash exe once through the Boundary crate backend.
  if [[ "$BACKEND" != "llvm" ]]; then
    echo "[FAIL] --exe requires --backend llvm" >&2; exit 2
  fi
  ensure_llvmc
  ensure_nyrt
  HAKO_EXE=$(mktemp --suffix .out)
  TMP_JSON=$(mktemp --suffix .json)
  # Default: use provider-first with AotPrep for maximum optimization
  # DEBUG: Show file paths
  echo "[matmul/debug] HAKO_FILE=$HAKO_FILE TMP_JSON=$TMP_JSON" >&2
  if ! \
       HAKO_SELFHOST_TRACE=1 \
       HAKO_SELFHOST_BUILDER_FIRST=0 HAKO_SELFHOST_NO_DELEGATE=0 \
       HAKO_APPLY_AOT_PREP=1 \
       NYASH_AOT_COLLECTIONS_HOT=1 NYASH_LLVM_FAST=1 NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_MAP_KEY_MODE=auto \
       HAKO_MIR_BUILDER_LOOP_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_JSONFRAG:-$([[ "${PERF_USE_JSONFRAG:-0}" = 1 ]] && echo 1 || echo 0)}" \
       HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG:-$([[ "${PERF_USE_JSONFRAG:-0}" = 1 ]] && echo 1 || echo 0)}" \
       HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE="${HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE:-1}" \
       HAKO_MIR_BUILDER_JSONFRAG_PURIFY="${HAKO_MIR_BUILDER_JSONFRAG_PURIFY:-1}" \
       NYASH_AOT_NUMERIC_CORE="${NYASH_AOT_NUMERIC_CORE:-0}" \
       NYASH_AOT_NUMERIC_CORE_TRACE="${NYASH_AOT_NUMERIC_CORE_TRACE:-0}" \
       NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
       NYASH_JSON_ONLY=1 bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$HAKO_FILE" 2>&1 | tee /tmp/matmul_emit_log.txt >/dev/null; then
    echo "[FAIL] emit MIR JSON failed (hint: set PERF_USE_PROVIDER=1 or HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1)" >&2; exit 3
  fi
  grep -E "\[prep:|provider/emit\]" /tmp/matmul_emit_log.txt >&2 || true

  # Quick diagnostics: ensure AotPrep rewrites are present and jsonfrag fallback is not used
  # DEBUG: Copy TMP_JSON for inspection
  cp "$TMP_JSON" /tmp/matmul_from_perf.json 2>/dev/null || true
  echo "[matmul/debug] TMP_JSON copied to /tmp/matmul_from_perf.json" >&2
  echo "[matmul/debug] Direct externcall count: $(grep -o '"op":"externcall"' "$TMP_JSON" 2>/dev/null | wc -l)" >&2
  diag_mir_json "$TMP_JSON"

  # AotPrep is now applied in hako-helper route via HAKO_APPLY_AOT_PREP=1
  # Build EXE via helper (selects crate backend ny-llvmc under the hood)
  if ! NYASH_LLVM_BACKEND=crate NYASH_LLVM_SKIP_BUILD=1 \
       NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
       NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
       NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 NYASH_LLVM_FAST=1 \
       bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$HAKO_EXE" --quiet >/dev/null 2>&1; then
    echo "[FAIL] build Nyash EXE failed (crate backend). Ensure ny-llvmc exists or try NYASH_LLVM_BACKEND=crate." >&2; exit 3
  fi

  # Execute runs. If BUDGET_MS>0, keep running until budget is exhausted.
  if [[ "$BUDGET_MS" != "0" ]]; then
    i=0; used=0
    while true; do
      i=$((i+1))
      t_c=$(time_exe_run "$C_EXE"); t_h=$(time_exe_run "$HAKO_EXE")
      sum_c=$((sum_c + t_c)); sum_h=$((sum_h + t_h)); used=$((used + t_h))
      if command -v python3 >/dev/null 2>&1; then ratio=$(python3 -c "print(round(${t_h}/max(${t_c},1)*100,2))" 2>/dev/null || echo NA); else ratio=NA; fi
      echo "run#$i c=${t_c}ms hak=${t_h}ms ratio=${ratio}% (budget used=${used}/${BUDGET_MS}ms)" >&2
      if [[ $used -ge $BUDGET_MS ]]; then RUNS=$i; break; fi
      # Safety valve to avoid infinite loop if t_h is 0ms
      if [[ $i -ge 999 ]]; then RUNS=$i; break; fi
    done
  else
    for i in $(seq 1 "$RUNS"); do
      t_c=$(time_exe_run "$C_EXE")
      t_h=$(time_exe_run "$HAKO_EXE")
      sum_c=$((sum_c + t_c)); sum_h=$((sum_h + t_h))
      if command -v python3 >/dev/null 2>&1; then
        ratio=$(python3 -c "print(round(${t_h}/max(${t_c},1)*100,2))" 2>/dev/null || echo NA)
      else
        ratio=NA
      fi
      echo "run#$i c=${t_c}ms hak=${t_h}ms ratio=${ratio}%" >&2
    done
  fi
  avg_c=$((sum_c / RUNS)); avg_h=$((sum_h / RUNS))
  echo "avg c=${avg_c}ms hak=${avg_h}ms" >&2
  if [ "$avg_c" -lt 5 ]; then
    echo "[WARN] C runtime is very small (${avg_c}ms). Increase --n to reduce timer granularity noise." >&2
  fi
  if command -v python3 >/dev/null 2>&1; then
    python3 - <<PY
c=$avg_c; h=$avg_h
ratio = (h/max(c,1))*100.0
print(f"ratio={ratio:.2f}%")
PY
  fi
  rm -f "$C_EXE" "$HAKO_EXE" "$TMP_JSON" 2>/dev/null || true
else
  for i in $(seq 1 "$RUNS"); do
    t_c=$(bench_c "$C_FILE" "${C_FILE%.c}")
    t_h=$(bench_hako "$HAKO_FILE" "$BACKEND")
    sum_c=$((sum_c + t_c)); sum_h=$((sum_h + t_h))
    if command -v python3 >/dev/null 2>&1; then
      ratio=$(python3 -c "print(round(${t_h}/max(${t_c},1)*100,2))" 2>/dev/null || echo NA)
    else
      ratio=NA
    fi
    echo "run#$i c=${t_c}ms hak=${t_h}ms ratio=${ratio}%" >&2
  done
  avg_c=$((sum_c / RUNS)); avg_h=$((sum_h / RUNS))
  echo "avg c=${avg_c}ms hak=${avg_h}ms" >&2
  if [ "$avg_c" -lt 5 ]; then
    echo "[WARN] C runtime is very small (${avg_c}ms). Increase --n to reduce timer granularity noise." >&2
  fi
  if command -v python3 >/dev/null 2>&1; then
    python3 - <<PY
c=$avg_c; h=$avg_h
ratio = (h/max(c,1))*100.0
print(f"ratio={ratio:.2f}%")
PY
  fi
fi

rm -f "$HAKO_FILE" "$C_FILE" "${C_FILE%.c}" 2>/dev/null || true
