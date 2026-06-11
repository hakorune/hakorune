#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_libc_process_malloc_dominance.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_RUNS=200
SNAPSHOTS=3

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-malloc-dominance] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

snapshot_results="$TMP_DIR/snapshots.txt"
: >"$snapshot_results"

for snapshot in $(seq 1 "$SNAPSHOTS"); do
  snapshot_out="$TMP_DIR/snapshot_${snapshot}.out"
  "$TOOL" --out "$snapshot_out" --startup-runs "$STARTUP_RUNS" --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null
  require_line "$snapshot_out" "output_contract=perf-userbox-startup-loader-owner-split-v0"
  require_line "$snapshot_out" "summary=ok"
  printf '%s\n' "$snapshot_out" >>"$snapshot_results"
done

python3 - "$snapshot_results" <<'PY'
from __future__ import annotations

import sys
from collections import Counter
from pathlib import Path

paths = [Path(line.strip()) for line in Path(sys.argv[1]).read_text().splitlines() if line.strip()]
symbol_counts = Counter()
family_counts = Counter()
snapshot_summaries = []
dynamic_loader_pct = None
libc_process_pct = None

def keyvals(path: Path) -> dict[str, str]:
    data: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        data[key.strip()] = value.strip()
    return data

for path in paths:
    data = keyvals(path)
    snapshot_summaries.append(data.get("trial_summaries", "missing"))
    if dynamic_loader_pct is None:
      dynamic_loader_pct = data.get("startup_loader_dynamic_loader_pct", "0")
    if libc_process_pct is None:
      libc_process_pct = data.get("startup_loader_libc_process_pct", "0")

    idx = 0
    while True:
        symbol_key = f"startup_loader_top_{idx}_symbol"
        family_key = f"startup_loader_top_{idx}_family"
        symbol = data.get(symbol_key)
        family = data.get(family_key)
        if symbol is None or family is None:
            break
        if family == "libc_process":
            symbol_counts[symbol] += 1
            family_counts[family] += 1
        idx += 1

dominant_symbol, dominant_symbol_count = ("missing", 0)
runner_up_symbol, runner_up_symbol_count = ("missing", 0)
if symbol_counts:
    ordered = sorted(symbol_counts.items(), key=lambda item: (-item[1], item[0]))
    dominant_symbol, dominant_symbol_count = ordered[0]
    if len(ordered) > 1:
        runner_up_symbol, runner_up_symbol_count = ordered[1]

if dominant_symbol != "_int_malloc":
    raise SystemExit(1)

print("output_contract=perf-userbox-startup-executable-ret0-bucket-libc-process-malloc-dominance-v0")
print("input_contract=perf-userbox-startup-loader-owner-split-v0")
print(f"snapshot_runs={len(paths)}")
print(f"startup_loader_dynamic_loader_pct={dynamic_loader_pct}")
print(f"startup_loader_libc_process_pct={libc_process_pct}")
print(f"libc_process_dominant_symbol={dominant_symbol}")
print(f"libc_process_dominant_symbol_count={dominant_symbol_count}")
print(f"libc_process_runner_up_symbol={runner_up_symbol}")
print(f"libc_process_runner_up_symbol_count={runner_up_symbol_count}")
print(f"libc_process_malloc_symbol_count={symbol_counts.get('malloc', 0)}")
print(f"libc_process_getenv_symbol_count={symbol_counts.get('getenv', 0)}")
print("libc_process_split_symbols=malloc,getenv")
print("libc_process_split_present=1")
for symbol, count in sorted(symbol_counts.items(), key=lambda item: (-item[1], item[0])):
    print(f"libc_process_symbol_count[{symbol}]={count}")
print(f"libc_process_family_count={family_counts['libc_process']}")
print(f"snapshot_summaries={';'.join(snapshot_summaries)}")
print("summary=ok")
PY
