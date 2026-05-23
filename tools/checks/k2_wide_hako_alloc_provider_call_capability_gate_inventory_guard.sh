#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-capability-gate-inventory"
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
    echo "[$TAG] ERROR: MIMAP-382A defers L3/L4 evidence to provider-call gate closeout or first provider-call execution seam" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-call-capability-gate-inventory-proof/main.hako"
APP_README="apps/hako-alloc-provider-call-capability-gate-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-provider-call-capability-gate-inventory-proof/test.sh"
CARD_380A="docs/development/current/main/phases/phase-293x/293x-1001-MIMAP-380A-PROVIDER-ACTIVATION-MODELED-OPEN-PILOT.md"
CARD_381A="docs/development/current/main/phases/phase-293x/293x-1002-MIMAP-381A-POST-PROVIDER-ACTIVATION-MODELED-OPEN-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1004-MIMAP-382A-PROVIDER-CALL-CAPABILITY-GATE-INVENTORY.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_call_capability_gate_inventory_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_activation_modeled_open_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_capability_gate_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-382A provider-call capability gate inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_380A" "$CARD_381A" "$CARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_380A" "MIMAP-380A modeled-open pilot must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_381A" "MIMAP-381A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-382A card must be landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-382A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-382A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-382A"
guard_expect_in_file "$TAG" 'row_kind = "capability-gate-inventory"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-382A must be a capability-gate-inventory row"
guard_expect_in_file "$TAG" 'memory.provider_call_capability_gate_inventory_box' "$MODULE" "module must export provider-call capability gate owner"
guard_expect_in_file "$TAG" 'provider_call_capability_gate_inventory_box.hako' "$MEMORY_README" "memory README must name provider-call capability gate owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderCallCapabilityGateInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderCallCapabilityGateInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryProviderCallCapabilityGate' "$OWNER" "owner must expose provider-call gate inventory route"
guard_expect_in_file "$TAG" 'HakoAllocProviderActivationModeledOpenPilotReport' "$OWNER" "owner must consume modeled-open activation report"
guard_expect_in_file "$TAG" 'inventory_count: usize = 0' "$OWNER" "capability gate owner-local counters must be exact usize"
guard_expect_in_file "$TAG" 'closed_execution_reject_count: usize = 0' "$OWNER" "closed execution owner-local counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "capability gate reason vocabulary must remain signed"
guard_expect_in_file "$TAG" 'provider_call_capability_present' "$OWNER" "owner must report provider-call capability presence"
guard_expect_in_file "$TAG" 'provider_call_execution_closed' "$OWNER" "owner must report provider-call execution closed state"
guard_expect_in_file "$TAG" 'would_call_provider: 0' "$OWNER" "provider calls must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|provider_api|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-382A owner/app must keep provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-call-capability-gate-inventory-proof|ProviderCallCapabilityGate|providerCallCapabilityGate' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-382A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap382_provider_call_gate.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap382.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-call-capability-gate-inventory-proof' "$vm_log"
rg -F -q 'gate=1,0,1,1,1,1,1' "$vm_log"
rg -F -q 'modeled=1,1,1,1,0' "$vm_log"
rg -F -q 'owner=6,1,5,1,1,1,1,1,5' "$vm_log"
rg -F -q 'closed=1,1,1,1,0,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderCallCapabilityGateInventory.makeProviderCallCapabilityGateInventoryReport/1",
    "HakoAllocProviderCallCapabilityGateInventory.inventoryProviderCallCapabilityGate/3",
    "HakoAllocProviderCallCapabilityGateInventory.reject/4",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderCallCapabilityGateInventoryReport")
if report is None:
    raise SystemExit("missing provider-call capability gate report typed object plan")
owner = plans.get("HakoAllocProviderCallCapabilityGateInventory")
if owner is None:
    raise SystemExit("missing provider-call capability gate typed object plan")
target = "HakoAllocProviderCallCapabilityGateInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider-call capability gate ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "inventory_count",
    "accepted_count",
    "reject_count",
    "missing_model_reject_count",
    "inactive_model_reject_count",
    "missing_capability_reject_count",
    "invalid_capability_reject_count",
    "closed_execution_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"owner-local counter {name} must be exact usize: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"last_reason must remain signed: {field}")
for name in (
    "inventory_count",
    "accepted_count",
    "reject_count",
    "missing_model_reject_count",
    "inactive_model_reject_count",
    "missing_capability_reject_count",
    "invalid_capability_reject_count",
    "closed_execution_reject_count",
    "gate_present",
    "provider_call_capability_present",
    "provider_call_capability_valid",
    "provider_call_execution_closed",
    "provider_call_inactive",
    "would_call_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap382a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
