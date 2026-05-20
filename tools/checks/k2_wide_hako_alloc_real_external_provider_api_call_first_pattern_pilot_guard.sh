#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-real-external-provider-api-call-first-pattern-pilot"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -eq 0 ]; then
  VALIDATION_LEVEL="L3"
else
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
fi
case "$VALIDATION_LEVEL" in
  L0|L1|L2|L3) ;;
  L4)
    echo "[$TAG] ERROR: MIMAP-415A does not define an L4 batch pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-real-external-provider-api-call-first-pattern-pilot-proof/main.hako"
APP_README="apps/hako-alloc-real-external-provider-api-call-first-pattern-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-real-external-provider-api-call-first-pattern-pilot-proof/test.sh"
CARD_410A="docs/development/current/main/phases/phase-293x/293x-1032-MIMAP-410A-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT.md"
CARD_412A="docs/development/current/main/phases/phase-293x/293x-1034-MIMAP-412A-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT-CLOSEOUT.md"
CARD_414A="docs/development/current/main/phases/phase-293x/293x-1036-MIMAP-414A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PLAN.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1037-MIMAP-415A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1038-MIMAP-416A-POST-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PILOT-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-real-external-provider-api-call-first-pattern-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/real_external_provider_api_call_first_pattern_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/real_external_provider_api_adapter_execution_preflight_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_real_external_provider_api_call_first_pattern_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-415A real external provider API call first-pattern pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_410A" "$CARD_412A" "$CARD_414A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_410A" "$CARD_412A" "$CARD_414A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-416A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-415A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-415A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-415A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-415A"
guard_expect_in_file "$TAG" 'first_pattern = true' "$PROOF_MANIFEST_INCLUDE" "MIMAP-415A must be first-pattern validation"
guard_expect_in_file "$TAG" 'memory.real_external_provider_api_call_first_pattern_pilot_box' "$MODULE" "module must export real external provider API call pilot owner"
guard_expect_in_file "$TAG" 'real_external_provider_api_call_first_pattern_pilot_box.hako' "$MEMORY_README" "memory README must name real external provider API call pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocRealExternalProviderApiCallFirstPatternPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeRealExternalProviderApiCallFirstPatternPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'executeRealExternalProviderApiCall' "$OWNER" "owner must expose real external call pilot route"
guard_expect_in_file "$TAG" 'HakoAllocRealExternalProviderApiAdapterExecutionPreflightReport' "$OWNER" "owner must consume preflight report"
guard_expect_in_file "$TAG" 'real_external_provider_api_call_executed: executed' "$OWNER" "owner must record real external provider API call execution"
guard_expect_in_file "$TAG" 'actual_external_provider_api_call_executed: executed' "$OWNER" "owner must record actual external provider API call pilot evidence"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-415A owner/app must keep replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'real-external-provider-api-call-first-pattern-pilot-proof|RealExternalProviderApiCallFirstPatternPilot|realExternalProviderApiCallFirstPatternPilot|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-415A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap415_real_external_call.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap415.mir.json"
vm_log="$tmp_dir/vm.log"
exe_out="$tmp_dir/mimap415.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-real-external-provider-api-call-first-pattern-pilot-proof' "$vm_log"
rg -F -q 'realcall=1,0,1,1,1,0' "$vm_log"
rg -F -q 'actual=1,1,1,1' "$vm_log"
rg -F -q 'preflight=1,1,0,1,1' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'closed=1,1,1,1,1,1,1,1,0,0,0,0' "$vm_log"
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
    "HakoAllocRealExternalProviderApiCallFirstPatternPilot.makeRealExternalProviderApiCallFirstPatternPilotReport/1",
    "HakoAllocRealExternalProviderApiCallFirstPatternPilot.executeRealExternalProviderApiCall/1",
    "HakoAllocRealExternalProviderApiCallFirstPatternPilot.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocRealExternalProviderApiCallFirstPatternPilotReport")
if report is None:
    raise SystemExit("missing real external provider API call pilot report typed object plan")
target = "HakoAllocRealExternalProviderApiCallFirstPatternPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing real external provider API call pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "real_external_provider_api_call_executed",
    "real_external_provider_api_result_present",
    "real_external_provider_api_result_code",
    "actual_external_provider_api_call_executed",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap415a-mir-json] ok")
PY

if pure_first_guard_level_allows_exe "$VALIDATION_LEVEL"; then
  pure_first_guard_build_toolchain
  pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
  pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
  pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"
  rg -F -q 'summary=ok' "$run_log"
  rg -F -q 'realcall=1,0,1,1,1,0' "$run_log"
fi

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
