#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -eq 0 ]; then
  VALIDATION_LEVEL="L2"
else
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
fi
case "$VALIDATION_LEVEL" in
  L0|L1|L2) ;;
  L3|L4)
    echo "[$TAG] ERROR: MIMAP-300A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-proof/test.sh"
CARD_298A="docs/development/current/main/phases/phase-293x/293x-901-MIMAP-298A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT.md"
CARD_299A="docs/development/current/main/phases/phase-293x/293x-902-MIMAP-299A-POST-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-903-MIMAP-300A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-CONTINUATION-BRIDGE-INVENTORY.md"
DESIGN_292A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
RECYCLE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_box.hako"
BRIDGE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_guard.sh"

printf '[%s] checking MIMAP-300A segment arena backing modeled allocation-ledger release/recycle lifecycle continuation bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_298A" \
  "$CARD_299A" \
  "$CARD" \
  "$DESIGN_292A" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$RECYCLE_OWNER" \
  "$BRIDGE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_298A" "MIMAP-298A second-release diagnostic closeout must be landed before lifecycle continuation"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_299A" "MIMAP-299A selection must be landed before lifecycle continuation"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-300A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_292A" "MIMAP-292A release-applied recycle design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-300A lifecycle continuation design must be accepted"
guard_expect_in_file "$TAG" 'model-only continuation token' "$DESIGN" "MIMAP-300A design must keep continuation token model-only"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-300A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-300A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-300A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-300A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-300A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_box' "$MODULE" "module must export lifecycle continuation bridge owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_box.hako' "$MEMORY_README" "memory README must name lifecycle continuation bridge owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleLifecycleContinuationBridgeReportFields' "$BRIDGE_OWNER" "bridge owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'makeContinuationReport' "$BRIDGE_OWNER" "bridge owner must expose ReportFields helper-argument scalarization helper"
guard_expect_in_file "$TAG" 'return me.makeContinuationReport' "$BRIDGE_OWNER" "bridge makeReport must delegate report copy through ReportFields helper"
guard_expect_in_file "$TAG" 'recordLifecycleContinuation' "$BRIDGE_OWNER" "bridge owner must expose record route"
guard_expect_in_file "$TAG" 'continuation_token: i64' "$BRIDGE_OWNER" "bridge report must carry continuation token"
guard_expect_in_file "$TAG" 'applied_backing_bytes: usize' "$BRIDGE_OWNER" "bridge byte/capacity group must use exact usize"
guard_expect_in_file "$TAG" 'remaining_source_bytes: usize' "$BRIDGE_OWNER" "bridge remaining bytes must use exact usize"
guard_expect_in_file "$TAG" 'check "mimap300a segment arena backing modeled allocation ledger release recycle lifecycle continuation bridge"' "$APP" "proof must use labelled check block"

if rg -n 'realLifecycle|generationToken|migrateReleaseLedger|releaseLedgerKeyMigration|recordReleaseAppliedRecycle[[:space:]]*\(' \
  "$BRIDGE_OWNER" >/tmp/"$TAG".scope_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-300A bridge owner must not define real lifecycle generation, migration, or recycle recording" >&2
  cat /tmp/"$TAG".scope_leak >&2
  rm -f /tmp/"$TAG".scope_leak
  exit 1
fi
rm -f /tmp/"$TAG".scope_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$BRIDGE_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-300A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-proof|ModeledAllocationLedgerReleaseRecycleLifecycleContinuationBridge|modeledAllocationLedgerReleaseRecycleLifecycleContinuationBridge' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-300A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap300_release_recycle_lifecycle_continuation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap300.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-proof' "$vm_log"
rg -F -q 'bridge=1,0,1,190,19,3,97019005005,98019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,5,98019005005' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "Main.makeReleaseApply/5",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseAppliedRecycleInventory.recordReleaseAppliedRecycle/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleLifecycleContinuationBridgeInventory.makeContinuationReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleLifecycleContinuationBridgeInventory.recordLifecycleContinuation/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
record_names = set()
for decl in data.get("record_decls", []):
    if isinstance(decl, str):
        record_names.add(decl)
    elif isinstance(decl, dict):
        record_names.add(decl.get("name"))
if "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleLifecycleContinuationBridgeReportFields" not in record_names:
    raise SystemExit("missing lifecycle continuation ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleLifecycleContinuationBridgeReport")
if report is None:
    raise SystemExit("missing lifecycle continuation report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
required_fields = (
    "accepted",
    "reason",
    "lifecycle_continuation_present",
    "modeled_lifecycle_continuation_present",
    "release_applied_recycle_token",
    "continuation_token",
    "applied_backing_bytes",
    "applied_committed_bytes",
    "remaining_source_bytes",
    "would_add_backend_matcher",
)
for name in required_fields:
    if name not in fields:
        raise SystemExit(f"missing lifecycle continuation field: {name}")

for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"lifecycle continuation {name} must be exact usize storage: {field}")

for name in ("reason", "row_index", "release_applied_recycle_token", "continuation_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"lifecycle continuation {name} must remain i64 storage: {field}")

print("[mimap300a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
