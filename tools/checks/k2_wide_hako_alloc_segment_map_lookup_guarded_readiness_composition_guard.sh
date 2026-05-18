#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-lookup-guarded-readiness-composition"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"
VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"

APP="apps/hako-alloc-segment-map-lookup-guarded-readiness-composition-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-lookup-guarded-readiness-composition-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-lookup-guarded-readiness-composition-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-673-MIMAP-153A-SEGMENT-MAP-LOOKUP-GUARDED-READINESS-COMPOSITION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-lookup-guarded-readiness-composition-ssot.md"
LOOKUP_SSOT="docs/development/current/main/design/hako-alloc-segment-map-scalar-lookup-boundary-inventory-ssot.md"
MEMBERSHIP_SSOT="docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md"
READINESS_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_map_lookup_guarded_readiness_composition_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh"

printf '[%s] checking MIMAP-153A segment-map lookup guarded readiness composition\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$LOOKUP_SSOT" \
  "$MEMBERSHIP_SSOT" \
  "$READINESS_SSOT" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-153A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-153A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$LOOKUP_SSOT" "lookup SSOT must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$MEMBERSHIP_SSOT" "membership SSOT must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$READINESS_SSOT" "readiness SSOT must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-153A granularity' "$PLAN" "granularity SSOT must describe MIMAP-153A"
guard_expect_in_file "$TAG" 'MIMAP-153A segment-map lookup guarded readiness composition' "$JOINT" "joint order must name MIMAP-153A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-153A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-153A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-153A"
guard_expect_in_file "$TAG" 'memory.segment_map_lookup_guarded_readiness_composition_box = "memory/segment_map_lookup_guarded_readiness_composition_box.hako"' "$MODULE" "hako module must export MIMAP-153A owner"
guard_expect_in_file "$TAG" 'segment_map_lookup_guarded_readiness_composition_box.hako` owns MIMAP-153A' "$MEMORY_README" "memory README must define MIMAP-153A owner"
guard_expect_in_file "$TAG" 'HakoAllocSegmentMapLookupGuardedReadinessComposition' "$ROOT_README" "root README must document MIMAP-153A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentMapLookupGuardedReadinessCompositionReport' "$OWNER" "MIMAP-153A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentMapLookupGuardedReadinessComposition' "$OWNER" "MIMAP-153A owner must exist"
guard_expect_in_file "$TAG" 'composeLookupReadiness' "$OWNER" "MIMAP-153A owner must expose composeLookupReadiness"
guard_expect_in_file "$TAG" 'HakoAllocSegmentMapScalarLookupBoundaryInventory' "$OWNER" "MIMAP-153A must compose lookup owner"
guard_expect_in_file "$TAG" 'HakoAllocSegmentPageMembershipScalar' "$OWNER" "MIMAP-153A must compose membership owner"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationReadinessScalar' "$OWNER" "MIMAP-153A must compose readiness owner"
guard_expect_in_file "$TAG" 'check "mimap153a segment map lookup guarded readiness composition"' "$APP" "MIMAP-153A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-153A must not add real execution, raw pointer lookup, atomics, page-source/OS release seams, or backend-visible release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-153A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-lookup-guarded-readiness-composition-proof|HakoAllocSegmentMapLookupGuardedReadinessComposition|segment_map_lookup_guarded_readiness_composition' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-153A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-153A guard must stay local-run/index-listed by default" >&2
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

tmp_dir="$(mktemp -d /tmp/hakorune_mimap153a_lookup_guarded_readiness.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap153a.mir.json"
exe_out="$tmp_dir/mimap153a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-lookup-guarded-readiness-composition-proof' "$vm_log"
rg -F -q 'composition=1,0,0,0,0,70,7,3,16,1,2,8,3,6' "$vm_log"
rg -F -q 'reject_reasons=1,2,3,1' "$vm_log"
rg -F -q 'subreasons=3,8,2,2' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=5,1,4,2,1,1,1,1' "$vm_log"
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
    "HakoAllocSegmentMapLookupGuardedReadinessComposition.composeLookupReadiness/10",
    "HakoAllocSegmentMapScalarLookupBoundaryInventory.lookupScalar/6",
    "HakoAllocSegmentPageMembershipScalar.classifyMembership/8",
    "HakoAllocSegmentAllocationReadinessScalar.classifyReadiness/7",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocSegmentMapLookupGuardedReadinessComposition",
    "HakoAllocSegmentMapLookupGuardedReadinessCompositionReport",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentMapLookupGuardedReadinessCompositionReport"].get("fields", [])
}
for name in (
    "accepted",
    "reason",
    "lookup_reason",
    "membership_reason",
    "readiness_reason",
    "segment_id",
    "page_id",
    "available_blocks",
    "would_use_raw_pointer",
    "would_use_real_segment_map",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing composition report field: {name}")

print("[mimap153a-mir-json] ok")
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

rg -F -q 'hako-alloc-segment-map-lookup-guarded-readiness-composition-proof' "$run_log"
rg -F -q 'composition=1,0,0,0,0,70,7,3,16,1,2,8,3,6' "$run_log"
rg -F -q 'reject_reasons=1,2,3,1' "$run_log"
rg -F -q 'subreasons=3,8,2,2' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=5,1,4,2,1,1,1,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
