#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_libc_process_callgraph_attribution.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

STARTUP_RUNS=120
TARGET_DIR="$ROOT_DIR/target"
HAKORUNE_BIN="$TARGET_DIR/release/hakorune"
if [ ! -x "$HAKORUNE_BIN" ]; then
  echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-callgraph-attribution] missing release hakorune: $HAKORUNE_BIN" >&2
  echo "[hint] run: cargo build --release --bin hakorune" >&2
  exit 2
fi
if ! command -v perf >/dev/null 2>&1; then
  echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-callgraph-attribution] perf is required" >&2
  exit 2
fi
if ! command -v "${CC:-cc}" >/dev/null 2>&1; then
  echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-callgraph-attribution] C compiler is required" >&2
  exit 2
fi

source "$ROOT_DIR/tools/perf/lib/aot_helpers.sh"

export NYASH_LLVM_USE_HARNESS=0
export NYASH_LLVM_LINK_WHOLE_ARCHIVE="${NYASH_LLVM_LINK_WHOLE_ARCHIVE:-0}"
export NYASH_LLVM_LINK_GC_SECTIONS="${NYASH_LLVM_LINK_GC_SECTIONS:-1}"

ret0_exe="$TMP_DIR/ret0.exe"
runner_c="$TMP_DIR/startup_runner.c"
runner_bin="$TMP_DIR/startup_runner.bin"

if ! perf_build_ret0_aot_exe "$ROOT_DIR" "$HAKORUNE_BIN" "$ret0_exe"; then
  echo "[perf-userbox-startup-executable-ret0-bucket-libc-process-callgraph-attribution] ret0 exact-AOT build failed" >&2
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

PATH_REQUIRED="${PATH:-/usr/bin:/bin}"
TMPDIR_KEEP="${TMPDIR:-/tmp}"

run_callgraph_variant() {
  local variant="$1"
  local data_file="$TMP_DIR/${variant}.perf.data"
  local script_file="$TMP_DIR/${variant}.perf.script"
  shift
  "$@" perf record -o "$data_file" -F 999 --call-graph dwarf,4096 -- "$runner_bin" "$STARTUP_RUNS" "$ret0_exe" >/dev/null 2>&1 || true
  perf script -i "$data_file" >"$script_file" 2>/dev/null || true
  printf '%s=%s\n' "$variant" "$script_file"
}

variant_map="$TMP_DIR/variants.txt"
: >"$variant_map"

run_callgraph_variant "inherited_env" env >>"$variant_map"
run_callgraph_variant "env_i_required" env -i PATH="$PATH_REQUIRED" TMPDIR="$TMPDIR_KEEP" LC_ALL=C >>"$variant_map"

