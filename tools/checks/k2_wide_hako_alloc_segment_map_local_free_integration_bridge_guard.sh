#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-integration-bridge"
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
    echo "[$TAG] ERROR: MIMAP-184A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-integration-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-integration-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-integration-bridge-proof/test.sh"
CARD_183A="docs/development/current/main/phases/phase-293x/293x-705-MIMAP-183A-POST-SEGMENT-MAP-LOCAL-FREE-PAGE-APPLY-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-706-MIMAP-184A-SEGMENT-MAP-LOCAL-FREE-INTEGRATION-BRIDGE.md"
CARD_185A="docs/development/current/main/phases/phase-293x/293x-707-MIMAP-185A-POST-SEGMENT-MAP-LOCAL-FREE-INTEGRATION-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-integration-bridge-ssot.md"
PAGE_APPLY_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-local-free-page-apply-bridge-closeout-ssot.md"
INTEGRATION_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-integration-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
INTEGRATION_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako"
PAGE_APPLY_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_integration_bridge_guard.sh"

printf '[%s] checking MIMAP-184A segment-map local-free integration bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_183A" \
  "$CARD" \
  "$CARD_185A" \
  "$DESIGN" \
  "$PAGE_APPLY_CLOSEOUT" \
  "$INTEGRATION_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$INTEGRATION_OWNER" \
  "$PAGE_APPLY_OWNER" \
  "$PAGE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_183A" "MIMAP-183A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-184A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_185A" "MIMAP-185A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-184A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-182A' "$PAGE_APPLY_CLOSEOUT" "integration bridge must stay after page-apply bridge closeout"
guard_expect_in_file "$TAG" 'MIMAP-119A' "$INTEGRATION_SSOT" "bridge row must reuse integration owner"
guard_expect_in_file "$TAG" 'MIMAP-184A' "$PLAN" "granularity SSOT must describe MIMAP-184A"
guard_expect_in_file "$TAG" 'MIMAP-184A segment-map local-free integration bridge' "$JOINT" "joint order must name MIMAP-184A"
guard_expect_in_file "$TAG" 'segment-map local-free integration bridge family' "$CADENCE" "validation cadence must define integration bridge family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-184A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-184A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-184A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-184A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-184A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-184A' "$MEMORY_README" "memory README must define MIMAP-184A owner boundary"
guard_expect_in_file "$TAG" 'releaseConsumedToken' "$OWNER" "consume ledger owner must expose release route"
guard_expect_in_file "$TAG" 'recordReleasedSpan' "$SPAN_OWNER" "released-span owner must expose released-span route"
guard_expect_in_file "$TAG" 'integrateLocalFree' "$INTEGRATION_OWNER" "integration owner must expose integration route"
guard_expect_in_file "$TAG" 'recordLocalFreePageApply' "$PAGE_APPLY_OWNER" "integration must continue through page-apply owner"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_OWNER" "page model must own local-free mutation seam"
guard_expect_in_file "$TAG" 'check "mimap184a segment map local free integration bridge"' "$APP" "MIMAP-184A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$INTEGRATION_OWNER" "$PAGE_APPLY_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-184A must keep real free/free-list/raw pointer/concurrency/segment-map/atomics/page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$INTEGRATION_OWNER" "$PAGE_APPLY_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-184A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-integration-bridge-proof|SegmentMapLocalFreeIntegration|integrateLocalFree' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-184A app/owner matcher leaked into .inc" >&2
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

tmp_dir="$(mktemp -d /tmp/hakorune_mimap184_integration_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap184.mir.json"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-integration-bridge-proof' "$vm_log"
rg -F -q 'first=1,0,0,0,0,70007002,70,7,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'missing=0,1,2' "$vm_log"
rg -F -q 'duplicate=0,1,3' "$vm_log"
rg -F -q 'wrong_page=0,3,4' "$vm_log"
rg -F -q 'unsupported=0,1,4' "$vm_log"
rg -F -q 'recycled=1,0,2,2,1,3,3' "$vm_log"
rg -F -q 'counts=6,2,4,3,0,1,3,3,2' "$vm_log"
rg -F -q 'page=3,2,3,5' "$vm_log"
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
    "Main.nextReleasedSpan/3",
    "HakoAllocSegmentAllocationModeledLocalFreeIntegration.integrateLocalFree/5",
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger.releaseConsumedToken/2",
    "HakoAllocSegmentAllocationModeledReleasedSpanLedger.recordReleasedSpan/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeIntegrationReport")
if report is None:
    raise SystemExit("missing local-free integration report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_integrate_local_free",
    "candidate_row_index",
    "apply_plan_row_index",
    "page_apply_row_index",
    "would_use_segment_map",
    "would_use_raw_pointer",
):
    if name not in fields:
        raise SystemExit(f"missing local-free integration field: {name}")

print("[mimap184a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
