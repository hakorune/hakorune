#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env >/dev/null || exit 2

HAKO_BIN="$ROOT/target/release/hakorune"
NY_LLVM_C="$ROOT/target/release/ny-llvmc"
KERNEL_A="$ROOT/target/release/libnyash_kernel.a"

for path in "$HAKO_BIN" "$NY_LLVM_C" "$KERNEL_A"; do
  if [ ! -e "$path" ]; then
    echo "[FAIL] phase29ck_small_entry_gc_sections_experiment: missing dependency: $path" >&2
    exit 1
  fi
done

TMPDIR_PROBE="${TMPDIR:-/tmp}/phase29ck_small_entry_gc_sections_$$"
mkdir -p "$TMPDIR_PROBE"
cleanup() {
  rm -rf "$TMPDIR_PROBE" >/dev/null 2>&1 || true
}
trap cleanup EXIT

JSON="$TMPDIR_PROBE/method_call_only_small.json"
OBJ="$TMPDIR_PROBE/method_call_only_small.o"
EXE_BASE="$TMPDIR_PROBE/method_call_only_small.base.exe"
EXE_GC="$TMPDIR_PROBE/method_call_only_small.gc.exe"

"$HAKO_BIN" --emit-mir-json "$JSON" "$ROOT/benchmarks/bench_method_call_only_small.hako" >/dev/null 2>&1
"$NY_LLVM_C" --in "$JSON" --emit obj --out "$OBJ" >/dev/null 2>&1

cc -o "$EXE_BASE" "$OBJ" -no-pie -Wl,--whole-archive "$KERNEL_A" -Wl,--no-whole-archive -ldl -lpthread -lm
cc -o "$EXE_GC" "$OBJ" -no-pie -Wl,--gc-sections -Wl,--whole-archive "$KERNEL_A" -Wl,--no-whole-archive -ldl -lpthread -lm

base_size="$(stat -c '%s' "$EXE_BASE")"
gc_size="$(stat -c '%s' "$EXE_GC")"
base_relocs="$(readelf -r "$EXE_BASE" | grep -c 'R_X86_64_' | tr -d ' ')"
gc_relocs="$(readelf -r "$EXE_GC" | grep -c 'R_X86_64_' | tr -d ' ')"

if [ "$gc_size" -ge "$base_size" ]; then
  echo "[FAIL] phase29ck_small_entry_gc_sections_experiment: gc-sections did not reduce exe size (${base_size} -> ${gc_size})" >&2
  exit 1
fi

if [ "$gc_relocs" -ge "$base_relocs" ]; then
  echo "[FAIL] phase29ck_small_entry_gc_sections_experiment: gc-sections did not reduce relocation count (${base_relocs} -> ${gc_relocs})" >&2
  exit 1
fi

echo "[PASS] phase29ck_small_entry_gc_sections_experiment: size ${base_size} -> ${gc_size}, reloc ${base_relocs} -> ${gc_relocs}"
