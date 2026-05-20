#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-real-api-stub-execution-pilot"
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
    echo "[$TAG] ERROR: MIMAP-396A defers L3/L4 evidence to provider-call closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-call-real-api-stub-execution-pilot-proof/main.hako"
APP_README="apps/hako-alloc-provider-call-real-api-stub-execution-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-provider-call-real-api-stub-execution-pilot-proof/test.sh"
CARD_392A="docs/development/current/main/phases/phase-293x/293x-1014-MIMAP-392A-PROVIDER-CALL-REAL-API-EXECUTION-PREFLIGHT.md"
CARD_395A="docs/development/current/main/phases/phase-293x/293x-1017-MIMAP-395A-POST-PROVIDER-CALL-REAL-API-FIRST-PATTERN-PLAN-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1018-MIMAP-396A-PROVIDER-CALL-REAL-API-STUB-EXECUTION-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1019-MIMAP-397A-POST-PROVIDER-CALL-REAL-API-STUB-EXECUTION-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-call-real-api-stub-execution-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_call_real_api_stub_execution_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_call_real_api_execution_preflight_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_real_api_stub_execution_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-396A provider-call real API stub execution pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_392A" "$CARD_395A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_392A" "MIMAP-392A real API preflight must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_395A" "MIMAP-395A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-396A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-397A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-396A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-396A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-396A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-396A"
guard_expect_in_file "$TAG" 'row_kind = "real-api-stub-execution-pilot"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-396A must be a real-api-stub-execution row"
guard_expect_in_file "$TAG" 'memory.provider_call_real_api_stub_execution_pilot_box' "$MODULE" "module must export provider-call real API stub execution owner"
guard_expect_in_file "$TAG" 'provider_call_real_api_stub_execution_pilot_box.hako' "$MEMORY_README" "memory README must name provider-call real API stub execution owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderCallRealApiStubExecutionPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderCallRealApiStubExecutionPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'executeProviderCallRealApiStub' "$OWNER" "owner must expose stub execution route"
guard_expect_in_file "$TAG" 'HakoAllocProviderCallRealApiExecutionPreflightReport' "$OWNER" "owner must consume real API preflight report"
guard_expect_in_file "$TAG" 'provider_call_stub_execution_open' "$OWNER" "owner must report stub execution open"
guard_expect_in_file "$TAG" 'provider_api_stub_call_executed' "$OWNER" "owner must report stub execution"
guard_expect_in_file "$TAG" 'provider_api_call_result_present' "$OWNER" "owner must report stub result"
guard_expect_in_file "$TAG" 'actual_provider_api_call_executed: 0' "$OWNER" "actual provider API calls must remain closed"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|provider_api_call[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-396A owner/app must keep actual provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-call-real-api-stub-execution-pilot-proof|ProviderCallRealApiStubExecutionPilot|providerCallRealApiStubExecution' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-396A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap396_provider_call_stubapi.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap396.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-call-real-api-stub-execution-pilot-proof' "$vm_log"
rg -F -q 'stub=1,0,1,1,1,1,0,0' "$vm_log"
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
    "HakoAllocProviderCallRealApiStubExecutionPilot.makeProviderCallRealApiStubExecutionPilotReport/1",
    "HakoAllocProviderCallRealApiStubExecutionPilot.executeProviderCallRealApiStub/1",
    "HakoAllocProviderCallRealApiStubExecutionPilot.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderCallRealApiStubExecutionPilotReport")
if report is None:
    raise SystemExit("missing provider-call real API stub execution report typed object plan")
target = "HakoAllocProviderCallRealApiStubExecutionPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider-call real API stub execution ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "stub_execution_present",
    "provider_call_stub_execution_open",
    "provider_api_stub_call_executed",
    "provider_api_call_result_present",
    "provider_api_call_result_code",
    "actual_provider_api_call_executed",
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
print("[mimap396a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
