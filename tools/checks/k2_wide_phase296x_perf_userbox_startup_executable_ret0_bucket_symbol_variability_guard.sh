#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
FLOOR_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_loader_libc_floor_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_symbol_variability.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_RUNS=50
TRIALS=3

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-symbol-variability] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-symbol-variability] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

FLOOR_REPORT="$TMP_DIR/loader_floor.out"
"$FLOOR_GUARD" >"$FLOOR_REPORT"
require_line "$FLOOR_REPORT" "output_contract=perf-userbox-loader-libc-floor-probe-v0"
require_line "$FLOOR_REPORT" "loader_floor_owner=link_mode/loader/libc"
require_line "$FLOOR_REPORT" "dynamic_needed_libgcc_s=0"
require_line "$FLOOR_REPORT" "dynamic_needed_libm=0"
require_line "$FLOOR_REPORT" "dynamic_needed_libc=1"
require_line "$FLOOR_REPORT" "dynamic_needed_ld_linux=1"
require_line "$FLOOR_REPORT" "summary=ok"

trial_results="$TMP_DIR/trials.txt"
: >"$trial_results"
ret0_count_min=""
ret0_count_max=""
startup_loader_dynamic_loader_pct=""
startup_loader_libc_process_pct=""

for trial in $(seq 1 "$TRIALS"); do
  trial_out="$TMP_DIR/trial_${trial}.out"
  "$TOOL" --out "$trial_out" --startup-runs "$STARTUP_RUNS" --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null
  require_line "$trial_out" "output_contract=perf-userbox-startup-loader-owner-split-v0"
  require_line "$trial_out" "summary=ok"
  require_positive_key "$trial_out" "startup_loader_dynamic_loader_pct"
  require_positive_key "$trial_out" "startup_loader_libc_process_pct"
  if [ -z "$startup_loader_dynamic_loader_pct" ]; then
    startup_loader_dynamic_loader_pct="$(awk -F= '$1=="startup_loader_dynamic_loader_pct" { print $2; exit }' "$trial_out")"
  fi
  if [ -z "$startup_loader_libc_process_pct" ]; then
    startup_loader_libc_process_pct="$(awk -F= '$1=="startup_loader_libc_process_pct" { print $2; exit }' "$trial_out")"
  fi

  primary_bucket="$(awk -F= '$1=="startup_loader_ret0_exe_primary_bucket" { print $2; exit }' "$trial_out")"
  ret0_top_count="$(awk -F= '$1=="startup_loader_ret0_exe_top_count" { print $2; exit }' "$trial_out")"
  case "$primary_bucket" in
    env|once|path|minimal_main|nyash_kernel_runtime|kernel|other|missing|alloc|ffi|string)
      :
      ;;
    *)
      echo "[perf-userbox-startup-executable-ret0-bucket-symbol-variability] unexpected primary bucket '$primary_bucket' in ${trial_out#$ROOT_DIR/}" >&2
      exit 1
      ;;
  esac

  env_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_env_first_symbol" { print $2; exit }' "$trial_out")"
  once_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_once_first_symbol" { print $2; exit }' "$trial_out")"
  path_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_path_first_symbol" { print $2; exit }' "$trial_out")"
  minimal_main_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_minimal_main_first_symbol" { print $2; exit }' "$trial_out")"
  nyash_kernel_runtime_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_nyash_kernel_runtime_first_symbol" { print $2; exit }' "$trial_out")"

  printf '%s %s %s %s %s %s %s\n' \
    "$primary_bucket" \
    "$ret0_top_count" \
    "$env_symbol" \
    "$once_symbol" \
    "$path_symbol" \
    "$minimal_main_symbol" \
    "$nyash_kernel_runtime_symbol" >>"$trial_results"

  if [ -z "$ret0_count_min" ] || [ "$ret0_top_count" -lt "$ret0_count_min" ]; then
    ret0_count_min="$ret0_top_count"
  fi
  if [ -z "$ret0_count_max" ] || [ "$ret0_top_count" -gt "$ret0_count_max" ]; then
    ret0_count_max="$ret0_top_count"
  fi
