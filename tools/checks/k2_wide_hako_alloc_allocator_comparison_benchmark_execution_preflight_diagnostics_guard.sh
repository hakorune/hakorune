#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-437A defers L3/L4 to the benchmark execution preflight closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics-proof/test.sh"
CARD_436A="docs/development/current/main/phases/phase-293x/293x-1058-MIMAP-436A-ALLOCATOR-COMPARISON-BENCHMARK-EXECUTION-PREFLIGHT-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1059-MIMAP-437A-ALLOCATOR-COMPARISON-BENCHMARK-EXECUTION-PREFLIGHT-DIAGNOSTICS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1060-MIMAP-438A-ALLOCATOR-COMPARISON-BENCHMARK-EXECUTION-PREFLIGHT-CLOSEOUT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-benchmark-execution-preflight-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MEMORY_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_benchmark_execution_preflight_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_benchmark_execution_preflight_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-437A allocator comparison benchmark execution preflight diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_436A" "$CARD" "$NEXT_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$MEMORY_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_436A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-438A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-437A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-436A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-437A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-437A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-437A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-437A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-comparison-benchmark-execution-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-437A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_benchmark_execution_preflight_diagnostic_box' "$MODULE" "module must export benchmark preflight diagnostic owner"
guard_expect_in_file "$TAG" 'allocator_comparison_benchmark_execution_preflight_diagnostic_box.hako' "$MEMORY_INDEX" "memory module index must name benchmark preflight diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonBenchmarkExecutionPreflightDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonBenchmarkExecutionPreflightDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonBenchmarkExecutionPreflight' "$OWNER" "owner must expose diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonBenchmarkExecutionPreflightInventoryReport' "$OWNER" "owner must consume MIMAP-436A inventory report"
guard_expect_in_file "$TAG" 'missing_runner_blocked' "$OWNER" "owner must diagnose missing runner"
guard_expect_in_file "$TAG" 'missing_output_capture_blocked' "$OWNER" "owner must diagnose missing output capture"
guard_expect_in_file "$TAG" 'missing_measurement_storage_blocked' "$OWNER" "owner must diagnose missing storage"
guard_expect_in_file "$TAG" 'workload_matrix_not_ready_blocked' "$OWNER" "owner must diagnose workload not ready"
guard_expect_in_file "$TAG" 'measurement_plan_not_ready_blocked' "$OWNER" "owner must diagnose measurement plan not ready"
guard_expect_in_file "$TAG" 'process_replacement_open_blocked' "$OWNER" "owner must diagnose process replacement seam"
guard_expect_in_file "$TAG" 'hidden_env_open_blocked' "$OWNER" "owner must diagnose hidden env seam"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER" "benchmark execution must stay closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"

if rg -n 'run_benchmark[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-437A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'allocator-comparison-benchmark-execution-preflight-diagnostics-proof|AllocatorComparisonBenchmarkExecutionPreflightDiagnostic|allocatorComparisonBenchmarkExecutionPreflightDiagnostic|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-437A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap437_benchmark_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap437.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,0' "$vm_log"
rg -F -q 'owner=12,1,11,1,1,1,1,1,1,1,1,1,1,1,11' "$vm_log"
rg -F -q 'blocked=1,1,1,1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6,7,8,9,10,11' "$vm_log"
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
    "HakoAllocAllocatorComparisonBenchmarkExecutionPreflightDiagnostic.makeAllocatorComparisonBenchmarkExecutionPreflightDiagnosticReport/1",
    "HakoAllocAllocatorComparisonBenchmarkExecutionPreflightDiagnostic.diagnoseAllocatorComparisonBenchmarkExecutionPreflight/1",
    "HakoAllocAllocatorComparisonBenchmarkExecutionPreflightDiagnostic.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonBenchmarkExecutionPreflightDiagnosticReport")
if report is None:
    raise SystemExit("missing allocator comparison benchmark preflight diagnostic report typed object plan")
target = "HakoAllocAllocatorComparisonBenchmarkExecutionPreflightDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing allocator comparison benchmark preflight diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "diagnostic_present",
    "blocked_preflight_present",
    "missing_runner_blocked",
    "missing_output_capture_blocked",
    "missing_measurement_storage_blocked",
    "workload_matrix_not_ready_blocked",
    "measurement_plan_not_ready_blocked",
    "process_replacement_open_blocked",
    "hidden_env_open_blocked",
    "closed_seam_blocked",
    "benchmark_executed",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap437a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
