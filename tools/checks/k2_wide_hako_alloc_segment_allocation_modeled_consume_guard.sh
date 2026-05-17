#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-consume"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-consume-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-consume-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-consume-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-588-MIMAP-091A-SEGMENT-ALLOCATION-MODELED-CONSUME-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-consume-ssot.md"
READINESS_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_consume_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_consume_guard.sh"

printf '[%s] checking MIMAP-091A segment allocation modeled consume route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$READINESS_SSOT" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-091A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-091A design must be accepted"
guard_expect_in_file "$TAG" 'accepted segment allocation-readiness fact' "$CARD" "MIMAP-091A card must connect to readiness facts"
guard_expect_in_file "$TAG" 'MIMAP-091A' "$PLAN" "granularity SSOT must describe MIMAP-091A"
guard_expect_in_file "$TAG" 'MIMAP-091A segment allocation modeled consume route' "$JOINT" "joint order must name MIMAP-091A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-091A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-091A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-091A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_consume_box = "memory/segment_allocation_modeled_consume_box.hako"' "$MODULE" "hako module must export MIMAP-091A owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_consume_box.hako` owns MIMAP-091A' "$MEMORY_README" "memory README must define MIMAP-091A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationModeledConsumeReport' "$OWNER" "MIMAP-091A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationModeledConsume' "$OWNER" "MIMAP-091A owner must exist"
guard_expect_in_file "$TAG" 'consumeReadiness' "$OWNER" "MIMAP-091A owner must expose consumeReadiness"
guard_expect_in_file "$TAG" 'makeModeledToken' "$OWNER" "MIMAP-091A owner must centralize token construction"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationModeledConsume' "$APP" "MIMAP-091A proof must construct consume owner"
guard_expect_in_file "$TAG" 'check "mimap091a segment allocation modeled consume route"' "$APP" "MIMAP-091A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-091A must not add real execution, concurrency, segment-map, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-091A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-consume-proof|HakoAllocSegmentAllocationModeledConsume|segment_allocation_modeled_consume' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-091A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_consume_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-091A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap091a_segment_allocation_consume.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap091a.mir.json"
exe_out="$tmp_dir/mimap091a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-consume-proof' "$vm_log"
rg -F -q 'success=1,0,60,18,2,3,5,3,2,60018002' "$vm_log"
rg -F -q 'edge=1,0,61,19,7,1,8,0,7,61019007' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=14,2,12,1,1,1,1,1,1,1,1,1,1,1,1,31,12,3' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledConsume.consumeReadiness/8",
    "HakoAllocSegmentAllocationModeledConsume.rejectUnsupportedRequirement/9",
    "HakoAllocSegmentAllocationModeledConsume.makeModeledToken/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocSegmentAllocationModeledConsumeReport", "HakoAllocSegmentAllocationModeledConsume"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentAllocationModeledConsumeReport"].get("fields", [])
}
required_fields = {
    "accepted",
    "reason",
    "upstream_reason",
    "segment_id",
    "page_id",
    "old_page_used",
    "page_capacity",
    "request_blocks",
    "new_page_used",
    "remaining_blocks",
    "modeled_block_start",
    "modeled_allocation_token",
    "modeled_consume_present",
    "would_execute_real_segment_allocation",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_allocate_arena_backing",
    "would_execute_atomic_bitmap",
    "would_call_osvm",
    "would_run_thread",
    "would_activate_provider",
    "would_add_backend_matcher",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap091a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-consume-proof' "$run_log"
rg -F -q 'success=1,0,60,18,2,3,5,3,2,60018002' "$run_log"
rg -F -q 'edge=1,0,61,19,7,1,8,0,7,61019007' "$run_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=14,2,12,1,1,1,1,1,1,1,1,1,1,1,1,31,12,3' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
