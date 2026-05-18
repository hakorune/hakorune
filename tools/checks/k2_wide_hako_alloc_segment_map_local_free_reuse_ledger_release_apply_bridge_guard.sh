#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge"
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
    echo "[$TAG] ERROR: MIMAP-200A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof/test.sh"
CARD_198A="docs/development/current/main/phases/phase-293x/293x-721-MIMAP-198A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE-CLOSEOUT-PACK.md"
CARD_199A="docs/development/current/main/phases/phase-293x/293x-722-MIMAP-199A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-723-MIMAP-200A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE.md"
CARD_201A="docs/development/current/main/phases/phase-293x/293x-724-MIMAP-201A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-ssot.md"
RELEASE_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-closeout-ssot.md"
APPLY_CARD="docs/development/current/main/phases/phase-293x/293x-644-MIMAP-138A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-ROUTE.md"
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
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_apply_bridge_guard.sh"

printf '[%s] checking MIMAP-200A segment-map local-free reuse ledger release apply bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_198A" \
  "$CARD_199A" \
  "$CARD" \
  "$CARD_201A" \
  "$DESIGN" \
  "$RELEASE_CLOSEOUT" \
  "$APPLY_CARD" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_198A" "MIMAP-198A must be landed before apply bridge"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_199A" "MIMAP-199A must be landed before apply bridge"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-200A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_201A" "MIMAP-201A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-200A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-198A' "$RELEASE_CLOSEOUT" "apply bridge must stay after release bridge closeout"
guard_expect_in_file "$TAG" 'MIMAP-138A' "$APPLY_CARD" "bridge row must reuse MIMAP-138A apply route"
guard_expect_in_file "$TAG" 'MIMAP-200A' "$PLAN" "granularity SSOT must describe MIMAP-200A"
guard_expect_in_file "$TAG" 'MIMAP-200A segment-map local-free reuse ledger release apply bridge' "$JOINT" "joint order must name MIMAP-200A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse ledger release apply bridge family' "$CADENCE" "validation cadence must define apply bridge family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-200A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-200A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-200A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-200A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-200A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-200A' "$MEMORY_README" "memory README must define MIMAP-200A owner boundary"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_ledger_box = "memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"' "$MODULE" "hako module must export source ledger owner"
guard_expect_in_file "$TAG" 'applyReuseLedgerRelease' "$SOURCE_LEDGER" "source reuse ledger must expose release apply route"
guard_expect_in_file "$TAG" 'recordReuseLedgerRelease' "$RELEASE_OWNER" "release owner must expose release route"
guard_expect_in_file "$TAG" 'check "mimap200a segment map local free reuse ledger release apply bridge"' "$APP" "MIMAP-200A proof must use labelled check block"

if rg -n 'segment_allocation_modeled_ledger_box|recordModeledConsume|releaseModeledToken' "$SOURCE_LEDGER" "$RELEASE_OWNER" >/tmp/"$TAG".bump_ledger_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-200A must not widen or depend on the bump-shaped modeled ledger" >&2
  cat /tmp/"$TAG".bump_ledger_leak >&2
  rm -f /tmp/"$TAG".bump_ledger_leak
  exit 1
fi
rm -f /tmp/"$TAG".bump_ledger_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$CONSUME_OWNER" "$SPAN_OWNER" "$REUSE_OWNER" "$SOURCE_LEDGER" "$RELEASE_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-200A must keep real free/free-list/raw pointer/concurrency/segment-map/atomics/page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)' "$SOURCE_LEDGER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-200A source ledger must not mutate page arrays directly" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$SOURCE_LEDGER" "$RELEASE_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-200A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof|SegmentMapLocalFreeReuseLedgerReleaseApply|applyReuseLedgerRelease' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-200A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap200_release_apply_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap200.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof' "$vm_log"
rg -F -q 'apply=1,0,0,-1,70007004,70007002,70,7,4,1,0,0' "$vm_log"
rg -F -q 'duplicate=0,3,0' "$vm_log"
rg -F -q 'missing=0,1,-1' "$vm_log"
rg -F -q 'unsupported=0,5' "$vm_log"
rg -F -q 'reads=-1,-1' "$vm_log"
rg -F -q 'counts=4,1,3,1,1,1,1,0' "$vm_log"
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
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport")
if report is None:
    raise SystemExit("missing local-free reuse ledger release apply report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_apply",
    "modeled_reuse_token",
    "source_modeled_allocation_token",
    "reused_block_id",
    "release_apply_count_after",
    "release_apply_reject_count_after",
    "ledger_live_count_after",
    "local_free_reuse_ledger_release_apply_present",
):
    if name not in fields:
        raise SystemExit(f"missing local-free reuse ledger release apply field: {name}")

print("[mimap200a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
