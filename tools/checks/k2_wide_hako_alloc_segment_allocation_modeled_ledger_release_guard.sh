#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-ledger-release"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-ledger-release-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-ledger-release-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-ledger-release-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-594-MIMAP-097A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASE-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
HELPER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_report_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard.sh"

printf '[%s] checking MIMAP-097A segment allocation modeled ledger release route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$LEDGER_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$HELPER" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-097A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-097A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-094A' "$LEDGER_SSOT" "release must stay downstream of modeled ledger"
guard_expect_in_file "$TAG" 'MIMAP-097A' "$PLAN" "granularity SSOT must describe MIMAP-097A"
guard_expect_in_file "$TAG" 'MIMAP-097A segment allocation modeled ledger release route' "$JOINT" "joint order must name MIMAP-097A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-097A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-097A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-097A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_box = "memory/segment_allocation_modeled_ledger_box.hako"' "$MODULE" "hako module must export modeled ledger owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_box.hako` owns MIMAP-097A' "$MEMORY_README" "memory README must define MIMAP-097A owner"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_report_box = "memory/segment_allocation_modeled_ledger_report_box.hako"' "$MODULE" "hako module must export MIMAP-097A report helper"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-097A report capsules' "$MEMORY_README" "memory README must define MIMAP-097A report helper"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.segment_allocation_modeled_ledger_report_box as HakoAllocSegmentAllocationModeledLedgerReportBox' "$OWNER" "MIMAP-097A owner must import report helper"
guard_expect_fixed_in_file "$TAG" 'report(accepted, reason, row_index, existing_index, segment_id, page_id, old_page_used, page_capacity, request_blocks, new_page_used, remaining_blocks, modeled_block_start, modeled_allocation_token)' "$OWNER" "MIMAP-097A owner must expose direct report builder"
guard_expect_fixed_in_file "$TAG" 'releaseReport(did_release, reason, row_index, modeled_allocation_token, segment_id, page_id, modeled_block_start, live_before, live_after)' "$OWNER" "MIMAP-097A owner must expose direct release report builder"
guard_expect_fixed_in_file "$TAG" 'releaseRejectUnsupportedRequirement(requirement, modeled_allocation_token)' "$OWNER" "MIMAP-097A owner must expose direct release reject helper"
if rg -n 'report_surface|HakoAllocSegmentAllocationModeledLedgerReportSurface' "$OWNER" >/tmp/"$TAG".surface_leak 2>&1; then
  echo "[$TAG] ERROR: report surface dispatch must stay removed" >&2
  cat /tmp/"$TAG".surface_leak >&2
  rm -f /tmp/"$TAG".surface_leak
  exit 1
fi
rm -f /tmp/"$TAG".surface_leak
guard_expect_in_file "$TAG" 'releaseModeledToken' "$OWNER" "MIMAP-097A owner must expose releaseModeledToken"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentAllocationModeledLedgerReleaseReport' "$HELPER" "MIMAP-097A release report box must live in helper"
guard_expect_in_file "$TAG" 'findAnyIndex' "$OWNER" "MIMAP-097A owner must expose any-token lookup"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationModeledLedger' "$APP" "MIMAP-097A proof must construct ledger owner"
guard_expect_in_file "$TAG" 'check "mimap097a segment allocation modeled ledger release route"' "$APP" "MIMAP-097A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-097A must not add real execution, concurrency, segment-map, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-097A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-ledger-release-proof|HakoAllocSegmentAllocationModeledLedgerRelease|releaseModeledToken' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-097A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-097A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap097a_segment_allocation_release.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap097a.mir.json"
exe_out="$tmp_dir/mimap097a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-ledger-release-proof' "$vm_log"
rg -F -q 'release_first=1,0,0,60018002,60,18,2,1,0,2,1' "$vm_log"
rg -F -q 'release_second=1,0,1,61019007,61,19,7,1,0,2,0' "$vm_log"
rg -F -q 'finds_after=-1,-1,0,1' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'release_counts=14,2,12,1,1,1,1,1,1,1,1,1,1,1,1,61019007,0,1,0' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLedger.releaseModeledToken/2",
    "HakoAllocSegmentAllocationModeledLedger.findAnyIndex/1",
    "HakoAllocSegmentAllocationModeledLedger.releaseReport/9",
    "HakoAllocSegmentAllocationModeledLedger.releaseReject/8",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocSegmentAllocationModeledLedgerReleaseReport", "HakoAllocSegmentAllocationModeledLedger"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentAllocationModeledLedgerReleaseReport"].get("fields", [])
}
required_fields = {
    "did_release",
    "reason",
    "row_index",
    "modeled_allocation_token",
    "segment_id",
    "page_id",
    "modeled_block_start",
    "live_before",
    "live_after",
    "ledger_count_after",
    "ledger_live_count_after",
    "modeled_release_present",
    "would_execute_real_segment_free",
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
    raise SystemExit(f"missing release report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad release report field {name}: {field}")

print("[mimap097a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-ledger-release-proof' "$run_log"
rg -F -q 'release_first=1,0,0,60018002,60,18,2,1,0,2,1' "$run_log"
rg -F -q 'release_second=1,0,1,61019007,61,19,7,1,0,2,0' "$run_log"
rg -F -q 'finds_after=-1,-1,0,1' "$run_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'release_counts=14,2,12,1,1,1,1,1,1,1,1,1,1,1,1,61019007,0,1,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
