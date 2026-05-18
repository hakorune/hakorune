#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge"
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
    echo "[$TAG] ERROR: MIMAP-204A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-proof/test.sh"
CARD_202A="docs/development/current/main/phases/phase-293x/293x-725-MIMAP-202A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE-CLOSEOUT-PACK.md"
CARD_203A="docs/development/current/main/phases/phase-293x/293x-726-MIMAP-203A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-727-MIMAP-204A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-BRIDGE.md"
CARD_205A="docs/development/current/main/phases/phase-293x/293x-728-MIMAP-205A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-ssot.md"
APPLY_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout-ssot.md"
RECYCLE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-applied-recycle-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
CONSUME_OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
REUSE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako"
SOURCE_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_bridge_guard.sh"

printf '[%s] checking MIMAP-204A segment-map local-free reuse ledger release-applied recycle bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_202A" \
  "$CARD_203A" \
  "$CARD" \
  "$CARD_205A" \
  "$DESIGN" \
  "$APPLY_CLOSEOUT" \
  "$RECYCLE_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$CONSUME_OWNER" \
  "$SPAN_OWNER" \
  "$REUSE_OWNER" \
  "$SOURCE_LEDGER" \
  "$RELEASE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_202A" "MIMAP-202A must be landed before recycle bridge"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_203A" "MIMAP-203A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-204A card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_205A" "MIMAP-205A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-204A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-202A' "$APPLY_CLOSEOUT" "recycle bridge must stay after release apply bridge closeout"
guard_expect_in_file "$TAG" 'MIMAP-142A' "$RECYCLE_SSOT" "bridge row must reuse MIMAP-142A recycle route"
guard_expect_in_file "$TAG" 'MIMAP-204A' "$PLAN" "granularity SSOT must describe MIMAP-204A"
guard_expect_in_file "$TAG" 'MIMAP-204A segment-map local-free reuse ledger release-applied recycle bridge' "$JOINT" "joint order must name MIMAP-204A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse ledger release-applied recycle bridge family' "$CADENCE" "validation cadence must define recycle bridge family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-204A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-204A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-204A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-204A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-204A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-204A' "$MEMORY_README" "memory README must define MIMAP-204A owner boundary"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_ledger_box = "memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"' "$MODULE" "hako module must export source ledger owner"
guard_expect_in_file "$TAG" 'applyReuseLedgerRelease' "$SOURCE_LEDGER" "source reuse ledger must expose release apply route"
guard_expect_in_file "$TAG" 'recordLocalFreeReuse' "$SOURCE_LEDGER" "source reuse ledger must expose recycle route"
guard_expect_in_file "$TAG" 'recordReuseLedgerRelease' "$RELEASE_OWNER" "release owner must expose release route"
guard_expect_in_file "$TAG" 'check "mimap204a segment map local free reuse ledger release-applied recycle bridge"' "$APP" "MIMAP-204A proof must use labelled check block"

if rg -n 'segment_allocation_modeled_ledger_box|recordModeledConsume|releaseModeledToken' "$SOURCE_LEDGER" "$RELEASE_OWNER" >/tmp/"$TAG".bump_ledger_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-204A must not widen or depend on the bump-shaped modeled ledger" >&2
  cat /tmp/"$TAG".bump_ledger_leak >&2
  rm -f /tmp/"$TAG".bump_ledger_leak
  exit 1
fi
rm -f /tmp/"$TAG".bump_ledger_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$CONSUME_OWNER" "$SPAN_OWNER" "$REUSE_OWNER" "$SOURCE_LEDGER" "$RELEASE_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-204A must keep real free/free-list/raw pointer/concurrency/segment-map/atomics/page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)' "$SOURCE_LEDGER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-204A source ledger must not mutate page arrays directly" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$SOURCE_LEDGER" "$RELEASE_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-204A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-proof|SegmentMapLocalFreeReuseLedgerReleaseAppliedRecycle|releaseAppliedRecycle' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-204A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap204_release_applied_recycle_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap204.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-proof' "$vm_log"
rg -F -q 'first=1,0,0,70007004,1' "$vm_log"
rg -F -q 'apply=1,0,0,70007004,0' "$vm_log"
rg -F -q 'recycle=1,0,1,70007004,1' "$vm_log"
rg -F -q 'live_duplicate=0,4,1' "$vm_log"
rg -F -q 'reads=-1,70007004,4' "$vm_log"
rg -F -q 'counts=3,2,1,1,2,1,1,1,0' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
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
    "Main.nextReuseReport/4",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.recordLocalFreeReuse/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.applyReuseLedgerRelease/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.findAnyIndex/1",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.findIndex/1",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerRelease.recordReuseLedgerRelease/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReport")
if report is None:
    raise SystemExit("missing local-free reuse ledger report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "modeled_reuse_token",
    "source_modeled_allocation_token",
    "segment_id",
    "page_id",
    "reused_block_id",
    "ledger_live_count_after",
    "local_free_reuse_ledger_present",
):
    if name not in fields:
        raise SystemExit(f"missing local-free reuse ledger field: {name}")

print("[mimap204a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
