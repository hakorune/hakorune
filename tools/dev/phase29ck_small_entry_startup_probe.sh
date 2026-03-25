#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/perf/lib/aot_helpers.sh"
require_env >/dev/null || exit 2

HAKO_BIN="$ROOT/target/release/hakorune"
if [ ! -x "$HAKO_BIN" ]; then
  echo "[FAIL] phase29ck_small_entry_startup_probe: missing release hakorune: $HAKO_BIN" >&2
  exit 1
fi

TMPDIR_PROBE="${TMPDIR:-/tmp}/phase29ck_small_entry_startup_probe_$$"
mkdir -p "$TMPDIR_PROBE"

cleanup() {
  rm -rf "$TMPDIR_PROBE" >/dev/null 2>&1 || true
}
trap cleanup EXIT

assert_contains() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[FAIL] phase29ck_small_entry_startup_probe: missing pattern '$pattern' in $file" >&2
    exit 1
  fi
}

assert_not_contains() {
  local file="$1"
  local pattern="$2"
  if grep -Fq "$pattern" "$file"; then
    echo "[FAIL] phase29ck_small_entry_startup_probe: unexpected pattern '$pattern' in $file" >&2
    exit 1
  fi
}

run_small_entry_case() {
  local key="$1"
  local expected_add="$2"
  shift 2
  local forbid_patterns=("$@")
  local ir_path="$TMPDIR_PROBE/${key}.ll"
  local out_log="$TMPDIR_PROBE/${key}.bench.log"

  set +e
  NYASH_LLVM_DUMP_IR="$ir_path" \
  PERF_AOT=1 \
  NYASH_LLVM_SKIP_BUILD=1 \
  NYASH_LLVM_BACKEND=crate \
  NYASH_LLVM_USE_HARNESS=0 \
  HAKO_LLVM_EMIT_PROVIDER= \
  bash "$ROOT/tools/perf/bench_compare_c_vs_hako.sh" "$key" 1 1 >"$out_log" 2>&1
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "[FAIL] phase29ck_small_entry_startup_probe: bench_compare failed for $key rc=$rc" >&2
    sed -n '1,80p' "$out_log" >&2
    exit 1
  fi

  if ! grep -Eq "\[bench\] name=${key}[[:space:]]*\(aot\).* status=ok( |$)" "$out_log"; then
    echo "[FAIL] phase29ck_small_entry_startup_probe: missing green AOT bench line for $key" >&2
    sed -n "1,40p" "$out_log" >&2
    exit 1
  fi
  assert_contains "$ir_path" "define i64 @\"ny_main\"()"
  assert_contains "$ir_path" "add i64 %acc.cur, $expected_add"
  for pattern in "${forbid_patterns[@]}"; do
    assert_not_contains "$ir_path" "$pattern"
  done
}

run_small_entry_case \
  "method_call_only_small" \
  "5" \
  "nyash.string.len_h" \
  "nyrt_string_length" \
  "nyash.any.length_h"

run_small_entry_case \
  "box_create_destroy_small" \
  "1" \
  "nyash.box.from_i8_string" \
  "nyash.string.len_h" \
  "nyrt_string_length"

EXE_PATH="$TMPDIR_PROBE/method_call_only_small.exe"
if ! NYASH_LLVM_BACKEND=crate \
  NYASH_LLVM_USE_HARNESS=0 \
  HAKO_LLVM_EMIT_PROVIDER= \
  perf_emit_and_build_aot_exe "$ROOT" "$HAKO_BIN" "$ROOT/benchmarks/bench_method_call_only_small.hako" "$EXE_PATH"; then
  echo "[FAIL] phase29ck_small_entry_startup_probe: failed to build startup probe exe status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi

FILE_OUT="$TMPDIR_PROBE/method_call_only_small.file.txt"
LDD_OUT="$TMPDIR_PROBE/method_call_only_small.ldd.txt"
READELF_D_OUT="$TMPDIR_PROBE/method_call_only_small.dynamic.txt"
READELF_R_OUT="$TMPDIR_PROBE/method_call_only_small.reloc.txt"

file "$EXE_PATH" >"$FILE_OUT"
ldd "$EXE_PATH" >"$LDD_OUT"
readelf -d "$EXE_PATH" >"$READELF_D_OUT"
readelf -r "$EXE_PATH" >"$READELF_R_OUT"

assert_contains "$FILE_OUT" "dynamically linked"
assert_contains "$FILE_OUT" "not stripped"
assert_contains "$LDD_OUT" "libm.so.6"
assert_contains "$LDD_OUT" "libgcc_s.so.1"
assert_contains "$LDD_OUT" "libc.so.6"
assert_contains "$READELF_D_OUT" "(NEEDED)"
assert_contains "$READELF_R_OUT" ".rela.dyn"
assert_contains "$READELF_R_OUT" ".rela.plt"

rela_dyn_count="$(grep -c 'R_X86_64_' "$READELF_R_OUT" | tr -d ' ')"
if [ "${rela_dyn_count}" -lt 100 ]; then
  echo "[FAIL] phase29ck_small_entry_startup_probe: relocation count unexpectedly small: ${rela_dyn_count}" >&2
  exit 1
fi

echo "[PASS] phase29ck_small_entry_startup_probe"
