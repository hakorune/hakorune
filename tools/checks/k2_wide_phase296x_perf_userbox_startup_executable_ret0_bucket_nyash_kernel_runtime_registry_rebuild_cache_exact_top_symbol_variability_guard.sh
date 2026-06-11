#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
FLOOR_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_loader_libc_floor_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_nyash_kernel_runtime_registry_rebuild_cache_exact_top_symbol_variability.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_RUNS=200
TRIALS=7

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-exact-top-symbol-variability] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-exact-top-symbol-variability] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
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
  top_0_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_nyash_kernel_runtime_top_0_symbol" { print $2; exit }' "$trial_out")"
  top_1_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_nyash_kernel_runtime_top_1_symbol" { print $2; exit }' "$trial_out")"
  top_2_symbol="$(awk -F= '$1=="startup_loader_ret0_exe_bucket_nyash_kernel_runtime_top_2_symbol" { print $2; exit }' "$trial_out")"

  case "$primary_bucket" in
    env|once|path|minimal_main|nyash_kernel_runtime|kernel|other|missing|alloc|ffi|string)
      :
      ;;
    *)
      echo "[perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-exact-top-symbol-variability] unexpected primary bucket '$primary_bucket' in ${trial_out#$ROOT_DIR/}" >&2
      exit 1
      ;;
  esac

  printf '%s %s %s %s %s\n' \
    "$primary_bucket" \
    "$ret0_top_count" \
    "$top_0_symbol" \
    "$top_1_symbol" \
    "$top_2_symbol" >>"$trial_results"

  if [ -z "$ret0_count_min" ] || [ "$ret0_top_count" -lt "$ret0_count_min" ]; then
    ret0_count_min="$ret0_top_count"
  fi
  if [ -z "$ret0_count_max" ] || [ "$ret0_top_count" -gt "$ret0_count_max" ]; then
    ret0_count_max="$ret0_top_count"
  fi
done

total_trials="$TRIALS"

read -r primary_bucket_env_count primary_bucket_once_count primary_bucket_path_count primary_bucket_minimal_main_count primary_bucket_nyash_kernel_runtime_count primary_bucket_kernel_count primary_bucket_other_count primary_bucket_missing_count primary_bucket_alloc_count primary_bucket_ffi_count primary_bucket_string_count ret0_count_min ret0_count_max trial_summaries registry_top_0_mode registry_top_0_mode_count registry_top_1_mode registry_top_1_mode_count registry_top_2_mode registry_top_2_mode_count registry_exact_build_mode registry_exact_build_mode_count registry_exact_registry_mode registry_exact_registry_mode_count registry_exact_runtime_mode registry_exact_runtime_mode_count registry_exact_once_mode registry_exact_once_mode_count registry_exact_other_mode registry_exact_other_mode_count registry_exact_rebuild_cache_mode registry_exact_rebuild_cache_mode_count registry_exact_register_many_mode registry_exact_register_many_mode_count registry_exact_create_default_registry_mode registry_exact_create_default_registry_mode_count registry_exact_gc_mode registry_exact_gc_mode_count registry_exact_ring0_mode registry_exact_ring0_mode_count registry_exact_scheduler_mode registry_exact_scheduler_mode_count < <(
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
top_symbol_counts = [Counter(), Counter(), Counter()]
exact_family_counts = Counter()
ret0_min = None
ret0_max = None
summaries = []


def classify_exact(symbol: str) -> str:
    text = symbol.lower()
    if "rebuild_cache" in text:
        return "rebuild_cache"
    if "register_many" in text:
        return "register_many"
    if "create_default_registry" in text:
        return "create_default_registry"
    if "gcmode::parse" in text or "gc_mode::parse" in text:
        return "gc_mode"
    if "ring0registry" in text or "init_global_ring0" in text:
        return "ring0"
    if "scheduler::new" in text:
        return "scheduler"
    if "nyashruntimebuilder::build" in text or "build_with_fs" in text:
        return "build"
    if "registry" in text or "box_factory" in text:
        return "registry"
    if "once" in text or "futex" in text:
        return "once"
    if "nyash_rust" in text or "hako_" in text or "runtime" in text:
        return "runtime"
    return "other"


def mode(counter: Counter[str]) -> tuple[str, int]:
    if not counter:
        return ("missing", 0)
    symbol, count = max(counter.items(), key=lambda item: (item[1], item[0]))
    return symbol, count


for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if not line.strip():
        continue
    parts = line.split()
    if len(parts) != 5:
        continue
    primary, top_count_s, top_0_symbol, top_1_symbol, top_2_symbol = parts
    if primary not in primary_counts:
        primary = "missing"
    primary_counts[primary] += 1
    try:
        top_count = int(top_count_s)
    except ValueError:
        top_count = 0
    ret0_min = top_count if ret0_min is None or top_count < ret0_min else ret0_min
    ret0_max = top_count if ret0_max is None or top_count > ret0_max else ret0_max
    summaries.append(f"{primary}:{top_count}")

    for idx, symbol in enumerate((top_0_symbol, top_1_symbol, top_2_symbol)):
        if symbol == "missing":
            continue
        top_symbol_counts[idx][symbol] += 1
        exact_family_counts[classify_exact(symbol)] += 1

registry_top_0_mode, registry_top_0_mode_count = mode(top_symbol_counts[0])
registry_top_1_mode, registry_top_1_mode_count = mode(top_symbol_counts[1])
registry_top_2_mode, registry_top_2_mode_count = mode(top_symbol_counts[2])
registry_exact_build_mode = "build" if exact_family_counts["build"] > 0 else "missing"
registry_exact_build_mode_count = exact_family_counts["build"]
registry_exact_registry_mode = "registry" if exact_family_counts["registry"] > 0 else "missing"
registry_exact_registry_mode_count = exact_family_counts["registry"]
registry_exact_runtime_mode = "runtime" if exact_family_counts["runtime"] > 0 else "missing"
registry_exact_runtime_mode_count = exact_family_counts["runtime"]
registry_exact_once_mode = "once" if exact_family_counts["once"] > 0 else "missing"
registry_exact_once_mode_count = exact_family_counts["once"]
registry_exact_other_mode = "other" if exact_family_counts["other"] > 0 else "missing"
registry_exact_other_mode_count = exact_family_counts["other"]
registry_exact_rebuild_cache_mode = "rebuild_cache" if exact_family_counts["rebuild_cache"] > 0 else "missing"
registry_exact_rebuild_cache_mode_count = exact_family_counts["rebuild_cache"]
registry_exact_register_many_mode = "register_many" if exact_family_counts["register_many"] > 0 else "missing"
registry_exact_register_many_mode_count = exact_family_counts["register_many"]
registry_exact_create_default_registry_mode = "create_default_registry" if exact_family_counts["create_default_registry"] > 0 else "missing"
registry_exact_create_default_registry_mode_count = exact_family_counts["create_default_registry"]
registry_exact_gc_mode = "gc_mode" if exact_family_counts["gc_mode"] > 0 else "missing"
registry_exact_gc_mode_count = exact_family_counts["gc_mode"]
registry_exact_ring0_mode = "ring0" if exact_family_counts["ring0"] > 0 else "missing"
registry_exact_ring0_mode_count = exact_family_counts["ring0"]
registry_exact_scheduler_mode = "scheduler" if exact_family_counts["scheduler"] > 0 else "missing"
registry_exact_scheduler_mode_count = exact_family_counts["scheduler"]

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
    registry_top_0_mode,
    registry_top_0_mode_count,
    registry_top_1_mode,
    registry_top_1_mode_count,
    registry_top_2_mode,
    registry_top_2_mode_count,
    registry_exact_build_mode,
    registry_exact_build_mode_count,
    registry_exact_registry_mode,
    registry_exact_registry_mode_count,
    registry_exact_runtime_mode,
    registry_exact_runtime_mode_count,
    registry_exact_once_mode,
    registry_exact_once_mode_count,
    registry_exact_other_mode,
    registry_exact_other_mode_count,
    registry_exact_rebuild_cache_mode,
    registry_exact_rebuild_cache_mode_count,
    registry_exact_register_many_mode,
    registry_exact_register_many_mode_count,
    registry_exact_create_default_registry_mode,
    registry_exact_create_default_registry_mode_count,
    registry_exact_gc_mode,
    registry_exact_gc_mode_count,
    registry_exact_ring0_mode,
    registry_exact_ring0_mode_count,
    registry_exact_scheduler_mode,
    registry_exact_scheduler_mode_count,
)
PY
)

