#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_FILE=""
STARTUP_RUNS=20

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/nyrt_startup_floor_bare_entry_ab_probe.sh --out FILE [--startup-runs N]

Builds one ret0 ny_main object, links it with:
  A. current minimal NyRT entry
  B. bare libc main
and reports the startup-floor delta. This is diagnostic-only evidence; it does
not change the default NyRT entry or product link path.
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
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[nyrt-startup-floor-ab] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$OUT_FILE" ]; then
  echo "[nyrt-startup-floor-ab] ERROR: --out is required" >&2
  usage
  exit 2
fi
case "$STARTUP_RUNS" in
  ''|*[!0-9]*)
    echo "[nyrt-startup-floor-ab] ERROR: --startup-runs must be an unsigned integer" >&2
    exit 2
    ;;
esac
if [ "$STARTUP_RUNS" -lt 1 ]; then
  echo "[nyrt-startup-floor-ab] ERROR: --startup-runs must be >= 1" >&2
  exit 2
fi

for cmd in perf sha256sum nm "${CC:-cc}"; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "[nyrt-startup-floor-ab] ERROR: missing command: $cmd" >&2
    exit 2
  fi
done

TARGET_DIR="$ROOT_DIR/target"
HAKORUNE_BIN="$TARGET_DIR/release/hakorune"
NYRT_ARCHIVE="$TARGET_DIR/release/libnyash_kernel.a"
if [ ! -x "$HAKORUNE_BIN" ]; then
  echo "[nyrt-startup-floor-ab] ERROR: missing release hakorune: $HAKORUNE_BIN" >&2
  echo "[hint] run: cargo build --release --bin hakorune" >&2
  exit 2
fi
if [ ! -f "$NYRT_ARCHIVE" ]; then
  echo "[nyrt-startup-floor-ab] ERROR: missing NyRT archive: $NYRT_ARCHIVE" >&2
  echo "[hint] run: cargo build --release -p nyash_kernel" >&2
  exit 2
fi

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_startup_floor_ab.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

ny_main_obj="$tmp_dir/ret0_ny_main.o"
nyrt_exe="$tmp_dir/current_minimal_nyrt.exe"
bare_main_c="$tmp_dir/bare_main.c"
bare_exe="$tmp_dir/bare_entry.exe"
runner_c="$tmp_dir/startup_runner.c"
runner_bin="$tmp_dir/startup_runner.bin"
nyrt_stat="$tmp_dir/current_minimal_nyrt.perf.stat"
bare_stat="$tmp_dir/bare_entry.perf.stat"
nyrt_data="$tmp_dir/current_minimal_nyrt.perf.data"
bare_data="$tmp_dir/bare_entry.perf.data"
nyrt_report="$tmp_dir/current_minimal_nyrt.perf.report"
bare_report="$tmp_dir/bare_entry.perf.report"

source "$ROOT_DIR/tools/perf/lib/aot_helpers.sh"

export NYASH_LLVM_USE_HARNESS=0
export NYASH_LLVM_LINK_WHOLE_ARCHIVE="${NYASH_LLVM_LINK_WHOLE_ARCHIVE:-0}"
export NYASH_LLVM_LINK_GC_SECTIONS="${NYASH_LLVM_LINK_GC_SECTIONS:-1}"
export NYASH_LLVM_LINK_SYSTEM_LIBS="${NYASH_LLVM_LINK_SYSTEM_LIBS:-minimal}"

if ! perf_build_ret0_aot_obj "$ROOT_DIR" "$HAKORUNE_BIN" "$ny_main_obj"; then
  echo "[nyrt-startup-floor-ab] ERROR: ret0 exact-AOT object build failed" >&2
  exit 1
fi
if ! nm -g --defined-only "$ny_main_obj" | awk '$3 == "ny_main" { found = 1 } END { exit(found ? 0 : 1) }'; then
  echo "[nyrt-startup-floor-ab] ERROR: ret0 object does not define ny_main" >&2
  exit 1
fi

cat >"$bare_main_c" <<'EOF'
#include <stdint.h>
extern int64_t ny_main(void);
int main(void) {
  return (int)ny_main();
}
EOF

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

