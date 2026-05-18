#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-release-bridge"
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
    echo "[$TAG] ERROR: MIMAP-196A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof/test.sh"
CARD_195A="docs/development/current/main/phases/phase-293x/293x-717-MIMAP-195A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-719-MIMAP-196A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE.md"
CARD_197A="docs/development/current/main/phases/phase-293x/293x-720-MIMAP-197A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-ssot.md"
LEDGER_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-bridge-closeout-ssot.md"
RELEASE_CARD="docs/development/current/main/phases/phase-293x/293x-640-MIMAP-134A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-ROUTE.md"
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
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_guard.sh"

printf '[%s] checking MIMAP-196A segment-map local-free reuse ledger release bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_195A" \
  "$CARD" \
  "$CARD_197A" \
  "$DESIGN" \
  "$LEDGER_CLOSEOUT" \
  "$RELEASE_CARD" \
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
  "$OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_195A" "MIMAP-195A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-196A card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_197A" "MIMAP-197A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-196A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-194A' "$LEDGER_CLOSEOUT" "release bridge must stay after reuse ledger bridge closeout"
guard_expect_in_file "$TAG" 'MIMAP-134A' "$RELEASE_CARD" "bridge row must reuse MIMAP-134A release owner"
guard_expect_in_file "$TAG" 'MIMAP-196A' "$PLAN" "granularity SSOT must describe MIMAP-196A"
guard_expect_in_file "$TAG" 'MIMAP-196A segment-map local-free reuse ledger release bridge' "$JOINT" "joint order must name MIMAP-196A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse ledger release bridge family' "$CADENCE" "validation cadence must define release bridge family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-196A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-196A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-196A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-196A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-196A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-196A' "$MEMORY_README" "memory README must define MIMAP-196A owner boundary"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_ledger_release_box = "memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"' "$MODULE" "hako module must export release owner"
guard_expect_in_file "$TAG" 'recordLocalFreeReuse' "$SOURCE_LEDGER" "source reuse ledger must expose record route"
guard_expect_in_file "$TAG" 'recordReuseLedgerRelease' "$OWNER" "release owner must expose release route"
guard_expect_in_file "$TAG" 'check "mimap196a segment map local free reuse ledger release bridge"' "$APP" "MIMAP-196A proof must use labelled check block"

if rg -n 'segment_allocation_modeled_ledger_box|recordModeledConsume|releaseModeledToken' "$OWNER" >/tmp/"$TAG".bump_ledger_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-196A release owner must not widen or depend on the bump-shaped modeled ledger" >&2
  cat /tmp/"$TAG".bump_ledger_leak >&2
  rm -f /tmp/"$TAG".bump_ledger_leak
  exit 1
fi
rm -f /tmp/"$TAG".bump_ledger_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$CONSUME_OWNER" "$SPAN_OWNER" "$REUSE_OWNER" "$SOURCE_LEDGER" "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-196A must keep real free/free-list/raw pointer/concurrency/segment-map/atomics/page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-196A release owner must not mutate page arrays directly" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-196A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof|SegmentMapLocalFreeReuseLedgerRelease|recordReuseLedgerRelease' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-196A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap196_reuse_ledger_release_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap196.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof' "$vm_log"
rg -F -q 'release=1,0,0,-1,70007004,70007002,70,7,4,1,0' "$vm_log"
rg -F -q 'duplicate=0,3,0' "$vm_log"
rg -F -q 'missing=0,1,-1' "$vm_log"
rg -F -q 'unsupported=0,4' "$vm_log"
rg -F -q 'reads=70007004,4' "$vm_log"
rg -F -q 'counts=4,1,3,1,1,1,1,1' "$vm_log"
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
    "Main.nextReuseReport/4",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.recordLocalFreeReuse/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerRelease.recordReuseLedgerRelease/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerRelease.findIndex/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseReport")
if report is None:
    raise SystemExit("missing local-free reuse ledger release report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_release",
    "modeled_reuse_token",
    "source_modeled_allocation_token",
    "reused_block_id",
    "release_count_after",
    "local_free_reuse_ledger_release_present",
    "would_execute_real_segment_free",
):
    if name not in fields:
        raise SystemExit(f"missing local-free reuse ledger release field: {name}")

print("[mimap196a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