cat <<EOF
output_contract=perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-exact-top-symbol-variability-v0
input_contract=perf-userbox-startup-loader-owner-split-v0
measurement_scope=exact_aot_startup_executable_ret0_bucket_nyash_kernel_runtime_registry_rebuild_cache_exact_top_symbol_variability
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
registry_top_0_mode=$registry_top_0_mode
registry_top_0_mode_count=$registry_top_0_mode_count
registry_top_1_mode=$registry_top_1_mode
registry_top_1_mode_count=$registry_top_1_mode_count
registry_top_2_mode=$registry_top_2_mode
registry_top_2_mode_count=$registry_top_2_mode_count
registry_exact_build_mode=$registry_exact_build_mode
registry_exact_build_mode_count=$registry_exact_build_mode_count
registry_exact_registry_mode=$registry_exact_registry_mode
registry_exact_registry_mode_count=$registry_exact_registry_mode_count
registry_exact_runtime_mode=$registry_exact_runtime_mode
registry_exact_runtime_mode_count=$registry_exact_runtime_mode_count
registry_exact_once_mode=$registry_exact_once_mode
registry_exact_once_mode_count=$registry_exact_once_mode_count
registry_exact_other_mode=$registry_exact_other_mode
registry_exact_other_mode_count=$registry_exact_other_mode_count
registry_exact_rebuild_cache_mode=$registry_exact_rebuild_cache_mode
registry_exact_rebuild_cache_mode_count=$registry_exact_rebuild_cache_mode_count
registry_exact_register_many_mode=$registry_exact_register_many_mode
registry_exact_register_many_mode_count=$registry_exact_register_many_mode_count
registry_exact_create_default_registry_mode=$registry_exact_create_default_registry_mode
registry_exact_create_default_registry_mode_count=$registry_exact_create_default_registry_mode_count
registry_exact_gc_mode=$registry_exact_gc_mode
registry_exact_gc_mode_count=$registry_exact_gc_mode_count
registry_exact_ring0_mode=$registry_exact_ring0_mode
registry_exact_ring0_mode_count=$registry_exact_ring0_mode_count
registry_exact_scheduler_mode=$registry_exact_scheduler_mode
registry_exact_scheduler_mode_count=$registry_exact_scheduler_mode_count
startup_loader_dynamic_loader_pct=$startup_loader_dynamic_loader_pct
startup_loader_libc_process_pct=$startup_loader_libc_process_pct
loader_floor_owner=$(awk -F= '$1=="loader_floor_owner" { print $2; exit }' "$FLOOR_REPORT")
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
summary=ok
EOF

if [ "${registry_exact_rebuild_cache_mode_count:-0}" -le 0 ]; then
  echo "[perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-exact-top-symbol-variability] expected aggregate registry_exact_rebuild_cache_mode_count > 0" >&2
  exit 1
fi
