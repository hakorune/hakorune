#!/bin/bash
# phase29bq_selfhost_runtime_route_program_reject_smoke_vm.sh
# Contract:
# - stage-a runtime route must fail-fast in strict+planner_required when Program(JSON v0)
#   reaches the runtime boundary (expected MIR(JSON v0) only).
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

stdout_log="$(mktemp /tmp/phase29bq_selfhost_runtime_route_program_reject_stdout.XXXXXX.log)"
stderr_log="$(mktemp /tmp/phase29bq_selfhost_runtime_route_program_reject_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stdout_log" "$stderr_log"
}
trap cleanup EXIT

set +e
NYASH_USE_NY_COMPILER=1 \
  NYASH_NY_COMPILER_EMIT_ONLY=1 \
  NYASH_NY_COMPILER_USE_TMP_ONLY=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  HAKO_JOINIR_STRICT=1 \
  HAKO_JOINIR_PLANNER_REQUIRED=1 \
  "$RUNNER" --runtime --runtime-mode stage-a --input "$FIXTURE" --timeout-ms "$TIMEOUT_MS" \
  > "$stdout_log" 2> "$stderr_log"
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  log_error "runtime stage-a reject smoke expected failure (rc=0)"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=pipeline-entry source=' "$stderr_log"; then
  log_error "missing runtime route tag (mode=pipeline-entry) in stderr"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=stage-a source=' "$stderr_log"; then
  log_error "missing runtime route tag (mode=stage-a) in stderr"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if ! rg -q '^\[contract\]\[runtime-route\]\[expected=mir-json\] route=stage-a source=' "$stderr_log"; then
  log_error "missing runtime route contract reject tag in stderr"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

log_success "phase29bq_selfhost_runtime_route_program_reject_smoke_vm: PASS ($(basename "$FIXTURE"), rc=$rc)"
