#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow"
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
    echo "[$TAG] ERROR: MIMAP-224A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof/test.sh"
CARD_222A="docs/development/current/main/phases/phase-293x/293x-745-MIMAP-222A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-CLOSEOUT-PACK.md"
CARD_223A="docs/development/current/main/phases/phase-293x/293x-746-MIMAP-223A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-TOKEN-RELEASE-KEY-PRECONDITION-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-747-MIMAP-224A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-PILOT.md"
CARD_225A="docs/development/current/main/phases/phase-293x/293x-748-MIMAP-225A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-ssot.md"
PRECONDITION_DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SOURCE_RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
SHADOW_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_shadow_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_shadow_guard.sh"

printf '[%s] checking MIMAP-224A lifecycle-keyed release shadow\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_222A" \
  "$CARD_223A" \
  "$CARD" \
  "$CARD_225A" \
  "$DESIGN" \
  "$PRECONDITION_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$SOURCE_RELEASE_OWNER" \
  "$SHADOW_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_222A" "MIMAP-222A closeout must be landed before shadow row"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_223A" "MIMAP-223A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-224A card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_225A" "MIMAP-225A must be selected current or landed after MIMAP-224A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-224A design must be accepted"
guard_expect_in_file "$TAG" 'not a migration of the existing source release ledger' "$DESIGN" "design must keep source release owner unmigrated"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PRECONDITION_DESIGN" "MIMAP-220A precondition design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-224A' "$PLAN" "granularity SSOT must describe MIMAP-224A"
guard_expect_in_file "$TAG" 'MIMAP-225A' "$PLAN" "granularity SSOT must describe MIMAP-225A"
guard_expect_in_file "$TAG" 'MIMAP-224A segment-map local-free reuse ledger lifecycle-keyed release shadow pilot' "$JOINT" "joint order must name MIMAP-224A"
guard_expect_in_file "$TAG" 'MIMAP-225A' "$JOINT" "joint order must name MIMAP-225A"
guard_expect_in_file "$TAG" 'segment-map local-free reuse ledger lifecycle-keyed release shadow family' "$CADENCE" "cadence SSOT must define shadow family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-224A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-224A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-224A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-224A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-224A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_shadow_box' "$MODULE" "module must export lifecycle-keyed release shadow owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_shadow_box.hako' "$MEMORY_README" "memory README must name lifecycle-keyed release shadow owner"
guard_expect_in_file "$TAG" 'recordLifecycleKeyedRelease' "$SHADOW_OWNER" "shadow owner must expose lifecycle-keyed release route"
guard_expect_in_file "$TAG" 'would_migrate_release_ledger_key: i64 = 0' "$SHADOW_OWNER" "shadow report must keep source release-key migration inactive"
guard_expect_in_file "$TAG" 'check "mimap224a segment map local free reuse ledger lifecycle-keyed release shadow"' "$APP" "MIMAP-224A proof must use labelled check block"

if rg -n 'reuse_lifecycle_token|recordLifecycleKeyedRelease|LifecycleKeyedReleaseShadow|would_migrate_release_ledger_key' \
  "$SOURCE_RELEASE_OWNER" >/tmp/"$TAG".source_migration_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-224A must not migrate source release owner to lifecycle keys" >&2
  cat /tmp/"$TAG".source_migration_leak >&2
  rm -f /tmp/"$TAG".source_migration_leak
  exit 1
fi
rm -f /tmp/"$TAG".source_migration_leak

if rg -n 'migrateReleaseLedger|recordLifecycleRelease|generationToken|realLifecycle|releaseLedgerKeyMigration|would_migrate_release_ledger_key[[:space:]]*=[[:space:]]*1' \
  "$APP" "$SHADOW_OWNER" >/tmp/"$TAG".migration_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-224A must not migrate release-ledger keys or define real lifecycle semantics" >&2
  cat /tmp/"$TAG".migration_leak >&2
  rm -f /tmp/"$TAG".migration_leak
  exit 1
fi
rm -f /tmp/"$TAG".migration_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$SHADOW_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-224A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof|LifecycleKeyedReleaseShadow|lifecycleKeyedReleaseShadow' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-224A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap224_lifecycle_shadow.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap224.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,0' "$vm_log"
rg -F -q 'shadow=1,0,0,70007004,2,70007004002,1,0' "$vm_log"
rg -F -q 'rejects=0,1,0,2,0,3,0,4,0,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseShadow.recordLifecycleKeyedRelease/3",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenReleaseKeyPrecondition.classify/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseShadowReport")
if report is None:
    raise SystemExit("missing lifecycle-keyed release shadow report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "modeled_reuse_token",
    "lifecycle_id",
    "reuse_lifecycle_token",
    "release_duplicate_seen",
    "lifecycle_keyed_release_shadow_present",
    "would_migrate_release_ledger_key",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_activate_provider",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-keyed release shadow report field: {name}")

print("[mimap224a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
