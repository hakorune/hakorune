#!/bin/bash
# phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh
# Compare runtime selfhost semantic result between stage-a and exe routes.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

RUNNER="$NYASH_ROOT/tools/selfhost/run.sh"
FIXTURE_INPUT="${1:-}"
TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-6000}"

if ! [[ "$TIMEOUT_MS" =~ ^[0-9]+$ ]]; then
  log_error "timeout must be integer: $TIMEOUT_MS"
  exit 2
fi

if [ ! -x "$RUNNER" ]; then
  log_error "selfhost runner not found/executable: $RUNNER"
  exit 2
fi

parser_exe=""
if [ -n "${NYASH_NY_COMPILER_EXE_PATH:-}" ]; then
  parser_exe="${NYASH_NY_COMPILER_EXE_PATH}"
elif [ -x "$NYASH_ROOT/dist/nyash_compiler/nyash_compiler" ]; then
  parser_exe="$NYASH_ROOT/dist/nyash_compiler/nyash_compiler"
elif command -v nyash_compiler >/dev/null 2>&1; then
  parser_exe="$(command -v nyash_compiler)"
fi

if [ -z "$parser_exe" ] || [ ! -x "$parser_exe" ]; then
  log_warn "selfhost runtime mode parity skipped (parser EXE not found)"
  exit 0
fi

tmp_fixture=""
fixture="$FIXTURE_INPUT"
if [ -z "$fixture" ]; then
  tmp_fixture="$(mktemp /tmp/phase29bq_runtime_mode_parity_input.XXXXXX.ny)"
  cat > "$tmp_fixture" <<'NY'
return 1 + 2 * 3
NY
  fixture="$tmp_fixture"
fi

if [[ "$fixture" != /* ]]; then
  fixture="$NYASH_ROOT/$fixture"
fi

if [ ! -f "$fixture" ]; then
  log_error "fixture not found: $fixture"
  exit 2
fi

stage_stdout="$(mktemp /tmp/phase29bq_runtime_stagea_stdout.XXXXXX.log)"
stage_stderr="$(mktemp /tmp/phase29bq_runtime_stagea_stderr.XXXXXX.log)"
exe_stdout="$(mktemp /tmp/phase29bq_runtime_exe_stdout.XXXXXX.log)"
exe_stderr="$(mktemp /tmp/phase29bq_runtime_exe_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stage_stdout" "$stage_stderr" "$exe_stdout" "$exe_stderr"
  if [ -n "$tmp_fixture" ]; then
    rm -f "$tmp_fixture"
  fi
}
trap cleanup EXIT

run_mode() {
  local mode="$1"
  local stdout_log="$2"
  local stderr_log="$3"
  local rc=0

  set +e
  NYASH_USE_NY_COMPILER=1 \
    NYASH_NY_COMPILER_USE_PY=0 \
    NYASH_NY_COMPILER_EMIT_ONLY=0 \
    NYASH_NY_COMPILER_USE_TMP_ONLY=1 \
    NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
    "$RUNNER" --runtime --runtime-mode "$mode" --input "$fixture" --timeout-ms "$TIMEOUT_MS" \
    > "$stdout_log" 2> "$stderr_log"
  rc=$?
  set -e

  echo "$rc"
}

extract_semantic_value() {
  local stdout_log="$1"
  local stderr_log="$2"
  local rc="$3"
  local value=""

  value="$(sed -n 's/^Result:[[:space:]]*//p' "$stdout_log" | tail -n 1)"
  if [ -z "$value" ]; then
    value="$(sed -n 's/^Result:[[:space:]]*//p' "$stderr_log" | tail -n 1)"
  fi
  if [ -z "$value" ]; then
    value="$(sed -n 's/^RC:[[:space:]]*/rc:/p' "$stderr_log" | tail -n 1)"
  fi
  if [ -z "$value" ]; then
    value="rc:$rc"
  fi

  echo "$value"
}

stage_rc="$(run_mode "stage-a-compat" "$stage_stdout" "$stage_stderr")"
exe_rc="$(run_mode "exe" "$exe_stdout" "$exe_stderr")"

if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=pipeline-entry source=' "$stage_stderr"; then
  log_error "stage-a-compat missing runtime route tag (mode=pipeline-entry)"
  echo "STAGE_STDERR: $stage_stderr"
  exit 1
fi
if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=stage-a-compat source=' "$stage_stderr"; then
  log_error "stage-a-compat missing runtime route tag (mode=stage-a-compat)"
  echo "STAGE_STDERR: $stage_stderr"
  exit 1
fi
if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=pipeline-entry source=' "$exe_stderr"; then
  log_error "exe missing runtime route tag (mode=pipeline-entry)"
  echo "EXE_STDERR: $exe_stderr"
  exit 1
fi
if ! rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=exe source=' "$exe_stderr"; then
  log_error "exe missing runtime route tag (mode=exe)"
  echo "EXE_STDERR: $exe_stderr"
  exit 1
fi
if rg -q '^\[selfhost/route\] id=SH-RUNTIME-SELFHOST mode=stage-a-compat source=' "$exe_stderr"; then
  log_error "runtime exe route fell back to stage-a-compat unexpectedly"
  echo "EXE_STDERR: $exe_stderr"
  exit 1
fi

stage_value="$(extract_semantic_value "$stage_stdout" "$stage_stderr" "$stage_rc")"
exe_value="$(extract_semantic_value "$exe_stdout" "$exe_stderr" "$exe_rc")"

if [ "$stage_value" != "$exe_value" ]; then
  log_error "runtime mode parity mismatch: stage-a-compat='$stage_value' exe='$exe_value'"
  echo "STAGE_STDOUT: $stage_stdout"
  echo "STAGE_STDERR: $stage_stderr"
  echo "EXE_STDOUT: $exe_stdout"
  echo "EXE_STDERR: $exe_stderr"
  exit 1
fi

log_success "phase29bq_selfhost_runtime_mode_parity_smoke_vm: PASS ($(basename "$fixture"), value=$stage_value, rc_stagea_compat=$stage_rc, rc_exe=$exe_rc)"
