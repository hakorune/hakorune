#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-vertical-slice-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-mimalloc-comparison-vertical-slice-closeout-proof/main.hako"
APP_TEST="apps/hako-alloc-mimalloc-comparison-vertical-slice-closeout-proof/test.sh"
APP_README="apps/hako-alloc-mimalloc-comparison-vertical-slice-closeout-proof/README.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
CARD="docs/development/current/main/phases/phase-294x/294x-59-MIMALLOC-COMPARISON-VERTICAL-SLICE-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-294x/294x-58-MIMALLOC-COMPARISON-HUGE-OSVM-SLICE-PILOT.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh"
V2_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh"
V3_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh"
V4_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh"
C_RUNNER_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_guard.sh"

echo "[$TAG] checking hako_alloc mimalloc comparison vertical-slice closeout"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$TASKBOARD" \
  "$CARD" \
  "$PREV_CARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$V2_GUARD" \
  "$V3_GUARD" \
  "$V4_GUARD" \
  "$C_RUNNER_GUARD"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$V2_GUARD" "$V3_GUARD" "$V4_GUARD" "$C_RUNNER_GUARD"

guard_expect_in_file "$TAG" 'schema=vertical-slice-v1' "$APP" "closeout proof must publish the stable vertical-slice schema id"
guard_expect_in_file "$TAG" 'hako_slices=' "$APP" "closeout proof must publish hako slice presence"
guard_expect_in_file "$TAG" 'c_mimalloc=' "$APP" "closeout proof must publish C mimalloc runner evidence fields"
guard_expect_in_file "$TAG" 'schema_bridge=' "$APP" "closeout proof must publish schema bridge fields"
guard_expect_in_file "$TAG" 'closed=' "$APP" "closeout proof must publish closed stop-line fields"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-007' "$CARD" "card must identify V5 blocker token"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-008' "$CARD" "card must select the next follow-on blocker"
guard_expect_in_file "$TAG" 'V5' "$TASKBOARD" "taskboard must track V5 closeout"
guard_expect_in_file "$TAG" "$APP" "$INDEX" "check script index must list the V5 proof app"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

if rg -n 'worker_local|RemoteFree|Atomic|fetch_add|cas_|load_ordered|store_ordered|externcall|hako_mem_|run_benchmark[[:space:]]*\(|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]' \
  "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: vertical-slice closeout leaked beyond comparison stop lines" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-alloc-mimalloc-comparison-vertical-slice-closeout|vertical-slice-v1|HakoAllocMimallocComparisonVerticalSlice' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: vertical-slice closeout leaked app/owner matcher into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

bash "$V2_GUARD"
bash "$V3_GUARD"
bash "$V4_GUARD"
bash "$C_RUNNER_GUARD" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_comparison_vslice_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

vm_log="$tmp_dir/vm.log"
mir_json="$tmp_dir/vslice-closeout.mir.json"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-mimalloc-comparison-vertical-slice-closeout-proof' "$vm_log"
rg -F -q 'schema=vertical-slice-v1' "$vm_log"
rg -F -q 'hako_slices=1,1,1' "$vm_log"
rg -F -q 'hako_requested=48,216,4194321,4194585' "$vm_log"
rg -F -q 'hako_evidence=4194433,7,4,6,6,0' "$vm_log"
rg -F -q 'hako_details=4,16,2' "$vm_log"
rg -F -q 'c_mimalloc=1,1,1,1,64,64,33254,4096,4096,0,1' "$vm_log"
rg -F -q 'schema_bridge=1,1,0,4194585,33254' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
python3 tools/checks/pure_first_route_preflight.py "$mir_json"
python3 - "$mir_json" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "ProofCheck.birth/1",
    "ProofCheck.expectEq/3",
    "ProofCheck.ok/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

unsupported = []
for fn in functions.values():
    for plan in fn.get("metadata", {}).get("lowering_plan", []):
        if plan.get("emit_kind") == "unsupported":
            unsupported.append((fn.get("name"), plan.get("site"), plan.get("symbol"), plan.get("reason")))
if unsupported:
    raise SystemExit(f"unsupported lowering plans remain: {unsupported[:5]}")

print("[mimalloc-comparison-vslice-closeout-mir-json] ok")
PY

cat "$vm_log"

echo "[$TAG] ok"
