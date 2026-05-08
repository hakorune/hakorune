#!/bin/bash
# BoxTorrent mini real-app smoke (pure-first EXE)
#
# Contract pin:
# - BoxTorrent mini now has direct EXE parity for the allocator-backed local
#   store path.
# - `materialize` must concatenate the StringBox returned by
#   `BoxTorrentStore.readData/1`, not treat it as scalar i64.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="boxtorrent_mini_exe"
APP="$HAKO_ROOT/apps/boxtorrent-mini/main.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_boxtorrent_mini_exe_$$"
EXE_OUT="${TMP_ROOT}.exe"
IR_OUT="${TMP_ROOT}.ll"
BUILD_LOG="${TMP_ROOT}.build.log"

cleanup() {
  rm -f "$EXE_OUT" "$IR_OUT" "$BUILD_LOG" 2>/dev/null || true
}
trap cleanup EXIT

if [ ! -f "$APP" ]; then
  test_fail "$SMOKE_NAME: App not found: $APP"
  exit 2
fi

set +e
NYASH_LLVM_DUMP_IR="$IR_OUT" \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout "$RUN_TIMEOUT_SECS" \
    "$HAKO_ROOT/tools/selfhost/selfhost_build.sh" \
      --in "$APP" \
      --exe "$EXE_OUT" \
      >"$BUILD_LOG" 2>&1
build_rc=$?
set -e

if [ "$build_rc" -ne 0 ]; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: EXE build failed (rc=$build_rc)"
  exit 1
fi

if ! grep -Fq '@"BoxTorrentStore.readData/1"' "$IR_OUT" ||
   ! grep -Fq "@nyash.string.concat_hh" "$IR_OUT"; then
  echo "[INFO] materialize/readData IR snippets:"
  grep -F 'BoxTorrentStore.readData/1' "$IR_OUT" || true
  grep -F 'nyash.string.concat_hh' "$IR_OUT" || true
  test_fail "$SMOKE_NAME: missing StringBox concat route in EXE IR"
  exit 1
fi

set +e
output=$(NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$EXE_OUT" 2>&1 | filter_noise)
run_rc=$?
set -e

if [ "$run_rc" -ne 0 ]; then
  echo "$output"
  test_fail "$SMOKE_NAME: EXE run failed (rc=$run_rc)"
  exit 1
fi

expected=$(cat << 'TXT'
boxtorrent-mini
root=bt-59-458365
chunks=5
bytes=40
dedupe_hits=5
root_equal=true
roundtrip=true
ref_before_release=2
ref_after_release=1
allocator_before_final_release=5
allocator_after_final_release=0
allocator_requested_bytes=40
allocator_releases=5
summary=ok
Result: 0
TXT
)

compare_outputs "$expected" "$output" "$SMOKE_NAME" || exit 1

test_pass "$SMOKE_NAME"
