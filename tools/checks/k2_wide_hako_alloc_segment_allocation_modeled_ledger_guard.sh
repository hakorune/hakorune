#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-ledger"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-ledger-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-ledger-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-591-MIMAP-094A-SEGMENT-ALLOCATION-MODELED-LEDGER-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md"
CONSUME_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-consume-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
HELPER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_report_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
CONSUME_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_consume_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_guard.sh"

printf '[%s] checking MIMAP-094A segment allocation modeled ledger route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$CONSUME_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$HELPER" \
  "$OWNER" \
  "$CONSUME_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-094A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-094A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-091A' "$CONSUME_SSOT" "ledger must stay downstream of modeled consume"
guard_expect_in_file "$TAG" 'MIMAP-094A' "$PLAN" "granularity SSOT must describe MIMAP-094A"
guard_expect_in_file "$TAG" 'MIMAP-094A segment allocation modeled ledger route' "$JOINT" "joint order must name MIMAP-094A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-094A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-094A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-094A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_box = "memory/segment_allocation_modeled_ledger_box.hako"' "$MODULE" "hako module must export MIMAP-094A owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_box.hako` owns MIMAP-094A' "$MEMORY_README" "memory README must define MIMAP-094A owner"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_report_box = "memory/segment_allocation_modeled_ledger_report_box.hako"' "$MODULE" "hako module must export MIMAP-094A report helper"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-094A report capsules' "$MEMORY_README" "memory README must define MIMAP-094A report helper"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.segment_allocation_modeled_ledger_report_box as HakoAllocSegmentAllocationModeledLedgerReportBox' "$OWNER" "MIMAP-094A owner must import report helper"
guard_expect_fixed_in_file "$TAG" 'report(accepted, reason, row_index, existing_index, segment_id, page_id, old_page_used, page_capacity, request_blocks, new_page_used, remaining_blocks, modeled_block_start, modeled_allocation_token)' "$OWNER" "MIMAP-094A owner must expose direct report builder"
guard_expect_fixed_in_file "$TAG" 'rejectUnsupportedRequirement(requirement, segment_id, page_id, old_page_used, page_capacity, request_blocks, new_page_used, remaining_blocks, modeled_block_start, modeled_allocation_token)' "$OWNER" "MIMAP-094A owner must expose direct reject helper"
guard_expect_fixed_in_file "$TAG" 'releaseReport(did_release, reason, row_index, modeled_allocation_token, segment_id, page_id, modeled_block_start, live_before, live_after)' "$OWNER" "MIMAP-094A owner must expose direct release report builder"
guard_expect_fixed_in_file "$TAG" 'releaseRejectUnsupportedRequirement(requirement, modeled_allocation_token)' "$OWNER" "MIMAP-094A owner must expose direct release reject helper"
if rg -n 'report_surface|HakoAllocSegmentAllocationModeledLedgerReportSurface' "$OWNER" >/tmp/"$TAG".surface_leak 2>&1; then
  echo "[$TAG] ERROR: report surface dispatch must stay removed" >&2
  cat /tmp/"$TAG".surface_leak >&2
  rm -f /tmp/"$TAG".surface_leak
  exit 1
fi
rm -f /tmp/"$TAG".surface_leak
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationModeledLedger' "$OWNER" "MIMAP-094A owner must exist"
guard_expect_in_file "$TAG" 'recordModeledConsume' "$OWNER" "MIMAP-094A owner must expose recordModeledConsume"
guard_expect_in_file "$TAG" 'findIndex' "$OWNER" "MIMAP-094A owner must expose token lookup"
guard_expect_in_file "$TAG" 'makeModeledToken' "$OWNER" "MIMAP-094A owner must centralize token construction"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationModeledLedgerReport' "$HELPER" "MIMAP-094A report box must live in helper"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationModeledLedger' "$APP" "MIMAP-094A proof must construct ledger owner"
guard_expect_in_file "$TAG" 'check "mimap094a segment allocation modeled ledger route"' "$APP" "MIMAP-094A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-094A must not add real execution, concurrency, segment-map, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-094A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-ledger-proof|HakoAllocSegmentAllocationModeledLedger|segment_allocation_modeled_ledger' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-094A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_ledger_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-094A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap094a_segment_allocation_ledger.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap094a.mir.json"
exe_out="$tmp_dir/mimap094a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-ledger-proof' "$vm_log"
rg -F -q 'first=1,0,0,-1,60018002,60,18,2,3,5,3,2,1,1' "$vm_log"
rg -F -q 'second=1,0,1,-1,61019007,61,19,7,1,8,0,7,2,2' "$vm_log"
rg -F -q 'finds=0,1,-1' "$vm_log"
rg -F -q 'reads=60018002,61,19,7,8,0' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12,13' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=15,2,13,1,1,1,1,1,1,1,1,1,1,1,1,1,72031002,13,2,2' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLedger.recordModeledConsume/12",
    "HakoAllocSegmentAllocationModeledLedger.findIndex/1",
    "HakoAllocSegmentAllocationModeledLedger.appendRow/9",
    "HakoAllocSegmentAllocationModeledLedger.makeModeledToken/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocSegmentAllocationModeledLedgerReport", "HakoAllocSegmentAllocationModeledLedger"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentAllocationModeledLedgerReport"].get("fields", [])
}
required_report_fields = {
    "accepted",
    "reason",
    "row_index",
    "existing_index",
    "segment_id",
    "page_id",
    "old_page_used",
    "page_capacity",
    "request_blocks",
    "new_page_used",
    "remaining_blocks",
    "modeled_block_start",
    "modeled_allocation_token",
    "ledger_count_after",
    "ledger_live_count_after",
    "modeled_ledger_present",
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
missing_fields = sorted(name for name in required_report_fields if name not in report_fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_report_fields:
    field = report_fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

ledger_fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentAllocationModeledLedger"].get("fields", [])
}
for name in ("tokens", "segment_ids", "page_ids", "block_starts", "live_flags"):
    if name not in ledger_fields:
        raise SystemExit(f"missing ledger column field: {name}")

print("[mimap094a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-ledger-proof' "$run_log"
rg -F -q 'first=1,0,0,-1,60018002,60,18,2,3,5,3,2,1,1' "$run_log"
rg -F -q 'second=1,0,1,-1,61019007,61,19,7,1,8,0,7,2,2' "$run_log"
rg -F -q 'finds=0,1,-1' "$run_log"
rg -F -q 'reads=60018002,61,19,7,8,0' "$run_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12,13' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=15,2,13,1,1,1,1,1,1,1,1,1,1,1,1,1,72031002,13,2,2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
