#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge"
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
    echo "[$TAG] ERROR: MIMAP-304A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-proof/test.sh"
CARD_302A="docs/development/current/main/phases/phase-293x/293x-905-MIMAP-302A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-CONTINUATION-BRIDGE-CLOSEOUT.md"
CARD_303A="docs/development/current/main/phases/phase-293x/293x-906-MIMAP-303A-POST-RELEASE-RECYCLE-LIFECYCLE-CONTINUATION-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-907-MIMAP-304A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-CONTINUATION-APPLICATION-BRIDGE-INVENTORY.md"
DESIGN_300A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
BRIDGE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_box.hako"
APPLICATION_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_guard.sh"

printf '[%s] checking MIMAP-304A segment arena backing modeled allocation-ledger release/recycle continuation application bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_302A" \
  "$CARD_303A" \
  "$CARD" \
  "$DESIGN_300A" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$BRIDGE_OWNER" \
  "$APPLICATION_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_302A" "MIMAP-302A lifecycle continuation closeout must be landed before application bridge"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_303A" "MIMAP-303A selection must be landed before application bridge"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-304A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_300A" "MIMAP-300A lifecycle continuation design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-304A application bridge design must be accepted"
guard_expect_in_file "$TAG" 'model-only application row' "$DESIGN" "MIMAP-304A design must keep application row model-only"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-304A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-304A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-304A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-304A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-304A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_box' "$MODULE" "module must export continuation application bridge owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_box.hako' "$MEMORY_README" "memory README must name continuation application bridge owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeReportFields' "$APPLICATION_OWNER" "application bridge owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'makeApplicationReport' "$APPLICATION_OWNER" "application bridge owner must expose ReportFields helper-argument scalarization helper"
guard_expect_in_file "$TAG" 'return me.makeApplicationReport' "$APPLICATION_OWNER" "application makeReport must delegate report copy through ReportFields helper"
guard_expect_in_file "$TAG" 'recordContinuationApplication' "$APPLICATION_OWNER" "application bridge owner must expose record route"
guard_expect_in_file "$TAG" 'application_token: i64' "$APPLICATION_OWNER" "application bridge report must carry application token"
guard_expect_in_file "$TAG" 'applied_backing_bytes: usize' "$APPLICATION_OWNER" "application bridge byte group must use exact usize"
guard_expect_in_file "$TAG" 'remaining_source_bytes: usize' "$APPLICATION_OWNER" "application bridge remaining bytes must use exact usize"
guard_expect_in_file "$TAG" 'check "mimap304a segment arena backing modeled allocation ledger release recycle continuation application bridge"' "$APP" "proof must use labelled check block"

if rg -n 'realLifecycle|generationToken|migrateReleaseLedger|releaseLedgerKeyMigration|recordLifecycleContinuation[[:space:]]*\(' \
  "$APPLICATION_OWNER" >/tmp/"$TAG".scope_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-304A application owner must not define real lifecycle generation, migration, or lifecycle continuation recording" >&2
  cat /tmp/"$TAG".scope_leak >&2
  rm -f /tmp/"$TAG".scope_leak
  exit 1
fi
rm -f /tmp/"$TAG".scope_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$APPLICATION_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-304A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-proof|ModeledAllocationLedgerReleaseRecycleContinuationApplicationBridge|modeledAllocationLedgerReleaseRecycleContinuationApplicationBridge' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-304A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap304_continuation_application.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap304.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-proof' "$vm_log"
rg -F -q 'application=1,0,1,190,19,3,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,5,99019005005' "$vm_log"
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
    "Main.makeRecycleReport/3",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleLifecycleContinuationBridgeInventory.recordLifecycleContinuation/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeInventory.makeApplicationReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeInventory.recordContinuationApplication/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
record_names = set()
record_decl = None
target_record = "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeReportFields"
for decl in data.get("record_decls", []):
    if isinstance(decl, str):
        record_names.add(decl)
    elif isinstance(decl, dict):
        record_names.add(decl.get("name"))
        if decl.get("name") == target_record:
            record_decl = decl
if target_record not in record_names:
    raise SystemExit("missing continuation application ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeReport")
if report is None:
    raise SystemExit("missing continuation application report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
required_fields = (
    "accepted",
    "reason",
    "continuation_application_present",
    "modeled_continuation_application_present",
    "continuation_token",
    "application_token",
    "applied_backing_bytes",
    "applied_committed_bytes",
    "remaining_source_bytes",
    "would_add_backend_matcher",
)
for name in required_fields:
    if name not in fields:
        raise SystemExit(f"missing continuation application field: {name}")

for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"continuation application {name} must be exact usize storage: {field}")

for name in ("reason", "row_index", "continuation_token", "application_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"continuation application {name} must remain i64 storage: {field}")

if record_decl is None:
    raise SystemExit("missing continuation application ReportFields details")
record_fields = {
    field.get("name"): field
    for field in record_decl.get("field_decls", [])
}
for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = record_fields.get(name)
    if field is None or field.get("declared_type", field.get("type")) != "usize":
        raise SystemExit(f"continuation application ReportFields {name} must be usize: {field}")
for name in ("reason", "row_index", "continuation_token", "application_token"):
    field = record_fields.get(name)
    if field is None or field.get("declared_type", field.get("type")) != "i64":
        raise SystemExit(f"continuation application ReportFields {name} must be i64: {field}")

print("[mimap304a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
