#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_FILE=""
STARTUP_RUNS=20
LANE_WARMUP=0
LANE_REPEAT=1
LANE_KERNEL_INNER_RUNS=3

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/userbox_startup_loader_owner_split.sh --out FILE [--startup-runs N]

Builds a minimal ret0 exact-AOT executable, profiles repeated startup through a
small spawn runner, and joins that top-symbol evidence with the userbox
floor/helper startup-loader attribution report.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out)
      OUT_FILE="${2:-}"
      shift 2
      ;;
    --startup-runs)
      STARTUP_RUNS="${2:-}"
      shift 2
      ;;
    --lane-warmup)
      LANE_WARMUP="${2:-}"
      shift 2
      ;;
    --lane-repeat)
      LANE_REPEAT="${2:-}"
      shift 2
      ;;
    --lane-kernel-inner-runs)
      LANE_KERNEL_INNER_RUNS="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[userbox-startup-loader-owner-split] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$OUT_FILE" ]; then
  echo "[userbox-startup-loader-owner-split] ERROR: --out is required" >&2
  usage
  exit 2
fi

for value in "$STARTUP_RUNS" "$LANE_WARMUP" "$LANE_REPEAT" "$LANE_KERNEL_INNER_RUNS"; do
  case "$value" in
    ''|*[!0-9]*)
      echo "[userbox-startup-loader-owner-split] ERROR: numeric arguments must be unsigned integers" >&2
      exit 2
      ;;
  esac
done
if [ "$STARTUP_RUNS" -lt 1 ] || [ "$LANE_REPEAT" -lt 1 ] || [ "$LANE_KERNEL_INNER_RUNS" -lt 1 ]; then
  echo "[userbox-startup-loader-owner-split] ERROR: runs/repeat/kernel-inner-runs must be >= 1" >&2
  exit 2
fi

if ! command -v perf >/dev/null 2>&1; then
  echo "[userbox-startup-loader-owner-split] ERROR: perf is required" >&2
  exit 2
fi
if ! command -v "${CC:-cc}" >/dev/null 2>&1; then
  echo "[userbox-startup-loader-owner-split] ERROR: C compiler is required" >&2
  exit 2
fi

TARGET_DIR="$ROOT_DIR/target"
HAKORUNE_BIN="$TARGET_DIR/release/hakorune"
if [ ! -x "$HAKORUNE_BIN" ]; then
  echo "[userbox-startup-loader-owner-split] ERROR: missing release hakorune: $HAKORUNE_BIN" >&2
  echo "[hint] run: cargo build --release --bin hakorune" >&2
  exit 2
fi

tmp_dir="$(mktemp -d /tmp/hakorune_userbox_startup_owner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

ret0_exe="$tmp_dir/ret0.exe"
runner_c="$tmp_dir/startup_runner.c"
runner_bin="$tmp_dir/startup_runner.bin"
perf_data="$tmp_dir/startup.perf.data"
perf_report="$tmp_dir/startup.perf.report"
lanes_report="$tmp_dir/userbox_floor_attribution.out"

source "$ROOT_DIR/tools/perf/lib/aot_helpers.sh"

export NYASH_LLVM_USE_HARNESS=0
export NYASH_LLVM_LINK_WHOLE_ARCHIVE="${NYASH_LLVM_LINK_WHOLE_ARCHIVE:-0}"
export NYASH_LLVM_LINK_GC_SECTIONS="${NYASH_LLVM_LINK_GC_SECTIONS:-1}"

NYASH_NYRT_RUNTIME_HOOKS=auto \
  "$ROOT_DIR/tools/allocator/userbox_direct_helper_floor_attribution.py" \
  --out "$lanes_report" \
  --warmup "$LANE_WARMUP" \
  --repeat "$LANE_REPEAT" \
  --kernel-inner-runs "$LANE_KERNEL_INNER_RUNS" >/dev/null

