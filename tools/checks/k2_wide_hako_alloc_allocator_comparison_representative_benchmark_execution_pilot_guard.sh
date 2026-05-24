#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-representative-benchmark-execution-pilot"
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
    echo "[$TAG] ERROR: MIMAP-444A defers L3/L4 native comparison evidence to a later closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-proof/test.sh"
CARD_443A="docs/development/current/main/phases/phase-293x/293x-1065-MIMAP-443A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1066-MIMAP-444A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1067-MIMAP-445A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-DIAGNOSTICS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-ssot.md"
CONTROLLED_DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-controlled-benchmark-execution-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MEMORY_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_representative_benchmark_execution_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-444A allocator comparison representative benchmark execution pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_443A" "$CARD" "$NEXT_CARD" "$DESIGN" "$CONTROLLED_DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$MEMORY_INDEX" "$OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_443A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-445A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-444A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$CONTROLLED_DESIGN" "controlled benchmark execution closeout must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-444A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-444A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-444A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-444A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'first_pattern = true' "$PROOF_MANIFEST_INCLUDE" "MIMAP-444A must be marked first-pattern"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_representative_benchmark_execution_pilot_box' "$MODULE" "module must export representative benchmark execution pilot owner"
guard_expect_in_file "$TAG" 'allocator_comparison_representative_benchmark_execution_pilot_box.hako' "$MEMORY_INDEX" "memory module index must name representative benchmark execution pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonRepresentativeBenchmarkExecutionPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'executeRepresentativeBenchmark' "$OWNER" "owner must expose representative execution route"
guard_expect_in_file "$TAG" 'HakoAllocProductionFacade' "$OWNER" "owner must use the hako_alloc production facade"
guard_expect_in_file "$TAG" 'allocation_count' "$OWNER" "owner must report allocation count"
guard_expect_in_file "$TAG" 'requested_bytes' "$OWNER" "owner must report requested bytes"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-444A owner/app must keep replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'allocator-comparison-representative-benchmark-execution-pilot-proof|AllocatorComparisonRepresentativeBenchmarkExecutionPilot|allocatorComparisonRepresentativeBenchmarkExecutionPilot|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-444A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap444_representative_benchmark.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap444.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-proof' "$vm_log"
rg -F -q 'exec=1,0,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'metrics=3,1,2,72,2,7,3' "$vm_log"
rg -F -q 'owner=6,1,5,1,1,1,1,1,5' "$vm_log"
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
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilot.makeAllocatorComparisonRepresentativeBenchmarkExecutionPilotReport/1",
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilot.executeRepresentativeBenchmark/4",
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilot.diagnosticHasClosedSeam/1",
    "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilot.reject/5",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilotReport")
if report is None:
    raise SystemExit("missing allocator comparison representative benchmark execution pilot report typed object plan")
target = "HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing allocator comparison representative benchmark execution pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "representative_benchmark_execution_present",
    "controlled_execution_diagnostic_present",
    "controlled_execution_shape_ready",
    "benchmark_executed",
    "allocation_count",
    "release_count",
    "reject_count",
    "requested_bytes",
    "outstanding_blocks",
    "process_replacement_executed",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap444a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
