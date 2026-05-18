#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-scalar-lookup-boundary-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-671-MIMAP-151A-SEGMENT-MAP-SCALAR-LOOKUP-BOUNDARY-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-scalar-lookup-boundary-inventory-ssot.md"
MATRIX_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-blocked-substrate-matrix-ssot.md"
GAP_LEDGER="docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_map_scalar_lookup_boundary_inventory_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_scalar_lookup_boundary_inventory_guard.sh"

printf '[%s] checking MIMAP-151A segment-map scalar lookup boundary inventory\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$MATRIX_SSOT" \
  "$GAP_LEDGER" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-151A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-151A design must be accepted"
guard_expect_in_file "$TAG" 'GAP-RAWBUF' "$GAP_LEDGER" "gap ledger must keep rawbuf requirement visible"
guard_expect_in_file "$TAG" 'MIMAP-151A granularity' "$PLAN" "granularity SSOT must describe MIMAP-151A"
guard_expect_in_file "$TAG" 'MIMAP-151A segment-map scalar lookup boundary inventory' "$JOINT" "joint order must name MIMAP-151A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-151A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-151A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-151A"
guard_expect_in_file "$TAG" 'memory.segment_map_scalar_lookup_boundary_inventory_box = "memory/segment_map_scalar_lookup_boundary_inventory_box.hako"' "$MODULE" "hako module must export MIMAP-151A owner"
guard_expect_in_file "$TAG" 'segment_map_scalar_lookup_boundary_inventory_box.hako` owns MIMAP-151A' "$MEMORY_README" "memory README must define MIMAP-151A owner"
guard_expect_in_file "$TAG" 'HakoAllocSegmentMapScalarLookupBoundaryInventory' "$ROOT_README" "root README must document MIMAP-151A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentMapScalarLookupBoundaryInventoryReport' "$OWNER" "MIMAP-151A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentMapScalarLookupBoundaryInventory' "$OWNER" "MIMAP-151A owner must exist"
guard_expect_in_file "$TAG" 'lookupScalar' "$OWNER" "MIMAP-151A owner must expose lookupScalar"
guard_expect_in_file "$TAG" 'reasonRawPointerLookup' "$OWNER" "MIMAP-151A owner must reject raw-pointer lookup requests"
guard_expect_in_file "$TAG" 'check "mimap151a segment map scalar lookup boundary inventory"' "$APP" "MIMAP-151A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-151A must not add real execution, raw pointer lookup, atomics, page-source/OS release seams, or backend-visible release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-151A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof|HakoAllocSegmentMapScalarLookupBoundaryInventory|segment_map_scalar_lookup_boundary_inventory' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-151A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_map_scalar_lookup_boundary_inventory_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-151A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap151a_segment_map_scalar_lookup.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap151a.mir.json"
exe_out="$tmp_dir/mimap151a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof' "$vm_log"
rg -F -q 'lookup=1,0,0,70,7,3,16,1' "$vm_log"
rg -F -q 'rejects=3,4,5,6,2' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,2' "$vm_log"
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
    "HakoAllocSegmentMapScalarLookupBoundaryInventory.lookupScalar/6",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocSegmentMapScalarLookupBoundaryInventory",
    "HakoAllocSegmentMapScalarLookupBoundaryInventoryReport",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentMapScalarLookupBoundaryInventoryReport"].get("fields", [])
}
for name in (
    "accepted",
    "reason",
    "row_index",
    "segment_id",
    "page_id",
    "slice_index",
    "generation",
    "would_use_raw_pointer",
    "would_use_real_segment_map",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing lookup report field: {name}")

print("[mimap151a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof' "$run_log"
rg -F -q 'lookup=1,0,0,70,7,3,16,1' "$run_log"
rg -F -q 'rejects=3,4,5,6,2' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
