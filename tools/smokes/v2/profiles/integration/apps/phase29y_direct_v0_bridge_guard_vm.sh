#!/bin/bash
# Phase 29y direct-v0 bridge guard smoke
#
# Contract pin:
# - retired parser flags are removed from mainline entrypoints.
# - `--parser ny` must be rejected by CLI parsing.
# - `NYASH_USE_NY_PARSER=1` must not activate direct-v0 retired route.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="${1:-$NYASH_ROOT/apps/tests/minimal_ssa_skip_ws.hako}"
if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi
if [ ! -f "$FIXTURE" ]; then
  test_fail "phase29y_direct_v0_bridge_guard_vm: fixture missing: $FIXTURE"
  exit 2
fi

stdout_log="$(mktemp /tmp/phase29y_direct_v0_stdout.XXXXXX.log)"
stderr_log="$(mktemp /tmp/phase29y_direct_v0_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stdout_log" "$stderr_log"
}
trap cleanup EXIT

run_cli_removed_case() {
  set +e
  "$NYASH_BIN" --parser ny --backend vm "$FIXTURE" >"$stdout_log" 2>"$stderr_log"
  local rc=$?
  set -e

  if [ "$rc" -ne 2 ]; then
    echo "[INFO] CASE: cli_parser_ny"
    echo "[INFO] STDERR_LOG: $stderr_log"
    test_fail "phase29y_direct_v0_bridge_guard_vm: expected rc=2 for removed CLI flag, got rc=$rc"
    exit 1
  fi

  if ! rg -q 'unexpected argument.*--parser' "$stderr_log"; then
    echo "[INFO] CASE: cli_parser_ny"
    echo "[INFO] STDERR_LOG: $stderr_log"
    test_fail "phase29y_direct_v0_bridge_guard_vm: missing clap rejection for --parser"
    exit 1
  fi
}

run_env_ignored_case() {
  set +e
  env NYASH_USE_NY_PARSER=1 "$NYASH_BIN" --backend vm "$FIXTURE" >"$stdout_log" 2>"$stderr_log"
  local rc=$?
  set -e

  # Fixture may fail for unrelated reasons, but retired direct-v0 route must not be activated.
  if rg -q '\[freeze:contract\]\[runtime-route/direct-v0-bridge-disabled\]' "$stderr_log"; then
    echo "[INFO] CASE: env_use_ny_parser"
    echo "[INFO] STDERR_LOG: $stderr_log"
    test_fail "phase29y_direct_v0_bridge_guard_vm: legacy env still activates retired direct-v0 route"
    exit 1
  fi

  if [ "$rc" -eq 2 ] && rg -q 'unexpected argument.*--parser' "$stderr_log"; then
    echo "[INFO] CASE: env_use_ny_parser"
    echo "[INFO] STDERR_LOG: $stderr_log"
    test_fail "phase29y_direct_v0_bridge_guard_vm: env case unexpectedly hit CLI parser rejection path"
    exit 1
  fi
}

run_cli_removed_case
run_env_ignored_case

test_pass "phase29y_direct_v0_bridge_guard_vm: PASS (retired parser flag entrypoints removed)"
