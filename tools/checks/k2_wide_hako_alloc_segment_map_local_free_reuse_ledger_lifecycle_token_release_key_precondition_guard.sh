#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition"
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
    echo "[$TAG] ERROR: MIMAP-220A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof/test.sh"
CARD_218A="docs/development/current/main/phases/phase-293x/293x-741-MIMAP-218A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC-CLOSEOUT-PACK.md"
CARD_219A="docs/development/current/main/phases/phase-293x/293x-742-MIMAP-219A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-743-MIMAP-220A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-OBSERVER.md"
CARD_221A="docs/development/current/main/phases/phase-293x/293x-744-MIMAP-221A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-ssot.md"
OBSERVER_DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
LIFECYCLE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako"
OBSERVER_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box.hako"
PRECONDITION_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_release_key_precondition_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh"

printf '[%s] checking MIMAP-220A lifecycle-token release-key precondition observer\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_218A" \
  "$CARD_219A" \
  "$CARD" \
  "$CARD_221A" \
  "$DESIGN" \
  "$OBSERVER_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$LIFECYCLE_OWNER" \
  "$OBSERVER_OWNER" \
  "$PRECONDITION_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_218A" "MIMAP-218A closeout must be landed before release-key precondition"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_219A" "MIMAP-219A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-220A card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_221A" "MIMAP-221A must be selected current or landed after MIMAP-220A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-220A design must be accepted"
guard_expect_in_file "$TAG" 'would_migrate_release_ledger_key = 0' "$DESIGN" "design must keep release-ledger key migration inactive"
guard_expect_in_file "$TAG" 'Decision: accepted' "$OBSERVER_DESIGN" "MIMAP-216A observer design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-220A' "$PLAN" "granularity SSOT must describe MIMAP-220A"
guard_expect_in_file "$TAG" 'MIMAP-221A' "$PLAN" "granularity SSOT must describe MIMAP-221A"
guard_expect_in_file "$TAG" 'MIMAP-220A segment-map local-free reuse ledger lifecycle-token release-key precondition observer' "$JOINT" "joint order must name MIMAP-220A"
guard_expect_in_file "$TAG" 'MIMAP-221A' "$JOINT" "joint order must name MIMAP-221A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse ledger lifecycle-token release-key precondition family' "$CADENCE" "cadence SSOT must define release-key precondition family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-220A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-220A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-220A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-220A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-220A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_lifecycle_token_release_key_precondition_box' "$MODULE" "module must export release-key precondition owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_lifecycle_token_release_key_precondition_box.hako' "$MEMORY_README" "memory README must name release-key precondition owner"
guard_expect_in_file "$TAG" 'classify' "$PRECONDITION_OWNER" "precondition owner must expose classify route"
guard_expect_in_file "$TAG" 'would_migrate_release_ledger_key: i64 = 0' "$PRECONDITION_OWNER" "precondition report must keep release-key migration inactive"
guard_expect_in_file "$TAG" 'check "mimap220a segment map local free reuse ledger lifecycle-token release-key precondition observer"' "$APP" "MIMAP-220A proof must use labelled check block"

if rg -n 'migrateReleaseLedger|recordLifecycleRelease|generationToken|realLifecycle|releaseLedgerKeyMigration|would_migrate_release_ledger_key[[:space:]]*=[[:space:]]*1' \
  "$APP" "$PRECONDITION_OWNER" "$OBSERVER_OWNER" "$LIFECYCLE_OWNER" >/tmp/"$TAG".migration_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-220A must not migrate release-ledger keys or define real lifecycle semantics" >&2
  cat /tmp/"$TAG".migration_leak >&2
  rm -f /tmp/"$TAG".migration_leak
  exit 1
fi
rm -f /tmp/"$TAG".migration_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$PRECONDITION_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-220A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof|LocalFreeReuseLifecycleTokenReleaseKeyPrecondition|releaseKeyPrecondition' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-220A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap220_release_key_precondition.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap220.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof' "$vm_log"
rg -F -q 'base=70007004,2,3' "$vm_log"
rg -F -q 'ready=1,0,70007004,2,1,1,1,0' "$vm_log"
rg -F -q 'blocked=0,1,0,2,0,3,0,4' "$vm_log"
rg -F -q 'counts=5,1,4,1,1,1,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenObserver.observeLifecycleBoundary/3",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenReleaseKeyPrecondition.classify/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenReleaseKeyPreconditionReport")
if report is None:
    raise SystemExit("missing lifecycle-token release-key precondition report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "ready",
    "reason",
    "modeled_reuse_token",
    "lifecycle_count",
    "release_duplicate_seen",
    "migration_candidate",
    "would_migrate_release_ledger_key",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_activate_provider",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-token release-key precondition report field: {name}")

print("[mimap220a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
