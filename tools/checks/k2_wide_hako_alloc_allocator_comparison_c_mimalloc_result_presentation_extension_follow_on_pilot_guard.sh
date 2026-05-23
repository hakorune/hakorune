#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot"
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
    echo "[$TAG] ERROR: MIMAP-492A defers any exact or closeout evidence beyond the presentation extension follow-on pilot" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot-proof/test.sh"
CARD_486A="docs/development/current/main/phases/phase-293x/293x-1116-MIMAP-486A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-PILOT.md"
CARD_490A="docs/development/current/main/phases/phase-293x/293x-1120-MIMAP-490A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-PLAN.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1122-MIMAP-492A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-PILOT.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-131-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-PILOT-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-132-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-PILOT-COUNTERS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot-ssot.md"
DESIGN_486A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_extension_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-492A allocator comparison C mimalloc result presentation extension follow-on pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_486A" "$CARD_490A" "$CARD" "$USIZE_SELECTION_CARD" "$USIZE_CARD" "$DESIGN" "$DESIGN_486A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_486A" "MIMAP-486A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_490A" "MIMAP-490A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-492A must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-131 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-132 usize migration card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-492A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_486A" "MIMAP-486A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-492A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-492A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-492A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-492A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-presentation-extension-follow-on-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-492A must defer closeout beyond the pilot"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_pilot_box' "$MODULE" "module must export presentation extension follow-on pilot owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_pilot_box.hako' "$MEMORY_README" "memory README must name presentation extension follow-on pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'pilotAllocatorComparisonCMimallocResultPresentationExtensionFollowOn' "$OWNER" "owner must expose presentation extension follow-on pilot route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionPilotReport' "$OWNER" "owner must consume MIMAP-486A presentation extension pilot report"
guard_expect_in_file "$TAG" 'follow_on_ready' "$OWNER" "pilot must publish extension follow-on readiness"
guard_expect_in_file "$TAG" 'requested_bytes_delta: report.requested_bytes_delta' "$OWNER" "pilot must preserve requested bytes delta field"
guard_expect_in_file "$TAG" 'provider_package_generated: report.provider_package_generated' "$OWNER" "pilot must preserve provider package field"
guard_expect_in_file "$TAG" 'follow_on_count: usize = 0' "$OWNER" "follow-on counter must be exact usize"
guard_expect_in_file "$TAG" 'accepted_count: usize = 0' "$OWNER" "accepted counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_count: usize = 0' "$OWNER" "blocked counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_pilot_reject_count: usize = 0' "$OWNER" "missing pilot reject counter must be exact usize"
guard_expect_in_file "$TAG" 'blocked_pilot_reject_count: usize = 0' "$OWNER" "blocked pilot reject counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_follow_on_input_reject_count: usize = 0' "$OWNER" "missing follow-on input reject counter must be exact usize"
guard_expect_in_file "$TAG" 'closed_stop_line_reject_count: usize = 0' "$OWNER" "closed stop-line reject counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "last reason must remain signed reason vocabulary"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-492A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilot|allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-492A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap492_c_mimalloc_result_presentation_extension_follow_on_pilot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap492.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot-proof' "$vm_log"
rg -F -q 'followon=1,0,1,1,1,0,1,1,1,1,1' "$vm_log"
rg -F -q 'metrics=3,72,64,33254,4096,61,33182' "$vm_log"
rg -F -q 'owner=5,1,4,1,1,1,1,4' "$vm_log"
rg -F -q 'blocked=0,0,0,0' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilot.makeAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilotReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilot.pilotAllocatorComparisonCMimallocResultPresentationExtensionFollowOn/1",
    "HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilot.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilot")
if owner is None:
    raise SystemExit("missing C mimalloc result presentation extension follow-on pilot owner typed object plan")
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilotReport")
if report is None:
    raise SystemExit("missing C mimalloc result presentation extension follow-on pilot report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc result presentation extension follow-on pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "follow_on_count",
    "accepted_count",
    "blocked_count",
    "missing_pilot_reject_count",
    "blocked_pilot_reject_count",
    "missing_follow_on_input_reject_count",
    "closed_stop_line_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"C mimalloc result presentation extension follow-on owner counter {name} must be usize storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"C mimalloc result presentation extension follow-on last_reason must remain i64 storage: {field}")
for name in (
    "extension_follow_on_present",
    "pilot_present",
    "follow_on_ready",
    "follow_on_memory_outcome_present",
    "follow_on_metrics_snapshot_present",
    "provisional_memory_winner",
    "provisional_memory_reason",
    "comparison_available",
    "requested_bytes_delta",
    "repeated_benchmark_executed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap492a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
