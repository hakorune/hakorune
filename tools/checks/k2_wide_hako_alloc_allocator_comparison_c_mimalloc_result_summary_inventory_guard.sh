#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory"
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
    echo "[$TAG] ERROR: MIMAP-457A is summary inventory; EXE evidence is deferred to a later summary closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-proof/test.sh"
CARD_454A="docs/development/current/main/phases/phase-293x/293x-1076-MIMAP-454A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-PILOT.md"
CARD_455A="docs/development/current/main/phases/phase-293x/293x-1079-MIMAP-455A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-DIAGNOSTICS.md"
CARD_456A="docs/development/current/main/phases/phase-293x/293x-1080-MIMAP-456A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-LEDGER-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1087-MIMAP-457A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-SUMMARY-INVENTORY.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-117-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-SUMMARY-INVENTORY-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-118-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-SUMMARY-INVENTORY-COUNTERS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-ssot.md"
DESIGN_455A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_summary_inventory_box.hako"
LEDGER_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_box.hako"
DIAG_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-457A allocator comparison C mimalloc result summary inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_454A" "$CARD_455A" "$CARD_456A" "$CARD" "$USIZE_SELECTION_CARD" "$USIZE_CARD" "$DESIGN" "$DESIGN_455A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$LEDGER_OWNER" "$DIAG_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_454A" "$CARD_455A"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: completed' "$CARD_456A" "MIMAP-456A closeout must be completed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-457A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-117 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-118 usize migration card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-457A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_455A" "MIMAP-455A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-457A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-457A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-457A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-457A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-c-mimalloc-result-summary-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-457A must defer EXE to summary closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_summary_inventory_box' "$MODULE" "module must export summary inventory owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_summary_inventory_box.hako' "$MEMORY_README" "memory README must name summary inventory owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultSummaryInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultSummaryInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'summarizeAllocatorComparisonCMimallocResult' "$OWNER" "owner must expose summary inventory route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultLedgerReport' "$OWNER" "owner must consume MIMAP-454A result ledger report"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnosticReport' "$OWNER" "owner must consume MIMAP-455A diagnostic report"
guard_expect_in_file "$TAG" 'performance_conclusion_made: ledger.performance_conclusion_made' "$OWNER" "summary must preserve performance conclusion field"
guard_expect_in_file "$TAG" 'memory_conclusion_made: ledger.memory_conclusion_made' "$OWNER" "summary must preserve memory conclusion field"
guard_expect_in_file "$TAG" 'repeated_benchmark_executed: ledger.repeated_benchmark_executed' "$OWNER" "summary must preserve repeated benchmark field"
guard_expect_in_file "$TAG" 'provider_package_generated: ledger.provider_package_generated' "$OWNER" "summary must preserve provider package field"
guard_expect_in_file "$TAG" 'summary_count: usize = 0' "$OWNER" "summary counter must be exact usize"
guard_expect_in_file "$TAG" 'ready_count: usize = 0' "$OWNER" "ready counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_count: usize = 0' "$OWNER" "blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_ledger_reject_count: usize = 0' "$OWNER" "missing ledger reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_diagnostic_reject_count: usize = 0' "$OWNER" "missing diagnostic reject counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_diagnostic_reject_count: usize = 0' "$OWNER" "blocked diagnostic reject counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "last reason must remain signed reason vocabulary"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-457A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultSummaryInventory|allocator-comparison-c-mimalloc-result-summary-inventory-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-457A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap457_c_mimalloc_result_summary.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap457.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-proof' "$vm_log"
rg -F -q 'summary=1,0,1,1,1,1,1,1' "$vm_log"
rg -F -q 'metrics=3,64,64,4096,61,33182' "$vm_log"
rg -F -q 'owner=4,1,3,1,1,1,3' "$vm_log"
rg -F -q 'blocked=0,0,0' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultSummaryInventory.makeAllocatorComparisonCMimallocResultSummaryInventoryReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultSummaryInventory.summarizeAllocatorComparisonCMimallocResult/2",
    "HakoAllocAllocatorComparisonCMimallocResultSummaryInventory.reasonFrom/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocAllocatorComparisonCMimallocResultSummaryInventory")
if owner is None:
    raise SystemExit("missing C mimalloc result summary inventory owner typed object plan")
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultSummaryInventoryReport")
if report is None:
    raise SystemExit("missing C mimalloc result summary inventory report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultSummaryInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc result summary inventory ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "summary_count",
    "ready_count",
    "blocked_count",
    "missing_ledger_reject_count",
    "missing_diagnostic_reject_count",
    "blocked_diagnostic_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"C mimalloc result summary inventory owner counter {name} must be usize storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"C mimalloc result summary inventory last_reason must remain i64 storage: {field}")
for name in (
    "summary_inventory_present",
    "result_ledger_present",
    "diagnostic_present",
    "accepted_result_present",
    "blocked_result_present",
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
print("[mimap457a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
