#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-apply-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-610-MIMAP-111A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-APPLY-PLAN-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md"
CANDIDATE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako"
SOURCE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_apply_plan_guard.sh"

printf '[%s] checking MIMAP-111A segment allocation modeled local-free apply plan route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$CANDIDATE_SSOT" \
  "$CADENCE" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$SOURCE_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-111A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-111A design must be accepted"
guard_expect_in_file "$TAG" 'L2 proof' "$CADENCE" "validation cadence must define L2 proof rows"
guard_expect_in_file "$TAG" 'MIMAP-109A' "$CANDIDATE_SSOT" "apply plan must stay downstream of MIMAP-109A"
guard_expect_in_file "$TAG" 'MIMAP-111A' "$PLAN" "granularity SSOT must describe MIMAP-111A"
guard_expect_in_file "$TAG" 'MIMAP-111A segment allocation modeled local-free apply plan route' "$JOINT" "joint order must name MIMAP-111A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-111A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-111A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-111A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_apply_plan_box = "memory/segment_allocation_modeled_local_free_apply_plan_box.hako"' "$MODULE" "hako module must export local-free apply-plan owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_apply_plan_box.hako` owns' "$MEMORY_README" "memory README must define MIMAP-111A owner"
guard_expect_in_file "$TAG" 'recordLocalFreeApplyPlan' "$OWNER" "local-free apply-plan ledger must expose record route"
guard_expect_in_file "$TAG" 'local_free_candidate_ledger_present' "$SOURCE_OWNER" "source candidate owner must expose present field"
guard_expect_in_file "$TAG" 'check "mimap111a segment allocation modeled local-free apply plan route"' "$APP" "MIMAP-111A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-111A must not add real execution, free-list/page-state mutation, concurrency, segment-map, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-111A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof|LocalFreeApplyPlan|recordLocalFreeApplyPlan' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-111A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_local_free_apply_plan_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-111A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap111a_segment_allocation_local_free_apply_plan.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap111a.mir.json"
exe_out="$tmp_dir/mimap111a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof' "$vm_log"
rg -F -q 'first=1,0,0,-1,0,60018002,60,18,2,5,3,1,1,1' "$vm_log"
rg -F -q 'missing=0,2,-1,1' "$vm_log"
rg -F -q 'duplicate=0,3,0,1' "$vm_log"
rg -F -q 'recycled=1,0,1,-1,1,60018002,60,18,2,5,3,1,2,2' "$vm_log"
rg -F -q 'unsupported=0,5,1' "$vm_log"
rg -F -q 'counts=5,2,2,3,0,1,1,0,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocSegmentAllocationModeledLocalFreeApplyPlan.recordLocalFreeApplyPlan/2",
    "HakoAllocSegmentAllocationModeledLocalFreeApplyPlan.findLocalFreeApplyPlanIndex/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeApplyPlanReport")
if report is None:
    raise SystemExit("missing local-free apply-plan report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_record_local_free_apply_plan",
    "local_free_candidate_row_index",
    "modeled_allocation_token",
    "modeled_block_start",
    "modeled_block_end",
    "candidate_blocks",
    "plan_kind",
    "would_mutate_free_list",
    "would_mutate_page_state",
):
    if name not in fields:
        raise SystemExit(f"missing local-free apply-plan field: {name}")

print("[mimap111a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof' "$run_log"
rg -F -q 'first=1,0,0,-1,0,60018002,60,18,2,5,3,1,1,1' "$run_log"
rg -F -q 'missing=0,2,-1,1' "$run_log"
rg -F -q 'duplicate=0,3,0,1' "$run_log"
rg -F -q 'recycled=1,0,1,-1,1,60018002,60,18,2,5,3,1,2,2' "$run_log"
rg -F -q 'unsupported=0,5,1' "$run_log"
rg -F -q 'counts=5,2,2,3,0,1,1,0,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
