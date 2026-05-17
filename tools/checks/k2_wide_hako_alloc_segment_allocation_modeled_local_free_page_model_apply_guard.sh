#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-page-model-apply"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-614-MIMAP-115A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-PAGE-MODEL-APPLY-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md"
APPLY_PLAN_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md"
PAGE_PILOT_SSOT="docs/development/current/main/design/mimalloc-page-free-list-pilot-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
SOURCE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_model_apply_guard.sh"

printf '[%s] checking MIMAP-115A segment allocation modeled local-free page-model apply route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$APPLY_PLAN_SSOT" \
  "$PAGE_PILOT_SSOT" \
  "$CADENCE" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$SOURCE_OWNER" \
  "$PAGE_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-115A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-115A design must be accepted"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_PILOT_SSOT" "page pilot SSOT must own releaseLocal"
guard_expect_in_file "$TAG" 'L2 proof' "$CADENCE" "validation cadence must define L2 proof rows"
guard_expect_in_file "$TAG" 'MIMAP-111A' "$APPLY_PLAN_SSOT" "page apply must stay downstream of MIMAP-111A"
guard_expect_in_file "$TAG" 'MIMAP-115A' "$PLAN" "granularity SSOT must describe MIMAP-115A"
guard_expect_in_file "$TAG" 'MIMAP-115A segment allocation modeled local-free page-model apply route' "$JOINT" "joint order must name MIMAP-115A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-115A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-115A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-115A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_page_apply_box = "memory/segment_allocation_modeled_local_free_page_apply_box.hako"' "$MODULE" "hako module must export page apply owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_page_apply_box.hako` owns' "$MEMORY_README" "memory README must define MIMAP-115A owner"
guard_expect_in_file "$TAG" 'recordLocalFreePageApply' "$OWNER" "page apply owner must expose record route"
guard_expect_in_file "$TAG" 'releaseLocal' "$OWNER" "page apply owner must call releaseLocal only"
guard_expect_in_file "$TAG" 'blockIsLive' "$OWNER" "page apply owner must preflight live blocks"
guard_expect_in_file "$TAG" 'local_free_apply_plan_ledger_present' "$SOURCE_OWNER" "source apply-plan owner must expose present field"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_OWNER" "page model must own releaseLocal"
guard_expect_in_file "$TAG" 'check "mimap115a segment allocation modeled local-free page-model apply route"' "$APP" "MIMAP-115A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-115A must not add real segment execution, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams, or backend-visible page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-115A owner must not mutate page arrays directly; use HakoAllocPageModel.releaseLocal only" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-115A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof|LocalFreePageApply|recordLocalFreePageApply' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-115A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_model_apply_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-115A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap115a_segment_allocation_local_free_page_apply.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap115a.mir.json"
exe_out="$tmp_dir/mimap115a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 90 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof' "$vm_log"
rg -F -q 'first=1,0,0,-1,0,60018002,60,18,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'missing=0,2,-1,1' "$vm_log"
rg -F -q 'duplicate=0,3,0,1' "$vm_log"
rg -F -q 'wrong_page=0,4,1' "$vm_log"
rg -F -q 'unsupported=0,6,1' "$vm_log"
rg -F -q 'recycled=1,0,1,-1,1,60018002,60,18,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'counts=6,2,2,4,0,1,1,1,0,1' "$vm_log"
rg -F -q 'page=3,2,3,5' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreePageApply.recordLocalFreePageApply/3",
    "HakoAllocSegmentAllocationModeledLocalFreePageApply.findLocalFreePageApplyIndex/2",
    "HakoAllocPageModel.releaseLocal/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreePageApplyReport")
if report is None:
    raise SystemExit("missing local-free page-apply report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_apply_local_free_to_page",
    "local_free_apply_plan_row_index",
    "modeled_allocation_token",
    "applied_blocks",
    "page_used_before",
    "page_used_after",
    "did_mutate_page_local_free_list",
    "would_directly_mutate_page_arrays",
):
    if name not in fields:
        raise SystemExit(f"missing local-free page-apply field: {name}")

print("[mimap115a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof' "$run_log"
rg -F -q 'first=1,0,0,-1,0,60018002,60,18,2,5,3,3,6,3,0,3' "$run_log"
rg -F -q 'missing=0,2,-1,1' "$run_log"
rg -F -q 'duplicate=0,3,0,1' "$run_log"
rg -F -q 'wrong_page=0,4,1' "$run_log"
rg -F -q 'unsupported=0,6,1' "$run_log"
rg -F -q 'recycled=1,0,1,-1,1,60018002,60,18,2,5,3,3,6,3,0,3' "$run_log"
rg -F -q 'counts=6,2,2,4,0,1,1,1,0,1' "$run_log"
rg -F -q 'page=3,2,3,5' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
