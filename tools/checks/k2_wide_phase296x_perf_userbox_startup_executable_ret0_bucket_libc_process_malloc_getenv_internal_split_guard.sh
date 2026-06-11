#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_libc_process_malloc_getenv_internal_split.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_RUNS=200
SNAPSHOTS=3

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-malloc-getenv-internal-split] missing line in ${file#$ROOT_DIR/}: $expected" >&2
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
bucket_counts = Counter()
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


def malloc_bucket(symbol: str) -> str:
    lowered = symbol.lower()
    if lowered == "_int_malloc":
        return "int_malloc"
    if lowered in {"malloc", "__libc_malloc"}:
        return "malloc_api"
    if any(token in lowered for token in ("calloc", "realloc", "free", "arena")):
        return "allocator_other"
    if "getenv" in lowered:
        return "getenv"
    if any(token in lowered for token in ("memmove", "memcpy", "strncmp", "strlen", "strcmp")):
        return "libc_string"
    return "other"


for path in paths:
    data = keyvals(path)
    snapshot_summaries.append(data.get("trial_summaries", "missing"))
    if dynamic_loader_pct is None:
        dynamic_loader_pct = data.get("startup_loader_dynamic_loader_pct", "0")
    if libc_process_pct is None:
        libc_process_pct = data.get("startup_loader_libc_process_pct", "0")

    idx = 0
    while True:
        symbol = data.get(f"startup_loader_top_{idx}_symbol")
        family = data.get(f"startup_loader_top_{idx}_family")
        if symbol is None or family is None:
            break
        if family == "libc_process":
            symbol_counts[symbol] += 1
            bucket_counts[malloc_bucket(symbol)] += 1
        idx += 1

dominant_symbol, dominant_symbol_count = ("missing", 0)
if symbol_counts:
    dominant_symbol, dominant_symbol_count = sorted(
        symbol_counts.items(), key=lambda item: (-item[1], item[0])
    )[0]

if bucket_counts["int_malloc"] < 1:
    raise SystemExit(1)
if bucket_counts["getenv"] < 1:
    raise SystemExit(1)

print("output_contract=perf-userbox-startup-executable-ret0-bucket-libc-process-malloc-getenv-internal-split-v0")
print("input_contract=perf-userbox-startup-loader-owner-split-v0")
print(f"snapshot_runs={len(paths)}")
print(f"startup_loader_dynamic_loader_pct={dynamic_loader_pct}")
print(f"startup_loader_libc_process_pct={libc_process_pct}")
print(f"libc_process_dominant_symbol={dominant_symbol}")
print(f"libc_process_dominant_symbol_count={dominant_symbol_count}")
print(f"libc_process_malloc_bucket_int_malloc_count={bucket_counts['int_malloc']}")
print(f"libc_process_malloc_bucket_malloc_api_count={bucket_counts['malloc_api']}")
print(f"libc_process_malloc_bucket_allocator_other_count={bucket_counts['allocator_other']}")
print(f"libc_process_malloc_bucket_getenv_count={bucket_counts['getenv']}")
print(f"libc_process_malloc_bucket_libc_string_count={bucket_counts['libc_string']}")
print(f"libc_process_malloc_bucket_other_count={bucket_counts['other']}")
print("libc_process_malloc_getenv_internal_split_present=1")
for symbol, count in sorted(symbol_counts.items(), key=lambda item: (-item[1], item[0])):
    print(f"libc_process_symbol_count[{symbol}]={count}")
print(f"snapshot_summaries={';'.join(snapshot_summaries)}")
print("summary=ok")
PY
