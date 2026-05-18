#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-blocked-substrate-matrix"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"
VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"

APP="apps/hako-alloc-segment-allocation-blocked-substrate-matrix-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-blocked-substrate-matrix-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-blocked-substrate-matrix-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-669-MIMAP-149A-SEGMENT-ALLOCATION-BLOCKED-SUBSTRATE-MATRIX-PROOF.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-blocked-substrate-matrix-ssot.md"
READINESS_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md"
MEMBERSHIP_SSOT="docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md"
BOUNDARY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_blocked_substrate_matrix_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_blocked_substrate_matrix_guard.sh"

printf '[%s] checking MIMAP-149A segment allocation blocked-substrate matrix proof\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$READINESS_SSOT" \
  "$MEMBERSHIP_SSOT" \
  "$BOUNDARY_SSOT" \
  "$CADENCE" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$ROOT_README" \
  "$MODULE" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-149A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-149A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$READINESS_SSOT" "readiness SSOT must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$MEMBERSHIP_SSOT" "membership SSOT must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$BOUNDARY_SSOT" "boundary SSOT must stay accepted"
guard_expect_in_file "$TAG" 'L2 MIR contract' "$CADENCE" "validation cadence must define L2 MIR rows"
guard_expect_in_file "$TAG" 'MIMAP-149A granularity' "$PLAN" "granularity SSOT must describe MIMAP-149A"
guard_expect_in_file "$TAG" 'MIMAP-149A segment allocation blocked-substrate matrix proof' "$JOINT" "joint order must name MIMAP-149A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-149A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-149A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-149A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_blocked_substrate_matrix_box = "memory/segment_allocation_blocked_substrate_matrix_box.hako"' "$MODULE" "hako module must export MIMAP-149A owner"
guard_expect_in_file "$TAG" 'segment_allocation_blocked_substrate_matrix_box.hako` owns MIMAP-149A' "$MEMORY_README" "memory README must define MIMAP-149A owner"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationBlockedSubstrateMatrix' "$ROOT_README" "root README must document MIMAP-149A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationBlockedSubstrateMatrixReport' "$OWNER" "MIMAP-149A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationBlockedSubstrateMatrix' "$OWNER" "MIMAP-149A owner must exist"
guard_expect_in_file "$TAG" 'buildMatrix' "$OWNER" "MIMAP-149A owner must expose buildMatrix"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationReadinessScalar' "$OWNER" "MIMAP-149A must compose readiness facts"
guard_expect_in_file "$TAG" 'HakoAllocSegmentPageMembershipScalar' "$OWNER" "MIMAP-149A must compose membership facts"
guard_expect_in_file "$TAG" 'HakoAllocSegmentArenaBitmapInventory' "$OWNER" "MIMAP-149A must compose boundary facts"
guard_expect_in_file "$TAG" 'check "mimap149a segment allocation blocked substrate matrix proof"' "$APP" "MIMAP-149A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-149A must not add real execution, concurrency, segment lookup, atomics, page-source/OS release seams, or backend-visible release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-149A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-blocked-substrate-matrix-proof|HakoAllocSegmentAllocationBlockedSubstrateMatrix|segment_allocation_blocked_substrate_matrix' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-149A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_blocked_substrate_matrix_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-149A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_build_hakorune_debug

tmp_dir="$(mktemp -d /tmp/hakorune_mimap149a_segment_allocation_blocked_matrix.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap149a.mir.json"
exe_out="$tmp_dir/mimap149a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-blocked-substrate-matrix-proof' "$vm_log"
rg -F -q 'matrix=0,8,3,255' "$vm_log"
rg -F -q 'accepted_reasons=0,0,0' "$vm_log"
rg -F -q 'blocker_reasons=2,3,4,3,4,10,5,12' "$vm_log"
rg -F -q 'blockers=1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=1,0,255' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

if ! pure_first_guard_level_allows_mir "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

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
    "HakoAllocSegmentAllocationBlockedSubstrateMatrix.buildMatrix/0",
    "HakoAllocSegmentAllocationReadinessScalar.classifyReadiness/7",
    "HakoAllocSegmentPageMembershipScalar.classifyMembership/8",
    "HakoAllocSegmentArenaBitmapInventory.classifyBoundary/9",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocSegmentAllocationBlockedSubstrateMatrix",
    "HakoAllocSegmentAllocationBlockedSubstrateMatrixReport",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentAllocationBlockedSubstrateMatrixReport"].get("fields", [])
}
for name in (
    "matrix_rows",
    "accepted_surface_count",
    "blocker_mask",
    "raw_pointer_reason",
    "segment_map_reason",
    "arena_backing_reason",
    "atomic_bitmap_reason",
    "osvm_reason",
    "thread_reason",
    "provider_reason",
    "segment_execution_reason",
    "would_execute_segment_allocation",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing matrix report field: {name}")

print("[mimap149a-mir-json] ok")
PY

if ! pure_first_guard_level_allows_exe "$VALIDATION_LEVEL"; then
  pure_first_guard_route_preflight "$TAG" "$ROOT_DIR" "$mir_json" "$build_log"
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_build_toolchain
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-blocked-substrate-matrix-proof' "$run_log"
rg -F -q 'matrix=0,8,3,255' "$run_log"
rg -F -q 'accepted_reasons=0,0,0' "$run_log"
rg -F -q 'blocker_reasons=2,3,4,3,4,10,5,12' "$run_log"
rg -F -q 'blockers=1,1,1,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=1,0,255' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
