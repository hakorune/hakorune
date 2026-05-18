#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-page-apply-bridge"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

VALIDATION_LEVEL="L2"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --level)
      if [ "$#" -lt 2 ]; then
        echo "[$TAG] ERROR: --level requires a value" >&2
        exit 2
      fi
      VALIDATION_LEVEL="$2"
      shift 2
      ;;
    --level=*)
      VALIDATION_LEVEL="${1#--level=}"
      shift
      ;;
    *)
      echo "[$TAG] ERROR: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

case "$VALIDATION_LEVEL" in
  L0|L1|L2) ;;
  L3|L4)
    echo "[$TAG] ERROR: MIMAP-180A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-page-apply-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-page-apply-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-page-apply-bridge-proof/test.sh"
CARD_179A="docs/development/current/main/phases/phase-293x/293x-701-MIMAP-179A-POST-SEGMENT-MAP-LOCAL-FREE-APPLY-PLAN-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-702-MIMAP-180A-SEGMENT-MAP-LOCAL-FREE-PAGE-APPLY-BRIDGE.md"
CARD_181A="docs/development/current/main/phases/phase-293x/293x-703-MIMAP-181A-POST-SEGMENT-MAP-LOCAL-FREE-PAGE-APPLY-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-page-apply-bridge-ssot.md"
APPLY_PLAN_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-local-free-apply-plan-bridge-closeout-ssot.md"
PAGE_APPLY_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
APPLY_PLAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako"
PAGE_APPLY_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh"

printf '[%s] checking MIMAP-180A segment-map local-free page-apply bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_179A" \
  "$CARD" \
  "$CARD_181A" \
  "$DESIGN" \
  "$APPLY_PLAN_CLOSEOUT" \
  "$PAGE_APPLY_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$CANDIDATE_OWNER" \
  "$APPLY_PLAN_OWNER" \
  "$PAGE_APPLY_OWNER" \
  "$PAGE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_179A" "MIMAP-179A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-180A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_181A" "MIMAP-181A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-180A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-178A' "$APPLY_PLAN_CLOSEOUT" "page-apply bridge must stay after apply-plan bridge closeout"
guard_expect_in_file "$TAG" 'MIMAP-115A' "$PAGE_APPLY_SSOT" "bridge row must reuse page-apply owner"
guard_expect_in_file "$TAG" 'MIMAP-180A' "$PLAN" "granularity SSOT must describe MIMAP-180A"
guard_expect_in_file "$TAG" 'MIMAP-180A segment-map local-free page-apply bridge' "$JOINT" "joint order must name MIMAP-180A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-180A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-180A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-180A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-180A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-180A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-180A' "$MEMORY_README" "memory README must define MIMAP-180A owner boundary"
guard_expect_in_file "$TAG" 'recordLocalFreePageApply' "$PAGE_APPLY_OWNER" "page-apply owner must expose record route"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_OWNER" "page model must own local-free mutation seam"
guard_expect_in_file "$TAG" 'check "mimap180a segment map local free page apply bridge"' "$APP" "MIMAP-180A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$PAGE_APPLY_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-180A must keep real free/free-list/raw pointer/concurrency/segment-map/atomics/page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$PAGE_APPLY_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-180A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-page-apply-bridge-proof|SegmentMapLocalFreePageApply|recordLocalFreePageApply' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-180A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_build_hakorune_debug

tmp_dir="$(mktemp -d /tmp/hakorune_mimap180a_page_apply_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap180a.mir.json"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-page-apply-bridge-proof' "$vm_log"
rg -F -q 'plan_first=1,0,0,-1,0,70007002,70,7,2,5,3' "$vm_log"
rg -F -q 'page_first=1,0,0,-1,0,70007002,70,7,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'page_missing=0,2,-1,1' "$vm_log"
rg -F -q 'page_duplicate=0,3,0,1' "$vm_log"
rg -F -q 'page_wrong=0,4,1' "$vm_log"
rg -F -q 'page_unsupported=0,6,1' "$vm_log"
rg -F -q 'page_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'page_counts=6,2,2,4,0,1,1,1,0,1' "$vm_log"
rg -F -q 'page_state=3,2,3,5' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeApplyPlan.recordLocalFreeApplyPlan/2",
    "HakoAllocSegmentAllocationModeledLocalFreePageApply.recordLocalFreePageApply/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
page_apply_report = plans.get("HakoAllocSegmentAllocationModeledLocalFreePageApplyReport")
if page_apply_report is None:
    raise SystemExit("missing local-free page-apply report typed object plan")
fields = {field.get("name"): field for field in page_apply_report.get("fields", [])}
for name in ("did_apply_local_free_to_page", "did_mutate_page_local_free_list", "would_directly_mutate_page_arrays", "would_use_raw_pointer"):
    if name not in fields:
        raise SystemExit(f"missing local-free page-apply report field: {name}")

print("[mimap180a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
