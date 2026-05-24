#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot"
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
    echo "[$TAG] ERROR: MIMAP-560A defers any exact or closeout evidence beyond the presentation-only extension pilot" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof/test.sh"
CARD_552A="docs/development/current/main/phases/phase-293x/293x-1182-MIMAP-552A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-PILOT.md"
CARD_558A="docs/development/current/main/phases/phase-293x/293x-1188-MIMAP-558A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1190-MIMAP-560A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-ssot.md"
DESIGN_552A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-560A allocator comparison C mimalloc result presentation-only extension pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_552A" "$CARD_558A" "$CARD" "$DESIGN" "$DESIGN_552A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MODULE_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_552A" "MIMAP-552A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_558A" "MIMAP-558A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-560A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-560A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_552A" "MIMAP-552A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-560A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-560A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-560A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-560A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-presentation-only-extension-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-560A must defer closeout beyond the pilot"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box' "$MODULE" "module must export presentation-only extension pilot owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako' "$MODULE_INDEX" "memory module index must name presentation-only extension pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'pilotAllocatorComparisonCMimallocResultPresentationOnlyExtension' "$OWNER" "owner must expose presentation-only extension pilot route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnPilotReport' "$OWNER" "owner must consume MIMAP-552A comparison-ready pilot report"
guard_expect_in_file "$TAG" 'presentation_only_extension_present' "$OWNER" "pilot must publish presentation-only extension state"
guard_expect_in_file "$TAG" 'comparison_preconditions_present: report.comparison_preconditions_present' "$OWNER" "pilot must preserve comparison preconditions field"
guard_expect_in_file "$TAG" 'presentation_count: usize = 0' "$OWNER" "pilot owner presentation_count must be usize"
guard_expect_in_file "$TAG" 'accepted_count: usize = 0' "$OWNER" "pilot owner accepted_count must be usize"
guard_expect_in_file "$TAG" 'blocked_count: usize = 0' "$OWNER" "pilot owner blocked_count must be usize"
guard_expect_in_file "$TAG" 'missing_pilot_reject_count: usize = 0' "$OWNER" "pilot owner missing_pilot_reject_count must be usize"
guard_expect_in_file "$TAG" 'blocked_pilot_reject_count: usize = 0' "$OWNER" "pilot owner blocked_pilot_reject_count must be usize"
guard_expect_in_file "$TAG" 'missing_presentation_input_reject_count: usize = 0' "$OWNER" "pilot owner missing_presentation_input_reject_count must be usize"
guard_expect_in_file "$TAG" 'closed_stop_line_reject_count: usize = 0' "$OWNER" "pilot owner closed_stop_line_reject_count must be usize"
guard_expect_in_file "$TAG" 'allocation_count: usize = 0' "$OWNER" "pilot report allocation_count must be usize"
guard_expect_in_file "$TAG" 'free_count: usize = 0' "$OWNER" "pilot report free_count must be usize"
guard_expect_in_file "$TAG" 'requested_bytes: usize = 0' "$OWNER" "pilot report requested_bytes must be usize"
guard_expect_in_file "$TAG" 'peak_rss_bytes: usize = 0' "$OWNER" "pilot report peak_rss_bytes must be usize"
guard_expect_in_file "$TAG" 'steady_rss_bytes: usize = 0' "$OWNER" "pilot report steady_rss_bytes must be usize"
guard_expect_in_file "$TAG" 'hako_allocation_count: usize = 0' "$OWNER" "pilot report hako_allocation_count must be usize"
guard_expect_in_file "$TAG" 'hako_requested_bytes: usize = 0' "$OWNER" "pilot report hako_requested_bytes must be usize"
guard_expect_in_file "$TAG" 'c_allocation_count: usize = 0' "$OWNER" "pilot report c_allocation_count must be usize"
guard_expect_in_file "$TAG" 'c_requested_bytes: usize = 0' "$OWNER" "pilot report c_requested_bytes must be usize"
guard_expect_in_file "$TAG" 'c_peak_rss_bytes: usize = 0' "$OWNER" "pilot report c_peak_rss_bytes must be usize"
guard_expect_in_file "$TAG" 'allocator_id: usize = 0' "$OWNER" "pilot report allocator_id must be usize"
guard_expect_in_file "$TAG" 'runner_kind: usize = 0' "$OWNER" "pilot report runner_kind must be usize"
guard_expect_in_file "$TAG" 'workload_id: usize = 0' "$OWNER" "pilot report workload_id must be usize"
guard_expect_in_file "$TAG" 'exit_code: usize = 0' "$OWNER" "pilot report exit_code must be usize"
guard_expect_in_file "$TAG" 'evidence_complete: usize = 0' "$OWNER" "pilot report evidence_complete must be usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "pilot owner last_reason must stay signed"
guard_expect_in_file "$TAG" 'performance_conclusion_made: usize = 0' "$OWNER" "pilot report performance_conclusion_made must be usize"
guard_expect_in_file "$TAG" 'memory_conclusion_made: usize = 0' "$OWNER" "pilot report memory_conclusion_made must be usize"
guard_expect_in_file "$TAG" 'repeated_benchmark_executed: usize = 0' "$OWNER" "pilot report repeated_benchmark_executed must be usize"
guard_expect_in_file "$TAG" 'process_replacement_executed: usize = 0' "$OWNER" "pilot report process_replacement_executed must be usize"
guard_expect_in_file "$TAG" 'hook_installed: usize = 0' "$OWNER" "pilot report hook_installed must be usize"
guard_expect_in_file "$TAG" 'backend_matcher_added: usize = 0' "$OWNER" "pilot report backend_matcher_added must be usize"
guard_expect_in_file "$TAG" 'global_allocator_installed: usize = 0' "$OWNER" "pilot report global_allocator_installed must be usize"
guard_expect_in_file "$TAG" 'hidden_discovery_used: usize = 0' "$OWNER" "pilot report hidden_discovery_used must be usize"
guard_expect_in_file "$TAG" 'provider_package_generated: usize = 0' "$OWNER" "pilot report provider_package_generated must be usize"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: usize = 0' "$OWNER" "pilot report would_replace_host_allocator must be usize"
guard_expect_in_file "$TAG" 'would_install_hook: usize = 0' "$OWNER" "pilot report would_install_hook must be usize"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: usize = 0' "$OWNER" "pilot report would_add_backend_matcher must be usize"
guard_expect_in_file "$TAG" 'would_run_thread: usize = 0' "$OWNER" "pilot report would_run_thread must be usize"

