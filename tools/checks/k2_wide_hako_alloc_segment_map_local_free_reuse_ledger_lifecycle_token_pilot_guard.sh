#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot"
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
    echo "[$TAG] ERROR: MIMAP-212A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof/test.sh"
CARD_210A="docs/development/current/main/phases/phase-293x/293x-733-MIMAP-210A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT-PACK.md"
CARD_211A="docs/development/current/main/phases/phase-293x/293x-734-MIMAP-211A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-735-MIMAP-212A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT.md"
CARD_213A="docs/development/current/main/phases/phase-293x/293x-736-MIMAP-213A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SOURCE_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
LIFECYCLE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh"

printf '[%s] checking MIMAP-212A lifecycle-token pilot\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_210A" \
  "$CARD_211A" \
  "$CARD" \
  "$CARD_213A" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$SOURCE_LEDGER" \
  "$RELEASE_OWNER" \
  "$LIFECYCLE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_210A" "MIMAP-210A must be landed before lifecycle-token pilot"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_211A" "MIMAP-211A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-212A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_213A" "MIMAP-213A must be selected current after lifecycle-token pilot"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-212A design must be accepted"
guard_expect_in_file "$TAG" 'reuse_lifecycle_token = modeled_reuse_token \* 1000 \+ lifecycle_id' "$DESIGN" "design must define lifecycle token derivation"
guard_expect_in_file "$TAG" 'No release ledger key migration' "$DESIGN" "design must keep release-ledger migration closed"
guard_expect_in_file "$TAG" 'MIMAP-212A' "$PLAN" "granularity SSOT must describe MIMAP-212A"
guard_expect_in_file "$TAG" 'MIMAP-213A' "$PLAN" "granularity SSOT must describe MIMAP-213A"
guard_expect_in_file "$TAG" 'MIMAP-212A segment-map local-free reuse ledger lifecycle-token pilot' "$JOINT" "joint order must name MIMAP-212A"
guard_expect_in_file "$TAG" 'MIMAP-213A' "$JOINT" "joint order must name MIMAP-213A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse ledger lifecycle-token pilot family' "$CADENCE" "cadence SSOT must define lifecycle-token family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-212A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-212A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-212A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-212A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-212A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_lifecycle_token_box' "$MODULE" "module must export lifecycle-token owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako' "$MEMORY_README" "memory README must name lifecycle-token owner"
guard_expect_in_file "$TAG" 'brand ModeledReuseToken: i64' "$LIFECYCLE_OWNER" "owner must brand modeled reuse token"
guard_expect_in_file "$TAG" 'brand ReuseLifecycleId: i64' "$LIFECYCLE_OWNER" "owner must brand lifecycle id"
guard_expect_in_file "$TAG" 'makeReuseLifecycleToken' "$LIFECYCLE_OWNER" "owner must derive lifecycle token through a helper"
guard_expect_in_file "$TAG" 'recordLifecycleToken' "$LIFECYCLE_OWNER" "owner must expose lifecycle-token record route"
guard_expect_in_file "$TAG" 'check "mimap212a segment map local free reuse ledger lifecycle-token pilot"' "$APP" "MIMAP-212A proof must use labelled check block"

if rg -n 'generation_id|lifecycle_id|generationToken|lifecycleToken|recordLifecycleToken|makeReuseLifecycleToken' \
  "$SOURCE_LEDGER" "$RELEASE_OWNER" >/tmp/"$TAG".ledger_migration_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-212A must not migrate source reuse ledger or release owner to lifecycle keys" >&2
  cat /tmp/"$TAG".ledger_migration_leak >&2
  rm -f /tmp/"$TAG".ledger_migration_leak
  exit 1
fi
rm -f /tmp/"$TAG".ledger_migration_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$LIFECYCLE_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-212A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof|LocalFreeReuseLifecycleToken|lifecycleTokenPilot' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-212A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap212_lifecycle_token.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap212.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof' "$vm_log"
rg -F -q 'base=70007004,1,0' "$vm_log"
rg -F -q 'lifecycle=1,0,0,70007004,1,70007004001' "$vm_log"
rg -F -q 'second=1,0,1,70007004,2,70007004002' "$vm_log"
rg -F -q 'duplicate=0,2,1,70007004002' "$vm_log"
rg -F -q 'rejects=0,1,0,3' "$vm_log"
rg -F -q 'counts=5,2,3,1,1,1,2' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleToken.recordLifecycleToken/3",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleToken.makeReuseLifecycleToken/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.recordLocalFreeReuse/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerRelease.recordReuseLedgerRelease/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenReport")
if report is None:
    raise SystemExit("missing lifecycle-token report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "row_index",
    "existing_index",
    "modeled_reuse_token",
    "lifecycle_id",
    "reuse_lifecycle_token",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_activate_provider",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-token report field: {name}")

print("[mimap212a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
