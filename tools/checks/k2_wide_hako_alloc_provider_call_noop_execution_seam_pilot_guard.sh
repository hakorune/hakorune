#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-noop-execution-seam-pilot"
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
    echo "[$TAG] ERROR: MIMAP-390A defers L3/L4 evidence to real provider-call execution or closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-call-noop-execution-seam-pilot-proof/main.hako"
APP_README="apps/hako-alloc-provider-call-noop-execution-seam-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-provider-call-noop-execution-seam-pilot-proof/test.sh"
CARD_388A="docs/development/current/main/phases/phase-293x/293x-1010-MIMAP-388A-PROVIDER-CALL-EXECUTION-CAPABILITY-PREFLIGHT.md"
CARD_389A="docs/development/current/main/phases/phase-293x/293x-1011-MIMAP-389A-POST-PROVIDER-CALL-EXECUTION-PREFLIGHT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1012-MIMAP-390A-PROVIDER-CALL-NOOP-EXECUTION-SEAM-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1013-MIMAP-391A-POST-PROVIDER-CALL-NOOP-EXECUTION-SEAM-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-call-noop-execution-seam-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/provider_call_noop_execution_seam_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_call_execution_capability_preflight_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_noop_execution_seam_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-390A provider-call no-op execution seam pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_388A" "$CARD_389A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$MODULE_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_388A" "MIMAP-388A execution preflight must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_389A" "MIMAP-389A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-390A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$NEXT_CARD" "MIMAP-391A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-390A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-390A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-390A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-390A"
guard_expect_in_file "$TAG" 'row_kind = "noop-execution-seam-pilot"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-390A must be a noop-execution-seam-pilot row"
guard_expect_in_file "$TAG" 'memory.provider_call_noop_execution_seam_pilot_box' "$MODULE" "module must export provider-call no-op execution seam owner"
guard_expect_in_file "$TAG" 'provider_call_noop_execution_seam_pilot_box.hako' "$MODULE_INDEX" "memory module index must name provider-call no-op execution seam owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderCallNoopExecutionSeamPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderCallNoopExecutionSeamPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'executeProviderCallNoopSeam' "$OWNER" "owner must expose no-op execution seam route"
guard_expect_in_file "$TAG" 'HakoAllocProviderCallExecutionCapabilityPreflightReport' "$OWNER" "owner must consume provider-call execution preflight report"
guard_expect_in_file "$TAG" 'seam_count: usize = 0' "$OWNER" "no-op seam owner-local counters must be exact usize"
guard_expect_in_file "$TAG" 'closed_backend_matcher_reject_count: usize = 0' "$OWNER" "backend matcher owner-local counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "no-op seam reason vocabulary must remain signed"
guard_expect_in_file "$TAG" 'provider_call_noop_execution_open' "$OWNER" "owner must report no-op execution open state"
guard_expect_in_file "$TAG" 'provider_call_noop_executed' "$OWNER" "owner must report no-op execution evidence"
guard_expect_in_file "$TAG" 'provider_api_call_executed: 0' "$OWNER" "actual provider API calls must not execute"
guard_expect_in_file "$TAG" 'would_execute_provider_api: 0' "$OWNER" "actual provider API call intent must remain zero"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|provider_api_call[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-390A owner/app must keep actual provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-call-noop-execution-seam-pilot-proof|ProviderCallNoopExecutionSeam|providerCallNoopExecutionSeam' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-390A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap390_provider_call_noop.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap390.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-call-noop-execution-seam-pilot-proof' "$vm_log"
rg -F -q 'noop=1,0,1,1,1,0' "$vm_log"
rg -F -q 'preflight=1,1,0,1,1,1,1' "$vm_log"
rg -F -q 'owner=8,1,7,1,1,1,1,1,1,1,7' "$vm_log"
rg -F -q 'closed=1,1,1,1,1,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6,7' "$vm_log"
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
    "HakoAllocProviderCallNoopExecutionSeamPilot.makeProviderCallNoopExecutionSeamPilotReport/1",
    "HakoAllocProviderCallNoopExecutionSeamPilot.executeProviderCallNoopSeam/1",
    "HakoAllocProviderCallNoopExecutionSeamPilot.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderCallNoopExecutionSeamPilotReport")
if report is None:
    raise SystemExit("missing provider-call no-op execution seam report typed object plan")
owner = plans.get("HakoAllocProviderCallNoopExecutionSeamPilot")
if owner is None:
    raise SystemExit("missing provider-call no-op execution seam typed object plan")
target = "HakoAllocProviderCallNoopExecutionSeamPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider-call no-op execution seam ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "seam_count",
    "accepted_count",
    "reject_count",
    "missing_preflight_reject_count",
    "rejected_preflight_reject_count",
    "not_ready_reject_count",
    "closed_execution_reject_count",
    "closed_host_replacement_reject_count",
    "closed_hook_reject_count",
    "closed_backend_matcher_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"owner-local counter {name} must be exact usize: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"last_reason must remain signed: {field}")
for name in (
    "seam_count",
    "accepted_count",
    "reject_count",
    "missing_preflight_reject_count",
    "rejected_preflight_reject_count",
    "not_ready_reject_count",
    "closed_execution_reject_count",
    "closed_host_replacement_reject_count",
    "closed_hook_reject_count",
    "closed_backend_matcher_reject_count",
    "noop_execution_seam_present",
    "provider_call_noop_execution_open",
    "provider_call_noop_executed",
    "provider_api_call_executed",
    "would_execute_provider_api",
    "would_call_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap390a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
