#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-ledger-release-span-facts"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-602-MIMAP-104A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASE-SPAN-FACTS-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-span-facts-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md"
RELEASE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-ssot.md"
RECYCLE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_span_facts_guard.sh"

printf '[%s] checking MIMAP-104A segment allocation modeled ledger release span facts route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$LEDGER_SSOT" \
  "$RELEASE_SSOT" \
  "$RECYCLE_SSOT" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-104A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-104A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-094A' "$LEDGER_SSOT" "span facts must stay downstream of modeled ledger"
guard_expect_in_file "$TAG" 'MIMAP-097A' "$RELEASE_SSOT" "span facts must stay downstream of modeled release"
guard_expect_in_file "$TAG" 'MIMAP-100A' "$RECYCLE_SSOT" "span facts must stay compatible with released-token recycle"
guard_expect_in_file "$TAG" 'MIMAP-104A' "$PLAN" "granularity SSOT must describe MIMAP-104A"
guard_expect_in_file "$TAG" 'MIMAP-104A segment allocation modeled ledger release span facts route' "$JOINT" "joint order must name MIMAP-104A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-104A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-104A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-104A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_box = "memory/segment_allocation_modeled_ledger_box.hako"' "$MODULE" "hako module must export modeled ledger owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_box.hako` owns MIMAP-104A' "$MEMORY_README" "memory README must define MIMAP-104A owner"
guard_expect_in_file "$TAG" 'release_span_present' "$OWNER" "ledger release report must expose span-present flag"
guard_expect_in_file "$TAG" 'modeled_block_end' "$OWNER" "ledger release report must expose block-end fact"
guard_expect_in_file "$TAG" 'released_blocks' "$OWNER" "ledger release report must expose released-block count"
guard_expect_in_file "$TAG" 'check "mimap104a segment allocation modeled ledger release span facts route"' "$APP" "MIMAP-104A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-104A must not add real execution, free-list mutation, concurrency, segment-map, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-104A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof|ReleaseSpanFacts|releaseSpanFacts' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-104A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_span_facts_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-104A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap104a_segment_allocation_release_span.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap104a.mir.json"
exe_out="$tmp_dir/mimap104a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof' "$vm_log"
rg -F -q 'first_span=1,0,60018002,2,8,3,5,3,2,5,3' "$vm_log"
rg -F -q 'missing_span=0,2,0' "$vm_log"
rg -F -q 'recycled_span=1,1,60018002,2,8,3,5,3,2,5,3' "$vm_log"
rg -F -q 'counts=2,2,2,0,3,2,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLedger.recordModeledConsume/12",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
release_report = plans.get("HakoAllocSegmentAllocationModeledLedgerReleaseReport")
if release_report is None:
    raise SystemExit("missing release report typed object plan")

fields = {field.get("name"): field for field in release_report.get("fields", [])}
for name in (
    "old_page_used_at_allocation",
    "page_capacity",
    "request_blocks",
    "new_page_used_at_allocation",
    "remaining_blocks_at_allocation",
    "modeled_block_end",
    "released_blocks",
    "release_span_present",
):
    if name not in fields:
        raise SystemExit(f"missing release span field: {name}")

print("[mimap104a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof' "$run_log"
rg -F -q 'first_span=1,0,60018002,2,8,3,5,3,2,5,3' "$run_log"
rg -F -q 'missing_span=0,2,0' "$run_log"
rg -F -q 'recycled_span=1,1,60018002,2,8,3,5,3,2,5,3' "$run_log"
rg -F -q 'counts=2,2,2,0,3,2,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
