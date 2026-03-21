#!/bin/bash
# phase21_5_perf_apps_emit_mir_jsonfile_route_contract_vm.sh
#
# Contract pin:
# - App emitted by `--emit-mir-json` must execute via `--mir-json-file` on VM.
# - Output SUMMARY must match source route output.
# - Route must not fail with undefined ValueId errors.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_emit_mir_jsonfile_route_contract_vm"

source "$(dirname "$0")/../../../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../../../lib/perf_vm_env.sh"
require_env || exit 2

APP="$NYASH_ROOT/apps/tools/mir_shape_guard/main.hako"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
SOURCE_HAKO="$NYASH_ROOT/benchmarks/bench_method_call_only_small.hako"

if [ ! -f "$APP" ]; then
  test_fail "$SMOKE_NAME: app not found: $APP"
  exit 2
fi
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
if [ ! -f "$SOURCE_HAKO" ]; then
  test_fail "$SMOKE_NAME: source fixture not found: $SOURCE_HAKO"
  exit 2
fi

TMP_APP_MIR="$(mktemp /tmp/phase21_5_perf_app_emit_route.XXXXXX.app.json)"
TMP_INPUT_MIR="$(mktemp /tmp/phase21_5_perf_app_emit_route.XXXXXX.input.json)"
TMP_ERR="$(mktemp /tmp/phase21_5_perf_app_emit_route.XXXXXX.err.log)"
cleanup() {
  rm -f "$TMP_APP_MIR" "$TMP_INPUT_MIR" "$TMP_ERR"
}
trap cleanup EXIT

"$EMIT_ROUTE" --route hako-helper --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$TMP_INPUT_MIR" --input "$SOURCE_HAKO" >/dev/null 2>"$TMP_ERR" || {
  cat "$TMP_ERR"
  test_fail "$SMOKE_NAME: failed to emit input MIR fixture"
  exit 1
}

"$NYASH_BIN" --emit-mir-json "$TMP_APP_MIR" "$APP" >/dev/null 2>"$TMP_ERR" || {
  cat "$TMP_ERR"
  test_fail "$SMOKE_NAME: failed to emit app MIR via --emit-mir-json"
  exit 1
}

set +e
source_out="$(
  MIR_SHAPE_INPUT="$TMP_INPUT_MIR" \
  MIR_SHAPE_STRICT=1 \
  perf_vm_lane_run "$NYASH_BIN" --backend vm "$APP" 2>&1
)"
source_rc=$?
set -e
if [ "$source_rc" -ne 0 ]; then
  echo "$source_out"
  test_fail "$SMOKE_NAME: source route execution failed (rc=$source_rc)"
  exit 1
fi
source_summary="$(printf '%s\n' "$source_out" | rg -m1 '^SUMMARY ' || true)"
if [ -z "$source_summary" ]; then
  echo "$source_out"
  test_fail "$SMOKE_NAME: source route missing SUMMARY line"
  exit 1
fi

set +e
mir_out="$(
  MIR_SHAPE_INPUT="$TMP_INPUT_MIR" \
  MIR_SHAPE_STRICT=1 \
  perf_vm_lane_run "$NYASH_BIN" --backend vm --mir-json-file "$TMP_APP_MIR" 2>&1
)"
mir_rc=${PIPESTATUS[0]}
set -e
if [ "$mir_rc" -ne 0 ]; then
  echo "$mir_out"
  test_fail "$SMOKE_NAME: --mir-json-file route failed (rc=$mir_rc)"
  exit 1
fi
if printf '%s\n' "$mir_out" | rg -q 'undefined value|Invalid value'; then
  echo "$mir_out"
  test_fail "$SMOKE_NAME: route hit undefined value / invalid value"
  exit 1
fi
mir_summary="$(printf '%s\n' "$mir_out" | rg -m1 '^SUMMARY ' || true)"
if [ -z "$mir_summary" ]; then
  echo "$mir_out"
  test_fail "$SMOKE_NAME: --mir-json-file route missing SUMMARY line"
  exit 1
fi

compare_outputs "$source_summary" "$mir_summary" "$SMOKE_NAME" || exit 1
test_pass "$SMOKE_NAME"