"${CC:-cc}" -O2 -no-pie -o "$nyrt_exe" "$ny_main_obj" "$NYRT_ARCHIVE" -Wl,--gc-sections -ldl -lpthread
"${CC:-cc}" -O2 -no-pie -o "$bare_exe" "$bare_main_c" "$ny_main_obj"

run_entry_probe() {
  local exe=$1
  local stat_file=$2
  local data_file=$3
  local report_file=$4

  env \
    NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
    NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
    HAKO_NYRT_PLUGIN_HOST="${HAKO_NYRT_PLUGIN_HOST:-off}" \
    NYASH_NYRT_RUNTIME_HOOKS="${NYASH_NYRT_RUNTIME_HOOKS:-off}" \
    NYASH_NYRT_RUNTIME_BUILD="${NYASH_NYRT_RUNTIME_BUILD:-auto}" \
    NYASH_NYRT_MINIMAL_STARTUP="${NYASH_NYRT_MINIMAL_STARTUP:-1}" \
    NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
    NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
    NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}" \
    perf stat -x, -e cycles,instructions -o "$stat_file" -- "$runner_bin" "$STARTUP_RUNS" "$exe" >/dev/null 2>&1 || true

  env \
    NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
    NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
    HAKO_NYRT_PLUGIN_HOST="${HAKO_NYRT_PLUGIN_HOST:-off}" \
    NYASH_NYRT_RUNTIME_HOOKS="${NYASH_NYRT_RUNTIME_HOOKS:-off}" \
    NYASH_NYRT_RUNTIME_BUILD="${NYASH_NYRT_RUNTIME_BUILD:-auto}" \
    NYASH_NYRT_MINIMAL_STARTUP="${NYASH_NYRT_MINIMAL_STARTUP:-1}" \
    NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
    NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
    NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}" \
    perf record -o "$data_file" -F 999 -- "$runner_bin" "$STARTUP_RUNS" "$exe" >/dev/null 2>&1 || true

  perf report --stdio --no-children -i "$data_file" >"$report_file" 2>/dev/null || true
}

run_entry_probe "$nyrt_exe" "$nyrt_stat" "$nyrt_data" "$nyrt_report"
run_entry_probe "$bare_exe" "$bare_stat" "$bare_data" "$bare_report"

python3 - "$OUT_FILE" "$STARTUP_RUNS" "$ny_main_obj" "$nyrt_stat" "$bare_stat" "$nyrt_report" "$bare_report" <<'PY'
from __future__ import annotations

import os
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

out_path = Path(sys.argv[1])
startup_runs = sys.argv[2]
ny_main_obj = Path(sys.argv[3])
nyrt_stat = Path(sys.argv[4])
bare_stat = Path(sys.argv[5])
nyrt_report = Path(sys.argv[6])
bare_report = Path(sys.argv[7])


def sha256(path: Path) -> str:
    return subprocess.check_output(["sha256sum", str(path)], text=True).split()[0]


def parse_stat(path: Path) -> dict[str, int | None]:
    values: dict[str, int | None] = {"cycles": None, "instructions": None}
    if not path.exists():
        return values
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#"):
            continue
        parts = line.split(",")
        if len(parts) < 3:
            continue
        raw_value = parts[0].strip()
        event = parts[2].strip().split(":", 1)[0]
        if event not in values:
            continue
        if raw_value in {"<not counted>", "<not supported>", ""}:
            values[event] = None
            continue
        try:
            values[event] = int(float(raw_value.replace(",", "")))
        except ValueError:
            values[event] = None
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
    if "bare_entry.exe" in text or "current_minimal_nyrt.exe" in text or symbol in {"main", "ny_main"}:
        return "minimal_main"
    if "kernel.kallsyms" in text or "[kernel" in text:
        return "kernel"
    return "other"


row_re = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+(\S+)\s+\[[^]]+\]\s+(.+?)\s*$")


