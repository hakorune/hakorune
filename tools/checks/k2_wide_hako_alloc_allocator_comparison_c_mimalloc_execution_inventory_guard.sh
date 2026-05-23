#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-execution-inventory"
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
    echo "[$TAG] ERROR: MIMAP-448A defers L3/L4 to C mimalloc execution closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-proof/test.sh"
CARD_447A="docs/development/current/main/phases/phase-293x/293x-1069-MIMAP-447A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-PLAN.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1070-MIMAP-448A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1071-MIMAP-449A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-DIAGNOSTICS.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-113-HAKO-ALLOC-USIZE-C-MIMALLOC-EXECUTION-INVENTORY-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-114-HAKO-ALLOC-USIZE-C-MIMALLOC-EXECUTION-INVENTORY-COUNTERS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-448A C mimalloc execution inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_447A" "$CARD" "$NEXT_CARD" "$USIZE_SELECTION_CARD" "$USIZE_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_447A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-449A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-113 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-114 usize migration card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-448A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-447A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-448A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-448A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-448A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-448A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-comparison-c-mimalloc-execution-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-448A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_execution_inventory_box' "$MODULE" "module must export C mimalloc execution inventory owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_execution_inventory_box.hako' "$MEMORY_README" "memory README must name C mimalloc execution inventory owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocExecutionInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocExecutionInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryAllocatorComparisonCMimallocExecution' "$OWNER" "owner must expose inventory route"
guard_expect_in_file "$TAG" 'c_mimalloc_runner_present' "$OWNER" "owner must track explicit C mimalloc runner"
guard_expect_in_file "$TAG" 'representative_workload_contract_present' "$OWNER" "owner must track representative workload contract"
guard_expect_in_file "$TAG" 'hako_representative_metrics_present' "$OWNER" "owner must track Hako representative metrics input"
guard_expect_in_file "$TAG" 'output_contract_present' "$OWNER" "owner must track output contract"
guard_expect_in_file "$TAG" 'memory_usage_contract_present' "$OWNER" "owner must track memory usage contract"
guard_expect_in_file "$TAG" 'evidence_storage_present' "$OWNER" "owner must track evidence storage"
guard_expect_in_file "$TAG" 'inventory_count: usize = 0' "$OWNER" "inventory counter must be exact usize"
guard_expect_in_file "$TAG" 'accepted_count: usize = 0' "$OWNER" "accepted counter must be exact usize"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$OWNER" "reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_runner_reject_count: usize = 0' "$OWNER" "missing runner reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_workload_reject_count: usize = 0' "$OWNER" "missing workload reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_hako_metrics_reject_count: usize = 0' "$OWNER" "missing hako metrics reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_output_contract_reject_count: usize = 0' "$OWNER" "missing output contract reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_memory_usage_contract_reject_count: usize = 0' "$OWNER" "missing memory usage contract reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_evidence_storage_reject_count: usize = 0' "$OWNER" "missing evidence storage reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_run_count_reject_count: usize = 0' "$OWNER" "missing run count reject counter must be exact usize"
guard_expect_in_file "$TAG" 'invalid_run_count_reject_count: usize = 0' "$OWNER" "invalid run count reject counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "last reason must remain signed reason vocabulary"
guard_expect_in_file "$TAG" 'c_mimalloc_executed: 0' "$OWNER" "C mimalloc execution must stay closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"

if rg -n 'run_c_mimalloc[[:space:]]*\(|run_benchmark[[:space:]]*\(|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-448A owner/app must keep C execution/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'allocator-comparison-c-mimalloc-execution-inventory-proof|AllocatorComparisonCMimallocExecutionInventory|allocatorComparisonCMimallocExecutionInventory|run_c_mimalloc|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-448A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap448_c_mimalloc_execution_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap448.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-proof' "$vm_log"
rg -F -q 'inventory=1,0,1,1,1,1,1,1,1,1,1,3,0' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocExecutionInventory.makeAllocatorComparisonCMimallocExecutionInventoryReport/1",
    "HakoAllocAllocatorComparisonCMimallocExecutionInventory.inventoryAllocatorComparisonCMimallocExecution/8",
    "HakoAllocAllocatorComparisonCMimallocExecutionInventory.reject/9",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocAllocatorComparisonCMimallocExecutionInventory")
if owner is None:
    raise SystemExit("missing C mimalloc execution inventory owner typed object plan")
report = plans.get("HakoAllocAllocatorComparisonCMimallocExecutionInventoryReport")
if report is None:
    raise SystemExit("missing C mimalloc execution inventory report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocExecutionInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc execution inventory ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "inventory_count",
    "accepted_count",
    "reject_count",
    "missing_runner_reject_count",
    "missing_workload_reject_count",
    "missing_hako_metrics_reject_count",
    "missing_output_contract_reject_count",
    "missing_memory_usage_contract_reject_count",
    "missing_evidence_storage_reject_count",
    "missing_run_count_reject_count",
    "invalid_run_count_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"C mimalloc execution inventory owner counter {name} must be usize storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"C mimalloc execution inventory last_reason must remain i64 storage: {field}")
for name in (
    "c_mimalloc_execution_inventory_present",
    "c_mimalloc_execution_ready",
    "c_mimalloc_runner_present",
    "representative_workload_contract_present",
    "hako_representative_metrics_present",
    "output_contract_present",
    "memory_usage_contract_present",
    "evidence_storage_present",
    "run_count",
    "c_mimalloc_executed",
    "process_replacement_executed",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap448a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
