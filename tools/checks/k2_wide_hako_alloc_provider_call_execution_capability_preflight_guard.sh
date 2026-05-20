#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-execution-capability-preflight"
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
    echo "[$TAG] ERROR: MIMAP-388A defers L3/L4 evidence to first provider-call execution seam or closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-call-execution-capability-preflight-proof/main.hako"
APP_README="apps/hako-alloc-provider-call-execution-capability-preflight-proof/README.md"
APP_TEST="apps/hako-alloc-provider-call-execution-capability-preflight-proof/test.sh"
CARD_386A="docs/development/current/main/phases/phase-293x/293x-1008-MIMAP-386A-PROVIDER-CALL-MODELED-OPEN-PILOT.md"
CARD_387A="docs/development/current/main/phases/phase-293x/293x-1009-MIMAP-387A-POST-PROVIDER-CALL-MODELED-OPEN-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1010-MIMAP-388A-PROVIDER-CALL-EXECUTION-CAPABILITY-PREFLIGHT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1011-MIMAP-389A-POST-PROVIDER-CALL-EXECUTION-PREFLIGHT-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-call-execution-capability-preflight-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_call_execution_capability_preflight_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_call_modeled_open_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_execution_capability_preflight_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-388A provider-call execution capability preflight\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_386A" "$CARD_387A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_386A" "MIMAP-386A modeled-open pilot must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_387A" "MIMAP-387A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-388A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-389A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-388A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-388A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-388A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-388A"
guard_expect_in_file "$TAG" 'row_kind = "execution-capability-preflight"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-388A must be an execution-capability-preflight row"
guard_expect_in_file "$TAG" 'memory.provider_call_execution_capability_preflight_box' "$MODULE" "module must export provider-call execution capability preflight owner"
guard_expect_in_file "$TAG" 'provider_call_execution_capability_preflight_box.hako' "$MEMORY_README" "memory README must name provider-call execution capability preflight owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderCallExecutionCapabilityPreflightReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderCallExecutionCapabilityPreflightReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'preflightProviderCallExecutionCapability' "$OWNER" "owner must expose provider-call execution capability preflight route"
guard_expect_in_file "$TAG" 'HakoAllocProviderCallModeledOpenPilotReport' "$OWNER" "owner must consume modeled-open provider-call report"
guard_expect_in_file "$TAG" 'provider_call_execution_capability_present' "$OWNER" "owner must report execution capability presence"
guard_expect_in_file "$TAG" 'provider_call_execution_ready' "$OWNER" "owner must report execution readiness"
guard_expect_in_file "$TAG" 'provider_call_execution_closed: modeled.provider_call_execution_closed' "$OWNER" "actual provider-call execution must remain closed"
guard_expect_in_file "$TAG" 'would_call_provider: modeled.would_call_provider' "$OWNER" "preflight report must carry modeled would_call_provider"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|provider_api|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-388A owner/app must keep provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-call-execution-capability-preflight-proof|ProviderCallExecutionCapabilityPreflight|providerCallExecutionCapabilityPreflight' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-388A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap388_provider_call_preflight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap388.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-call-execution-capability-preflight-proof' "$vm_log"
rg -F -q 'preflight=1,0,1,1,1,1' "$vm_log"
rg -F -q 'modeled=1,1,1,1,0,1' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'closed=1,1,1,1,1,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderCallExecutionCapabilityPreflight.makeProviderCallExecutionCapabilityPreflightReport/1",
    "HakoAllocProviderCallExecutionCapabilityPreflight.preflightProviderCallExecutionCapability/3",
    "HakoAllocProviderCallExecutionCapabilityPreflight.reject/4",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderCallExecutionCapabilityPreflightReport")
if report is None:
    raise SystemExit("missing provider-call execution capability preflight report typed object plan")
target = "HakoAllocProviderCallExecutionCapabilityPreflightReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider-call execution capability preflight ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "preflight_present",
    "provider_call_execution_capability_present",
    "provider_call_execution_capability_valid",
    "provider_call_execution_ready",
    "provider_call_execution_closed",
    "would_call_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap388a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