if ! perf_build_ret0_aot_exe "$ROOT_DIR" "$HAKORUNE_BIN" "$ret0_exe"; then
  echo "[userbox-startup-loader-owner-split] ERROR: ret0 exact-AOT build failed" >&2
  exit 1
fi

cat >"$runner_c" <<'EOF'
#include <errno.h>
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

extern char **environ;

int main(int argc, char **argv) {
  if (argc != 3) {
    fprintf(stderr, "usage: %s <runs> <exe>\n", argv[0]);
    return 2;
  }
  char *end = NULL;
  long runs = strtol(argv[1], &end, 10);
  if (!end || *end != '\0' || runs < 1) {
    fprintf(stderr, "invalid runs: %s\n", argv[1]);
    return 2;
  }
  char *const child_argv[] = { argv[2], NULL };
  for (long i = 0; i < runs; ++i) {
    pid_t pid = 0;
    int rc = posix_spawn(&pid, argv[2], NULL, NULL, child_argv, environ);
    if (rc != 0) {
      fprintf(stderr, "posix_spawn failed: %s\n", strerror(rc));
      return 1;
    }
    int status = 0;
    if (waitpid(pid, &status, 0) < 0) {
      fprintf(stderr, "waitpid failed: %s\n", strerror(errno));
      return 1;
    }
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
      return 1;
    }
  }
  return 0;
}
EOF

"${CC:-cc}" -O2 -std=c11 -Wall -Wextra -o "$runner_bin" "$runner_c"

env \
  NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
  NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
  NYASH_NYRT_RUNTIME_HOOKS="${NYASH_NYRT_RUNTIME_HOOKS:-auto}" \
  NYASH_NYRT_MINIMAL_STARTUP="${NYASH_NYRT_MINIMAL_STARTUP:-0}" \
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}" \
  perf record -o "$perf_data" -F 999 -- "$runner_bin" "$STARTUP_RUNS" "$ret0_exe" >/dev/null 2>&1 || true

perf report --stdio --no-children -i "$perf_data" >"$perf_report" || true

python3 - "$lanes_report" "$perf_report" "$OUT_FILE" "$STARTUP_RUNS" <<'PY'
from __future__ import annotations

import re
import os
import sys
from collections import defaultdict
from pathlib import Path

lanes_path = Path(sys.argv[1])
perf_path = Path(sys.argv[2])
out_path = Path(sys.argv[3])
startup_runs = sys.argv[4]


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def classify(dso: str, symbol: str) -> str:
    text = f"{dso} {symbol}".lower()
    if "ld-linux" in text or "ld.so" in text or symbol.startswith("_dl_"):
        return "dynamic_loader"
    if any(token in text for token in ("posix_spawn", "waitpid", "clone", "fork", "startup_runner")):
        return "process_spawn_wait"
    if "libc" in text or "__libc_start" in text or symbol in {"start", "_start"}:
        return "libc_process"
    if "nyash_kernel" in text or "nyash_rust" in text or "nyrt" in text or "hako_" in text:
        return "nyash_kernel_runtime"
    if "ret0.exe" in text or "app.exe" in text or symbol in {"main", "ny_main"}:
        return "minimal_main"
    if "kernel.kallsyms" in text or "[kernel" in text:
        return "kernel"
    return "other"


row_re = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+(\S+)\s+\[[^]]+\]\s+(.+?)\s*$")
rows: list[tuple[float, str, str, str]] = []
for line in perf_path.read_text(encoding="utf-8", errors="replace").splitlines():
    match = row_re.match(line)
    if not match:
        continue
    pct = float(match.group(1))
    dso = match.group(2)
    symbol = match.group(3).strip()
    category = classify(dso, symbol)
    rows.append((pct, dso, symbol, category))

category_pct: dict[str, float] = defaultdict(float)
for pct, _dso, _symbol, category in rows:
    category_pct[category] += pct

primary = "missing"
if category_pct:
    primary = max(category_pct.items(), key=lambda item: item[1])[0]

