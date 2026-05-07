#!/bin/bash
# Real-app EXE boundary probe
#
# Contract pin:
# - VM real-app suite is the executable correctness gate.
# - Direct EXE currently reaches ny-llvmc pure-first, lowers general user-box
#   allocation/field slots through TypedObjectPlan, and stops at the known
#   typed user-box/generic method routes, and stops at the remaining known
#   BoxTorrentChunker.ingest user-box method route boundary.
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

  if [ "$app_name" = "boxtorrent-mini" ]; then
    if ! grep -Fq "reason=mir_call_no_route" "$build_log"; then
      echo "[INFO] build output tail for $app_name:"
      tail -n 120 "$build_log" || true
      test_fail "$SMOKE_NAME: $app_name did not stop at the pinned ingest boundary"
      return 1
    fi
    if ! grep -Fq "bname=BoxTorrentChunker mname=ingest" "$build_log"; then
      echo "[INFO] build output tail for $app_name:"
      tail -n 120 "$build_log" || true
      test_fail "$SMOKE_NAME: $app_name ingest blocker changed"
      return 1
    fi
  else
    if ! grep -Fq "first_op=mir_call" "$build_log"; then
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

probe_one "boxtorrent-mini" \
  "consumer=mir_call_user_box_birth_same_module_emit site=b116.i11 route=user_box.method_call core_op=UserBoxMethodCall tier=DirectAbi symbol=HakoAllocHeap.birth/0"
probe_one "binary-trees"
probe_one "mimalloc-lite"
probe_one "allocator-stress"
probe_one "json-stream-aggregator"

test_pass "$SMOKE_NAME"
