#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-445A defers L3/L4 to representative benchmark execution closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics-proof/test.sh"
CARD_444A="docs/development/current/main/phases/phase-293x/293x-1066-MIMAP-444A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1067-MIMAP-445A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-DIAGNOSTICS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1068-MIMAP-446A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-CLOSEOUT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_representative_benchmark_execution_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_representative_benchmark_execution_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-445A allocator comparison representative benchmark execution diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_444A" "$CARD" "$NEXT_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_444A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-446A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-445A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-444A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-445A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-445A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-445A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-445A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-comparison-representative-benchmark-execution-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-445A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_representative_benchmark_execution_diagnostic_box' "$MODULE" "module must export representative benchmark execution diagnostic owner"
guard_expect_in_file "$TAG" 'allocator_comparison_representative_benchmark_execution_diagnostic_box.hako' "$MEMORY_README" "memory README must name representative benchmark execution diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonRepresentativeBenchmarkExecutionDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonRepresentativeBenchmarkExecution' "$OWNER" "owner must expose diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilotReport' "$OWNER" "owner must consume MIMAP-444A execution report"
guard_expect_in_file "$TAG" 'not_ready_blocked' "$OWNER" "owner must diagnose not-ready execution"
guard_expect_in_file "$TAG" 'invalid_run_count_blocked' "$OWNER" "owner must diagnose invalid run count"
guard_expect_in_file "$TAG" 'missing_output_contract_blocked' "$OWNER" "owner must diagnose missing output contract"
guard_expect_in_file "$TAG" 'missing_evidence_storage_blocked' "$OWNER" "owner must diagnose missing evidence storage"
guard_expect_in_file "$TAG" 'closed_seam_blocked' "$OWNER" "owner must diagnose closed-seam leakage"
guard_expect_in_file "$TAG" 'allocation_count' "$OWNER" "owner must preserve allocation count diagnostics"
guard_expect_in_file "$TAG" 'requested_bytes' "$OWNER" "owner must preserve requested byte diagnostics"
guard_expect_in_file "$TAG" 'benchmark_executed: (0|i64 = 0)' "$OWNER" "benchmark execution must stay closed in diagnostic report"
guard_expect_in_file "$TAG" 'process_replacement_executed: (0|i64 = 0)' "$OWNER" "process replacement must stay closed in diagnostic report"
guard_expect_in_file "$TAG" 'hook_installed: (0|i64 = 0)' "$OWNER" "hook install must stay closed in diagnostic report"
guard_expect_in_file "$TAG" 'backend_matcher_added: (0|i64 = 0)' "$OWNER" "backend matcher addition must stay closed in diagnostic report"
guard_expect_in_file "$TAG" 'global_allocator_installed: (0|i64 = 0)' "$OWNER" "global allocator install must stay closed in diagnostic report"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: (0|i64 = 0)' "$OWNER" "host replacement must not execute in diagnostic report"
guard_expect_in_file "$TAG" 'would_install_hook: (0|i64 = 0)' "$OWNER" "hook installation must not execute in diagnostic report"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: (0|i64 = 0)' "$OWNER" "backend matcher addition must not execute in diagnostic report"
guard_expect_in_file "$TAG" 'would_run_thread: (0|i64 = 0)' "$OWNER" "thread execution must not execute in diagnostic report"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-445A owner/app must keep replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'allocator-comparison-representative-benchmark-execution-diagnostics-proof|AllocatorComparisonRepresentativeBenchmarkExecutionDiagnostic|allocatorComparisonRepresentativeBenchmarkExecutionDiagnostic|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-445A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap445_representative_benchmark_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap445.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,1,1,0' "$vm_log"
rg -F -q 'metrics=3,1,2,72,2,7,3' "$vm_log"
rg -F -q 'owner=6,1,5,1,1,1,1,1,5' "$vm_log"
rg -F -q 'blocked=1,1,1,1,1' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5' "$vm_log"
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
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnostic.makeAllocatorComparisonRepresentativeBenchmarkExecutionDiagnosticReport/1",
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnostic.diagnoseAllocatorComparisonRepresentativeBenchmarkExecution/1",
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnostic.reasonFrom/1",
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnostic.reportHasClosedSeam/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnosticReport")
if report is None:
    raise SystemExit("missing allocator comparison representative benchmark execution diagnostic report typed object plan")
target = "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing allocator comparison representative benchmark execution diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "diagnostic_present",
    "representative_benchmark_execution_present",
    "benchmark_executed",
    "execution_metrics_present",
    "ready_execution_present",
    "blocked_execution_present",
    "not_ready_blocked",
    "invalid_run_count_blocked",
    "missing_output_contract_blocked",
    "missing_evidence_storage_blocked",
    "closed_seam_blocked",
    "allocation_count",
    "requested_bytes",
    "process_replacement_executed",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap445a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