lanes = read_kv(lanes_path)
runtime_hooks_mode = os.environ.get("NYASH_NYRT_RUNTIME_HOOKS", "auto").strip().lower() or "auto"
runtime_hooks_init_skipped = "1" if runtime_hooks_mode == "off" else "0"
minimal_startup_mode = os.environ.get("NYASH_NYRT_MINIMAL_STARTUP", "0").strip().lower() or "0"
lines = [
    "output_contract=perf-userbox-startup-loader-owner-split-v0",
    "input_contract=perf-userbox-direct-helper-floor-attribution-v0",
    "measurement_scope=userbox_exact_aot_startup_loader_owner_split",
    f"startup_probe=ret0_exact_aot_spawn_runner",
    f"startup_runs={startup_runs}",
    f"runtime_hooks_mode={runtime_hooks_mode}",
    f"runtime_hooks_init_skipped={runtime_hooks_init_skipped}",
    f"minimal_startup_mode={minimal_startup_mode}",
    f"ret0_perf_top_available={1 if rows else 0}",
    f"ret0_perf_top_row_count={len(rows)}",
    f"startup_loader_primary_owner_family={primary}",
]
for category in (
    "dynamic_loader",
    "process_spawn_wait",
    "libc_process",
    "nyash_kernel_runtime",
    "minimal_main",
    "kernel",
    "other",
):
    lines.append(f"startup_loader_{category}_pct={category_pct.get(category, 0.0):.2f}")

for idx, (pct, dso, symbol, category) in enumerate(rows[:10]):
    safe_symbol = symbol.replace(" ", "_")
    lines.append(f"startup_loader_top_{idx}_pct={pct:.2f}")
    lines.append(f"startup_loader_top_{idx}_dso={dso}")
    lines.append(f"startup_loader_top_{idx}_symbol={safe_symbol}")
    lines.append(f"startup_loader_top_{idx}_family={category}")

ret0_rows = [(pct, dso, symbol, category) for pct, dso, symbol, category in rows if dso == "ret0.exe"]
if ret0_rows:
    ret0_pct, _ret0_dso, ret0_symbol, ret0_family = ret0_rows[0]
    lines.extend(
        [
            f"startup_loader_ret0_exe_top_count={len(ret0_rows)}",
            f"startup_loader_ret0_exe_first_pct={ret0_pct:.2f}",
            f"startup_loader_ret0_exe_first_symbol={ret0_symbol.replace(' ', '_')}",
            f"startup_loader_ret0_exe_first_family={ret0_family}",
        ]
    )
else:
    lines.extend(
        [
            "startup_loader_ret0_exe_top_count=0",
            "startup_loader_ret0_exe_first_pct=missing",
            "startup_loader_ret0_exe_first_symbol=missing",
            "startup_loader_ret0_exe_first_family=missing",
        ]
    )

for key in (
    "floor_run_status",
    "direct_helper_floor_invalid_arraybox_handle_count",
    "counter_step_chain_helper_vs_floor_measured",
    "point_add_helper_vs_floor_measured",
    "startup_loader_attribution_report",
    "measurement_harness_failure_count",
    "counter_step_chain_floor_startup_loader_cycles",
    "counter_step_chain_helper_startup_loader_cycles",
    "point_add_floor_startup_loader_cycles",
    "point_add_helper_startup_loader_cycles",
):
    lines.append(f"attribution_{key}={lanes.get(key, 'missing')}")

summary_ok = bool(rows) and lanes.get("summary") == "ok"
lines.extend(
    [
        "touch_hako_source=0",
        "touch_mirbuilder=0",
        "touch_route_planner=0",
        "touch_exact_helper_lowering=0",
        "touch_runtime_object_representation=0",
        f"summary={'ok' if summary_ok else 'fail'}",
    ]
)
out_path.parent.mkdir(parents=True, exist_ok=True)
out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
print("\n".join(lines))
sys.exit(0 if summary_ok else 1)
PY
