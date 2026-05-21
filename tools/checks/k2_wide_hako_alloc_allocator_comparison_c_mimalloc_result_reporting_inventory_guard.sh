#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory"
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
    echo "[$TAG] ERROR: MIMAP-460A is reporting inventory; EXE evidence is deferred to a later reporting closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-proof/test.sh"
CARD_458A="docs/development/current/main/phases/phase-293x/293x-1088-MIMAP-458A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-DIAGNOSTICS.md"
CARD_459A="docs/development/current/main/phases/phase-293x/293x-1089-MIMAP-459A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1090-MIMAP-460A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-REPORTING-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-ssot.md"
DESIGN_458A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_reporting_inventory_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_summary_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-460A allocator comparison C mimalloc result reporting inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_458A" "$CARD_459A" "$CARD" "$DESIGN" "$DESIGN_458A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_458A" "MIMAP-458A must be landed"
guard_expect_in_file "$TAG" 'Status: completed' "$CARD_459A" "MIMAP-459A closeout must be completed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-460A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-460A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_458A" "MIMAP-458A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-460A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-460A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-460A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-460A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-c-mimalloc-result-reporting-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-460A must defer EXE to reporting closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_reporting_inventory_box' "$MODULE" "module must export reporting inventory owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_reporting_inventory_box.hako' "$MEMORY_README" "memory README must name reporting inventory owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultReportingInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultReportingInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryAllocatorComparisonCMimallocResultReporting' "$OWNER" "owner must expose reporting inventory route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnosticReport' "$OWNER" "owner must consume MIMAP-458A summary diagnostic report"
guard_expect_in_file "$TAG" 'performance_conclusion_made: report.performance_conclusion_made' "$OWNER" "reporting inventory must preserve performance conclusion field"
guard_expect_in_file "$TAG" 'memory_conclusion_made: report.memory_conclusion_made' "$OWNER" "reporting inventory must preserve memory conclusion field"
guard_expect_in_file "$TAG" 'repeated_benchmark_executed: report.repeated_benchmark_executed' "$OWNER" "reporting inventory must preserve repeated benchmark field"
guard_expect_in_file "$TAG" 'provider_package_generated: report.provider_package_generated' "$OWNER" "reporting inventory must preserve provider package field"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-460A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultReportingInventory|allocator-comparison-c-mimalloc-result-reporting-inventory-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-460A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap460_c_mimalloc_result_reporting.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap460.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-proof' "$vm_log"
rg -F -q 'report=1,0,1,1,1,0,1,1' "$vm_log"
rg -F -q 'metrics=3,72,64,33254,4096,61,33182' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultReportingInventory.makeAllocatorComparisonCMimallocResultReportingInventoryReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultReportingInventory.inventoryAllocatorComparisonCMimallocResultReporting/1",
    "HakoAllocAllocatorComparisonCMimallocResultReportingInventory.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultReportingInventoryReport")
if report is None:
    raise SystemExit("missing C mimalloc result reporting inventory report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultReportingInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc result reporting inventory ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "reporting_inventory_present",
    "summary_diagnostic_present",
    "accepted_summary_diagnostic_present",
    "blocked_summary_diagnostic_present",
    "comparison_available",
    "hako_allocation_count",
    "hako_requested_bytes",
    "c_allocation_count",
    "c_requested_bytes",
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
print("[mimap460a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
