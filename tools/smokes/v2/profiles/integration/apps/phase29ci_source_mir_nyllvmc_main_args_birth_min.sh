#!/bin/bash
# Phase 29ci P26: direct source -> MIR(JSON) -> ny-llvmc pure-first
# entry-args birth shape pin.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29ci_source_mir_nyllvmc_main_args_birth_min"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"
NYLLVMC="${NYASH_NY_LLVM_COMPILER:-$ROOT_DIR/target/release/ny-llvmc}"
NYRT_DIR="${NYASH_EMIT_EXE_NYRT:-$ROOT_DIR/target/release}"
EMIT_ROUTE="$ROOT_DIR/tools/smokes/v2/lib/emit_mir_route.sh"
DIALECT_PROBE="$ROOT_DIR/tools/dev/phase29ck_stage1_mir_dialect_probe.sh"

TMP_DIR="$(mktemp -d /tmp/phase29ci_p26.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR" 2>/dev/null || true
}
trap cleanup EXIT

SRC="$TMP_DIR/main_args_return7.hako"
MIR_JSON="$TMP_DIR/main_args_return7.mir.json"
OUT_EXE="$TMP_DIR/main_args_return7.exe"
EMIT_LOG="$TMP_DIR/emit.log"
BUILD_LOG="$TMP_DIR/build.log"
RUN_LOG="$TMP_DIR/run.log"
FFI_BUILD_LOG="$TMP_DIR/ffi_build.log"

cat >"$SRC" <<'HAKO'
static box Main {
  method main(args) {
    return 7
  }
}
HAKO

if [ ! -x "$NYLLVMC" ]; then
  test_fail "$SMOKE_NAME: ny-llvmc missing: $NYLLVMC"
  exit 1
fi

if ! bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >"$FFI_BUILD_LOG" 2>&1; then
  tail -n 120 "$FFI_BUILD_LOG" >&2 || true
  test_fail "$SMOKE_NAME: FFI build failed"
  exit 1
fi

if ! bash "$EMIT_ROUTE" --route direct --timeout-secs "$RUN_TIMEOUT_SECS" --out "$MIR_JSON" --input "$SRC" >"$EMIT_LOG" 2>&1; then
  tail -n 120 "$EMIT_LOG" >&2 || true
  test_fail "$SMOKE_NAME: direct MIR emit failed"
  exit 1
fi

if ! grep -q '"functions"' "$MIR_JSON"; then
  test_fail "$SMOKE_NAME: MIR JSON functions missing"
  exit 1
fi
if grep -q '"functions_0"' "$MIR_JSON"; then
  test_fail "$SMOKE_NAME: collapsed functions_0 leaked into MIR JSON"
  exit 1
fi

if ! bash "$DIALECT_PROBE" --mir-json "$MIR_JSON" --strict-stage1 >"$TMP_DIR/dialect.log" 2>&1; then
  tail -n 120 "$TMP_DIR/dialect.log" >&2 || true
  test_fail "$SMOKE_NAME: strict Stage1 MIR dialect probe failed"
  exit 1
fi

set +e
NYASH_LLVM_ROUTE_TRACE=1 \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NYLLVMC" --in "$MIR_JSON" --emit exe --nyrt "$NYRT_DIR" --out "$OUT_EXE" \
  >"$TMP_DIR/nyllvmc.stdout" 2>"$BUILD_LOG"
BUILD_RC=$?
set -e

if [ "$BUILD_RC" -eq 124 ]; then
  test_fail "$SMOKE_NAME: ny-llvmc build timed out"
  exit 1
fi
if [ "$BUILD_RC" -ne 0 ]; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  test_fail "$SMOKE_NAME: ny-llvmc pure-first build failed (rc=$BUILD_RC)"
  exit 1
fi
if [ ! -x "$OUT_EXE" ]; then
  test_fail "$SMOKE_NAME: linked executable missing"
  exit 1
fi
if ! grep -q 'owner=boundary recipe=pure-first compat_replay=none' "$BUILD_LOG"; then
  tail -n 120 "$BUILD_LOG" >&2 || true
  test_fail "$SMOKE_NAME: boundary pure-first route trace missing"
  exit 1
fi
if grep -q '\[llvm-route/replay\] lane=harness' "$BUILD_LOG"; then
  tail -n 120 "$BUILD_LOG" >&2 || true
  test_fail "$SMOKE_NAME: harness replay used unexpectedly"
  exit 1
fi

set +e
NYASH_NYRT_SILENT_RESULT=1 timeout "$RUN_TIMEOUT_SECS" "$OUT_EXE" >"$RUN_LOG" 2>&1
RUN_RC=$?
set -e

if [ "$RUN_RC" -eq 124 ]; then
  test_fail "$SMOKE_NAME: executable run timed out"
  exit 1
fi
if [ "$RUN_RC" -ne 7 ]; then
  tail -n 120 "$RUN_LOG" >&2 || true
  test_fail "$SMOKE_NAME: executable rc=$RUN_RC (expected 7)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct main(args) MIR compiles through ny-llvmc boundary pure-first without harness replay)"
