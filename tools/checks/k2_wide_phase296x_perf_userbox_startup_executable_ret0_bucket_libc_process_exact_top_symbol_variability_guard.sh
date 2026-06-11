#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_libc_process_exact_top_symbol_variability.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_RUNS=200
TRIALS=3

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-exact-top-symbol-variability] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

trial_results="$TMP_DIR/trials.txt"
: >"$trial_results"

for trial in $(seq 1 "$TRIALS"); do
  trial_out="$TMP_DIR/trial_${trial}.out"
  "$TOOL" --out "$trial_out" --startup-runs "$STARTUP_RUNS" --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null
  require_line "$trial_out" "output_contract=perf-userbox-startup-loader-owner-split-v0"
  require_line "$trial_out" "summary=ok"

  printf '%s\n' "$trial_out" >>"$trial_results"
done

python3 - "$trial_results" <<'PY'
from __future__ import annotations

import sys
from collections import Counter
from pathlib import Path

paths = [Path(line.strip()) for line in Path(sys.argv[1]).read_text().splitlines() if line.strip()]
primary_family_counts = Counter()
symbol_counts = Counter()
primary_family_summaries = []
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
    primary_family = data.get("startup_loader_primary_owner_family", "missing")
    primary_family_summaries.append(primary_family)
    primary_family_counts[primary_family] += 1
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
        idx += 1

dominant_symbol, dominant_symbol_count = ("missing", 0)
if symbol_counts:
    dominant_symbol, dominant_symbol_count = max(symbol_counts.items(), key=lambda item: (item[1], item[0]))

print(f"output_contract=perf-userbox-startup-executable-ret0-bucket-libc-process-exact-top-symbol-variability-v0")
print(f"input_contract=perf-userbox-startup-loader-owner-split-v0")
print(f"snapshot_runs={len(paths)}")
primary_family_dominant_mode, primary_family_dominant_mode_count = ("missing", 0)
if primary_family_counts:
    primary_family_dominant_mode, primary_family_dominant_mode_count = max(primary_family_counts.items(), key=lambda item: (item[1], item[0]))
print(f"primary_family_dominant_mode={primary_family_dominant_mode}")
print(f"primary_family_dominant_mode_count={primary_family_dominant_mode_count}")
for family, count in sorted(primary_family_counts.items(), key=lambda item: (-item[1], item[0])):
    print(f"primary_family_count[{family}]={count}")
print(f"startup_loader_dynamic_loader_pct={dynamic_loader_pct}")
print(f"startup_loader_libc_process_pct={libc_process_pct}")
print(f"libc_process_dominant_symbol={dominant_symbol}")
print(f"libc_process_dominant_symbol_count={dominant_symbol_count}")
for symbol, count in sorted(symbol_counts.items(), key=lambda item: (-item[1], item[0])):
    safe = symbol.replace("\n", " ")
    print(f"libc_process_symbol_count[{safe}]={count}")
print(f"primary_family_summaries={';'.join(primary_family_summaries)}")
print("summary=ok")
PY
