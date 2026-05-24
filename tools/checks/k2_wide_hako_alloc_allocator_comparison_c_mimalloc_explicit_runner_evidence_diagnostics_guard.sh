#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-452A defers repeated C mimalloc benchmark evidence to a later closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-proof/test.sh"
CARD_451A="docs/development/current/main/phases/phase-293x/293x-1073-MIMAP-451A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1074-MIMAP-452A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTICS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1075-MIMAP-453A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-CLOSEOUT.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-107-HAKO-ALLOC-USIZE-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTIC-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-108-HAKO-ALLOC-USIZE-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTIC-COUNTERS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-ssot.md"
DESIGN_451A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-452A explicit C mimalloc runner evidence diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_451A" "$CARD" "$NEXT_CARD" "$USIZE_SELECTION_CARD" "$USIZE_CARD" "$DESIGN" "$DESIGN_451A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MODULE_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_451A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-453A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-107 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-108 usize migration card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-452A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_451A" "MIMAP-451A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-452A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-452A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-452A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-452A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-c-mimalloc-explicit-runner-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-452A must defer repeated EXE/runner evidence to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box' "$MODULE" "module must export explicit runner evidence diagnostics owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box.hako' "$MODULE_INDEX" "memory module index must name explicit runner evidence diagnostics owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonCMimallocExplicitRunnerEvidence' "$OWNER" "owner must expose evidence diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReport' "$OWNER" "owner must consume MIMAP-451A pilot report"
guard_expect_in_file "$TAG" 'missing_runner_blocked' "$OWNER" "owner must diagnose missing runner"
guard_expect_in_file "$TAG" 'missing_output_blocked' "$OWNER" "owner must diagnose missing output"
guard_expect_in_file "$TAG" 'missing_memory_evidence_blocked' "$OWNER" "owner must diagnose missing memory evidence"
guard_expect_in_file "$TAG" 'missing_output_contract_blocked' "$OWNER" "owner must diagnose missing output contract"
guard_expect_in_file "$TAG" 'failed_runner_blocked' "$OWNER" "owner must diagnose failed runner"
guard_expect_in_file "$TAG" 'invalid_run_count_blocked' "$OWNER" "owner must diagnose invalid run count"
guard_expect_in_file "$TAG" 'diagnostic_count: usize = 0' "$OWNER" "diagnostic counter must be exact usize"
guard_expect_in_file "$TAG" 'ready_count: usize = 0' "$OWNER" "ready counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_count: usize = 0' "$OWNER" "blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_diagnostic_blocked_count: usize = 0' "$OWNER" "missing diagnostic blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'rejected_diagnostic_blocked_count: usize = 0' "$OWNER" "rejected diagnostic blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_runner_blocked_count: usize = 0' "$OWNER" "missing runner blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_output_blocked_count: usize = 0' "$OWNER" "missing output blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_memory_evidence_blocked_count: usize = 0' "$OWNER" "missing memory evidence blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_output_contract_blocked_count: usize = 0' "$OWNER" "missing output contract blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'failed_runner_blocked_count: usize = 0' "$OWNER" "failed runner blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'invalid_run_count_blocked_count: usize = 0' "$OWNER" "invalid run count blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "last reason must remain signed reason vocabulary"
guard_expect_in_file "$TAG" 'process_replacement_executed: report.process_replacement_executed' "$OWNER" "diagnostics must preserve process replacement closed field"
guard_expect_in_file "$TAG" 'provider_package_generated: report.provider_package_generated' "$OWNER" "diagnostics must preserve provider package closed field"

if rg -n 'bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|run_c_mimalloc[[:space:]]*\(|run_benchmark[[:space:]]*\(|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-452A owner/app must keep runner/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic|allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-proof|run_c_mimalloc|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-452A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap452_c_mimalloc_evidence_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap452.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,0,1,1' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic.makeAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReport/1",
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic.diagnoseAllocatorComparisonCMimallocExplicitRunnerEvidence/1",
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic")
if owner is None:
    raise SystemExit("missing explicit C mimalloc runner evidence diagnostic owner typed object plan")
report = plans.get("HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReport")
if report is None:
    raise SystemExit("missing explicit C mimalloc runner evidence diagnostic report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing explicit C mimalloc runner evidence diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "diagnostic_count",
    "ready_count",
    "blocked_count",
    "missing_diagnostic_blocked_count",
    "rejected_diagnostic_blocked_count",
    "missing_runner_blocked_count",
    "missing_output_blocked_count",
    "missing_memory_evidence_blocked_count",
    "missing_output_contract_blocked_count",
    "failed_runner_blocked_count",
    "invalid_run_count_blocked_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"explicit runner evidence diagnostic owner counter {name} must be usize storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"explicit runner evidence diagnostic last_reason must remain i64 storage: {field}")
for name in (
    "diagnostic_present",
    "execution_pilot_present",
    "c_mimalloc_execution_evidence_present",
    "blocked_evidence_present",
    "missing_runner_blocked",
    "missing_output_blocked",
    "missing_memory_evidence_blocked",
    "missing_output_contract_blocked",
    "failed_runner_blocked",
    "invalid_run_count_blocked",
    "c_mimalloc_executed",
    "process_replacement_executed",
    "global_allocator_installed",
    "provider_package_generated",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap452a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
