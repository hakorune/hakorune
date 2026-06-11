#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STATIC_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_aot_static_libgcc_probe_guard.sh"
MINIMAL_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_aot_minimal_system_libs_probe_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_loader_libc_floor.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STATIC_REPORT="$TMP_DIR/static_libgcc.out"
MINIMAL_REPORT="$TMP_DIR/minimal_system_libs.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-loader-libc-floor] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

"$STATIC_GUARD" >"$STATIC_REPORT"
"$MINIMAL_GUARD" >"$MINIMAL_REPORT"

require_line "$STATIC_REPORT" "output_contract=perf-aot-static-libgcc-probe-v0"
require_line "$STATIC_REPORT" "aot_ldflags=-static-libgcc"
require_line "$STATIC_REPORT" "dynamic_needed_libgcc_s=0"
require_line "$STATIC_REPORT" "dynamic_needed_libm=1"
require_line "$STATIC_REPORT" "default_link_mode_changed=0"
require_line "$STATIC_REPORT" "summary=ok"

require_line "$MINIMAL_REPORT" "output_contract=perf-aot-minimal-system-libs-probe-v0"
require_line "$MINIMAL_REPORT" "aot_ldflags=-static-libgcc"
require_line "$MINIMAL_REPORT" "link_system_libs=minimal"
require_line "$MINIMAL_REPORT" "dynamic_needed_libgcc_s=0"
require_line "$MINIMAL_REPORT" "dynamic_needed_libm=0"
require_line "$MINIMAL_REPORT" "default_link_mode_changed=0"
require_line "$MINIMAL_REPORT" "summary=ok"

cat <<'EOF'
output_contract=perf-userbox-loader-libc-floor-probe-v0
input_contract=perf-userbox-startup-loader-owner-split-v0
measurement_scope=exact_aot_loader_libc_floor_link_probe
probe_rows=perf-aot-static-libgcc-probe-v0,perf-aot-minimal-system-libs-probe-v0
aot_ldflags=-static-libgcc
link_system_libs=minimal
dynamic_needed_libgcc_s=0
dynamic_needed_libm=0
dynamic_needed_libc=1
dynamic_needed_ld_linux=1
default_link_mode_changed=0
loader_floor_owner=link_mode/loader/libc
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
summary=ok
EOF
