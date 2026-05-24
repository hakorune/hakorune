#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-ledger"
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
    echo "[$TAG] ERROR: MIMAP-454A is a scalar result ledger, not a repeated benchmark pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-proof/test.sh"
CARD_445A="docs/development/current/main/phases/phase-293x/293x-1067-MIMAP-445A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-DIAGNOSTICS.md"
CARD_452A="docs/development/current/main/phases/phase-293x/293x-1074-MIMAP-452A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1076-MIMAP-454A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1079-MIMAP-455A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-DIAGNOSTICS.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-109-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-LEDGER-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-110-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-LEDGER-COUNTERS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-454A allocator comparison C mimalloc result ledger\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_445A" "$CARD_452A" "$CARD" "$NEXT_CARD" "$USIZE_SELECTION_CARD" "$USIZE_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MODULE_INDEX" "$OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_445A" "$CARD_452A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-455A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-109 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-110 usize migration card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-454A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-454A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-454A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-454A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-454A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-c-mimalloc-result-ledger-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-454A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_ledger_box' "$MODULE" "module must export C mimalloc result ledger owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_ledger_box.hako' "$MODULE_INDEX" "memory module index must name C mimalloc result ledger owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultLedgerReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultLedgerReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordAllocatorComparisonCMimallocResult' "$OWNER" "owner must expose result ledger route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionDiagnosticReport' "$OWNER" "owner must consume Hako representative diagnostics"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReport' "$OWNER" "owner must consume C mimalloc evidence diagnostics"
guard_expect_in_file "$TAG" 'ledger_count: usize = 0' "$OWNER" "ledger counter must be exact usize"
guard_expect_in_file "$TAG" 'accepted_count: usize = 0' "$OWNER" "accepted counter must be exact usize"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$OWNER" "reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_hako_diagnostic_reject_count: usize = 0' "$OWNER" "missing hako diagnostic reject counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_hako_diagnostic_reject_count: usize = 0' "$OWNER" "blocked hako diagnostic reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_c_diagnostic_reject_count: usize = 0' "$OWNER" "missing C diagnostic reject counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_c_diagnostic_reject_count: usize = 0' "$OWNER" "blocked C diagnostic reject counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "last reason must remain signed reason vocabulary"
guard_expect_in_file "$TAG" 'performance_conclusion_made: 0' "$OWNER" "performance conclusion must stay closed"
guard_expect_in_file "$TAG" 'memory_conclusion_made: 0' "$OWNER" "memory conclusion must stay closed"
guard_expect_in_file "$TAG" 'repeated_benchmark_executed: 0' "$OWNER" "repeated benchmark execution must stay closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook installation must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator installation must stay closed"
guard_expect_in_file "$TAG" 'provider_package_generated: c_report.provider_package_generated' "$OWNER" "provider package field must be preserved from C evidence"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-454A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultLedger|allocator-comparison-c-mimalloc-result-ledger-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-454A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap454_c_mimalloc_result_ledger.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap454.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-ledger-proof' "$vm_log"
rg -F -q 'ledger=1,0,1,1,1,1,1,1' "$vm_log"
rg -F -q 'hako=3,1,2,72,2,7,3' "$vm_log"
rg -F -q 'c=64,64,33254,4096,1,61,33182' "$vm_log"
rg -F -q 'owner=5,1,4,1,1,1,1,4' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultLedger.makeAllocatorComparisonCMimallocResultLedgerReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultLedger.recordAllocatorComparisonCMimallocResult/2",
    "HakoAllocAllocatorComparisonCMimallocResultLedger.reject/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocAllocatorComparisonCMimallocResultLedger")
if owner is None:
    raise SystemExit("missing C mimalloc result ledger owner typed object plan")
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultLedgerReport")
if report is None:
    raise SystemExit("missing C mimalloc result ledger report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultLedgerReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc result ledger ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "ledger_count",
    "accepted_count",
    "reject_count",
    "missing_hako_diagnostic_reject_count",
    "blocked_hako_diagnostic_reject_count",
    "missing_c_diagnostic_reject_count",
    "blocked_c_diagnostic_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"C mimalloc result ledger owner counter {name} must be usize storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"C mimalloc result ledger last_reason must remain i64 storage: {field}")
for name in (
    "result_ledger_present",
    "hako_ready_execution_present",
    "c_ready_evidence_present",
    "comparison_available",
    "hako_allocation_count",
    "c_allocation_count",
    "c_peak_rss_bytes",
    "allocation_count_delta",
    "requested_bytes_delta",
    "performance_conclusion_made",
    "memory_conclusion_made",
    "repeated_benchmark_executed",
    "process_replacement_executed",
    "provider_package_generated",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap454a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
