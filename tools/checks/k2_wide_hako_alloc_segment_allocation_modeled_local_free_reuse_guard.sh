#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-reuse"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-632-MIMAP-126A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-ROUTE.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako"
INTEGRATION_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_guard.sh"

printf '[%s] checking MIMAP-126A segment allocation modeled local-free reuse route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$INTEGRATION_OWNER" \
  "$PAGE_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-126A card must be landed"
guard_expect_in_file "$TAG" 'MIMAP-126A' "$PLAN" "granularity SSOT must describe MIMAP-126A"
guard_expect_in_file "$TAG" 'MIMAP-126A segment allocation modeled local-free reuse route' "$JOINT" "joint order must name MIMAP-126A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-126A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-126A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-126A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_box = "memory/segment_allocation_modeled_local_free_reuse_box.hako"' "$MODULE" "hako module must export reuse owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_box.hako` owns' "$MEMORY_README" "memory README must define MIMAP-126A owner"
guard_expect_in_file "$TAG" 'integrateAndReuseLocalFree' "$OWNER" "reuse owner must expose local-free reuse route"
guard_expect_in_file "$TAG" 'integrateLocalFree' "$OWNER" "reuse owner must compose integration route"
guard_expect_in_file "$TAG" 'page[.]acquire' "$OWNER" "reuse owner must reuse through page acquire"
guard_expect_in_file "$TAG" 'local_free_collect_count' "$OWNER" "reuse owner must observe local-free collection"
guard_expect_in_file "$TAG" 'check "mimap126a segment allocation modeled local-free reuse route"' "$APP" "MIMAP-126A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-126A must not add real segment execution, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams, or backend-visible page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-126A owner must not mutate page arrays directly" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-126A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-reuse-proof|LocalFreeReuse|integrateAndReuseLocalFree' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-126A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-126A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap126a_segment_allocation_local_free_reuse.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap126a.mir.json"
exe_out="$tmp_dir/mimap126a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-reuse-proof' "$vm_log"
rg -F -q 'reuse=1,0,4,5,6,0,0,3,2,1' "$vm_log"
rg -F -q 'integration=0,60018002,60,18,2,5,3' "$vm_log"
rg -F -q 'missing=0,1,1' "$vm_log"
rg -F -q 'partial=0,2,2' "$vm_log"
rg -F -q 'unsupported=0,1,1' "$vm_log"
rg -F -q 'counts=4,1,3,2,1,0,0' "$vm_log"
rg -F -q 'page=6,0,2,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuse.integrateAndReuseLocalFree/6",
    "HakoAllocSegmentAllocationModeledLocalFreeReuse.rejectFreeListNotEmpty/2",
    "HakoAllocSegmentAllocationModeledLocalFreeIntegration.integrateLocalFree/5",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseReport")
if report is None:
    raise SystemExit("missing local-free reuse report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_reuse_local_free",
    "reused_block_id",
    "page_local_free_before_reuse",
    "collect_count_after_reuse",
    "did_collect_local_free_before_reuse",
    "would_directly_mutate_page_arrays",
):
    if name not in fields:
        raise SystemExit(f"missing local-free reuse field: {name}")

print("[mimap126a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-reuse-proof' "$run_log"
rg -F -q 'reuse=1,0,4,5,6,0,0,3,2,1' "$run_log"
rg -F -q 'integration=0,60018002,60,18,2,5,3' "$run_log"
rg -F -q 'missing=0,1,1' "$run_log"
rg -F -q 'partial=0,2,2' "$run_log"
rg -F -q 'unsupported=0,1,1' "$run_log"
rg -F -q 'counts=4,1,3,2,1,0,0' "$run_log"
rg -F -q 'page=6,0,2,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