def parse_report(path: Path) -> tuple[list[tuple[float, str, str, str]], dict[str, float]]:
    rows: list[tuple[float, str, str, str]] = []
    category_pct: dict[str, float] = defaultdict(float)
    if not path.exists():
        return rows, category_pct
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = row_re.match(line)
        if not match:
            continue
        pct = float(match.group(1))
        dso = match.group(2)
        symbol = match.group(3).strip()
        category = classify(dso, symbol)
        rows.append((pct, dso, symbol, category))
        category_pct[category] += pct
    return rows, category_pct


def primary_family(category_pct: dict[str, float]) -> str:
    if not category_pct:
        return "missing"
    return max(category_pct.items(), key=lambda item: item[1])[0]


nyrt_stats = parse_stat(nyrt_stat)
bare_stats = parse_stat(bare_stat)
nyrt_rows, nyrt_category_pct = parse_report(nyrt_report)
bare_rows, bare_category_pct = parse_report(bare_report)

nyrt_cycles = nyrt_stats["cycles"]
bare_cycles = bare_stats["cycles"]
entry_delta_cycles = None
entry_delta_ratio = None
if nyrt_cycles is not None and bare_cycles is not None:
    entry_delta_cycles = nyrt_cycles - bare_cycles
    entry_delta_ratio = (nyrt_cycles / bare_cycles) if bare_cycles else None

obj_hash = sha256(ny_main_obj)
lines = [
    "output_contract=nyrt-startup-floor-bare-entry-ab-v0",
    "startup_floor_probe=bare_entry_ab",
    "measurement_scope=nyrt_entry_startup_floor",
    "observation_only=1",
    "rewrite_executed=0",
    "default_behavior_changed=0",
    "same_ny_main_object=1",
    f"ny_main_object_sha256={obj_hash}",
    "entry_a=current_minimal_nyrt",
    "entry_b=bare_libc_main",
    f"startup_runs={startup_runs}",
    f"runtime_build_mode={os.environ.get('NYASH_NYRT_RUNTIME_BUILD', 'auto').strip().lower() or 'auto'}",
    "current_minimal_run_status=ok",
    "bare_entry_run_status=ok",
    f"current_minimal_cycles={nyrt_cycles if nyrt_cycles is not None else 'missing'}",
    f"bare_entry_cycles={bare_cycles if bare_cycles is not None else 'missing'}",
    f"entry_delta_cycles={entry_delta_cycles if entry_delta_cycles is not None else 'missing'}",
    f"entry_delta_ratio={entry_delta_ratio:.6f}" if entry_delta_ratio is not None else "entry_delta_ratio=missing",
    f"current_minimal_instructions={nyrt_stats['instructions'] if nyrt_stats['instructions'] is not None else 'missing'}",
    f"bare_entry_instructions={bare_stats['instructions'] if bare_stats['instructions'] is not None else 'missing'}",
    f"perf_top_symbols_reported={1 if nyrt_rows and bare_rows else 0}",
    f"current_minimal_perf_top_available={1 if nyrt_rows else 0}",
    f"bare_entry_perf_top_available={1 if bare_rows else 0}",
    f"current_minimal_primary_owner_family={primary_family(nyrt_category_pct)}",
    f"bare_entry_primary_owner_family={primary_family(bare_category_pct)}",
]

for prefix, category_pct in (
    ("current_minimal", nyrt_category_pct),
    ("bare_entry", bare_category_pct),
):
    for category in (
        "dynamic_loader",
        "process_spawn_wait",
        "libc_process",
        "nyash_kernel_runtime",
        "minimal_main",
        "kernel",
        "other",
    ):
        lines.append(f"{prefix}_{category}_pct={category_pct.get(category, 0.0):.2f}")

for prefix, rows in (("current_minimal", nyrt_rows), ("bare_entry", bare_rows)):
    for idx, (pct, dso, symbol, category) in enumerate(rows[:10]):
        safe_symbol = symbol.replace(" ", "_")
        lines.append(f"{prefix}_top_{idx}_pct={pct:.2f}")
        lines.append(f"{prefix}_top_{idx}_dso={dso}")
        lines.append(f"{prefix}_top_{idx}_symbol={safe_symbol}")
        lines.append(f"{prefix}_top_{idx}_family={category}")

summary_ok = (
    nyrt_cycles is not None
    and bare_cycles is not None
    and bool(nyrt_rows)
    and bool(bare_rows)
)
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
