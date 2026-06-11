#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
FLOOR_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_loader_libc_floor_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_loader_symbol_split.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_REPORT="$TMP_DIR/startup_owner.out"
FLOOR_REPORT="$TMP_DIR/loader_floor.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-loader-symbol-split] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[perf-userbox-loader-symbol-split] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

"$TOOL" --out "$STARTUP_REPORT" --startup-runs 10 --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null
"$FLOOR_GUARD" >"$FLOOR_REPORT"

ld_symbol="$(python3 - "$STARTUP_REPORT" <<'PY'
from __future__ import annotations
import sys
from pathlib import Path

path = Path(sys.argv[1])
current = {}
for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if "=" not in line:
        continue
    key, value = line.split("=", 1)
    current[key] = value
for idx in range(100):
    dso = current.get(f"startup_loader_top_{idx}_dso")
    symbol = current.get(f"startup_loader_top_{idx}_symbol")
    if dso == "ld-linux-x86-64.so.2":
        print(symbol or "missing")
        break
else:
    print("missing")
PY
)"
ld_pct="$(python3 - "$STARTUP_REPORT" <<'PY'
from __future__ import annotations
import sys
from pathlib import Path

path = Path(sys.argv[1])
current = {}
for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if "=" not in line:
        continue
    key, value = line.split("=", 1)
    current[key] = value
for idx in range(100):
    dso = current.get(f"startup_loader_top_{idx}_dso")
    pct = current.get(f"startup_loader_top_{idx}_pct")
    if dso == "ld-linux-x86-64.so.2":
        print(pct or "missing")
        break
else:
    print("missing")
PY
)"
libc_symbol="$(python3 - "$STARTUP_REPORT" <<'PY'
from __future__ import annotations
import sys
from pathlib import Path

path = Path(sys.argv[1])
current = {}
for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if "=" not in line:
        continue
    key, value = line.split("=", 1)
    current[key] = value
for idx in range(100):
    dso = current.get(f"startup_loader_top_{idx}_dso")
    symbol = current.get(f"startup_loader_top_{idx}_symbol")
    if dso == "libc.so.6":
        print(symbol or "missing")
        break
else:
    print("missing")
PY
)"
libc_pct="$(python3 - "$STARTUP_REPORT" <<'PY'
from __future__ import annotations
import sys
from pathlib import Path

path = Path(sys.argv[1])
current = {}
for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
    if "=" not in line:
        continue
    key, value = line.split("=", 1)
    current[key] = value
for idx in range(100):
    dso = current.get(f"startup_loader_top_{idx}_dso")
    pct = current.get(f"startup_loader_top_{idx}_pct")
    if dso == "libc.so.6":
        print(pct or "missing")
        break
else:
    print("missing")
PY
)"

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

if [ "$ld_symbol" = "missing" ] || [ "$ld_pct" = "missing" ]; then
  echo "[perf-userbox-loader-symbol-split] missing ld-linux top symbol in ${STARTUP_REPORT#$ROOT_DIR/}" >&2
  exit 1
fi
if [ "$libc_symbol" = "missing" ] || [ "$libc_pct" = "missing" ]; then
  echo "[perf-userbox-loader-symbol-split] missing libc top symbol in ${STARTUP_REPORT#$ROOT_DIR/}" >&2
  exit 1
fi

cat <<EOF
output_contract=perf-userbox-loader-symbol-split-v0
input_contract=perf-userbox-startup-loader-owner-split-v0
measurement_scope=exact_aot_loader_symbol_split
probe_rows=perf-userbox-startup-loader-owner-split-v0,perf-userbox-loader-libc-floor-probe-v0
startup_loader_primary_owner_family=$(awk -F= '$1=="startup_loader_primary_owner_family" { print $2; exit }' "$STARTUP_REPORT")
startup_loader_ld_linux_symbol=$ld_symbol
startup_loader_ld_linux_pct=$ld_pct
startup_loader_libc_symbol=$libc_symbol
startup_loader_libc_pct=$libc_pct
loader_floor_owner=$(awk -F= '$1=="loader_floor_owner" { print $2; exit }' "$FLOOR_REPORT")
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
summary=ok
EOF
