#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-449A defers L3/L4 to C mimalloc execution closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-proof/test.sh"
CARD_448A="docs/development/current/main/phases/phase-293x/293x-1070-MIMAP-448A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1071-MIMAP-449A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-DIAGNOSTICS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1072-MIMAP-450A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-CLOSEOUT.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-115-HAKO-ALLOC-USIZE-C-MIMALLOC-EXECUTION-DIAGNOSTIC-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-116-HAKO-ALLOC-USIZE-C-MIMALLOC-EXECUTION-DIAGNOSTIC-COUNTERS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-449A C mimalloc execution diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_448A" "$CARD" "$NEXT_CARD" "$USIZE_SELECTION_CARD" "$USIZE_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_448A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-450A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-115 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-116 usize migration card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-449A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-448A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-449A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-449A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-449A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-449A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-comparison-c-mimalloc-execution-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-449A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_execution_diagnostic_box' "$MODULE" "module must export C mimalloc execution diagnostic owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_execution_diagnostic_box.hako' "$MEMORY_README" "memory README must name C mimalloc execution diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocExecutionDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocExecutionDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonCMimallocExecution' "$OWNER" "owner must expose diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocExecutionInventoryReport' "$OWNER" "owner must consume MIMAP-448A inventory report"
guard_expect_in_file "$TAG" 'missing_runner_blocked' "$OWNER" "owner must diagnose missing C runner"
guard_expect_in_file "$TAG" 'missing_workload_blocked' "$OWNER" "owner must diagnose missing workload"
guard_expect_in_file "$TAG" 'missing_hako_metrics_blocked' "$OWNER" "owner must diagnose missing Hako metrics"
guard_expect_in_file "$TAG" 'missing_output_contract_blocked' "$OWNER" "owner must diagnose missing output contract"
guard_expect_in_file "$TAG" 'missing_memory_usage_contract_blocked' "$OWNER" "owner must diagnose missing memory usage contract"
guard_expect_in_file "$TAG" 'missing_evidence_storage_blocked' "$OWNER" "owner must diagnose missing evidence storage"
guard_expect_in_file "$TAG" 'missing_run_count_blocked' "$OWNER" "owner must diagnose missing run count"
guard_expect_in_file "$TAG" 'invalid_run_count_blocked' "$OWNER" "owner must diagnose invalid run count"
guard_expect_in_file "$TAG" 'diagnostic_count: usize = 0' "$OWNER" "diagnostic counter must be exact usize"
guard_expect_in_file "$TAG" 'ready_count: usize = 0' "$OWNER" "ready counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_count: usize = 0' "$OWNER" "blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_runner_blocked_count: usize = 0' "$OWNER" "missing runner blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_workload_blocked_count: usize = 0' "$OWNER" "missing workload blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_hako_metrics_blocked_count: usize = 0' "$OWNER" "missing hako metrics blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_output_contract_blocked_count: usize = 0' "$OWNER" "missing output contract blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_memory_usage_contract_blocked_count: usize = 0' "$OWNER" "missing memory usage contract blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_evidence_storage_blocked_count: usize = 0' "$OWNER" "missing evidence storage blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_run_count_blocked_count: usize = 0' "$OWNER" "missing run count blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'invalid_run_count_blocked_count: usize = 0' "$OWNER" "invalid run count blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "last reason must remain signed reason vocabulary"
guard_expect_in_file "$TAG" 'c_mimalloc_executed: 0' "$OWNER" "C mimalloc execution must stay closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"

if rg -n 'run_c_mimalloc[[:space:]]*\(|run_benchmark[[:space:]]*\(|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-449A owner/app must keep C execution/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'allocator-comparison-c-mimalloc-execution-diagnostics-proof|AllocatorComparisonCMimallocExecutionDiagnostic|allocatorComparisonCMimallocExecutionDiagnostic|run_c_mimalloc|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-449A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap449_c_mimalloc_execution_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap449.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,0,3,0' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'blocked=1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6,7,8' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

if ! pure_first_guard_level_allows_mir "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
python3 tools/checks/pure_first_route_preflight.py "$mir_json"
python3 - "$mir_json" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic.makeAllocatorComparisonCMimallocExecutionDiagnosticReport/1",
    "HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic.diagnoseAllocatorComparisonCMimallocExecution/1",
    "HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic")
if owner is None:
    raise SystemExit("missing C mimalloc execution diagnostic owner typed object plan")
report = plans.get("HakoAllocAllocatorComparisonCMimallocExecutionDiagnosticReport")
if report is None:
    raise SystemExit("missing C mimalloc execution diagnostic report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocExecutionDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc execution diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "diagnostic_count",
    "ready_count",
    "blocked_count",
    "missing_runner_blocked_count",
    "missing_workload_blocked_count",
    "missing_hako_metrics_blocked_count",
    "missing_output_contract_blocked_count",
    "missing_memory_usage_contract_blocked_count",
    "missing_evidence_storage_blocked_count",
    "missing_run_count_blocked_count",
    "invalid_run_count_blocked_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"C mimalloc execution diagnostic owner counter {name} must be usize storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"C mimalloc execution diagnostic last_reason must remain i64 storage: {field}")
for name in (
    "diagnostic_present",
    "c_mimalloc_execution_inventory_present",
    "c_mimalloc_execution_ready",
    "blocked_execution_present",
    "missing_runner_blocked",
    "missing_workload_blocked",
    "missing_hako_metrics_blocked",
    "missing_output_contract_blocked",
    "missing_memory_usage_contract_blocked",
    "missing_evidence_storage_blocked",
    "missing_run_count_blocked",
    "invalid_run_count_blocked",
    "c_mimalloc_executed",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap449a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
