#!/bin/bash
# Phase 29y no-compat mainline smoke
#
# Contract pin:
# 1) Mainline identity smoke (stage1-first) must not emit `[identity/compat-fallback]`.
# 2) Mainline stage-a runtime probe must not emit `lane=compat-rust-json-v0-bridge`.
# 3) Probe runs with explicit fallback disabled (`NYASH_VM_USE_FALLBACK=0`).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

IDENTITY_SCRIPT="$NYASH_ROOT/tools/selfhost_identity_check.sh"
RUNNER="$NYASH_ROOT/tools/selfhost/run.sh"
FIXTURE="${1:-$NYASH_ROOT/apps/examples/string_p0.hako}"
TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-6000}"

if ! [[ "$TIMEOUT_MS" =~ ^[0-9]+$ ]]; then
  test_fail "phase29y_no_compat_mainline_vm: timeout must be integer: $TIMEOUT_MS"
  exit 2
fi

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

if [ ! -x "$IDENTITY_SCRIPT" ]; then
  test_fail "phase29y_no_compat_mainline_vm: identity script missing/executable: $IDENTITY_SCRIPT"
  exit 2
fi
if [ ! -x "$RUNNER" ]; then
  test_fail "phase29y_no_compat_mainline_vm: selfhost runner missing/executable: $RUNNER"
  exit 2
fi
if [ ! -f "$FIXTURE" ]; then
  test_fail "phase29y_no_compat_mainline_vm: fixture missing: $FIXTURE"
  exit 2
fi

identity_stdout="$(mktemp /tmp/phase29y_no_compat_identity_stdout.XXXXXX.log)"
identity_stderr="$(mktemp /tmp/phase29y_no_compat_identity_stderr.XXXXXX.log)"
runtime_stdout="$(mktemp /tmp/phase29y_no_compat_runtime_stdout.XXXXXX.log)"
runtime_stderr="$(mktemp /tmp/phase29y_no_compat_runtime_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$identity_stdout" "$identity_stderr" "$runtime_stdout" "$runtime_stderr"
}
trap cleanup EXIT

set +e
"$IDENTITY_SCRIPT" --mode smoke --skip-build >"$identity_stdout" 2>"$identity_stderr"
identity_rc=$?
set -e

if [ "$identity_rc" -ne 0 ]; then
  echo "[INFO] STDERR_LOG(identity): $identity_stderr"
  test_fail "phase29y_no_compat_mainline_vm: identity smoke failed (rc=$identity_rc)"
  exit 1
fi

if rg -q '^\[identity/compat-fallback\]' "$identity_stderr"; then
  echo "[INFO] STDERR_LOG(identity): $identity_stderr"
  test_fail "phase29y_no_compat_mainline_vm: identity smoke leaked compat fallback tag"
  exit 1
fi

set +e
NYASH_USE_NY_COMPILER=1 \
NYASH_NY_COMPILER_EMIT_ONLY=1 \
NYASH_NY_COMPILER_USE_TMP_ONLY=1 \
NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
NYASH_VM_USE_FALLBACK=0 \
HAKO_JOINIR_STRICT=1 \
HAKO_JOINIR_PLANNER_REQUIRED=1 \
NYASH_JOINIR_DEV=1 \
NYASH_JOINIR_STRICT=1 \
"$RUNNER" --runtime --runtime-mode stage-a --input "$FIXTURE" --timeout-ms "$TIMEOUT_MS" \
>"$runtime_stdout" 2>"$runtime_stderr"
runtime_rc=$?
set -e

if [ "$runtime_rc" -ne 0 ]; then
  echo "[INFO] STDERR_LOG(runtime): $runtime_stderr"
  test_fail "phase29y_no_compat_mainline_vm: runtime stage-a probe failed (rc=$runtime_rc)"
  exit 1
fi

if rg -q 'lane=compat-rust-json-v0-bridge' "$runtime_stderr"; then
  echo "[INFO] STDERR_LOG(runtime): $runtime_stderr"
  test_fail "phase29y_no_compat_mainline_vm: runtime probe leaked compat-rust-json-v0-bridge lane"
  exit 1
fi

test_pass "phase29y_no_compat_mainline_vm: PASS (no compat fallback on mainline routes)"
