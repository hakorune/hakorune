#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-readiness-scalar"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-readiness-scalar-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-readiness-scalar-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-readiness-scalar-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-585-MIMAP-088A-SEGMENT-ALLOCATION-READINESS-SCALAR-CONTRACT.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md"
MEMBERSHIP_SSOT="docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md"
SEGMENT_STATE_SSOT="docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_readiness_scalar_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_readiness_scalar_guard.sh"

printf '[%s] checking MIMAP-088A segment allocation readiness scalar contract\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$MEMBERSHIP_SSOT" \
  "$SEGMENT_STATE_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-088A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-088A design must be accepted"
guard_expect_in_file "$TAG" 'segment_id' "$DESIGN" "MIMAP-088A design must name segment_id"
guard_expect_in_file "$TAG" 'request block count' "$DESIGN" "MIMAP-088A design must name request block count"
guard_expect_in_file "$TAG" 'Decision: accepted' "$MEMBERSHIP_SSOT" "membership SSOT must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$SEGMENT_STATE_SSOT" "segment lifecycle SSOT must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-088A granularity' "$PLAN" "granularity SSOT must describe MIMAP-088A"
guard_expect_in_file "$TAG" 'MIMAP-088A segment allocation readiness scalar contract' "$JOINT" "joint order must name MIMAP-088A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-088A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-088A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-088A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_readiness_scalar_box = "memory/segment_allocation_readiness_scalar_box.hako"' "$MODULE" "hako module must export MIMAP-088A owner"
guard_expect_in_file "$TAG" 'segment_allocation_readiness_scalar_box.hako` owns MIMAP-088A' "$MEMORY_README" "memory README must define MIMAP-088A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationReadinessScalarReport' "$OWNER" "MIMAP-088A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationReadinessScalar' "$OWNER" "MIMAP-088A owner must exist"
guard_expect_in_file "$TAG" 'classifyReadiness' "$OWNER" "MIMAP-088A owner must expose classifyReadiness"
guard_expect_in_file "$TAG" 'supportsAllocationReadiness' "$OWNER" "MIMAP-088A owner must centralize allocation state policy"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationReadinessScalar' "$APP" "MIMAP-088A proof must construct readiness owner"
guard_expect_in_file "$TAG" 'check "mimap088a segment allocation readiness scalar contract"' "$APP" "MIMAP-088A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-088A must not add execution, concurrency, segment-map, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-088A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-readiness-scalar-proof|HakoAllocSegmentAllocationReadinessScalar|segment_allocation_readiness_scalar' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-088A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_readiness_scalar_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-088A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap088a_segment_allocation_readiness.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap088a.mir.json"
exe_out="$tmp_dir/mimap088a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-readiness-scalar-proof' "$vm_log"
rg -F -q 'ready=1,0,60,18,1,2,8,3,6' "$vm_log"
rg -F -q 'accepted=1,1' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=14,2,12,1,1,1,1,1,1,1,1,1,1,1,1,31,12' "$vm_log"
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
    "HakoAllocSegmentAllocationReadinessScalar.classifyReadiness/7",
    "HakoAllocSegmentAllocationReadinessScalar.supportsAllocationReadiness/1",
    "HakoAllocSegmentAllocationReadinessScalar.rejectUnsupportedRequirement/8",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocSegmentAllocationReadinessScalarReport") is None:
    raise SystemExit("missing typed object plan: HakoAllocSegmentAllocationReadinessScalarReport")
if plans.get("HakoAllocSegmentAllocationReadinessScalar") is None:
    raise SystemExit("missing typed object plan: HakoAllocSegmentAllocationReadinessScalar")

fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentAllocationReadinessScalarReport"].get("fields", [])
}
required_fields = {
    "accepted",
    "reason",
    "segment_id",
    "page_id",
    "segment_state",
    "page_used",
    "page_capacity",
    "request_blocks",
    "available_blocks",
    "readiness_contract_present",
    "would_execute_segment_allocation",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_allocate_arena_backing",
    "would_execute_atomic_bitmap",
    "would_call_osvm",
    "would_run_thread",
    "would_activate_provider",
    "would_replace_process_allocator",
    "would_add_backend_matcher",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap088a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-readiness-scalar-proof' "$run_log"
rg -F -q 'ready=1,0,60,18,1,2,8,3,6' "$run_log"
rg -F -q 'accepted=1,1' "$run_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=14,2,12,1,1,1,1,1,1,1,1,1,1,1,1,31,12' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
