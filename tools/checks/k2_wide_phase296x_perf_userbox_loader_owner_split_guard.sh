#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STARTUP_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_startup_loader_owner_split_guard.sh"
FLOOR_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_loader_libc_floor_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_loader_owner_split.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_REPORT="$TMP_DIR/startup_owner.out"
FLOOR_REPORT="$TMP_DIR/loader_floor.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-loader-owner-split] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_key() {
  local file="$1"
  local key="$2"
  if ! grep -q "^${key}=" "$file"; then
    echo "[perf-userbox-loader-owner-split] missing key in ${file#$ROOT_DIR/}: $key" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-loader-owner-split] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

value_of() {
  local file="$1"
  local key="$2"
  awk -F= -v key="$key" '$1 == key { print $2; exit }' "$file"
}

"$STARTUP_GUARD" >"$STARTUP_REPORT"
"$FLOOR_GUARD" >"$FLOOR_REPORT"

require_line "$STARTUP_REPORT" "output_contract=perf-userbox-startup-loader-owner-split-v0"
require_line "$STARTUP_REPORT" "startup_loader_primary_owner_family=dynamic_loader"
require_line "$STARTUP_REPORT" "summary=ok"
require_positive_key "$STARTUP_REPORT" "startup_loader_dynamic_loader_pct"
require_positive_key "$STARTUP_REPORT" "startup_loader_libc_process_pct"

require_line "$FLOOR_REPORT" "output_contract=perf-userbox-loader-libc-floor-probe-v0"
require_line "$FLOOR_REPORT" "loader_floor_owner=link_mode/loader/libc"
require_line "$FLOOR_REPORT" "dynamic_needed_libgcc_s=0"
require_line "$FLOOR_REPORT" "dynamic_needed_libm=0"
require_line "$FLOOR_REPORT" "dynamic_needed_libc=1"
require_line "$FLOOR_REPORT" "dynamic_needed_ld_linux=1"
require_line "$FLOOR_REPORT" "summary=ok"

cat <<EOF
output_contract=perf-userbox-loader-owner-split-v0
input_contract=perf-userbox-startup-loader-owner-split-v0
measurement_scope=exact_aot_loader_owner_split
probe_rows=perf-userbox-startup-loader-owner-split-v0,perf-userbox-loader-libc-floor-probe-v0
startup_loader_primary_owner_family=$(value_of "$STARTUP_REPORT" startup_loader_primary_owner_family)
startup_loader_top_0_family=$(value_of "$STARTUP_REPORT" startup_loader_top_0_family)
startup_loader_dynamic_loader_pct=$(value_of "$STARTUP_REPORT" startup_loader_dynamic_loader_pct)
startup_loader_libc_process_pct=$(value_of "$STARTUP_REPORT" startup_loader_libc_process_pct)
loader_floor_owner=$(value_of "$FLOOR_REPORT" loader_floor_owner)
dynamic_needed_libgcc_s=$(value_of "$FLOOR_REPORT" dynamic_needed_libgcc_s)
dynamic_needed_libm=$(value_of "$FLOOR_REPORT" dynamic_needed_libm)
dynamic_needed_libc=$(value_of "$FLOOR_REPORT" dynamic_needed_libc)
dynamic_needed_ld_linux=$(value_of "$FLOOR_REPORT" dynamic_needed_ld_linux)
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
summary=ok
EOF
