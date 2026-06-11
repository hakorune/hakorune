#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
FLOOR_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_loader_libc_floor_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_split.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_REPORT="$TMP_DIR/startup_owner.out"
FLOOR_REPORT="$TMP_DIR/loader_floor.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-split] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-startup-executable-split] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

"$TOOL" --out "$STARTUP_REPORT" --startup-runs 10 --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null
"$FLOOR_GUARD" >"$FLOOR_REPORT"

ret0_exe_pct="$(python3 - "$STARTUP_REPORT" <<'PY'
from __future__ import annotations
import sys
from pathlib import Path

path = Path(sys.argv[1])
total = 0.0
current = {}
for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if "=" not in line:
        continue
    key, value = line.split("=", 1)
    current[key] = value
for idx in range(100):
    pct = current.get(f"startup_loader_top_{idx}_pct")
    dso = current.get(f"startup_loader_top_{idx}_dso")
    if pct is None or dso is None:
        continue
    if dso == "ret0.exe":
        total += float(pct)
print(f"{total:.2f}")
PY
)"

ret0_exe_top_count="$(grep -c '^startup_loader_top_[0-9]\+_dso=ret0.exe$' "$STARTUP_REPORT" || true)"

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

if [ "$ret0_exe_pct" = "0.00" ] || [ "$ret0_exe_pct" = "missing" ]; then
  echo "[perf-userbox-startup-executable-split] expected ret0.exe to contribute in ${STARTUP_REPORT#$ROOT_DIR/}" >&2
  exit 1
fi
if [ "$ret0_exe_top_count" -lt 1 ]; then
  echo "[perf-userbox-startup-executable-split] expected ret0.exe top rows in ${STARTUP_REPORT#$ROOT_DIR/}" >&2
  exit 1
fi

cat <<EOF
output_contract=perf-userbox-startup-executable-split-v0
input_contract=perf-userbox-startup-loader-owner-split-v0
measurement_scope=exact_aot_startup_executable_split
probe_rows=perf-userbox-startup-loader-owner-split-v0,perf-userbox-loader-libc-floor-probe-v0
startup_loader_ret0_exe_pct=$ret0_exe_pct
startup_loader_ret0_exe_top_count=$ret0_exe_top_count
startup_loader_dynamic_loader_pct=$(awk -F= '$1=="startup_loader_dynamic_loader_pct" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_libc_process_pct=$(awk -F= '$1=="startup_loader_libc_process_pct" { print $2; exit }' "$STARTUP_REPORT")
loader_floor_owner=$(awk -F= '$1=="loader_floor_owner" { print $2; exit }' "$FLOOR_REPORT")
dynamic_needed_libgcc_s=$(awk -F= '$1=="dynamic_needed_libgcc_s" { print $2; exit }' "$FLOOR_REPORT")
dynamic_needed_libm=$(awk -F= '$1=="dynamic_needed_libm" { print $2; exit }' "$FLOOR_REPORT")
dynamic_needed_libc=$(awk -F= '$1=="dynamic_needed_libc" { print $2; exit }' "$FLOOR_REPORT")
dynamic_needed_ld_linux=$(awk -F= '$1=="dynamic_needed_ld_linux" { print $2; exit }' "$FLOOR_REPORT")
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
summary=ok
EOF
