#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
FLOOR_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_loader_libc_floor_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_split.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_REPORT="$TMP_DIR/startup_owner.out"
FLOOR_REPORT="$TMP_DIR/loader_floor.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-split] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-split] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

"$TOOL" --out "$STARTUP_REPORT" --startup-runs 30 --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null
"$FLOOR_GUARD" >"$FLOOR_REPORT"

ret0_top_count="$(awk -F= '$1=="startup_loader_ret0_exe_top_count" { print $2; exit }' "$STARTUP_REPORT")"
ret0_primary_bucket="$(awk -F= '$1=="startup_loader_ret0_exe_primary_bucket" { print $2; exit }' "$STARTUP_REPORT")"

require_line "$STARTUP_REPORT" "output_contract=perf-userbox-startup-loader-owner-split-v0"
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

if [ "$ret0_top_count" -lt 1 ]; then
  echo "[perf-userbox-startup-executable-ret0-bucket-split] expected ret0.exe rows in ${STARTUP_REPORT#$ROOT_DIR/}" >&2
  exit 1
fi
if [ "$ret0_primary_bucket" = "missing" ]; then
  echo "[perf-userbox-startup-executable-ret0-bucket-split] expected ret0.exe primary bucket in ${STARTUP_REPORT#$ROOT_DIR/}" >&2
  exit 1
fi

bucket_sum="$(python3 - "$STARTUP_REPORT" <<'PY'
from __future__ import annotations
import sys
from pathlib import Path

path = Path(sys.argv[1])
vals = {}
for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if "=" not in line:
        continue
    k, v = line.split("=", 1)
    vals[k] = v

total = 0
for bucket in ("env", "path", "ffi", "alloc", "once", "string", "nyash_kernel_runtime", "minimal_main"):
    value = vals.get(f"startup_loader_ret0_exe_bucket_{bucket}_count")
    if value is None:
        continue
    try:
        total += int(value)
    except ValueError:
        pass
print(total)
PY
)"

if [ "$bucket_sum" -lt 1 ]; then
  echo "[perf-userbox-startup-executable-ret0-bucket-split] expected ret0.exe bucket counts in ${STARTUP_REPORT#$ROOT_DIR/}" >&2
  exit 1
fi

cat <<EOF
output_contract=perf-userbox-startup-executable-ret0-bucket-split-v0
input_contract=perf-userbox-startup-loader-owner-split-v0
measurement_scope=exact_aot_startup_executable_ret0_bucket_split
probe_rows=perf-userbox-startup-loader-owner-split-v0,perf-userbox-loader-libc-floor-probe-v0
startup_loader_ret0_exe_top_count=$ret0_top_count
startup_loader_ret0_exe_primary_bucket=$ret0_primary_bucket
startup_loader_ret0_exe_bucket_env_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_env_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ret0_exe_bucket_path_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_path_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ret0_exe_bucket_ffi_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_ffi_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ret0_exe_bucket_alloc_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_alloc_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ret0_exe_bucket_once_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_once_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ret0_exe_bucket_string_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_string_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ret0_exe_bucket_nyash_kernel_runtime_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_nyash_kernel_runtime_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ret0_exe_bucket_minimal_main_count=$(awk -F= '$1=="startup_loader_ret0_exe_bucket_minimal_main_count" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_dynamic_loader_pct=$(awk -F= '$1=="startup_loader_dynamic_loader_pct" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_libc_process_pct=$(awk -F= '$1=="startup_loader_libc_process_pct" { print $2; exit }' "$STARTUP_REPORT")
loader_floor_owner=$(awk -F= '$1=="loader_floor_owner" { print $2; exit }' "$FLOOR_REPORT")
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
summary=ok
EOF