done

total_trials="$TRIALS"

read -r primary_bucket_env_count primary_bucket_once_count primary_bucket_path_count primary_bucket_minimal_main_count primary_bucket_nyash_kernel_runtime_count primary_bucket_kernel_count primary_bucket_other_count primary_bucket_missing_count primary_bucket_alloc_count primary_bucket_ffi_count primary_bucket_string_count ret0_count_min ret0_count_max trial_summaries bucket_env_present_count bucket_once_present_count bucket_path_present_count bucket_minimal_main_present_count bucket_nyash_kernel_runtime_present_count bucket_env_symbol_mode bucket_env_symbol_mode_count bucket_once_symbol_mode bucket_once_symbol_mode_count bucket_path_symbol_mode bucket_path_symbol_mode_count bucket_minimal_main_symbol_mode bucket_minimal_main_symbol_mode_count bucket_nyash_kernel_runtime_symbol_mode bucket_nyash_kernel_runtime_symbol_mode_count < <(
  python3 - "$trial_results" <<'PY'
from __future__ import annotations
import sys
from collections import Counter
from pathlib import Path

path = Path(sys.argv[1])
primary_counts = {
    "env": 0,
    "once": 0,
    "path": 0,
    "minimal_main": 0,
    "nyash_kernel_runtime": 0,
    "kernel": 0,
    "other": 0,
    "missing": 0,
    "alloc": 0,
    "ffi": 0,
    "string": 0,
}
bucket_symbols = {
    "env": Counter(),
    "once": Counter(),
    "path": Counter(),
    "minimal_main": Counter(),
    "nyash_kernel_runtime": Counter(),
}
bucket_present_counts = {
    "env": 0,
    "once": 0,
    "path": 0,
    "minimal_main": 0,
    "nyash_kernel_runtime": 0,
}
ret0_min = None
ret0_max = None
summaries = []

def mode(counter: Counter[str]) -> tuple[str, int]:
    if not counter:
        return ("missing", 0)
    symbol, count = max(counter.items(), key=lambda item: (item[1], item[0]))
    return symbol, count

for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if not line.strip():
        continue
    parts = line.split()
    if len(parts) != 7:
        continue
    primary, top_count_s, env_symbol, once_symbol, path_symbol, minimal_main_symbol, nyash_kernel_runtime_symbol = parts
    if primary not in primary_counts:
        primary = "missing"
    primary_counts[primary] += 1
    try:
        top_count = int(top_count_s)
    except ValueError:
        top_count = 0
    ret0_min = top_count if ret0_min is None or top_count < ret0_min else ret0_min
    ret0_max = top_count if ret0_max is None or top_count > ret0_max else ret0_max
    if env_symbol != "missing":
        bucket_present_counts["env"] += 1
        bucket_symbols["env"][env_symbol] += 1
    if once_symbol != "missing":
        bucket_present_counts["once"] += 1
        bucket_symbols["once"][once_symbol] += 1
    if path_symbol != "missing":
        bucket_present_counts["path"] += 1
        bucket_symbols["path"][path_symbol] += 1
    if minimal_main_symbol != "missing":
        bucket_present_counts["minimal_main"] += 1
        bucket_symbols["minimal_main"][minimal_main_symbol] += 1
    if nyash_kernel_runtime_symbol != "missing":
        bucket_present_counts["nyash_kernel_runtime"] += 1
        bucket_symbols["nyash_kernel_runtime"][nyash_kernel_runtime_symbol] += 1
    summaries.append(f"{primary}:{top_count}")

env_mode, env_mode_count = mode(bucket_symbols["env"])
once_mode, once_mode_count = mode(bucket_symbols["once"])
path_mode, path_mode_count = mode(bucket_symbols["path"])
minimal_main_mode, minimal_main_mode_count = mode(bucket_symbols["minimal_main"])
nyash_kernel_runtime_mode, nyash_kernel_runtime_mode_count = mode(bucket_symbols["nyash_kernel_runtime"])

print(
    primary_counts["env"],
    primary_counts["once"],
    primary_counts["path"],
    primary_counts["minimal_main"],
    primary_counts["nyash_kernel_runtime"],
    primary_counts["kernel"],
    primary_counts["other"],
    primary_counts["missing"],
    primary_counts["alloc"],
    primary_counts["ffi"],
    primary_counts["string"],
    ret0_min if ret0_min is not None else 0,
    ret0_max if ret0_max is not None else 0,
    ";".join(summaries),
    bucket_present_counts["env"],
    bucket_present_counts["once"],
    bucket_present_counts["path"],
    bucket_present_counts["minimal_main"],
    bucket_present_counts["nyash_kernel_runtime"],
    env_mode,
    env_mode_count,
    once_mode,
    once_mode_count,
    path_mode,
    path_mode_count,
    minimal_main_mode,
    minimal_main_mode_count,
    nyash_kernel_runtime_mode,
    nyash_kernel_runtime_mode_count,
)
PY
)

