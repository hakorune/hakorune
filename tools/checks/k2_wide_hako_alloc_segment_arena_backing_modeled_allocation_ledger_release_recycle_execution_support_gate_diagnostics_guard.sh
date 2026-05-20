#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-325A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-proof/test.sh"
CARD_324A="docs/development/current/main/phases/phase-293x/293x-939-MIMAP-324A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-GATE-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-940-MIMAP-325A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-GATE-DIAGNOSTICS.md"
DESIGN_324A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
GATE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostics_guard.sh"

printf '[%s] checking MIMAP-325A segment arena backing modeled allocation-ledger release/recycle execution support gate diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_324A" "$CARD" "$DESIGN_324A" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$GATE_OWNER" "$DIAGNOSTIC_OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_324A" "MIMAP-324A support gate must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-325A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_324A" "MIMAP-324A design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-325A diagnostic design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$DESIGN" "MIMAP-325A design must call out observer-only diagnostics"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-325A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-325A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-325A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-325A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-325A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostic_box' "$MODULE" "module must export support gate diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostic_box.hako' "$MEMORY_README" "memory README must name support gate diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportGateDiagnosticReportFields' "$DIAGNOSTIC_OWNER" "diagnostic owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeExecutionSupportGateDiagnosticReport' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'observeExecutionSupportGate' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$DIAGNOSTIC_OWNER" "diagnostic must mirror backing bytes as usize"
guard_expect_in_file "$TAG" 'report_applied_committed_bytes: usize' "$DIAGNOSTIC_OWNER" "diagnostic must mirror committed bytes as usize"
guard_expect_in_file "$TAG" 'report_remaining_source_bytes: usize' "$DIAGNOSTIC_OWNER" "diagnostic must mirror remaining bytes as usize"

if rg -n 'recordExecutionSupportGate|realLifecycle|generationToken|lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-325A diagnostic owner must keep support-gate recording/lifecycle/pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-proof|ModeledAllocationLedgerReleaseRecycleExecutionSupportGateDiagnostic|modeledAllocationLedgerReleaseRecycleExecutionSupportGateDiagnostic' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-325A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap325_support_gate_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap325.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,0,1,0,1' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'owner=4,1,3,1,1,1,1,3' "$vm_log"
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
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportGateDiagnostic.makeExecutionSupportGateDiagnosticReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportGateDiagnostic.observeExecutionSupportGate/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportGateDiagnosticReport")
if report is None:
    raise SystemExit("missing support gate diagnostic report typed object plan")
target = "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportGateDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing support gate diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
print("[mimap325a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
