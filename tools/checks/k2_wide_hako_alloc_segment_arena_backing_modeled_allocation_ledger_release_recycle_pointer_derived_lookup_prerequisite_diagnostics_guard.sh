#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-341A defers L3/L4 evidence to a future real-seam or batch pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics-proof/test.sh"
CARD_340A="docs/development/current/main/phases/phase-293x/293x-955-MIMAP-340A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-POINTER-DERIVED-LOOKUP-PREREQUISITE-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-956-MIMAP-341A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-POINTER-DERIVED-LOOKUP-PREREQUISITE-DIAGNOSTICS.md"
DESIGN_340A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
PREREQ_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-341A segment arena backing modeled allocation-ledger release/recycle pointer-derived lookup prerequisite diagnostics+closeout\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_340A" "$CARD" "$DESIGN_340A" "$DESIGN" "$CADENCE" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$PREREQ_OWNER" "$DIAGNOSTIC_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_340A" "MIMAP-340A prerequisite inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-341A card must be landed after diagnostics+closeout are accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_340A" "MIMAP-340A design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-341A diagnostic design must be accepted"
guard_expect_in_file "$TAG" 'Compressed Model-Only Prerequisite Cadence' "$CADENCE" "cadence SSOT must define compressed model-only prerequisite cadence"
guard_expect_in_file "$TAG" 'observer-only' "$DESIGN" "MIMAP-341A design must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'closeout' "$DESIGN" "MIMAP-341A design must absorb closeout"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-341A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-341A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-341A"
guard_expect_in_file "$TAG" 'row_kind = "diagnostic-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-341A must be a diagnostic-closeout row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-341A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-341A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_diagnostic_box' "$MODULE" "module must export pointer-derived lookup prerequisite diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_diagnostic_box.hako' "$MEMORY_README" "memory README must name pointer-derived lookup prerequisite diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerDerivedLookupPrerequisiteDiagnosticReportFields' "$DIAGNOSTIC_OWNER" "diagnostic owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makePointerDerivedLookupPrerequisiteDiagnosticReport' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'observePointerDerivedLookupPrerequisite' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$DIAGNOSTIC_OWNER" "diagnostic must mirror backing bytes as usize"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "pointer-derived lookup prerequisite L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-340A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-340A"
guard_expect_in_file "$TAG" "MIMAP-341A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-341A"
rm -f /tmp/"$TAG".proof_dry_run

if rg -n 'recordPointerDerivedLookupPrerequisite|realLifecycle|generationToken|createLifecycleGeneration|rawPointer|pointerResidence|lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-341A diagnostic owner must keep prerequisite recording/pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics-proof|ModeledAllocationLedgerReleaseRecyclePointerDerivedLookupPrerequisiteDiagnostic|modeledAllocationLedgerReleaseRecyclePointerDerivedLookupPrerequisiteDiagnostic' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-341A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap341_pointer_lookup_prereq_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap341.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,0,1,1,0' "$vm_log"
rg -F -q 'residence=1,1,0' "$vm_log"
rg -F -q 'owner=4,1,3,1,1,1,3' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejected=0,1,0,2,0,3,2' "$vm_log"
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
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerDerivedLookupPrerequisiteDiagnostic.makePointerDerivedLookupPrerequisiteDiagnosticReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerDerivedLookupPrerequisiteDiagnostic.observePointerDerivedLookupPrerequisite/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerDerivedLookupPrerequisiteDiagnosticReport")
if report is None:
    raise SystemExit("missing pointer-derived lookup prerequisite diagnostic report typed object plan")
target = "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecyclePointerDerivedLookupPrerequisiteDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing pointer-derived lookup prerequisite diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
print("[mimap341a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
