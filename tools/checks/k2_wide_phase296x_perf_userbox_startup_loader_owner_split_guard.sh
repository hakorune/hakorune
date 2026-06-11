#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-loader-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_key() {
  local file="$1"
  local key="$2"
  if ! grep -q "^${key}=" "$file"; then
    echo "[perf-userbox-startup-loader-owner] missing key in ${file#$ROOT_DIR/}: $key" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-startup-loader-owner] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

"$TOOL" --out "$REPORT" --startup-runs 10 --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null

require_line "$REPORT" "output_contract=perf-userbox-startup-loader-owner-split-v0"
require_line "$REPORT" "input_contract=perf-userbox-direct-helper-floor-attribution-v0"
require_line "$REPORT" "measurement_scope=userbox_exact_aot_startup_loader_owner_split"
require_line "$REPORT" "startup_probe=ret0_exact_aot_spawn_runner"
require_line "$REPORT" "startup_runs=10"
require_line "$REPORT" "ret0_perf_top_available=1"
require_line "$REPORT" "attribution_floor_run_status=ok"
require_line "$REPORT" "attribution_direct_helper_floor_invalid_arraybox_handle_count=0"
require_line "$REPORT" "attribution_counter_step_chain_helper_vs_floor_measured=1"
require_line "$REPORT" "attribution_point_add_helper_vs_floor_measured=1"
require_line "$REPORT" "attribution_startup_loader_attribution_report=1"
require_line "$REPORT" "attribution_measurement_harness_failure_count=0"
require_line "$REPORT" "touch_hako_source=0"
require_line "$REPORT" "touch_mirbuilder=0"
require_line "$REPORT" "touch_route_planner=0"
require_line "$REPORT" "touch_exact_helper_lowering=0"
require_line "$REPORT" "touch_runtime_object_representation=0"
require_line "$REPORT" "summary=ok"

require_key "$REPORT" "startup_loader_primary_owner_family"
require_key "$REPORT" "startup_loader_dynamic_loader_pct"
require_key "$REPORT" "startup_loader_process_spawn_wait_pct"
require_key "$REPORT" "startup_loader_libc_process_pct"
require_key "$REPORT" "startup_loader_nyash_kernel_runtime_pct"
require_key "$REPORT" "startup_loader_minimal_main_pct"
require_key "$REPORT" "startup_loader_top_0_symbol"
require_key "$REPORT" "startup_loader_top_0_family"
require_line "$REPORT" "startup_loader_primary_owner_family=dynamic_loader"
require_positive_key "$REPORT" "ret0_perf_top_row_count"
require_positive_key "$REPORT" "attribution_counter_step_chain_floor_startup_loader_cycles"
require_positive_key "$REPORT" "attribution_point_add_floor_startup_loader_cycles"

cat "$REPORT"
