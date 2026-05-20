#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-external-api-call-stub-execution-pilot"
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
    echo "[$TAG] ERROR: MIMAP-406A defers L3/L4 evidence to external provider API call closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-call-external-api-call-stub-execution-pilot-proof/main.hako"
APP_README="apps/hako-alloc-provider-call-external-api-call-stub-execution-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-provider-call-external-api-call-stub-execution-pilot-proof/test.sh"
CARD_402A="docs/development/current/main/phases/phase-293x/293x-1024-MIMAP-402A-PROVIDER-CALL-EXTERNAL-API-ADAPTER-PREFLIGHT.md"
CARD_404A="docs/development/current/main/phases/phase-293x/293x-1026-MIMAP-404A-PROVIDER-CALL-EXTERNAL-API-ADAPTER-CLOSEOUT.md"
CARD_405A="docs/development/current/main/phases/phase-293x/293x-1027-MIMAP-405A-POST-PROVIDER-CALL-EXTERNAL-API-ADAPTER-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1028-MIMAP-406A-PROVIDER-CALL-EXTERNAL-API-CALL-STUB-EXECUTION-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1029-MIMAP-407A-POST-PROVIDER-CALL-EXTERNAL-API-CALL-STUB-EXECUTION-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-call-external-api-call-stub-execution-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_call_external_api_call_stub_execution_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_call_external_api_adapter_preflight_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_external_api_call_stub_execution_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-406A external provider API call stub execution pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_402A" "$CARD_404A" "$CARD_405A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_402A" "$CARD_404A" "$CARD_405A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-407A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-406A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-406A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-406A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-406A"
guard_expect_in_file "$TAG" 'row_kind = "external-api-call-stub-execution-pilot"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-406A must be an external-api-call-stub-execution row"
guard_expect_in_file "$TAG" 'memory.provider_call_external_api_call_stub_execution_pilot_box' "$MODULE" "module must export external API call stub execution owner"
guard_expect_in_file "$TAG" 'provider_call_external_api_call_stub_execution_pilot_box.hako' "$MEMORY_README" "memory README must name external API call stub execution owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderCallExternalApiCallStubExecutionPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderCallExternalApiCallStubExecutionPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'executeProviderCallExternalApiStub' "$OWNER" "owner must expose external API call stub execution route"
guard_expect_in_file "$TAG" 'HakoAllocProviderCallExternalApiAdapterPreflightReport' "$OWNER" "owner must consume adapter preflight report"
guard_expect_in_file "$TAG" 'external_provider_api_stub_call_executed' "$OWNER" "owner must report stub external API call"
guard_expect_in_file "$TAG" 'external_provider_api_stub_result_present' "$OWNER" "owner must report stub external API result"
guard_expect_in_file "$TAG" 'actual_external_provider_api_call_executed: 0' "$OWNER" "actual external provider API calls must remain closed"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|external_provider_api_call[[:space:]]*\(|provider_api_call[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-406A owner/app must keep actual external provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-call-external-api-call-stub-execution-pilot-proof|ProviderCallExternalApiCallStubExecutionPilot|providerCallExternalApiCallStubExecution|callProvider|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-406A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap406_external_api_stub.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap406.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-call-external-api-call-stub-execution-pilot-proof' "$vm_log"
rg -F -q 'stub=1,0,1,1,1,0,0' "$vm_log"
rg -F -q 'preflight=1,1,0,1,1,1' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'closed=1,1,1,1,1,1,1,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderCallExternalApiCallStubExecutionPilot.makeProviderCallExternalApiCallStubExecutionPilotReport/1",
    "HakoAllocProviderCallExternalApiCallStubExecutionPilot.executeProviderCallExternalApiStub/1",
    "HakoAllocProviderCallExternalApiCallStubExecutionPilot.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderCallExternalApiCallStubExecutionPilotReport")
if report is None:
    raise SystemExit("missing external provider API call stub execution report typed object plan")
target = "HakoAllocProviderCallExternalApiCallStubExecutionPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing external provider API call stub execution ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "external_stub_execution_present",
    "external_provider_api_stub_call_executed",
    "external_provider_api_stub_result_present",
    "external_provider_api_stub_result_code",
    "actual_external_provider_api_call_executed",
    "would_call_external_provider_api",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap406a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
