#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/nyrt_startup_floor_bare_entry_ab_probe.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_nyrt_ring0_init_off_guard.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[nyrt-ring0-init-off] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_not_missing() {
  local file="$1"
  local key="$2"
  if grep -q "^${key}=missing$" "$file"; then
    echo "[nyrt-ring0-init-off] key unexpectedly missing in ${file#$ROOT_DIR/}: $key" >&2
    exit 1
  fi
}

HAKO_NYRT_PLUGIN_HOST=off \
NYASH_NYRT_RING0_INIT=off \
NYASH_NYRT_RUNTIME_HOOKS=off \
NYASH_NYRT_RUNTIME_BUILD=off \
NYASH_NYRT_ENTRY_PATH_PREP=off \
NYASH_NYRT_MINIMAL_STARTUP=1 \
NYASH_NYRT_SILENT_RESULT=1 \
"$TOOL" --out "$REPORT" --startup-runs 10 >/dev/null

require_line "$REPORT" "output_contract=nyrt-startup-floor-bare-entry-ab-v0"
require_line "$REPORT" "startup_floor_probe=bare_entry_ab"
require_line "$REPORT" "ring0_init_mode=off"
require_line "$REPORT" "runtime_build_mode=off"
require_line "$REPORT" "entry_path_prep_mode=off"
require_line "$REPORT" "same_ny_main_object=1"
require_line "$REPORT" "current_minimal_run_status=ok"
require_line "$REPORT" "bare_entry_run_status=ok"
require_line "$REPORT" "perf_top_symbols_reported=1"
require_line "$REPORT" "touch_hako_source=0"
require_line "$REPORT" "touch_mirbuilder=0"
require_line "$REPORT" "touch_route_planner=0"
require_line "$REPORT" "touch_exact_helper_lowering=0"
require_line "$REPORT" "touch_runtime_object_representation=0"
require_line "$REPORT" "summary=ok"
require_not_missing "$REPORT" "current_minimal_cycles"
require_not_missing "$REPORT" "bare_entry_cycles"
require_not_missing "$REPORT" "entry_delta_cycles"
require_not_missing "$REPORT" "entry_delta_ratio"

cat "$REPORT"
