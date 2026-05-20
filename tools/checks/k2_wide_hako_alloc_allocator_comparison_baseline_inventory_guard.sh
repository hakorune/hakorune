#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-baseline-inventory"
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
    echo "[$TAG] ERROR: MIMAP-427A defers L3/L4 to the comparison baseline closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-baseline-inventory-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-baseline-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-baseline-inventory-proof/test.sh"
CARD_426A="docs/development/current/main/phases/phase-293x/293x-1048-MIMAP-426A-POST-HOST-REPLACEMENT-OPTIONAL-LADDER-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1049-MIMAP-427A-ALLOCATOR-COMPARISON-BASELINE-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1050-MIMAP-428A-ALLOCATOR-COMPARISON-BASELINE-DIAGNOSTICS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-baseline-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_baseline_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-427A allocator comparison baseline inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_426A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_426A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-428A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-427A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-427A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-427A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-427A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-427A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-comparison-baseline-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-427A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_baseline_inventory_box' "$MODULE" "module must export comparison baseline owner"
guard_expect_in_file "$TAG" 'allocator_comparison_baseline_inventory_box.hako' "$MEMORY_README" "memory README must name comparison baseline owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonBaselineInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonBaselineInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryAllocatorComparisonBaseline' "$OWNER" "owner must expose baseline inventory route"
guard_expect_in_file "$TAG" 'c_mimalloc_baseline_present' "$OWNER" "owner must track C mimalloc baseline"
guard_expect_in_file "$TAG" 'hako_alloc_baseline_present' "$OWNER" "owner must track hako_alloc baseline"
guard_expect_in_file "$TAG" 'throughput_target_present' "$OWNER" "owner must track throughput target"
guard_expect_in_file "$TAG" 'memory_usage_target_present' "$OWNER" "owner must track memory usage target"
guard_expect_in_file "$TAG" 'workload_matrix_present' "$OWNER" "owner must track workload matrix"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-427A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'allocator-comparison-baseline-inventory-proof|AllocatorComparisonBaselineInventory|allocatorComparisonBaselineInventory|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-427A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap427_comparison_baseline.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap427.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-baseline-inventory-proof' "$vm_log"
rg -F -q 'baseline=1,0,1,1,1,1,1,1,1,3' "$vm_log"
rg -F -q 'owner=7,1,6,1,1,1,1,1,1,6' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6' "$vm_log"
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
    "HakoAllocAllocatorComparisonBaselineInventory.makeAllocatorComparisonBaselineInventoryReport/1",
    "HakoAllocAllocatorComparisonBaselineInventory.inventoryAllocatorComparisonBaseline/6",
    "HakoAllocAllocatorComparisonBaselineInventory.reject/7",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonBaselineInventoryReport")
if report is None:
    raise SystemExit("missing allocator comparison baseline report typed object plan")
target = "HakoAllocAllocatorComparisonBaselineInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing allocator comparison baseline ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "c_mimalloc_baseline_present",
    "hako_alloc_baseline_present",
    "throughput_target_present",
    "memory_usage_target_present",
    "workload_matrix_present",
    "repeat_count",
    "process_replacement_executed",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap427a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
