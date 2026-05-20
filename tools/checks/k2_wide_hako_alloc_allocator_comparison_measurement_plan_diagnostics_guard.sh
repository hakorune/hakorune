#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-measurement-plan-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-434A defers L3/L4 to the measurement plan closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-measurement-plan-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-measurement-plan-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-measurement-plan-diagnostics-proof/test.sh"
CARD_433A="docs/development/current/main/phases/phase-293x/293x-1055-MIMAP-433A-ALLOCATOR-COMPARISON-MEASUREMENT-PLAN-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1056-MIMAP-434A-ALLOCATOR-COMPARISON-MEASUREMENT-PLAN-DIAGNOSTICS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1057-MIMAP-435A-ALLOCATOR-COMPARISON-MEASUREMENT-PLAN-CLOSEOUT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-measurement-plan-diagnostics-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-measurement-plan-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_measurement_plan_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_measurement_plan_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-434A allocator comparison measurement plan diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_433A" "$CARD" "$NEXT_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_433A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-435A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-434A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-433A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-434A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-434A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-434A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-434A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-comparison-measurement-plan-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-434A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_measurement_plan_diagnostic_box' "$MODULE" "module must export measurement plan diagnostic owner"
guard_expect_in_file "$TAG" 'allocator_comparison_measurement_plan_diagnostic_box.hako' "$MEMORY_README" "memory README must name measurement plan diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonMeasurementPlanDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonMeasurementPlanDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonMeasurementPlan' "$OWNER" "owner must expose diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonMeasurementPlanInventoryReport' "$OWNER" "owner must consume MIMAP-433A inventory report"
guard_expect_in_file "$TAG" 'missing_run_count_blocked' "$OWNER" "owner must diagnose missing run count"
guard_expect_in_file "$TAG" 'missing_warmup_plan_blocked' "$OWNER" "owner must diagnose missing warmup plan"
guard_expect_in_file "$TAG" 'missing_output_contract_blocked' "$OWNER" "owner must diagnose missing output contract"
guard_expect_in_file "$TAG" 'missing_throughput_measurement_blocked' "$OWNER" "owner must diagnose missing throughput measurement"
guard_expect_in_file "$TAG" 'missing_memory_usage_measurement_blocked' "$OWNER" "owner must diagnose missing memory measurement"
guard_expect_in_file "$TAG" 'invalid_run_count_blocked' "$OWNER" "owner must diagnose invalid run count"
guard_expect_in_file "$TAG" 'invalid_warmup_count_blocked' "$OWNER" "owner must diagnose invalid warmup count"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER" "benchmark execution must stay closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"
guard_expect_in_file "$TAG" 'would_run_benchmark: 0' "$OWNER" "benchmark execution must not run"

if rg -n 'run_benchmark[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-434A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'allocator-comparison-measurement-plan-diagnostics-proof|AllocatorComparisonMeasurementPlanDiagnostic|allocatorComparisonMeasurementPlanDiagnostic|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-434A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap434_measurement_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap434.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-measurement-plan-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,0' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'blocked=1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocAllocatorComparisonMeasurementPlanDiagnostic.makeAllocatorComparisonMeasurementPlanDiagnosticReport/1",
    "HakoAllocAllocatorComparisonMeasurementPlanDiagnostic.diagnoseAllocatorComparisonMeasurementPlan/1",
    "HakoAllocAllocatorComparisonMeasurementPlanDiagnostic.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonMeasurementPlanDiagnosticReport")
if report is None:
    raise SystemExit("missing allocator comparison measurement plan diagnostic report typed object plan")
target = "HakoAllocAllocatorComparisonMeasurementPlanDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing allocator comparison measurement plan diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "diagnostic_present",
    "blocked_measurement_present",
    "missing_run_count_blocked",
    "missing_warmup_plan_blocked",
    "missing_output_contract_blocked",
    "missing_throughput_measurement_blocked",
    "missing_memory_usage_measurement_blocked",
    "invalid_run_count_blocked",
    "closed_seam_blocked",
    "benchmark_executed",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap434a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
