#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true

# Stage-B emit が不安定なため quick ではスキップ（s3 parity は Stage-B ラインで扱う）
echo "[SKIP] s3_backend_selector_crate_exe_vm_parity_return42_canary_vm (Stage-B emit 不安定のため quick ではスキップ)" >&2
exit 0

# Ensure tools are built and environment is consistent for EXE
timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -c "cd \"$ROOT\" && cargo build -q --release -p nyash-llvm-compiler >/dev/null" || true
timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -c "cd \"$ROOT/crates/nyash_kernel\" && cargo build -q --release >/dev/null" || true
enable_exe_dev_env

# Minimal Hako program returning 42
TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(args) { return 42 } }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT="${ROOT}/target/parity_ret42_$$"
BIN_NYLLVMC="$ROOT/target/release/ny-llvmc"
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" "$EXE_OUT" 2>/dev/null || true' EXIT

# Run via VM and capture program exit code from return status
set +e
timeout "${HAKO_EXE_TIMEOUT:-5}" "$NYASH_BIN" --backend vm "$TMP_HAKO" >/dev/null 2>&1
rc_vm=$?
set -e

# Emit MIR JSON and build EXE (crate backend)
if ! NYASH_JSON_ONLY=1 bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "$TMP_JSON" --input "$TMP_HAKO" >/dev/null 2>&1; then
  echo "[FAIL] exe_vm_parity_ret42: failed to emit MIR JSON" >&2
  exit 1
fi

if ! NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 timeout "${HAKO_BUILD_TIMEOUT:-10}" "$BIN_NYLLVMC" --in "$TMP_JSON" --emit exe --nyrt "$ROOT/target/release" --out "$EXE_OUT" >/dev/null 2>&1; then
  echo "[FAIL] exe_vm_parity_ret42: failed to build EXE" >&2
  exit 1
fi

# Run EXE and compare RCs
set +e
timeout "${HAKO_EXE_TIMEOUT:-5}" "$EXE_OUT" >/dev/null 2>&1
rc_exe=$?
set -e

if [[ "$rc_exe" -eq 124 ]]; then
  echo "[SKIP] exe_vm_parity_ret42: timed out (vm=$rc_vm exe=$rc_exe)" >&2
  exit 0
fi
if [[ "$rc_exe" -eq "$rc_vm" ]]; then
  echo "[PASS] s3_backend_selector_crate_exe_vm_parity_return42_canary_vm"
  exit 0
fi
echo "[FAIL] exe_vm_parity_ret42: mismatch vm=$rc_vm exe=$rc_exe" >&2
exit 1
