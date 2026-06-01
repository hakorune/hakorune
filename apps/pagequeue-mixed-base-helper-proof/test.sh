#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/pagequeue-mixed-base-helper-proof/main.hako"
NY_LLVM_C="$ROOT_DIR/target/release/ny-llvmc"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_pagequeue_mixed_base_helper_$$"
MIR_OUT="${TMP_ROOT}.mir.json"
EXE_OUT="${TMP_ROOT}.exe"
BUILD_LOG="${TMP_ROOT}.build.log"
RUN_LOG="${TMP_ROOT}.run.log"

cleanup() {
  rm -f "$MIR_OUT" "$EXE_OUT" "$BUILD_LOG" "$RUN_LOG" 2>/dev/null || true
}
trap cleanup EXIT

NYASH_DISABLE_PLUGINS=1 "$ROOT_DIR/target/release/hakorune" "$APP"

if [ ! -x "$NY_LLVM_C" ]; then
  echo "[pagequeue-mixed-base-helper-proof] skip: ny-llvmc missing: $NY_LLVM_C"
  exit 0
fi

NYASH_DISABLE_PLUGINS=1 \
  timeout "$RUN_TIMEOUT_SECS" \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" \
    --in "$APP" \
    --mir "$MIR_OUT" \
    >"$BUILD_LOG" 2>&1

if ! grep -Fq '"proof": "typed_user_box_method_same_module"' "$MIR_OUT"; then
  cat "$MIR_OUT" >&2
  echo "[pagequeue-mixed-base-helper-proof] missing same-module method route proof" >&2
  exit 1
fi

NYASH_DISABLE_PLUGINS=1 \
  NYASH_LLVM_ROUTE_TRACE=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NY_LLVM_C" \
    --in "$MIR_OUT" \
    --emit exe \
    --nyrt "$ROOT_DIR/target/release" \
    --out "$EXE_OUT" \
    >>"$BUILD_LOG" 2>&1

if ! grep -Fq "mir_call_user_box_method_same_module_emit" "$BUILD_LOG"; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  echo "[pagequeue-mixed-base-helper-proof] same-module method emit trace missing" >&2
  exit 1
fi

set +e
NYASH_DISABLE_PLUGINS=1 "$EXE_OUT" >"$RUN_LOG" 2>&1
exe_rc=$?
set -e

if [ "$exe_rc" -ne 0 ]; then
  cat "$RUN_LOG" >&2 || true
  echo "[pagequeue-mixed-base-helper-proof] EXE expected rc=0 got rc=$exe_rc" >&2
  exit 1
fi
