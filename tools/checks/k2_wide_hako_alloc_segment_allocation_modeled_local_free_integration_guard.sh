#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-integration"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-618-MIMAP-119A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-INTEGRATION-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-integration-ssot.md"
PAGE_APPLY_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako"
PAGE_APPLY_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh"

printf '[%s] checking MIMAP-119A segment allocation modeled local-free integration route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$PAGE_APPLY_SSOT" \
  "$CADENCE" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$PAGE_APPLY_OWNER" \
  "$PAGE_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-119A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-119A design must be accepted"
guard_expect_in_file "$TAG" 'L2 proof' "$CADENCE" "validation cadence must define L2 proof rows"
guard_expect_in_file "$TAG" 'MIMAP-119A' "$PLAN" "granularity SSOT must describe MIMAP-119A"
guard_expect_in_file "$TAG" 'MIMAP-119A segment allocation modeled local-free integration route' "$JOINT" "joint order must name MIMAP-119A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-119A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-119A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-119A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_integration_box = "memory/segment_allocation_modeled_local_free_integration_box.hako"' "$MODULE" "hako module must export integration owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_integration_box.hako` owns' "$MEMORY_README" "memory README must define MIMAP-119A owner"
guard_expect_in_file "$TAG" 'integrateLocalFree' "$OWNER" "integration owner must expose integration route"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields' "$OWNER" "integration owner must group report scalars in a record payload"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields \{' "$OWNER" "integration owner must construct report field records locally"
guard_expect_in_file "$TAG" 'result.did_integrate_local_free = fields.did_integrate_local_free' "$OWNER" "integration owner must copy record fields into the public report box"
guard_expect_in_file "$TAG" 'local candidate_result: Result<i64, i64>' "$OWNER" "integration owner must keep the Result pilot boundary local to integrateLocalFree"
guard_expect_in_file "$TAG" 'guard let Result::Ok\(candidate_row_index\) = candidate_result else' "$OWNER" "integration owner must consume the Result pilot through guard-let"
guard_expect_in_file "$TAG" 'recordLocalFreeCandidate' "$OWNER" "integration owner must compose candidate ledger"
guard_expect_in_file "$TAG" 'recordLocalFreeApplyPlan' "$OWNER" "integration owner must compose apply plan ledger"
guard_expect_in_file "$TAG" 'recordLocalFreePageApply' "$OWNER" "integration owner must compose page apply route"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_APPLY_OWNER" "page apply owner must call releaseLocal only"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_OWNER" "page model must own releaseLocal"
guard_expect_in_file "$TAG" 'check "mimap119a segment allocation modeled local-free integration route"' "$APP" "MIMAP-119A proof must use labelled check block"

if rg -n '(^[[:space:]]*report[[:space:]]*\(|me\.report[[:space:]]*\()' "$OWNER" >/tmp/"$TAG".report_helper_leak 2>&1; then
  echo "[$TAG] ERROR: local-free integration report must not use the legacy scalar report(...) helper boundary" >&2
  cat /tmp/"$TAG".report_helper_leak >&2
  rm -f /tmp/"$TAG".report_helper_leak
  exit 1
fi
rm -f /tmp/"$TAG".report_helper_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-119A must not add real segment execution, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams, or backend-visible page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-119A owner must not mutate page arrays directly" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-119A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-integration-proof|LocalFreeIntegration|integrateLocalFree' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-119A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-119A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap119a_segment_allocation_local_free_integration.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap119a.mir.json"
exe_out="$tmp_dir/mimap119a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-integration-proof' "$vm_log"
rg -F -q 'first=1,0,0,0,0,60018002,60,18,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'missing=0,1,2' "$vm_log"
rg -F -q 'duplicate=0,1,3' "$vm_log"
rg -F -q 'wrong_page=0,3,4' "$vm_log"
rg -F -q 'unsupported=0,1,4' "$vm_log"
rg -F -q 'recycled=1,0,2,2,1,3,3' "$vm_log"
rg -F -q 'counts=6,2,4,3,0,1,3,3,2' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeIntegration.integrateLocalFree/5",
    "HakoAllocSegmentAllocationModeledLocalFreeIntegration.rejectFromCandidate/1",
    "HakoAllocSegmentAllocationModeledLocalFreePageApply.recordLocalFreePageApply/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeIntegrationReport")
if report is None:
    raise SystemExit("missing local-free integration report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_integrate_local_free",
    "candidate_row_index",
    "apply_plan_row_index",
    "page_apply_row_index",
    "did_mutate_page_local_free_list",
    "would_directly_mutate_page_arrays",
):
    if name not in fields:
        raise SystemExit(f"missing local-free integration field: {name}")

print("[mimap119a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-integration-proof' "$run_log"
rg -F -q 'first=1,0,0,0,0,60018002,60,18,2,5,3,3,6,3,0,3' "$run_log"
rg -F -q 'missing=0,1,2' "$run_log"
rg -F -q 'duplicate=0,1,3' "$run_log"
rg -F -q 'wrong_page=0,3,4' "$run_log"
rg -F -q 'unsupported=0,1,4' "$run_log"
rg -F -q 'recycled=1,0,2,2,1,3,3' "$run_log"
rg -F -q 'counts=6,2,4,3,0,1,3,3,2' "$run_log"
rg -F -q 'page=3,2,3,5' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