dummy_env=(env -i PATH="$PATH_REQUIRED" TMPDIR="$TMPDIR_KEEP" LC_ALL=C)
for i in $(seq -w 0 199); do
  dummy_env+=("HAKO_DUMMY_$i=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
done
run_callgraph_variant "env_i_many_dummy" "${dummy_env[@]}" >>"$variant_map"

python3 - "$variant_map" <<'PY'
from __future__ import annotations

import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

TARGETS = (
    "getenv",
    "__strlen_evex",
    "__strncmp_evex",
    "__memmove_avx512_unaligned_erms",
    "_int_malloc",
)


def classify_owner(frames: list[str]) -> tuple[str, str]:
    for frame in frames:
        lowered = frame.lower()
        if "hako_llvmc" in lowered or "hako_aot" in lowered or "nyash_plugin" in lowered:
            return "cabi_shim", frame
        if (
            "nyash_rust" in lowered
            or "nyash_kernel" in lowered
            or "nyrt" in lowered
            or "std::env" in lowered
            or "std::path" in lowered
        ):
            return "nyrt_entry", frame
    for frame in frames:
        lowered = frame.lower()
        if (
            "ld-linux" in lowered
            or "ld.so" in lowered
            or "__libc_start" in lowered
            or "libc.so" in lowered
            or frame.startswith("_dl_")
        ):
            return "loader_libc", frame
    if frames:
        return "unknown", frames[0]
    return "unknown", "missing"


def parse_stacks(path: Path) -> list[list[str]]:
    stacks: list[list[str]] = []
    current: list[str] = []
    frame_re = re.compile(r"^\s+[0-9a-fA-F]+\s+(.+?)\s*(?:\(|$)")
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line.strip():
            if current:
                stacks.append(current)
                current = []
            continue
        match = frame_re.match(line)
        if match:
            symbol = match.group(1).strip()
            if symbol:
                current.append(symbol)
    if current:
        stacks.append(current)
    return stacks


variant_paths: dict[str, Path] = {}
for line in Path(sys.argv[1]).read_text().splitlines():
    if "=" not in line:
        continue
    name, value = line.split("=", 1)
    variant_paths[name.strip()] = Path(value.strip())

if len(variant_paths) != 3:
    raise SystemExit(1)

total_owner_counts = Counter()
variant_target_counts: dict[str, Counter[str]] = {}
variant_owner_counts: dict[str, Counter[str]] = {}
top_owner: dict[tuple[str, str], tuple[str, str, int]] = {}
callgraph_samples = 0
target_samples = 0

for variant, path in variant_paths.items():
    target_counts = Counter()
    owner_counts = Counter()
    owner_by_target: dict[str, Counter[tuple[str, str]]] = defaultdict(Counter)
    stacks = parse_stacks(path)
    callgraph_samples += len(stacks)
    for stack in stacks:
        for idx, frame in enumerate(stack):
            matched = next((target for target in TARGETS if target in frame), None)
            if not matched:
                continue
            target_samples += 1
            target_counts[matched] += 1
            owner, owner_frame = classify_owner(stack[idx + 1 : idx + 9])
            owner_counts[owner] += 1
            total_owner_counts[owner] += 1
            owner_by_target[matched][(owner, owner_frame)] += 1
    variant_target_counts[variant] = target_counts
    variant_owner_counts[variant] = owner_counts
    for target in TARGETS:
        if owner_by_target[target]:
            (owner, owner_frame), count = sorted(
                owner_by_target[target].items(), key=lambda item: (-item[1], item[0][0], item[0][1])
            )[0]
            top_owner[(variant, target)] = (owner, owner_frame, count)

if callgraph_samples < 1 or target_samples < 1:
    raise SystemExit(1)

print("output_contract=perf-userbox-startup-executable-ret0-bucket-libc-process-callgraph-attribution-v0")
print("input_contract=perf-userbox-startup-loader-owner-split-v0")
print("startup_probe=libc_process_callgraph_attribution")
print("startup_probe_diagnostic_only=1")
print("default_behavior_changed=0")
print(f"env_variant_count={len(variant_paths)}")
print(f"callgraph_sample_count={callgraph_samples}")
print(f"callgraph_target_sample_count={target_samples}")
print("callgraph_attribution_available=1")

for variant in ("inherited_env", "env_i_required", "env_i_many_dummy"):
    print(f"env_variant[{variant}]=1")
    for target in TARGETS:
        print(f"{variant}_callgraph_symbol_count[{target}]={variant_target_counts[variant][target]}")
        owner, frame, count = top_owner.get((variant, target), ("missing", "missing", 0))
        print(f"{variant}_callgraph_top_owner[{target}]={owner}")
        print(f"{variant}_callgraph_top_frame[{target}]={frame.replace(' ', '_')}")
        print(f"{variant}_callgraph_top_owner_count[{target}]={count}")
    for owner in ("nyrt_entry", "cabi_shim", "loader_libc", "unknown"):
        print(f"{variant}_libc_process_owner_{owner}_count={variant_owner_counts[variant][owner]}")

for owner in ("nyrt_entry", "cabi_shim", "loader_libc", "unknown"):
    print(f"libc_process_owner_{owner}_count={total_owner_counts[owner]}")
print("summary=ok")
PY
