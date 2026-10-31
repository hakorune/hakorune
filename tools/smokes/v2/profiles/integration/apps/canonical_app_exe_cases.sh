#!/bin/bash
# Canonical app EXE cases
#
# Contract pin, for every case:
# - the app enters the selected physical lifecycle V4 owner through the LLVM C
#   API, with no compat replay and no unsupported pure shape;
# - the EXE exits with the case's expected code (and prints its expected line).
# Cases: the typed-object seeds (allocation-only New, explicit i64 fields,
# birth, birth with a parameter, method; exit 30) and loop-basics (twelve
# everyday loop shapes compared with hand-computed results; exit 0).
#
# Usage: canonical_app_exe_cases.sh [case]   (no case runs all of them)

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="canonical_app_exe_cases"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
# case|app directory|expected exit|expected stdout line|extra compile flag
CASES=(
  "typed_object_newbox_min|typed-object-newbox-min|30||--grammar-profile canonical"
  "typed_object_birth_min|typed-object-birth-min|30||"
  "typed_object_method_min|typed-object-method-min|30||"
  "typed_object_birth_param_min|typed-object-birth-param-min|30||"
  "ownership_move_min|ownership-move-min|30||"
  "loop_basics|loop-basics|0|loop-basics failures=0|"
)
ONLY_CASE="${1:-}"

resolve_lifecycle_runtime() {
  if [ -n "${HAKO_LIFECYCLE_NYRT:-}" ]; then
    printf '%s' "$HAKO_LIFECYCLE_NYRT"
  elif [ -n "${NYASH_EMIT_EXE_NYRT:-}" ] && [ -f "$NYASH_EMIT_EXE_NYRT/libnyash_lifecycle_kernel.a" ]; then
    printf '%s' "$NYASH_EMIT_EXE_NYRT"
  else
    printf '%s' "$HAKO_ROOT/target/lifecycle-kernel/release"
  fi
}

NYRT_DIR="$(resolve_lifecycle_runtime)"
if [ ! -f "$NYRT_DIR/libnyash_lifecycle_kernel.a" ]; then
  test_skip "$SMOKE_NAME: lifecycle runtime archive missing: $NYRT_DIR/libnyash_lifecycle_kernel.a"
  exit 0
fi

TMP_ROOT="${TMPDIR:-/tmp}/hakorune_canonical_app_exe_$$"
cleanup() {
  rm -f "$TMP_ROOT".* 2>/dev/null || true
}
trap cleanup EXIT

run_case() {
  local name="$1" app="$HAKO_ROOT/apps/$2/main.hako" expected_rc="$3" expected_line="$4" flag="$5"
  local exe="$TMP_ROOT.$name.exe" build_log="$TMP_ROOT.$name.build.log" run_log="$TMP_ROOT.$name.run.log"
  if [ ! -f "$app" ]; then
    test_fail "$SMOKE_NAME/$name: app missing: $app"
    return 1
  fi
  set +e
  # shellcheck disable=SC2086
  NYASH_DISABLE_PLUGINS=1 \
    NYASH_LLVM_ROUTE_TRACE=1 \
    HAKO_BACKEND_COMPILE_RECIPE=pure-first \
    HAKO_BACKEND_COMPAT_REPLAY=none \
    timeout "$RUN_TIMEOUT_SECS" \
    "$NYASH_BIN" $flag --backend mir --emit-exe "$exe" --emit-exe-nyrt "$NYRT_DIR" "$app" \
    >"$build_log" 2>&1
  local build_rc=$?
  set -e
  local problem=""
  if [ "$build_rc" -ne 0 ]; then
    problem="physical EXE emit failed rc=$build_rc"
  elif ! grep -Fq 'stage=lifecycle-v4-measure result=ok' "$build_log"; then
    problem="lifecycle V4 route trace missing"
  elif ! grep -Fq 'toolchain=llvm-c-api' "$build_log"; then
    problem="LLVM C API physical route missing"
  elif grep -Fq "unsupported pure shape" "$build_log" || grep -Fq "compat_replay=harness" "$build_log"; then
    problem="fallback or unsupported pure shape was used"
  elif ! grep -Fq "EXE written:" "$build_log" || [ ! -x "$exe" ]; then
    problem="physical EXE artifact missing"
  fi
  if [ -n "$problem" ]; then
    echo "[INFO] $name EXE build output tail:"
    tail -n 160 "$build_log" || true
    test_fail "$SMOKE_NAME/$name: $problem"
    return 1
  fi
  set +e
  NYASH_DISABLE_PLUGINS=1 "$exe" >"$run_log" 2>&1
  local exe_rc=$?
  set -e
  if [ "$exe_rc" -ne "$expected_rc" ] || { [ -n "$expected_line" ] && ! grep -Fxq "$expected_line" "$run_log"; }; then
    echo "[INFO] $name EXE stdout/stderr:"
    cat "$run_log" || true
    test_fail "$SMOKE_NAME/$name: expected exit $expected_rc${expected_line:+ and '$expected_line'}, got $exe_rc"
    return 1
  fi
}

ran=0
for row in "${CASES[@]}"; do
  IFS='|' read -r name app expected_rc expected_line flag <<<"$row"
  if [ -n "$ONLY_CASE" ] && [ "$name" != "$ONLY_CASE" ]; then
    continue
  fi
  run_case "$name" "$app" "$expected_rc" "$expected_line" "$flag" || exit 1
  ran=$((ran + 1))
done
if [ "$ran" -eq 0 ]; then
  test_fail "$SMOKE_NAME: unknown case: $ONLY_CASE"
  exit 2
fi

test_pass "$SMOKE_NAME"
