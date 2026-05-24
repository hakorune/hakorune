#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-458A defers EXE evidence to summary closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-proof/test.sh"
CARD_457A="docs/development/current/main/phases/phase-293x/293x-1087-MIMAP-457A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1088-MIMAP-458A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-DIAGNOSTICS.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-119-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-SUMMARY-DIAGNOSTIC-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-120-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-SUMMARY-DIAGNOSTIC-COUNTERS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-ssot.md"
DESIGN_457A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_summary_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_summary_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-458A allocator comparison C mimalloc result summary diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_457A" "$CARD" "$USIZE_SELECTION_CARD" "$USIZE_CARD" "$DESIGN" "$DESIGN_457A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MODULE_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_457A" "MIMAP-457A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-458A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-119 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-120 usize migration card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-458A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_457A" "MIMAP-457A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-458A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-458A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-458A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-458A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-c-mimalloc-result-summary-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-458A must defer EXE to summary closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_summary_diagnostic_box' "$MODULE" "module must export summary diagnostic owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_summary_diagnostic_box.hako' "$MODULE_INDEX" "memory module index must name summary diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultSummaryDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonCMimallocResultSummary' "$OWNER" "owner must expose summary diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultSummaryInventoryReport' "$OWNER" "owner must consume MIMAP-457A summary report"
guard_expect_in_file "$TAG" 'performance_conclusion_made: report.performance_conclusion_made' "$OWNER" "diagnostics must preserve performance conclusion field"
guard_expect_in_file "$TAG" 'memory_conclusion_made: report.memory_conclusion_made' "$OWNER" "diagnostics must preserve memory conclusion field"
guard_expect_in_file "$TAG" 'repeated_benchmark_executed: report.repeated_benchmark_executed' "$OWNER" "diagnostics must preserve repeated benchmark field"
guard_expect_in_file "$TAG" 'provider_package_generated: report.provider_package_generated' "$OWNER" "diagnostics must preserve provider package field"
guard_expect_in_file "$TAG" 'diagnostic_count: usize = 0' "$OWNER" "diagnostic counter must be exact usize"
guard_expect_in_file "$TAG" 'ready_count: usize = 0' "$OWNER" "ready counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_count: usize = 0' "$OWNER" "blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_summary_blocked_count: usize = 0' "$OWNER" "missing summary blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_summary_blocked_count: usize = 0' "$OWNER" "blocked summary blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "last reason must remain signed reason vocabulary"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-458A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultSummaryDiagnostic|allocator-comparison-c-mimalloc-result-summary-diagnostics-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-458A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap458_c_mimalloc_result_summary_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap458.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,0,1,1' "$vm_log"
rg -F -q 'metrics=3,64,64,4096,61,33182' "$vm_log"
rg -F -q 'owner=3,1,2,1,1,2' "$vm_log"
rg -F -q 'blocked=0,0' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic.makeAllocatorComparisonCMimallocResultSummaryDiagnosticReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic.diagnoseAllocatorComparisonCMimallocResultSummary/1",
    "HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic")
if owner is None:
    raise SystemExit("missing C mimalloc result summary diagnostic owner typed object plan")
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnosticReport")
if report is None:
    raise SystemExit("missing C mimalloc result summary diagnostic report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc result summary diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "diagnostic_count",
    "ready_count",
    "blocked_count",
    "missing_summary_blocked_count",
    "blocked_summary_blocked_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"C mimalloc result summary diagnostic owner counter {name} must be usize storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"C mimalloc result summary diagnostic last_reason must remain i64 storage: {field}")
for name in (
    "diagnostic_present",
    "summary_inventory_present",
    "accepted_summary_present",
    "blocked_summary_present",
    "comparison_available",
    "hako_allocation_count",
    "c_allocation_count",
    "c_peak_rss_bytes",
    "allocation_count_delta",
    "requested_bytes_delta",
    "performance_conclusion_made",
    "memory_conclusion_made",
    "repeated_benchmark_executed",
    "provider_package_generated",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap458a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
