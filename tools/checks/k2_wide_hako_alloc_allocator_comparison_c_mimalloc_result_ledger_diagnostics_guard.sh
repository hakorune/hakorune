#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-455A defers repeated benchmark evidence to closeout/reporting rows" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-proof/test.sh"
CARD_454A="docs/development/current/main/phases/phase-293x/293x-1076-MIMAP-454A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1079-MIMAP-455A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-DIAGNOSTICS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1080-MIMAP-456A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-CLOSEOUT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-ssot.md"
DESIGN_454A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-455A allocator comparison C mimalloc result ledger diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_454A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_454A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_454A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-456A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-455A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_454A" "MIMAP-454A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-455A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-455A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-455A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-455A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-c-mimalloc-result-ledger-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-455A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_ledger_diagnostic_box' "$MODULE" "module must export result ledger diagnostic owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_ledger_diagnostic_box.hako' "$MEMORY_README" "memory README must name result ledger diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultLedgerDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonCMimallocResultLedger' "$OWNER" "owner must expose result ledger diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultLedgerReport' "$OWNER" "owner must consume MIMAP-454A result ledger report"
guard_expect_in_file "$TAG" 'performance_conclusion_made: report.performance_conclusion_made' "$OWNER" "diagnostics must preserve performance conclusion field"
guard_expect_in_file "$TAG" 'memory_conclusion_made: report.memory_conclusion_made' "$OWNER" "diagnostics must preserve memory conclusion field"
guard_expect_in_file "$TAG" 'repeated_benchmark_executed: report.repeated_benchmark_executed' "$OWNER" "diagnostics must preserve repeated benchmark field"
guard_expect_in_file "$TAG" 'provider_package_generated: report.provider_package_generated' "$OWNER" "diagnostics must preserve provider package field"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-455A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultLedgerDiagnostic|allocator-comparison-c-mimalloc-result-ledger-diagnostics-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-455A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap455_c_mimalloc_result_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap455.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,0,1,1' "$vm_log"
rg -F -q 'metrics=3,64,64,4096,61,33182' "$vm_log"
rg -F -q 'owner=5,1,4,1,1,1,1,4' "$vm_log"
rg -F -q 'blocked=1,1,1,1' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic.makeAllocatorComparisonCMimallocResultLedgerDiagnosticReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic.diagnoseAllocatorComparisonCMimallocResultLedger/1",
    "HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnosticReport")
if report is None:
    raise SystemExit("missing C mimalloc result ledger diagnostic report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc result ledger diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "diagnostic_present",
    "result_ledger_present",
    "accepted_result_present",
    "blocked_result_present",
    "comparison_available",
    "missing_hako_blocked",
    "blocked_hako_blocked",
    "missing_c_blocked",
    "blocked_c_blocked",
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
print("[mimap455a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
