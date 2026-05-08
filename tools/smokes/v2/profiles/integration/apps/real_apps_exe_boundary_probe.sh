#!/bin/bash
# Real-app EXE boundary probe
#
# Contract pin:
# - VM real-app suite is the executable correctness gate.
# - Direct EXE currently reaches ny-llvmc pure-first for the remaining real
#   apps that do not yet have parity smokes. As accepted routes expand, a
#   remaining app may move from a call-route stop to a same-module prepass stop.
# - BoxTorrent mini moved to `boxtorrent_mini_exe.sh`; this probe should only
#   pin the next unsupported-shape boundary for the remaining apps.
# - binary-trees moved to `binary_trees_exe.sh`.
# - json-stream-aggregator moved to
#   `json_stream_aggregator_exe_runtime_boundary.sh`.
# - Do not enable compat replay as mainline proof.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="real_apps_exe_boundary_probe"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_real_apps_exe_boundary_$$"

cleanup() {
  rm -f "${TMP_ROOT}"_* 2>/dev/null || true
}
trap cleanup EXIT

if [ ! -x "$NY_LLVM_C" ]; then
  test_skip "$SMOKE_NAME: ny-llvmc missing: $NY_LLVM_C"
  exit 0
fi

probe_one() {
  local app_name="$1"
  shift || true
  local app_path="$NYASH_ROOT/apps/$app_name/main.hako"
  local exe_out="${TMP_ROOT}_${app_name//-/_}"
  local build_log="${TMP_ROOT}_${app_name//-/_}.log"
  local expected_fragment=""

  if [ ! -f "$app_path" ]; then
    test_fail "$SMOKE_NAME: app missing: $app_path"
    return 1
  fi

  set +e
  NYASH_DISABLE_PLUGINS=1 \
    NYASH_LLVM_ROUTE_TRACE=1 \
    HAKO_BACKEND_COMPILE_RECIPE=pure-first \
    HAKO_BACKEND_COMPAT_REPLAY=none \
    timeout "$RUN_TIMEOUT_SECS" \
      "$NYASH_ROOT/tools/selfhost/selfhost_build.sh" \
        --in "$app_path" \
        --exe "$exe_out" \
        >"$build_log" 2>&1
  local build_rc=$?
  set -e

  if [ "$build_rc" -eq 124 ]; then
    echo "[INFO] build output tail for $app_name:"
    tail -n 120 "$build_log" || true
    test_fail "$SMOKE_NAME: $app_name compile timed out (>${RUN_TIMEOUT_SECS}s)"
    return 1
  fi

  if [ "$build_rc" -eq 0 ]; then
    echo "[INFO] build output tail for $app_name:"
    tail -n 120 "$build_log" || true
    test_fail "$SMOKE_NAME: $app_name unexpectedly built EXE; replace this probe with an EXE parity smoke"
    return 1
  fi

  if ! grep -Fq "unsupported pure shape for current backend recipe" "$build_log"; then
    echo "[INFO] build output tail for $app_name:"
    tail -n 120 "$build_log" || true
    test_fail "$SMOKE_NAME: $app_name did not stop at unsupported pure shape"
    return 1
  fi

  if ! grep -Fq "[llvm-pure/unsupported-shape]" "$build_log"; then
    echo "[INFO] build output tail for $app_name:"
    tail -n 120 "$build_log" || true
    test_fail "$SMOKE_NAME: $app_name missing unsupported-shape trace"
    return 1
  fi

  if ! grep -Fq "first_op=mir_call" "$build_log" &&
     ! grep -Fq "first_op=field_get" "$build_log"; then
    echo "[INFO] build output tail for $app_name:"
    tail -n 120 "$build_log" || true
    test_fail "$SMOKE_NAME: $app_name unsupported-shape owner changed"
    return 1
  fi

  if ! grep -Fq "reason=mir_call_no_route" "$build_log" &&
     ! grep -Fq "reason=module_generic_prepass_failed" "$build_log"; then
    echo "[INFO] build output tail for $app_name:"
    tail -n 120 "$build_log" || true
    test_fail "$SMOKE_NAME: $app_name did not stop at a pinned route/prepass boundary"
    return 1
  fi

  if grep -Fq "compat_replay=harness" "$build_log"; then
    echo "[INFO] build output tail for $app_name:"
    tail -n 120 "$build_log" || true
    test_fail "$SMOKE_NAME: $app_name used compat replay"
    return 1
  fi

  for expected_fragment in "$@"; do
    if ! grep -Fq "$expected_fragment" "$build_log"; then
      echo "[INFO] build output tail for $app_name:"
      tail -n 120 "$build_log" || true
      test_fail "$SMOKE_NAME: $app_name missing expected boundary fragment: $expected_fragment"
      return 1
    fi
  done

  echo "[INFO] $app_name: EXE boundary pinned at pure-first unsupported shape"
  return 0
}

probe_one "mimalloc-lite" \
  "first_op=field_get" \
  "target_shape_blocker_symbol=HakoAllocHeap.release/1"
probe_one "allocator-stress" \
  "first_op=field_get" \
  "target_shape_blocker_symbol=HakoAllocHeap.release/1"

test_pass "$SMOKE_NAME"
