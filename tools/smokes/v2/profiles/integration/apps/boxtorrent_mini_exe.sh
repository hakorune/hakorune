#!/bin/bash
# BoxTorrent mini real-app smoke (pure-first EXE)
#
# Contract pin:
# - BoxTorrent mini now has direct EXE parity for the allocator-backed local
#   store path.
# - The exact root digest and roundtrip output exercise returned chunk Text
#   through materialize. String carrier/domain negatives remain with existing
#   source publication and C borrowed-ingress tests, independent of LLVM names.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="boxtorrent_mini_exe"
APP="$HAKO_ROOT/apps/boxtorrent-mini/main.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
BUILD_TIMEOUT_SECS="${BUILD_TIMEOUT_SECS:-300}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_boxtorrent_mini_exe_$$"
EXE_OUT="${TMP_ROOT}.exe"
BUILD_LOG="${TMP_ROOT}.build.log"

cleanup() {
  rm -f "$EXE_OUT" "$BUILD_LOG" 2>/dev/null || true
}
trap cleanup EXIT

if [ ! -f "$APP" ]; then
  test_fail "$SMOKE_NAME: App not found: $APP"
  exit 2
fi

NYRT_DIR="${HAKO_LIFECYCLE_NYRT:-$HAKO_ROOT/target/lifecycle-kernel/release}"
if [ -z "${HAKO_LIFECYCLE_NYRT:-}" ] &&
   [ -f "${NYASH_EMIT_EXE_NYRT:-}/libnyash_lifecycle_kernel.a" ]; then
  NYRT_DIR="$NYASH_EMIT_EXE_NYRT"
fi
if [ ! -f "$NYRT_DIR/libnyash_lifecycle_kernel.a" ]; then
  test_skip "$SMOKE_NAME: lifecycle runtime archive missing: $NYRT_DIR/libnyash_lifecycle_kernel.a"
  exit 0
fi

set +e
NYASH_LLVM_ROUTE_TRACE=1 \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout "$BUILD_TIMEOUT_SECS" \
    "$NYASH_BIN" --backend mir --emit-exe "$EXE_OUT" \
      --emit-exe-nyrt "$NYRT_DIR" "$APP" \
      >"$BUILD_LOG" 2>&1
build_rc=$?
set -e

if [ "$build_rc" -ne 0 ]; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: EXE build failed (rc=$build_rc)"
  exit 1
fi

if ! grep -Fq 'stage=lifecycle-v4-measure result=ok' "$BUILD_LOG" ||
   ! grep -Fq 'toolchain=llvm-c-api' "$BUILD_LOG" ||
   grep -Fq 'compat_replay=harness' "$BUILD_LOG" ||
   ! grep -Fq 'EXE written:' "$BUILD_LOG" || [ ! -x "$EXE_OUT" ]; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: canonical LLVM C API EXE route missing or replayed"
  exit 1
fi

set +e
output=$(NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$EXE_OUT" 2>&1 | filter_noise)
run_rc=$?
set -e

if [ "$run_rc" -ne 0 ]; then
  echo "$output"
  test_fail "$SMOKE_NAME: EXE run failed (rc=$run_rc)"
  exit 1
fi

expected=$(cat << 'TXT'
boxtorrent-mini
root=bt-59-458365
chunks=5
bytes=40
dedupe_hits=5
root_equal=true
roundtrip=true
ref_before_release=2
ref_after_release=1
allocator_before_final_release=5
allocator_after_final_release=0
allocator_requested_bytes=40
allocator_releases=5
summary=ok
Result: 0
TXT
)

compare_outputs "$expected" "$output" "$SMOKE_NAME" || exit 1

test_pass "$SMOKE_NAME"
