#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-bridge"
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
    echo "[$TAG] ERROR: MIMAP-188A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-bridge-proof/test.sh"
CARD_187A="docs/development/current/main/phases/phase-293x/293x-709-MIMAP-187A-POST-SEGMENT-MAP-LOCAL-FREE-INTEGRATION-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-710-MIMAP-188A-SEGMENT-MAP-LOCAL-FREE-REUSE-BRIDGE.md"
CARD_189A="docs/development/current/main/phases/phase-293x/293x-711-MIMAP-189A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-bridge-ssot.md"
INTEGRATION_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-local-free-integration-bridge-closeout-ssot.md"
REUSE_OWNER_CARD="docs/development/current/main/phases/phase-293x/293x-632-MIMAP-126A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-ROUTE.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
REUSE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako"
INTEGRATION_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako"
PAGE_APPLY_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_bridge_guard.sh"

printf '[%s] checking MIMAP-188A segment-map local-free reuse bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_187A" \
  "$CARD" \
  "$CARD_189A" \
  "$DESIGN" \
  "$INTEGRATION_CLOSEOUT" \
  "$REUSE_OWNER_CARD" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$REUSE_OWNER" \
  "$INTEGRATION_OWNER" \
  "$PAGE_APPLY_OWNER" \
  "$PAGE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_187A" "MIMAP-187A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-188A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_189A" "MIMAP-189A must be landed after selecting the closeout row"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-188A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-186A' "$INTEGRATION_CLOSEOUT" "reuse bridge must stay after integration bridge closeout"
guard_expect_in_file "$TAG" 'MIMAP-126A' "$REUSE_OWNER_CARD" "bridge row must reuse MIMAP-126A reuse owner"
guard_expect_in_file "$TAG" 'MIMAP-188A' "$PLAN" "granularity SSOT must describe MIMAP-188A"
guard_expect_in_file "$TAG" 'MIMAP-188A segment-map local-free reuse bridge' "$JOINT" "joint order must name MIMAP-188A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse bridge family' "$CADENCE" "validation cadence must define reuse bridge family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-188A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-188A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-188A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-188A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-188A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-188A' "$MEMORY_README" "memory README must define MIMAP-188A owner boundary"
guard_expect_in_file "$TAG" 'releaseConsumedToken' "$OWNER" "consume ledger owner must expose release route"
guard_expect_in_file "$TAG" 'recordReleasedSpan' "$SPAN_OWNER" "released-span owner must expose released-span route"
guard_expect_in_file "$TAG" 'integrateAndReuseLocalFree' "$REUSE_OWNER" "reuse owner must expose local-free reuse route"
guard_expect_in_file "$TAG" 'integrateLocalFree' "$REUSE_OWNER" "reuse owner must compose integration route"
guard_expect_in_file "$TAG" 'page[.]acquire' "$REUSE_OWNER" "reuse owner must reuse through page acquire"
guard_expect_in_file "$TAG" 'finishReport' "$REUSE_OWNER" "reuse owner must keep report construction local for closeout parity"
guard_expect_in_file "$TAG" 'recordLocalFreePageApply' "$INTEGRATION_OWNER" "integration owner must continue through page-apply owner"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_APPLY_OWNER" "page-apply owner must still reach page-local release"
guard_expect_in_file "$TAG" 'check "mimap188a segment map local free reuse bridge"' "$APP" "MIMAP-188A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$REUSE_OWNER" "$INTEGRATION_OWNER" "$PAGE_APPLY_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-188A must keep real free/free-list/raw pointer/concurrency/segment-map/atomics/page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$REUSE_OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-188A owner must not mutate page arrays directly" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$REUSE_OWNER" "$INTEGRATION_OWNER" "$PAGE_APPLY_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-188A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-bridge-proof|SegmentMapLocalFreeReuse|integrateAndReuseLocalFree' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-188A app/owner matcher leaked into .inc" >&2
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

tmp_dir="$(mktemp -d /tmp/hakorune_mimap188_reuse_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap188.mir.json"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-bridge-proof' "$vm_log"
rg -F -q 'reuse=1,0,4,5,6,0,0,3,2,1' "$vm_log"
rg -F -q 'integration=0,70007002,70,7,2,5,3' "$vm_log"
rg -F -q 'missing=0,1,1' "$vm_log"
rg -F -q 'duplicate=0,1,1' "$vm_log"
rg -F -q 'partial=0,2,2' "$vm_log"
rg -F -q 'unsupported=0,1,1' "$vm_log"
rg -F -q 'recycled=1,0,4,6,2,1' "$vm_log"
rg -F -q 'counts=6,2,4,3,1,0,0' "$vm_log"
rg -F -q 'page=6,0,2,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuse.integrateAndReuseLocalFree/6",
    "HakoAllocSegmentAllocationModeledLocalFreeIntegration.integrateLocalFree/5",
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger.releaseConsumedToken/2",
    "HakoAllocSegmentAllocationModeledReleasedSpanLedger.recordReleasedSpan/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseReport")
if report is None:
    raise SystemExit("missing local-free reuse report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_reuse_local_free",
    "reused_block_id",
    "page_local_free_before_reuse",
    "collect_count_after_reuse",
    "did_collect_local_free_before_reuse",
    "would_use_segment_map",
    "would_use_raw_pointer",
):
    if name not in fields:
        raise SystemExit(f"missing local-free reuse field: {name}")

print("[mimap188a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
