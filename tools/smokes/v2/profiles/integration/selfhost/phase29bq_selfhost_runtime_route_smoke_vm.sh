#!/bin/bash
# phase29bq_selfhost_runtime_route_smoke_vm.sh
# Verify runtime selfhost route tag contract on stderr.
# Contract:
# - mode=pipeline-entry is emitted when runtime route is engaged
# - mode=<stage-a|exe> is emitted for selected runtime route
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

RUNNER="$NYASH_ROOT/tools/selfhost/run.sh"
FIXTURE="${1:-$NYASH_ROOT/apps/examples/string_p0.hako}"
RUNTIME_MODE="${2:-stage-a}"
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

if [[ "$RUNTIME_MODE" != "stage-a" && "$RUNTIME_MODE" != "exe" ]]; then
  log_error "runtime mode must be stage-a|exe (got: $RUNTIME_MODE)"
  exit 2
fi

if [ ! -x "$RUNNER" ]; then
  log_error "selfhost runner not found/executable: $RUNNER"
  exit 2
fi

if [ "$RUNTIME_MODE" = "exe" ]; then
  parser_exe=""
  if [ -n "${NYASH_NY_COMPILER_EXE_PATH:-}" ]; then
    parser_exe="${NYASH_NY_COMPILER_EXE_PATH}"
  elif [ -x "$NYASH_ROOT/dist/nyash_compiler/nyash_compiler" ]; then
    parser_exe="$NYASH_ROOT/dist/nyash_compiler/nyash_compiler"
  elif command -v nyash_compiler >/dev/null 2>&1; then
    parser_exe="$(command -v nyash_compiler)"
  fi

  if [ -z "$parser_exe" ] || [ ! -x "$parser_exe" ]; then
    log_warn "selfhost runtime exe route skipped (parser EXE not found)"
    exit 0
  fi
fi

stdout_log="$(mktemp /tmp/phase29bq_selfhost_runtime_route_stdout.XXXXXX.log)"
stderr_log="$(mktemp /tmp/phase29bq_selfhost_runtime_route_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stdout_log" "$stderr_log"
}
trap cleanup EXIT

set +e
NYASH_USE_NY_COMPILER=1 \
  NYASH_NY_COMPILER_EMIT_ONLY=1 \
  NYASH_NY_COMPILER_USE_TMP_ONLY=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  "$RUNNER" --runtime --runtime-mode "$RUNTIME_MODE" --input "$FIXTURE" --timeout-ms "$TIMEOUT_MS" \
  > "$stdout_log" 2> "$stderr_log"
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  log_error "runtime selfhost route smoke failed (rc=$rc)"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=pipeline-entry source=' "$stderr_log"; then
  log_error "missing runtime route tag (mode=pipeline-entry) in stderr"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if ! rg -q "^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=${RUNTIME_MODE} source=" "$stderr_log"; then
  log_error "missing runtime route tag (mode=${RUNTIME_MODE}) in stderr"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if [ "$RUNTIME_MODE" = "stage-a" ]; then
  if ! rg -q '^\[contract\]\[runtime-route\]\[accepted=mir-json\] route=stage-a source=' "$stderr_log"; then
    log_error "missing runtime route contract tag (accepted=mir-json) in stderr"
    echo "STDERR_LOG: $stderr_log"
    exit 1
  fi
fi

if [ "$RUNTIME_MODE" = "exe" ] && rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=stage-a source=' "$stderr_log"; then
  log_error "runtime exe route fell back to stage-a unexpectedly"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

log_success "phase29bq_selfhost_runtime_route_smoke_vm: PASS ($(basename "$FIXTURE"), mode=${RUNTIME_MODE})"
