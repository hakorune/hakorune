#!/bin/bash
# Phase 29x X21: non-strict compat boundary smoke
#
# Contract pin:
# 1) non-strict compat route rejects compat lanes unless explicit.
# 2) `NYASH_VM_USE_FALLBACK=1` explicitly enables compatibility lanes.
# 3) explicit compat run emits route-first selfhost tags; raw vm-family tags are optional.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

RUNNER="$NYASH_ROOT/tools/selfhost/run.sh"
FIXTURE="${1:-$NYASH_ROOT/apps/examples/string_p0.hako}"
TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-6000}"

if ! [[ "$TIMEOUT_MS" =~ ^[0-9]+$ ]]; then
  log_error "timeout must be integer: $TIMEOUT_MS"
  exit 2
fi

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

if [ ! -f "$FIXTURE" ]; then
  log_error "fixture not found: $FIXTURE"
  exit 2
fi

if [ ! -x "$RUNNER" ]; then
  log_error "selfhost runner not found/executable: $RUNNER"
  exit 2
fi

stdout_no_compat="$(mktemp /tmp/phase29x_vm_route_non_strict_compat_no_compat_stdout.XXXXXX.log)"
stderr_no_compat="$(mktemp /tmp/phase29x_vm_route_non_strict_compat_no_compat_stderr.XXXXXX.log)"
stdout_with_compat="$(mktemp /tmp/phase29x_vm_route_non_strict_compat_with_compat_stdout.XXXXXX.log)"
stderr_with_compat="$(mktemp /tmp/phase29x_vm_route_non_strict_compat_with_compat_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stdout_no_compat" "$stderr_no_compat" "$stdout_with_compat" "$stderr_with_compat"
}
trap cleanup EXIT

set +e
NYASH_USE_NY_COMPILER=1 \
  NYASH_NY_COMPILER_EMIT_ONLY=1 \
  NYASH_NY_COMPILER_USE_TMP_ONLY=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  HAKO_JOINIR_STRICT=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_VM_USE_FALLBACK=0 \
  "$RUNNER" --runtime --runtime-route compat --input "$FIXTURE" --timeout-ms "$TIMEOUT_MS" \
  > "$stdout_no_compat" 2> "$stderr_no_compat"
rc_no_compat=$?
set -e

if [ "$rc_no_compat" -eq 0 ]; then
  log_error "expected non-strict reject without explicit compat fallback (rc=0)"
  echo "STDERR_LOG(no-compat): $stderr_no_compat"
  exit 1
fi

if ! rg -q '^\[contract\]\[runtime-route\]\[expected=mir-json\] route=compat source=.* non_strict_compat=disabled require=NYASH_VM_USE_FALLBACK=1$' "$stderr_no_compat"; then
  log_error "missing non-strict compat-disabled contract tag"
  echo "STDERR_LOG(no-compat): $stderr_no_compat"
  exit 1
fi

set +e
NYASH_USE_NY_COMPILER=1 \
  NYASH_NY_COMPILER_EMIT_ONLY=1 \
  NYASH_NY_COMPILER_USE_TMP_ONLY=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  HAKO_JOINIR_STRICT=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_VM_ROUTE_TRACE=1 \
  NYASH_VM_USE_FALLBACK=1 \
  "$RUNNER" --runtime --runtime-route compat --input "$FIXTURE" --timeout-ms "$TIMEOUT_MS" \
  > "$stdout_with_compat" 2> "$stderr_with_compat"
rc_with_compat=$?
set -e

if [ "$rc_with_compat" -ne 0 ]; then
  log_error "expected success with explicit compat fallback (rc=$rc_with_compat)"
  echo "STDERR_LOG(with-compat): $stderr_with_compat"
  exit 1
fi

if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=pipeline-entry source=' "$stderr_with_compat"; then
  log_error "missing runtime route tag (mode=pipeline-entry) under explicit fallback"
  echo "STDERR_LOG(with-compat): $stderr_with_compat"
  exit 1
fi

if ! rg -q '^\[selfhost/run\] mode=runtime runtime_route=compat runtime_mode=compat ' "$stderr_with_compat"; then
  log_error "missing runtime run tag (route=compat, mode=compat) under explicit fallback"
  echo "STDERR_LOG(with-compat): $stderr_with_compat"
  exit 1
fi

if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=compat source=' "$stderr_with_compat"; then
  log_error "missing runtime route tag (mode=compat) under explicit fallback"
  echo "STDERR_LOG(with-compat): $stderr_with_compat"
  exit 1
fi

if rg -q '^\[contract\]\[runtime-route\]\[expected=mir-json\] route=compat source=.* non_strict_compat=disabled require=NYASH_VM_USE_FALLBACK=1$' "$stderr_with_compat"; then
  log_error "compat reject contract unexpectedly remained on explicit fallback success path"
  echo "STDERR_LOG(with-compat): $stderr_with_compat"
  exit 1
fi

log_success "phase29x_vm_route_non_strict_compat_boundary_vm: PASS (non-strict compat explicit-only)"