cat <<EOF
output_contract=perf-userbox-startup-executable-ret0-bucket-symbol-variability-v0
input_contract=perf-userbox-startup-loader-owner-split-v0
measurement_scope=exact_aot_startup_executable_ret0_bucket_symbol_variability
trial_count=$total_trials
startup_runs_per_trial=$STARTUP_RUNS
primary_bucket_env_count=$primary_bucket_env_count
primary_bucket_once_count=$primary_bucket_once_count
primary_bucket_path_count=$primary_bucket_path_count
primary_bucket_minimal_main_count=$primary_bucket_minimal_main_count
primary_bucket_nyash_kernel_runtime_count=$primary_bucket_nyash_kernel_runtime_count
primary_bucket_kernel_count=$primary_bucket_kernel_count
primary_bucket_other_count=$primary_bucket_other_count
primary_bucket_missing_count=$primary_bucket_missing_count
primary_bucket_alloc_count=$primary_bucket_alloc_count
primary_bucket_ffi_count=$primary_bucket_ffi_count
primary_bucket_string_count=$primary_bucket_string_count
ret0_top_count_min=$ret0_count_min
ret0_top_count_max=$ret0_count_max
trial_summaries=$trial_summaries
bucket_env_present_count=$bucket_env_present_count
bucket_once_present_count=$bucket_once_present_count
bucket_path_present_count=$bucket_path_present_count
bucket_minimal_main_present_count=$bucket_minimal_main_present_count
bucket_nyash_kernel_runtime_present_count=$bucket_nyash_kernel_runtime_present_count
bucket_env_symbol_mode=$bucket_env_symbol_mode
bucket_env_symbol_mode_count=$bucket_env_symbol_mode_count
bucket_once_symbol_mode=$bucket_once_symbol_mode
bucket_once_symbol_mode_count=$bucket_once_symbol_mode_count
bucket_path_symbol_mode=$bucket_path_symbol_mode
bucket_path_symbol_mode_count=$bucket_path_symbol_mode_count
bucket_minimal_main_symbol_mode=$bucket_minimal_main_symbol_mode
bucket_minimal_main_symbol_mode_count=$bucket_minimal_main_symbol_mode_count
bucket_nyash_kernel_runtime_symbol_mode=$bucket_nyash_kernel_runtime_symbol_mode
bucket_nyash_kernel_runtime_symbol_mode_count=$bucket_nyash_kernel_runtime_symbol_mode_count
loader_floor_owner=$(awk -F= '$1=="loader_floor_owner" { print $2; exit }' "$FLOOR_REPORT")
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
summary=ok
EOF
