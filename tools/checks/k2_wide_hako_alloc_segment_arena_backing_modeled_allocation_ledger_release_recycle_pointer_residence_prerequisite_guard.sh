#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite"
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
    echo "[$TAG] ERROR: MIMAP-336A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-proof/test.sh"
CARD_334A="docs/development/current/main/phases/phase-293x/293x-949-MIMAP-334A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-GENERATION-PREREQUISITE-CLOSEOUT.md"
CARD_335A="docs/development/current/main/phases/phase-293x/293x-950-MIMAP-335A-POST-RELEASE-RECYCLE-LIFECYCLE-GENERATION-PREREQUISITE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-951-MIMAP-336A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-POINTER-RESIDENCE-PREREQUISITE-INVENTORY.md"
LIFECYCLE_DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
LIFECYCLE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_generation_prerequisite_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_guard.sh"

printf '[%s] checking MIMAP-336A segment arena backing modeled allocation-ledger release/recycle pointer residence prerequisite\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_334A" "$CARD_335A" "$CARD" "$LIFECYCLE_DESIGN" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$LIFECYCLE_OWNER" "$OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_334A" "MIMAP-334A closeout must be landed before pointer residence prerequisite inventory"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_335A" "MIMAP-335A selection must be landed before pointer residence prerequisite inventory"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-336A card must be landed after prerequisite inventory is accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$LIFECYCLE_DESIGN" "lifecycle generation prerequisite design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-336A design must be accepted"
guard_expect_in_file "$TAG" 'model-only pointer residence prerequisite' "$DESIGN" "MIMAP-336A design must call out model-only pointer residence prerequisite"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-336A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-336A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-336A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-336A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-336A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_box' "$MODULE" "module must export pointer residence prerequisite owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_box.hako' "$MEMORY_README" "memory README must name pointer residence prerequisite owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerResidencePrerequisiteReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'recordPointerResidencePrerequisite' "$OWNER" "owner must expose pointer residence prerequisite inventory route"
guard_expect_in_file "$TAG" 'pointer_residence_supported: i64' "$OWNER" "owner must expose unsupported pointer residence flag"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$OWNER" "owner must mirror backing bytes as usize"

if rg -n 'realLifecycle|generationToken|createLifecycleGeneration|rawPointer|pointerResidence|lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-336A owner must keep lifecycle/pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-proof|ModeledAllocationLedgerReleaseRecyclePointerResidencePrerequisite|modeledAllocationLedgerReleaseRecyclePointerResidencePrerequisite' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-336A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap336_pointer_prereq.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap336.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-proof' "$vm_log"
rg -F -q 'prereq=1,0,1,1,0,0' "$vm_log"
rg -F -q 'lifecycle=1,1,0' "$vm_log"
rg -F -q 'owner=4,1,3,1,1,1,3' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejected=0,1,0,2,0,3' "$vm_log"
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

with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerResidencePrerequisiteInventory.makePointerResidencePrerequisiteReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerResidencePrerequisiteInventory.recordPointerResidencePrerequisite/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerResidencePrerequisiteReport")
if report is None:
    raise SystemExit("missing pointer residence prerequisite report typed object plan")
target = "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerResidencePrerequisiteReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing pointer residence prerequisite ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
print("[mimap336a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
