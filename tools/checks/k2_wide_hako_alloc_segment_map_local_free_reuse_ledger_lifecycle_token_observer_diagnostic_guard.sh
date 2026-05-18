#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic"
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
    echo "[$TAG] ERROR: MIMAP-216A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof/test.sh"
CARD_214A="docs/development/current/main/phases/phase-293x/293x-737-MIMAP-214A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT-CLOSEOUT-PACK.md"
CARD_215A="docs/development/current/main/phases/phase-293x/293x-738-MIMAP-215A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-PILOT-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-739-MIMAP-216A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC.md"
CARD_217A="docs/development/current/main/phases/phase-293x/293x-740-MIMAP-217A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-OBSERVER-DIAGNOSTIC-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-ssot.md"
PILOT_DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
LIFECYCLE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako"
OBSERVER_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh"

printf '[%s] checking MIMAP-216A lifecycle-token observer diagnostic\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_214A" \
  "$CARD_215A" \
  "$CARD" \
  "$CARD_217A" \
  "$DESIGN" \
  "$PILOT_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$LIFECYCLE_OWNER" \
  "$OBSERVER_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_214A" "MIMAP-214A must be landed before observer diagnostic"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_215A" "MIMAP-215A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-216A card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_217A" "MIMAP-217A must be selected current or landed after observer diagnostic"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-216A design must be accepted"
guard_expect_in_file "$TAG" 'release ledger still keys release records by modeled reuse token' "$DESIGN" "design must state modeled-reuse-token release key boundary"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PILOT_DESIGN" "MIMAP-212A pilot design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-216A' "$PLAN" "granularity SSOT must describe MIMAP-216A"
guard_expect_in_file "$TAG" 'MIMAP-217A' "$PLAN" "granularity SSOT must describe MIMAP-217A"
guard_expect_in_file "$TAG" 'MIMAP-216A segment-map local-free reuse ledger lifecycle-token observer diagnostic' "$JOINT" "joint order must name MIMAP-216A"
guard_expect_in_file "$TAG" 'MIMAP-217A' "$JOINT" "joint order must name MIMAP-217A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse ledger lifecycle-token observer diagnostic family' "$CADENCE" "cadence SSOT must define observer family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-216A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-216A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-216A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-216A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-216A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box' "$MODULE" "module must export lifecycle-token observer owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box.hako' "$MEMORY_README" "memory README must name observer owner"
guard_expect_in_file "$TAG" 'observeLifecycleBoundary' "$OBSERVER_OWNER" "observer owner must expose observer route"
guard_expect_in_file "$TAG" 'would_migrate_release_ledger_key: i64 = 0' "$OBSERVER_OWNER" "observer report must keep release-key migration inactive"
guard_expect_in_file "$TAG" 'check "mimap216a segment map local free reuse ledger lifecycle-token observer diagnostic"' "$APP" "MIMAP-216A proof must use labelled check block"

if rg -n 'migrateReleaseLedger|recordLifecycleRelease|generationToken|realLifecycle|releaseLedgerKeyMigration' \
  "$APP" "$OBSERVER_OWNER" "$LIFECYCLE_OWNER" >/tmp/"$TAG".migration_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-216A must not migrate release-ledger keys or define real lifecycle semantics" >&2
  cat /tmp/"$TAG".migration_leak >&2
  rm -f /tmp/"$TAG".migration_leak
  exit 1
fi
rm -f /tmp/"$TAG".migration_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$OBSERVER_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-216A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof|LocalFreeReuseLifecycleTokenObserver|lifecycleTokenObserver' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-216A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap216_lifecycle_observer.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap216.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof' "$vm_log"
rg -F -q 'base=70007004,2,3' "$vm_log"
rg -F -q 'observer=1,0,70007004,2,2,3,1,1,0' "$vm_log"
rg -F -q 'rejects=0,1,0,2' "$vm_log"
rg -F -q 'counts=3,1,2,1,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenObserver.observeLifecycleBoundary/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenObserverReport")
if report is None:
    raise SystemExit("missing lifecycle-token observer report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "modeled_reuse_token",
    "lifecycle_count",
    "release_duplicate_seen",
    "release_key_still_modeled_reuse_token",
    "would_migrate_release_ledger_key",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_activate_provider",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-token observer report field: {name}")

print("[mimap216a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
