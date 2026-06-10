#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_direct_helper_floor_attribution.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_floor_attr.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-floor-attribution] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-floor-attribution] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

"$TOOL" --out "$REPORT" --warmup 0 --repeat 1 --kernel-inner-runs 3 >/dev/null

require_line "$REPORT" "output_contract=perf-userbox-direct-helper-floor-attribution-v0"
require_line "$REPORT" "measurement_scope=userbox_typed_object_floor_helper_startup_loader_attribution"
require_line "$REPORT" "typed_object_floor_backend=single_thread_exact"
require_line "$REPORT" "typed_object_helper_backend=single_thread_exact"
require_line "$REPORT" "typed_object_helper_gate=HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1"
require_line "$REPORT" "array_slot_backend=unset"
require_line "$REPORT" "direct_helper_floor_run_status=ok"
require_line "$REPORT" "direct_helper_helper_run_status=ok"
require_line "$REPORT" "floor_run_status=ok"
require_line "$REPORT" "direct_helper_floor_invalid_arraybox_handle_count=0"
require_line "$REPORT" "counter_step_chain_helper_vs_floor_measured=1"
require_line "$REPORT" "point_add_helper_vs_floor_measured=1"
require_line "$REPORT" "startup_loader_attribution_report=1"
require_line "$REPORT" "startup_loader_attribution_case_count=2"
require_line "$REPORT" "measurement_harness_failure_count=0"
require_line "$REPORT" "counter_step_chain_startup_loader_attribution=available"
require_line "$REPORT" "point_add_startup_loader_attribution=available"
require_line "$REPORT" "touch_hako_source=0"
require_line "$REPORT" "touch_mirbuilder=0"
require_line "$REPORT" "touch_route_planner=0"
require_line "$REPORT" "touch_exact_helper_lowering=0"
require_line "$REPORT" "touch_runtime_object_representation=0"
require_line "$REPORT" "summary=ok"

require_positive_key "$REPORT" "counter_step_chain_floor_startup_loader_cycles"
require_positive_key "$REPORT" "counter_step_chain_helper_startup_loader_cycles"
require_positive_key "$REPORT" "point_add_floor_startup_loader_cycles"
require_positive_key "$REPORT" "point_add_helper_startup_loader_cycles"

cat "$REPORT"