if rg -n 'presentation_count: i64 = 0|accepted_count: i64 = 0|blocked_count: i64 = 0|missing_pilot_reject_count: i64 = 0|blocked_pilot_reject_count: i64 = 0|missing_presentation_input_reject_count: i64 = 0|closed_stop_line_reject_count: i64 = 0|allocation_count: i64 = 0|free_count: i64 = 0|requested_bytes: i64 = 0|peak_rss_bytes: i64 = 0|steady_rss_bytes: i64 = 0|hako_allocation_count: i64 = 0|hako_requested_bytes: i64 = 0|c_allocation_count: i64 = 0|c_requested_bytes: i64 = 0|c_peak_rss_bytes: i64 = 0|allocator_id: i64 = 0|runner_kind: i64 = 0|workload_id: i64 = 0|exit_code: i64 = 0|evidence_complete: i64 = 0|performance_conclusion_made: i64 = 0|memory_conclusion_made: i64 = 0|repeated_benchmark_executed: i64 = 0|process_replacement_executed: i64 = 0|hook_installed: i64 = 0|backend_matcher_added: i64 = 0|global_allocator_installed: i64 = 0|hidden_discovery_used: i64 = 0|provider_package_generated: i64 = 0|would_replace_host_allocator: i64 = 0|would_install_hook: i64 = 0|would_add_backend_matcher: i64 = 0|would_run_thread: i64 = 0' "$OWNER" >/tmp/"$TAG".pilot_counter_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-560A pilot owner/report payload fields must be exact usize" >&2
  cat /tmp/"$TAG".pilot_counter_leak >&2
  rm -f /tmp/"$TAG".pilot_counter_leak
  exit 1
fi
rm -f /tmp/"$TAG".pilot_counter_leak

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-560A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot|allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-560A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap560_c_mimalloc_result_presentation_only_extension_pilot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap560.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof' "$vm_log"
rg -F -q 'presentationext=1,0,1,1,1,1,0,1,1,1,1,1' "$vm_log"
rg -F -q 'contract=1,1,1,64,64,33254,4096,4096,0,1' "$vm_log"
rg -F -q 'metrics=3,72,64,33254,4096,61,33182' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot.makeAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot.pilotAllocatorComparisonCMimallocResultPresentationOnlyExtension/1",
    "HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport")
if report is None:
    raise SystemExit("missing presentation-only extension pilot report typed object plan")
owner = plans.get("HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot")
if owner is None:
    raise SystemExit("missing presentation-only extension pilot typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing presentation-only extension pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "presentation_only_extension_present",
    "comparison_ready_present",
    "hako_alloc_report_contract_present",
    "c_mimalloc_runner_contract_present",
    "shared_workload_id_present",
    "comparison_preconditions_present",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"{name} must stay i64: {field}")
for name in (
    "performance_conclusion_made",
    "memory_conclusion_made",
    "repeated_benchmark_executed",
    "process_replacement_executed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be usize: {field}")
for name in (
    "allocation_count",
    "free_count",
    "requested_bytes",
    "peak_rss_bytes",
    "steady_rss_bytes",
    "hako_allocation_count",
    "hako_requested_bytes",
    "c_allocation_count",
    "c_requested_bytes",
    "c_peak_rss_bytes",
    "allocator_id",
    "runner_kind",
    "workload_id",
    "exit_code",
    "evidence_complete",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be usize in report plan: {field}")
for name in (
    "hook_installed",
    "backend_matcher_added",
    "global_allocator_installed",
    "hidden_discovery_used",
    "provider_package_generated",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be usize in report plan: {field}")
for name in (
    "allocation_count_delta",
    "requested_bytes_delta",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"{name} must stay i64 in report plan: {field}")
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "presentation_count",
    "accepted_count",
    "blocked_count",
    "missing_pilot_reject_count",
    "blocked_pilot_reject_count",
    "missing_presentation_input_reject_count",
    "closed_stop_line_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be usize in owner plan: {field}")
last_reason = owner_fields.get("last_reason")
if last_reason is None or last_reason.get("declared_type") != "i64" or last_reason.get("storage") != "i64":
    raise SystemExit(f"last_reason must stay i64 in owner plan: {last_reason}")
print("[mimap560a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
