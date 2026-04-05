#!/bin/bash
# phase29bq_selfhost_runtime_route_smoke_vm.sh
# Verify runtime selfhost route tag contract on stderr.
# Contract:
# - mode=pipeline-entry is emitted when runtime route is engaged
# - runtime-route uses canonical `mainline|compat`
# - mode=<compat|mainline> remains a route-first tag for selected runtime route
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

RUNNER="$NYASH_ROOT/tools/selfhost/run.sh"
FIXTURE="${1:-$NYASH_ROOT/apps/examples/string_p0.hako}"
RUNTIME_ROUTE_INPUT="${2:-compat}"
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

case "$RUNTIME_ROUTE_INPUT" in
  compat|stage-a|stage-a-compat)
    RUNTIME_ROUTE="compat"
    EXPECTED_MODE="compat"
    ;;
  mainline|exe)
    RUNTIME_ROUTE="mainline"
    EXPECTED_MODE="mainline"
    ;;
  *)
    log_error "runtime route must be compat|mainline (compat aliases: stage-a|stage-a-compat|exe; got: $RUNTIME_ROUTE_INPUT)"
    exit 2
    ;;
esac

if [ ! -x "$RUNNER" ]; then
  log_error "selfhost runner not found/executable: $RUNNER"
  exit 2
fi

if [ "$RUNTIME_ROUTE" = "mainline" ]; then
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

runtime_env_prefix=()
if [ "$RUNTIME_ROUTE" = "compat" ]; then
  # Keep the positive stage-a smoke on the explicit compat-success path.
  runtime_env_prefix+=("NYASH_VM_USE_FALLBACK=1")
fi

stdout_log="$(mktemp /tmp/phase29bq_selfhost_runtime_route_stdout.XXXXXX.log)"
stderr_log="$(mktemp /tmp/phase29bq_selfhost_runtime_route_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stdout_log" "$stderr_log"
}
trap cleanup EXIT

set +e
env "${runtime_env_prefix[@]}" \
  NYASH_USE_NY_COMPILER=1 \
  NYASH_NY_COMPILER_EMIT_ONLY=1 \
  NYASH_NY_COMPILER_USE_TMP_ONLY=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  "$RUNNER" --runtime --runtime-route "$RUNTIME_ROUTE" --input "$FIXTURE" --timeout-ms "$TIMEOUT_MS" \
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

if ! rg -q "^\[selfhost/run\] mode=runtime runtime_route=${RUNTIME_ROUTE} runtime_mode=${EXPECTED_MODE} " "$stderr_log"; then
  log_error "missing runtime run tag (route=${RUNTIME_ROUTE}, mode=${EXPECTED_MODE}) in stderr"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if ! rg -q "^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=${EXPECTED_MODE} source=" "$stderr_log"; then
  log_error "missing runtime route tag (mode=${EXPECTED_MODE}) in stderr"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

if [ "$RUNTIME_ROUTE" = "compat" ]; then
  log_success "compat runtime route success path"
fi

if [ "$RUNTIME_ROUTE" = "mainline" ] && rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=compat source=' "$stderr_log"; then
  log_error "runtime mainline route fell back to compat unexpectedly"
  echo "STDERR_LOG: $stderr_log"
  exit 1
fi

log_success "phase29bq_selfhost_runtime_route_smoke_vm: PASS ($(basename "$FIXTURE"), route=${RUNTIME_ROUTE}, mode=${EXPECTED_MODE})"
