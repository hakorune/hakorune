#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-external-api-adapter-inventory"
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
    echo "[$TAG] ERROR: MIMAP-400A defers L3/L4 evidence to external provider API call pilot or closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-call-external-api-adapter-inventory-proof/main.hako"
APP_README="apps/hako-alloc-provider-call-external-api-adapter-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-provider-call-external-api-adapter-inventory-proof/test.sh"
CARD_396A="docs/development/current/main/phases/phase-293x/293x-1018-MIMAP-396A-PROVIDER-CALL-REAL-API-STUB-EXECUTION-PILOT.md"
CARD_398A="docs/development/current/main/phases/phase-293x/293x-1020-MIMAP-398A-PROVIDER-CALL-REAL-API-STUB-EXECUTION-CLOSEOUT.md"
CARD_399A="docs/development/current/main/phases/phase-293x/293x-1021-MIMAP-399A-POST-PROVIDER-CALL-REAL-API-STUB-EXECUTION-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1022-MIMAP-400A-PROVIDER-CALL-EXTERNAL-API-ADAPTER-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1023-MIMAP-401A-POST-PROVIDER-CALL-EXTERNAL-API-ADAPTER-INVENTORY-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-call-external-api-adapter-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_call_external_api_adapter_inventory_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_call_real_api_stub_execution_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_external_api_adapter_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-400A provider-call external API adapter inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_396A" "$CARD_398A" "$CARD_399A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_396A" "$CARD_398A" "$CARD_399A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-401A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-400A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-400A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-400A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-400A"
guard_expect_in_file "$TAG" 'row_kind = "external-api-adapter-inventory"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-400A must be an external-api-adapter-inventory row"
guard_expect_in_file "$TAG" 'memory.provider_call_external_api_adapter_inventory_box' "$MODULE" "module must export provider-call external API adapter inventory owner"
guard_expect_in_file "$TAG" 'provider_call_external_api_adapter_inventory_box.hako' "$MEMORY_README" "memory README must name external API adapter inventory owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderCallExternalApiAdapterInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderCallExternalApiAdapterInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryProviderCallExternalApiAdapter' "$OWNER" "owner must expose adapter inventory route"
guard_expect_in_file "$TAG" 'HakoAllocProviderCallRealApiStubExecutionPilotReport' "$OWNER" "owner must consume stub execution report"
guard_expect_in_file "$TAG" 'external_provider_adapter_present' "$OWNER" "owner must report external adapter presence"
guard_expect_in_file "$TAG" 'external_provider_adapter_valid' "$OWNER" "owner must report external adapter validity"
guard_expect_in_file "$TAG" 'external_provider_api_call_ready' "$OWNER" "owner must report external API readiness"
guard_expect_in_file "$TAG" 'external_provider_api_call_executed: 0' "$OWNER" "external provider API calls must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|external_provider_api_call[[:space:]]*\(|provider_api_call[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-400A owner/app must keep external provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-call-external-api-adapter-inventory-proof|ProviderCallExternalApiAdapterInventory|providerCallExternalApiAdapterInventory|callProvider|replace_process_allocator|install_hook|global_allocator|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-400A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap400_provider_call_adapter.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap400.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-call-external-api-adapter-inventory-proof' "$vm_log"
rg -F -q 'adapter=1,0,1,1,1,1,0' "$vm_log"
rg -F -q 'stub=1,1,0,1,1,0' "$vm_log"
rg -F -q 'owner=10,1,9,1,1,1,1,1,1,1,1,1,9' "$vm_log"
rg -F -q 'closed=1,1,1,1,1,1,1,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6,7,8,9' "$vm_log"
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
    "HakoAllocProviderCallExternalApiAdapterInventory.makeProviderCallExternalApiAdapterInventoryReport/1",
    "HakoAllocProviderCallExternalApiAdapterInventory.inventoryProviderCallExternalApiAdapter/3",
    "HakoAllocProviderCallExternalApiAdapterInventory.reject/4",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderCallExternalApiAdapterInventoryReport")
if report is None:
    raise SystemExit("missing provider-call external API adapter inventory report typed object plan")
target = "HakoAllocProviderCallExternalApiAdapterInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider-call external API adapter inventory ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "adapter_inventory_present",
    "external_provider_adapter_present",
    "external_provider_adapter_valid",
    "external_provider_api_call_ready",
    "external_provider_api_call_executed",
    "would_call_external_provider_api",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap400a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
