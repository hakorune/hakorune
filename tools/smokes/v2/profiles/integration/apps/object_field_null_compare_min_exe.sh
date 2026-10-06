#!/bin/bash
# S0: borrowed me-field read and eq/ne null comparisons reach physical EXE.
# Predicate result identity is pinned by the publication tests; these apps
# return i64 zero, so this smoke does not claim Bool-return or branch coverage.
set -euo pipefail
source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="object_field_null_compare_min_exe"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
NYRT_DIR="${HAKO_LIFECYCLE_NYRT:-${NYASH_EMIT_EXE_NYRT:-$HAKO_ROOT/target/lifecycle-kernel/release}}"
if [ ! -f "$NYRT_DIR/libnyash_lifecycle_kernel.a" ]; then
  test_skip "$SMOKE_NAME: lifecycle runtime archive missing: $NYRT_DIR"
  exit 0
fi
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/hakorune_object_field_null_compare.XXXXXX")"
trap 'rm -rf "$TMP_ROOT"' EXIT

for predicate in eq ne; do
  APP="$HAKO_ROOT/apps/object-field-null-compare-min/$predicate.hako"
  EXE_OUT="$TMP_ROOT/$predicate.exe"
  BUILD_LOG="$TMP_ROOT/$predicate.build.log"
  RUN_LOG="$TMP_ROOT/$predicate.run.log"
  if [ ! -f "$APP" ]; then
    test_fail "$SMOKE_NAME: app missing: $APP"
    exit 2
  fi
  set +e
  NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
    HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none \
    timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend mir \
      --emit-exe "$EXE_OUT" --emit-exe-nyrt "$NYRT_DIR" "$APP" \
      >"$BUILD_LOG" 2>&1
  build_rc=$?
  set -e
  if [ "$build_rc" -ne 0 ] || [ ! -x "$EXE_OUT" ] || \
    ! rg -Fq 'stage=lifecycle-v4-measure result=ok' "$BUILD_LOG" || \
    ! rg -Fq 'toolchain=llvm-c-api' "$BUILD_LOG" || \
    ! rg -Fq 'EXE written:' "$BUILD_LOG" || \
    rg -q 'unsupported pure shape|compat_replay=harness' "$BUILD_LOG"; then
    tail -n 120 "$BUILD_LOG"
    test_fail "$SMOKE_NAME/$predicate: physical EXE evidence missing, rc=$build_rc"
    exit 1
  fi
  set +e
  NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$EXE_OUT" >"$RUN_LOG" 2>&1
  exe_rc=$?
  set -e
  if [ "$exe_rc" -ne 0 ]; then
    cat "$RUN_LOG"
    test_fail "$SMOKE_NAME/$predicate: expected exit 0, got $exe_rc"
    exit 1
  fi
  test_pass "$SMOKE_NAME/$predicate"
done

# An undeclared field must stop before any executable is published.
set +e
NYASH_DISABLE_PLUGINS=1 HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" \
    --backend mir --emit-exe "$TMP_ROOT/missing.exe" --emit-exe-nyrt "$NYRT_DIR" \
    "$HAKO_ROOT/apps/object-field-null-compare-min/missing-field.hako" \
    >"$TMP_ROOT/missing.log" 2>&1
negative_rc=$?
set -e
if [ "$negative_rc" -eq 0 ] || [ "$negative_rc" -eq 124 ] || \
  [ -e "$TMP_ROOT/missing.exe" ] || \
  ! rg -Fq 'instruction-unsupported' "$TMP_ROOT/missing.log"; then
  tail -n 120 "$TMP_ROOT/missing.log"
  test_fail "$SMOKE_NAME/missing-field: expected pre-artifact rejection, rc=$negative_rc"
  exit 1
fi
test_pass "$SMOKE_NAME/missing-field"
