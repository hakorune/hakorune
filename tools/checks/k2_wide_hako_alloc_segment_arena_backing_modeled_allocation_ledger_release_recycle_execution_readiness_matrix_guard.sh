#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix"
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
    echo "[$TAG] ERROR: MIMAP-312A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-proof/test.sh"
CARD_310A="docs/development/current/main/phases/phase-293x/293x-925-MIMAP-310A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-APPLIED-STATE-SUMMARY-CLOSEOUT.md"
CARD_311A="docs/development/current/main/phases/phase-293x/293x-926-MIMAP-311A-POST-RELEASE-RECYCLE-APPLIED-STATE-SUMMARY-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-927-MIMAP-312A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-READINESS-MATRIX-INVENTORY.md"
DESIGN_308A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SUMMARY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_box.hako"
MATRIX_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_guard.sh"

printf '[%s] checking MIMAP-312A segment arena backing modeled allocation-ledger release/recycle execution readiness matrix\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_310A" \
  "$CARD_311A" \
  "$CARD" \
  "$DESIGN_308A" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST_INCLUDE" \
  "$MODULE" \
  "$MEMORY_README" \
  "$SUMMARY_OWNER" \
  "$MATRIX_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_310A" "MIMAP-310A closeout must be landed before readiness matrix"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_311A" "MIMAP-311A selection must be landed before readiness matrix"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-312A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_308A" "MIMAP-308A summary design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-312A design must be accepted"
guard_expect_in_file "$TAG" 'model-only readiness matrix' "$DESIGN" "MIMAP-312A design must keep matrix model-only"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-312A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-312A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-312A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-312A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-312A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box' "$MODULE" "module must export execution readiness matrix owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako' "$MEMORY_README" "memory README must name execution readiness matrix owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixReportFields' "$MATRIX_OWNER" "matrix owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'makeExecutionReadinessMatrixReport' "$MATRIX_OWNER" "matrix owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordExecutionReadinessMatrix' "$MATRIX_OWNER" "matrix owner must expose record route"
guard_expect_in_file "$TAG" 'applied_backing_bytes: usize' "$MATRIX_OWNER" "matrix report must mirror backing bytes as usize"
guard_expect_in_file "$TAG" 'applied_committed_bytes: usize' "$MATRIX_OWNER" "matrix report must mirror committed bytes as usize"
guard_expect_in_file "$TAG" 'remaining_source_bytes: usize' "$MATRIX_OWNER" "matrix report must mirror remaining bytes as usize"
guard_expect_in_file "$TAG" 'check "mimap312a segment arena backing modeled allocation ledger release recycle execution readiness matrix inventory"' "$APP" "proof must use labelled check block"

if rg -n 'summarizeAppliedState|recordContinuationApplication|recordLifecycleContinuation|realLifecycle|generationToken|lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$MATRIX_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-312A matrix owner must keep lifecycle/pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-proof|ModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrix|modeledAllocationLedgerReleaseRecycleExecutionReadinessMatrix' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-312A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap312_execution_readiness_matrix.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap312.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-proof' "$vm_log"
rg -F -q 'matrix=1,0,1,1,1,0' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,15' "$vm_log"
rg -F -q 'blocked=1,1,1,1' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,8,9,15' "$vm_log"
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
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleAppliedStateSummaryInventory.summarizeAppliedState/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixInventory.makeExecutionReadinessMatrixReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixInventory.recordExecutionReadinessMatrix/12",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
record_decl = None
target_record = "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixReportFields"
for decl in data.get("record_decls", []):
    if isinstance(decl, dict) and decl.get("name") == target_record:
        record_decl = decl
        break
    if decl == target_record:
        record_decl = {"field_decls": []}
        break
if record_decl is None:
    raise SystemExit("missing execution readiness matrix ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionReadinessMatrixReport")
if report is None:
    raise SystemExit("missing execution readiness matrix report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "matrix_present",
    "summary_present",
    "summary_summarized",
    "requires_arena_backing_release",
    "requires_arena_backing_recycle",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing execution readiness matrix field: {name}")

for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"execution readiness matrix {name} must be exact usize storage: {field}")

record_fields = {
    field.get("name"): field
    for field in record_decl.get("field_decls", [])
}
for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = record_fields.get(name)
    if field is None or field.get("declared_type", field.get("type")) != "usize":
        raise SystemExit(f"execution readiness matrix ReportFields {name} must be usize: {field}")

print("[mimap312a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
