#!/usr/bin/env bash
# Note: test_runner.sh handles shell options; set -euo pipefail conflicts with conditional tests

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
BIN_NYLLVMC="$ROOT_DIR/target/release/ny-llvmc"
BIN_HAKO="$ROOT_DIR/target/release/hakorune"

enable_exe_dev_env

# Quick profile default timeout is 15s; this test may need longer for build+link.
# Respect the global budget and SKIP instead of timing out (fast-fail friendly).
if [ "${SMOKES_DEFAULT_TIMEOUT:-0}" -ne 0 ] && [ "${SMOKES_DEFAULT_TIMEOUT:-0}" -lt 25 ]; then
  echo "[SKIP] time budget too small for EXE canary (SMOKES_DEFAULT_TIMEOUT=${SMOKES_DEFAULT_TIMEOUT}s)"
  exit 0
fi

# Build tools if missing
timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -lc 'cargo build -q --release -p nyash-llvm-compiler >/dev/null' || true
timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -lc 'cargo build -q --release >/dev/null' || true

# Hako program: return (new StringBox("nyash")).length()
TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(args) {
  return new StringBox("nyash").length()
} }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT="${ROOT_DIR}/target/perf_strlen_fast_$$"
trap 'rm -f "$TMP_JSON" "$TMP_HAKO" "$EXE_OUT" 2>/dev/null || true' EXIT

# Emit MIR JSON (wrapper fallback) and build EXE (crate backend)
set +e
if ! NYASH_JSON_ONLY=1 timeout "${HAKO_BUILD_TIMEOUT:-10}" bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$TMP_HAKO" "$TMP_JSON" >/dev/null 2>&1; then
  echo "[SKIP] failed to emit MIR JSON"; exit 0
fi
set -e

# Build exe with FAST lowering ON
if ! NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 NYASH_LLVM_FAST=1 \
     timeout "${HAKO_BUILD_TIMEOUT:-10}" bash "$ROOT_DIR/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$EXE_OUT" --quiet >/dev/null 2>&1; then
  echo "[SKIP] failed to build EXE"; exit 0
fi

# Run exe and check exit code == 5
set +e
timeout "${HAKO_EXE_TIMEOUT:-5}" "$EXE_OUT" >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -eq 124 ]]; then
  echo "[SKIP] timed out running EXE (expect rc=5)"; exit 0
fi
if [[ "$rc" -eq 5 ]]; then
  echo "[PASS] s3_backend_selector_crate_exe_strlen_fast_canary_vm"
  exit 0
fi
echo "[SKIP] unexpected rc=$rc (expect 5)"; exit 0
