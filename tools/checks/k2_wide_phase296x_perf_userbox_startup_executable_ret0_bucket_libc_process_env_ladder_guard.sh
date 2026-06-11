#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL="$ROOT_DIR/tools/allocator/userbox_startup_loader_owner_split.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_libc_process_env_ladder.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_RUNS=200
PATH_REQUIRED="${PATH:-/usr/bin:/bin}"
TMPDIR_KEEP="${TMPDIR:-/tmp}"

run_variant() {
  local variant="$1"
  local out_file="$2"
  shift 2
  "$@" "$TOOL" --out "$out_file" --startup-runs "$STARTUP_RUNS" --lane-warmup 0 --lane-repeat 1 --lane-kernel-inner-runs 3 >/dev/null
  if ! grep -q '^output_contract=perf-userbox-startup-loader-owner-split-v0$' "$out_file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-env-ladder] missing output contract for $variant" >&2
    exit 1
  fi
  if ! grep -q '^summary=ok$' "$out_file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-env-ladder] variant failed: $variant" >&2
    exit 1
  fi
}

inherited_out="$TMP_DIR/inherited_env.out"
required_out="$TMP_DIR/env_i_required.out"
many_dummy_out="$TMP_DIR/env_i_many_dummy.out"
required_total_bytes=$((${#PATH_REQUIRED} + ${#TMPDIR_KEEP} + 1))
required_max_value_bytes=${#PATH_REQUIRED}
if [ "${#TMPDIR_KEEP}" -gt "$required_max_value_bytes" ]; then
  required_max_value_bytes=${#TMPDIR_KEEP}
fi

run_variant "inherited_env" "$inherited_out" env
run_variant "env_i_required" "$required_out" \
  env -i PATH="$PATH_REQUIRED" TMPDIR="$TMPDIR_KEEP" LC_ALL=C

dummy_env=(env -i PATH="$PATH_REQUIRED" TMPDIR="$TMPDIR_KEEP" LC_ALL=C)
dummy_total_bytes=0
dummy_max_value_bytes=32
for i in $(seq -w 0 199); do
  dummy_key="HAKO_DUMMY_$i"
  dummy_value="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
  dummy_env+=("$dummy_key=$dummy_value")
  dummy_total_bytes=$((dummy_total_bytes + ${#dummy_key} + 1 + ${#dummy_value}))
done
many_dummy_total_bytes=$((required_total_bytes + dummy_total_bytes))
many_dummy_max_value_bytes="$dummy_max_value_bytes"
if [ "$required_max_value_bytes" -gt "$many_dummy_max_value_bytes" ]; then
  many_dummy_max_value_bytes="$required_max_value_bytes"
fi
run_variant "env_i_many_dummy" "$many_dummy_out" "${dummy_env[@]}"

python3 - "$inherited_out" "$required_out" "$many_dummy_out" \
  "$required_total_bytes" "$required_max_value_bytes" \
  "$many_dummy_total_bytes" "$many_dummy_max_value_bytes" <<'PY'
from __future__ import annotations

import sys
from collections import Counter
from pathlib import Path


def keyvals(path: Path) -> dict[str, str]:
    data: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        data[key.strip()] = value.strip()
    return data


def bucket(symbol: str) -> str:
    lowered = symbol.lower()
    if "getenv" in lowered:
        return "env_api"
    if any(token in lowered for token in ("strncmp", "strlen", "strcmp")):
        return "env_string_scan"
    if any(token in lowered for token in ("memmove", "memcpy")):
        return "memory_string"
    if any(token in lowered for token in ("malloc", "calloc", "realloc", "cfree", "free", "arena")):
        return "allocator"
    return "other"


def parse_variant(name: str, path: Path, env_count: int, env_total_bytes: int, env_max_value_bytes: int) -> dict[str, object]:
    data = keyvals(path)
    symbol_counts = Counter()
    bucket_counts = Counter()
    idx = 0
    while True:
        symbol = data.get(f"startup_loader_top_{idx}_symbol")
        family = data.get(f"startup_loader_top_{idx}_family")
        if symbol is None or family is None:
            break
        if family == "libc_process":
            symbol_counts[symbol] += 1
            bucket_counts[bucket(symbol)] += 1
        idx += 1

    dominant_symbol, dominant_symbol_count = ("missing", 0)
    if symbol_counts:
        dominant_symbol, dominant_symbol_count = sorted(
            symbol_counts.items(), key=lambda item: (-item[1], item[0])
        )[0]

    return {
        "name": name,
        "env_count": env_count,
        "env_total_bytes": env_total_bytes,
        "env_max_value_bytes": env_max_value_bytes,
        "dynamic_loader_pct": data.get("startup_loader_dynamic_loader_pct", "0"),
        "libc_process_pct": data.get("startup_loader_libc_process_pct", "0"),
        "dominant_symbol": dominant_symbol,
        "dominant_symbol_count": dominant_symbol_count,
        "allocator": bucket_counts["allocator"],
        "env_api": bucket_counts["env_api"],
        "env_string_scan": bucket_counts["env_string_scan"],
        "memory_string": bucket_counts["memory_string"],
        "other": bucket_counts["other"],
        "symbols": symbol_counts,
    }


paths = {
    "inherited_env": Path(sys.argv[1]),
    "env_i_required": Path(sys.argv[2]),
    "env_i_many_dummy": Path(sys.argv[3]),
}
required_total_bytes = int(sys.argv[4])
required_max_value_bytes = int(sys.argv[5])
many_dummy_total_bytes = int(sys.argv[6])
many_dummy_max_value_bytes = int(sys.argv[7])
variants = [
    parse_variant("inherited_env", paths["inherited_env"], -1, -1, -1),
    parse_variant("env_i_required", paths["env_i_required"], 3, required_total_bytes, required_max_value_bytes),
    parse_variant("env_i_many_dummy", paths["env_i_many_dummy"], 203, many_dummy_total_bytes, many_dummy_max_value_bytes),
]

if len(variants) < 3:
    raise SystemExit(1)
for variant in variants:
    total = (
        int(variant["allocator"])
        + int(variant["env_api"])
        + int(variant["env_string_scan"])
        + int(variant["memory_string"])
        + int(variant["other"])
    )
    if total < 1:
        raise SystemExit(1)

inherited = variants[0]
required = variants[1]
many = variants[2]

print("output_contract=perf-userbox-startup-executable-ret0-bucket-libc-process-env-ladder-v0")
print("input_contract=perf-userbox-startup-loader-owner-split-v0")
print("startup_probe=libc_process_env_ladder")
print("startup_probe_diagnostic_only=1")
print("default_behavior_changed=0")
print(f"env_variant_count={len(variants)}")

for variant in variants:
    prefix = f"env_ladder_{variant['name']}"
    print(f"{prefix}_env_var_count={variant['env_count']}")
    print(f"{prefix}_env_total_bytes={variant['env_total_bytes']}")
    print(f"{prefix}_env_max_value_bytes={variant['env_max_value_bytes']}")
    print(f"{prefix}_dynamic_loader_pct={variant['dynamic_loader_pct']}")
    print(f"{prefix}_libc_process_pct={variant['libc_process_pct']}")
    print(f"{prefix}_libc_process_dominant_symbol={variant['dominant_symbol']}")
    print(f"{prefix}_libc_process_dominant_symbol_count={variant['dominant_symbol_count']}")
    print(f"{prefix}_libc_process_bucket_allocator_count={variant['allocator']}")
    print(f"{prefix}_libc_process_bucket_env_api_count={variant['env_api']}")
    print(f"{prefix}_libc_process_bucket_env_string_scan_count={variant['env_string_scan']}")
    print(f"{prefix}_libc_process_bucket_memory_string_count={variant['memory_string']}")
    print(f"{prefix}_libc_process_bucket_other_count={variant['other']}")
    for symbol, count in sorted(variant["symbols"].items(), key=lambda item: (-item[1], item[0])):
        print(f"{prefix}_libc_process_symbol_count[{symbol}]={count}")

for bucket_name in ("allocator", "env_api", "env_string_scan", "memory_string"):
    print(
        f"delta_required_vs_inherited_{bucket_name}_count="
        f"{int(required[bucket_name]) - int(inherited[bucket_name])}"
    )
    print(
        f"delta_many_dummy_vs_required_{bucket_name}_count="
        f"{int(many[bucket_name]) - int(required[bucket_name])}"
    )

print("all_variants_run_status=ok")
print("libc_process_bucket_counts_present=1")
print("summary=ok")
PY
