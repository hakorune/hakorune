#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-candidate-ledger"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-608-MIMAP-109A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-CANDIDATE-LEDGER-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md"
RELEASED_SPAN_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
SOURCE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_candidate_ledger_guard.sh"

printf '[%s] checking MIMAP-109A segment allocation modeled local-free candidate ledger route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$RELEASED_SPAN_SSOT" \
  "$CADENCE" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$SOURCE_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-109A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-109A design must be accepted"
guard_expect_in_file "$TAG" 'L2 proof' "$CADENCE" "validation cadence must define L2 proof rows"
guard_expect_in_file "$TAG" 'MIMAP-107A' "$RELEASED_SPAN_SSOT" "local-free candidate ledger must stay downstream of MIMAP-107A"
guard_expect_in_file "$TAG" 'MIMAP-109A' "$PLAN" "granularity SSOT must describe MIMAP-109A"
guard_expect_in_file "$TAG" 'MIMAP-109A segment allocation modeled local-free candidate ledger route' "$JOINT" "joint order must name MIMAP-109A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-109A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-109A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-109A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_candidate_ledger_box = "memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"' "$MODULE" "hako module must export local-free candidate ledger owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_candidate_ledger_box.hako` owns' "$MEMORY_README" "memory README must define MIMAP-109A owner"
guard_expect_in_file "$TAG" 'recordLocalFreeCandidate' "$OWNER" "local-free candidate ledger must expose record route"
guard_expect_in_file "$TAG" 'released_span_ledger_present' "$SOURCE_OWNER" "source released-span owner must expose present field"
guard_expect_in_file "$TAG" 'check "mimap109a segment allocation modeled local-free candidate ledger route"' "$APP" "MIMAP-109A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-109A must not add real execution, free-list mutation, concurrency, segment-map, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-109A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof|LocalFreeCandidateLedger|recordLocalFreeCandidate' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-109A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_local_free_candidate_ledger_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-109A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap109a_segment_allocation_local_free_candidate.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap109a.mir.json"
exe_out="$tmp_dir/mimap109a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof' "$vm_log"
rg -F -q 'first=1,0,0,-1,60018002,60,18,2,5,3,1,1' "$vm_log"
rg -F -q 'missing=0,2,-1,1' "$vm_log"
rg -F -q 'duplicate=0,3,0,1' "$vm_log"
rg -F -q 'recycled=1,0,1,-1,60018002,60,18,2,5,3,2,2' "$vm_log"
rg -F -q 'unsupported=0,4,1' "$vm_log"
rg -F -q 'counts=5,2,2,3,0,1,1,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger.recordLocalFreeCandidate/2",
    "HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger.findLocalFreeCandidateIndex/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeCandidateLedgerReport")
if report is None:
    raise SystemExit("missing local-free candidate report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_record_local_free_candidate",
    "modeled_allocation_token",
    "modeled_block_start",
    "modeled_block_end",
    "candidate_blocks",
    "would_mutate_free_list",
):
    if name not in fields:
        raise SystemExit(f"missing local-free candidate field: {name}")

print("[mimap109a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof' "$run_log"
rg -F -q 'first=1,0,0,-1,60018002,60,18,2,5,3,1,1' "$run_log"
rg -F -q 'missing=0,2,-1,1' "$run_log"
rg -F -q 'duplicate=0,3,0,1' "$run_log"
rg -F -q 'recycled=1,0,1,-1,60018002,60,18,2,5,3,2,2' "$run_log"
rg -F -q 'unsupported=0,4,1' "$run_log"
rg -F -q 'counts=5,2,2,3,0,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
