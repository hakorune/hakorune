#!/bin/bash
# phase21_5_perf_mir_shape_contract_vm.sh
#
# Contract pin:
# - perf microbench sources emit non-collapsed MIR JSON.
# - loop/control-flow scaffold and hot-path ops stay present.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_mir_shape_contract_vm"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
# Keep legacy-stable behavior by default; direct route can be probed via EMIT_ROUTE_KIND=direct.
EMIT_ROUTE_KIND="${EMIT_ROUTE_KIND:-hako-helper}"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"

if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
if [ "$EMIT_ROUTE_KIND" != "direct" ] && [ "$EMIT_ROUTE_KIND" != "hako-mainline" ] && [ "$EMIT_ROUTE_KIND" != "hako-helper" ]; then
  test_fail "$SMOKE_NAME: EMIT_ROUTE_KIND must be direct|hako-mainline|hako-helper (actual=$EMIT_ROUTE_KIND)"
  exit 2
fi
if ! [[ "$EMIT_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: EMIT_TIMEOUT_SECS must be integer: $EMIT_TIMEOUT_SECS"
  exit 2
fi

count_op() {
  local mir_path="$1"
  local op="$2"
  jq "[.functions[]?.blocks[]?.instructions[]? | select(.op == \"$op\")] | length" "$mir_path"
}

run_case() {
  local case_name="$1"
  local input="$2"
  local tmp_mir tmp_log
  tmp_mir="$(mktemp "/tmp/${SMOKE_NAME}_${case_name}.XXXXXX.json")"
  tmp_log="$(mktemp "/tmp/${SMOKE_NAME}_${case_name}.XXXXXX.log")"

  cleanup_case() {
    rm -f "$tmp_mir" "$tmp_log" >/dev/null 2>&1 || true
  }
  trap cleanup_case RETURN

  if [ ! -f "$input" ]; then
    test_fail "$SMOKE_NAME($case_name): missing input: $input"
    return 1
  fi

  set +e
  "$EMIT_ROUTE" --route "$EMIT_ROUTE_KIND" --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$tmp_mir" --input "$input" >"$tmp_log" 2>&1
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    tail -n 40 "$tmp_log" || true
    test_fail "$SMOKE_NAME($case_name): emit failed rc=$rc"
    return 1
  fi

  if ! jq -e '.functions | length >= 1' "$tmp_mir" >/dev/null; then
    test_fail "$SMOKE_NAME($case_name): MIR missing functions payload"
    return 1
  fi

  local blocks branch compare jump phi ret mir_call boxcall call_like newbox collapsed
  blocks="$(jq '[.functions[]?.blocks[]?] | length' "$tmp_mir")"
  branch="$(count_op "$tmp_mir" "branch")"
  compare="$(count_op "$tmp_mir" "compare")"
  jump="$(count_op "$tmp_mir" "jump")"
  phi="$(count_op "$tmp_mir" "phi")"
  ret="$(count_op "$tmp_mir" "ret")"
  mir_call="$(count_op "$tmp_mir" "mir_call")"
  boxcall="$(count_op "$tmp_mir" "boxcall")"
  call_like=$((mir_call + boxcall))
  newbox="$(count_op "$tmp_mir" "newbox")"

  collapsed=0
  if [ "$blocks" -le 1 ] && [ "$branch" -eq 0 ] && [ "$compare" -eq 0 ]; then
    collapsed=1
  fi

  if [ "$collapsed" -ne 0 ]; then
    test_fail "$SMOKE_NAME($case_name): collapsed MIR detected"
    return 1
  fi
  if [ "$blocks" -lt 4 ]; then
    test_fail "$SMOKE_NAME($case_name): expected blocks>=4, got $blocks"
    return 1
  fi
  if [ "$branch" -lt 1 ]; then
    test_fail "$SMOKE_NAME($case_name): expected branch>=1, got $branch"
    return 1
  fi
  if [ "$compare" -lt 1 ]; then
    test_fail "$SMOKE_NAME($case_name): expected compare>=1, got $compare"
    return 1
  fi
  if [ "$jump" -lt 1 ]; then
    test_fail "$SMOKE_NAME($case_name): expected jump>=1, got $jump"
    return 1
  fi
  if [ "$phi" -lt 1 ]; then
    test_fail "$SMOKE_NAME($case_name): expected phi>=1, got $phi"
    return 1
  fi
  if [ "$ret" -lt 1 ]; then
    test_fail "$SMOKE_NAME($case_name): expected ret>=1, got $ret"
    return 1
  fi
  if [ "$call_like" -lt 1 ]; then
    test_fail "$SMOKE_NAME($case_name): expected call-like op>=1, got mir_call=$mir_call boxcall=$boxcall"
    return 1
  fi
  if [ "$newbox" -lt 1 ]; then
    test_fail "$SMOKE_NAME($case_name): expected newbox>=1, got $newbox"
    return 1
  fi

  log_info "$SMOKE_NAME($case_name): blocks=$blocks branch=$branch compare=$compare jump=$jump phi=$phi mir_call=$mir_call boxcall=$boxcall newbox=$newbox ret=$ret"
  return 0
}

run_case "method_call_only_small" "$NYASH_ROOT/benchmarks/bench_method_call_only_small.hako" || exit 1
run_case "box_create_destroy_small" "$NYASH_ROOT/benchmarks/bench_box_create_destroy_small.hako" || exit 1

test_pass "$SMOKE_NAME"
