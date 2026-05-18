#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-apply-plan-bridge"
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
    echo "[$TAG] ERROR: MIMAP-176A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-apply-plan-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-apply-plan-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-apply-plan-bridge-proof/test.sh"
CARD_175A="docs/development/current/main/phases/phase-293x/293x-697-MIMAP-175A-POST-SEGMENT-MAP-RELEASED-SPAN-LOCAL-FREE-CANDIDATE-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-698-MIMAP-176A-SEGMENT-MAP-LOCAL-FREE-APPLY-PLAN-BRIDGE.md"
CARD_177A="docs/development/current/main/phases/phase-293x/293x-699-MIMAP-177A-POST-SEGMENT-MAP-LOCAL-FREE-APPLY-PLAN-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-apply-plan-bridge-ssot.md"
CANDIDATE_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-released-span-local-free-candidate-bridge-closeout-ssot.md"
APPLY_PLAN_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
APPLY_PLAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_guard.sh"

printf '[%s] checking MIMAP-176A segment-map local-free apply-plan bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_175A" \
  "$CARD" \
  "$CARD_177A" \
  "$DESIGN" \
  "$CANDIDATE_CLOSEOUT" \
  "$APPLY_PLAN_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$CANDIDATE_OWNER" \
  "$APPLY_PLAN_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_175A" "MIMAP-175A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-176A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_177A" "MIMAP-177A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-176A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-174A' "$CANDIDATE_CLOSEOUT" "bridge row must stay after local-free candidate bridge closeout"
guard_expect_in_file "$TAG" 'MIMAP-111A' "$APPLY_PLAN_SSOT" "bridge row must reuse local-free apply-plan ledger"
guard_expect_in_file "$TAG" 'MIMAP-176A' "$PLAN" "granularity SSOT must describe MIMAP-176A"
guard_expect_in_file "$TAG" 'MIMAP-176A segment-map local-free apply-plan bridge' "$JOINT" "joint order must name MIMAP-176A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-176A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-176A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-176A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-176A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-176A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-176A' "$MEMORY_README" "memory README must define MIMAP-176A owner boundary"
guard_expect_in_file "$TAG" 'recordLocalFreeApplyPlan' "$APPLY_PLAN_OWNER" "local-free apply-plan ledger must expose record route"
guard_expect_in_file "$TAG" 'check "mimap176a segment map local free apply plan bridge"' "$APP" "MIMAP-176A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-176A must keep real free, free-list/page-state mutation, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-176A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-apply-plan-bridge-proof|SegmentMapLocalFreeApplyPlan|recordLocalFreeApplyPlan' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-176A app/owner matcher leaked into .inc" >&2
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

tmp_dir="$(mktemp -d /tmp/hakorune_mimap176a_apply_plan_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap176a.mir.json"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-apply-plan-bridge-proof' "$vm_log"
rg -F -q 'candidate_first=1,0,0,-1,70007002,70,7,2,5,3' "$vm_log"
rg -F -q 'plan_first=1,0,0,-1,0,70007002,70,7,2,5,3,1,1,1' "$vm_log"
rg -F -q 'plan_missing=0,2,-1,1' "$vm_log"
rg -F -q 'plan_duplicate=0,3,0,1' "$vm_log"
rg -F -q 'plan_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,1,2,2' "$vm_log"
rg -F -q 'plan_unsupported=0,5,1' "$vm_log"
rg -F -q 'plan_counts=5,2,2,3,0,1,1,0,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger.recordLocalFreeCandidate/2",
    "HakoAllocSegmentAllocationModeledLocalFreeApplyPlan.recordLocalFreeApplyPlan/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
apply_report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeApplyPlanReport")
if apply_report is None:
    raise SystemExit("missing local-free apply-plan report typed object plan")
fields = {field.get("name"): field for field in apply_report.get("fields", [])}
for name in ("did_record_local_free_apply_plan", "local_free_candidate_row_index", "plan_kind", "would_mutate_page_state"):
    if name not in fields:
        raise SystemExit(f"missing local-free apply-plan report field: {name}")

print("[mimap176a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
