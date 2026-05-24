#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot"
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
    echo "[$TAG] ERROR: MIMAP-566A remains a terminal planning pilot and does not define L3/L4 execution packs" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-proof/test.sh"
CARD_564A="docs/development/current/main/phases/phase-293x/293x-1194-MIMAP-564A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-EXPLICIT-RUNNER-PLANNING-FOLLOW-ON.md"
CARD_565A="docs/development/current/main/phases/phase-293x/293x-1195-MIMAP-565A-POST-EXPLICIT-RUNNER-PLANNING-FOLLOW-ON-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1196-MIMAP-566A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-EXPLICIT-RUNNER-PLANNING-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1197-MIMAP-567A-MIMALLOC-BLUEPRINT-LANE-CLOSE-CRITERIA.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_guard.sh"
PREV_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_follow_on_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-566A allocator comparison C mimalloc result explicit runner planning pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_564A" "$CARD_565A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MODULE_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$PREV_GUARD" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$PREV_GUARD" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: (completed|landed)' "$CARD_564A" "MIMAP-564A must be completed/landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_565A" "MIMAP-565A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-566A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: (selected current|completed|landed)' "$NEXT_CARD" "MIMAP-567A must be selected current/completed/landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-566A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-566A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-566A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-566A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-566A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-phase-293x-close-criteria"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-566A must defer closeout to close-criteria rows"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_box' "$MODULE" "module must export explicit runner planning pilot owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_box.hako' "$MODULE_INDEX" "memory module index must name explicit runner planning pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilotReportFields' "$OWNER" "owner must use explicit runner planning ReportFields record"
guard_expect_in_file "$TAG" 'pilotAllocatorComparisonCMimallocResultExplicitRunnerPlanning' "$OWNER" "owner must expose explicit runner planning pilot route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport' "$OWNER" "owner must consume presentation-only extension pilot report"
guard_expect_in_file "$TAG" 'reason == 5' "$OWNER" "owner must keep accidental execution seam reason vocabulary"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-566A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultExplicitRunnerPlanningPilot|allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-566A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$PREV_GUARD" --level L2

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap566_c_mimalloc_result_explicit_runner_planning_pilot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap566.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-proof' "$vm_log"
rg -F -q 'explicitrunner=1,0,1,1,1,0,1,1,1,1,1' "$vm_log"
rg -F -q 'contract=1,1,1,64,64,33254,4096,4096,0,1' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilot.makeAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilotReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilot.pilotAllocatorComparisonCMimallocResultExplicitRunnerPlanning/1",
    "HakoAllocAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilot.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilotReport")
if report is None:
    raise SystemExit("missing explicit runner planning pilot report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing explicit runner planning pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "explicit_runner_planning_pilot_present",
    "runner_output_contract_present",
    "memory_evidence_contract_present",
    "runner_execution_performed",
    "benchmark_rerun_executed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap566a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
