#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary"
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
    echo "[$TAG] ERROR: MIMAP-308A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-proof/test.sh"
CARD_306A="docs/development/current/main/phases/phase-293x/293x-909-MIMAP-306A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-CONTINUATION-APPLICATION-BRIDGE-CLOSEOUT.md"
CARD_307A="docs/development/current/main/phases/phase-293x/293x-910-MIMAP-307A-POST-RELEASE-RECYCLE-CONTINUATION-APPLICATION-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-911-MIMAP-308A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-APPLIED-STATE-SUMMARY-INVENTORY.md"
DESIGN_304A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APPLICATION_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_box.hako"
SUMMARY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_guard.sh"

printf '[%s] checking MIMAP-308A segment arena backing modeled allocation-ledger release/recycle applied-state summary\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_306A" \
  "$CARD_307A" \
  "$CARD" \
  "$DESIGN_304A" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$PROOF_MANIFEST_INCLUDE" \
  "$MODULE" \
  "$MEMORY_README" \
  "$APPLICATION_OWNER" \
  "$SUMMARY_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_306A" "MIMAP-306A continuation application bridge closeout must be landed before applied-state summary"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_307A" "MIMAP-307A selection must be landed before applied-state summary"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-308A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_304A" "MIMAP-304A application bridge design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-308A summary design must be accepted"
guard_expect_in_file "$TAG" 'model-only applied-state summary' "$DESIGN" "MIMAP-308A design must keep summary model-only"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-308A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-308A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-308A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-308A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-308A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_box' "$MODULE" "module must export applied-state summary owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_box.hako' "$MEMORY_README" "memory README must name applied-state summary owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleAppliedStateSummaryReportFields' "$SUMMARY_OWNER" "summary owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAppliedStateSummaryReport' "$SUMMARY_OWNER" "summary owner must expose ReportFields helper-argument scalarization helper"
guard_expect_in_file "$TAG" 'return me.makeAppliedStateSummaryReport' "$SUMMARY_OWNER" "summary makeReport must delegate report copy through ReportFields helper"
guard_expect_in_file "$TAG" 'summarizeAppliedState' "$SUMMARY_OWNER" "summary owner must expose summary route"
guard_expect_in_file "$TAG" 'applied_state_ready: i64' "$SUMMARY_OWNER" "summary report must publish ready bit"
guard_expect_in_file "$TAG" 'applied_backing_bytes: usize' "$SUMMARY_OWNER" "summary applied backing bytes must be exact usize"
guard_expect_in_file "$TAG" 'applied_committed_bytes: usize' "$SUMMARY_OWNER" "summary applied committed bytes must be exact usize"
guard_expect_in_file "$TAG" 'remaining_source_bytes: usize' "$SUMMARY_OWNER" "summary remaining source bytes must be exact usize"
guard_expect_in_file "$TAG" 'check "mimap308a segment arena backing modeled allocation ledger release recycle applied state summary inventory"' "$APP" "proof must use labelled check block"

if rg -n 'recordContinuationApplication|recordLifecycleContinuation|realLifecycle|generationToken|migrateReleaseLedger|lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$SUMMARY_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-308A summary owner must keep lifecycle/pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-proof|ModeledAllocationLedgerReleaseRecycleAppliedStateSummary|modeledAllocationLedgerReleaseRecycleAppliedStateSummary' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-308A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap308_applied_state_summary.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap308.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-proof' "$vm_log"
rg -F -q 'summary=1,0,1,190,19,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejects=1,2,3,4' "$vm_log"
rg -F -q 'owner=5,1,4,1,1,1,1,4,99019005009' "$vm_log"
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
    "Main.makeRecycleReport/3",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeInventory.recordContinuationApplication/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleAppliedStateSummaryInventory.makeAppliedStateSummaryReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleAppliedStateSummaryInventory.summarizeAppliedState/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
record_decl = None
target_record = "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleAppliedStateSummaryReportFields"
for decl in data.get("record_decls", []):
    if isinstance(decl, dict) and decl.get("name") == target_record:
        record_decl = decl
        break
    if decl == target_record:
        record_decl = {"field_decls": []}
        break
if record_decl is None:
    raise SystemExit("missing applied-state summary ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleAppliedStateSummaryReport")
if report is None:
    raise SystemExit("missing applied-state summary report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "summarized",
    "reason",
    "applied_state_summary_present",
    "application_token",
    "applied_state_ready",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing applied-state summary field: {name}")

for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"applied-state summary {name} must be exact usize storage: {field}")

for name in ("reason", "row_index", "continuation_token", "application_token", "applied_state_ready"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"applied-state summary {name} must remain i64 storage: {field}")

record_fields = {
    field.get("name"): field
    for field in record_decl.get("field_decls", [])
}
for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = record_fields.get(name)
    if field is None or field.get("declared_type", field.get("type")) != "usize":
        raise SystemExit(f"applied-state summary ReportFields {name} must be usize: {field}")

print("[mimap308a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
