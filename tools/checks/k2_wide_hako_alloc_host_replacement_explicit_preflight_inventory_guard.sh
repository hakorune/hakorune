#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-host-replacement-explicit-preflight-inventory"
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
    echo "[$TAG] ERROR: MIMAP-420A defers L3/L4 to the host replacement preflight closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-host-replacement-explicit-preflight-inventory-proof/main.hako"
APP_README="apps/hako-alloc-host-replacement-explicit-preflight-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-host-replacement-explicit-preflight-inventory-proof/test.sh"
CARD_415A="docs/development/current/main/phases/phase-293x/293x-1037-MIMAP-415A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PILOT.md"
CARD_417A="docs/development/current/main/phases/phase-293x/293x-1039-MIMAP-417A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-CLOSEOUT.md"
CARD_419A="docs/development/current/main/phases/phase-293x/293x-1041-MIMAP-419A-HOST-REPLACEMENT-OPTIONAL-LADDER-PLAN.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1042-MIMAP-420A-HOST-REPLACEMENT-EXPLICIT-PREFLIGHT-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1043-MIMAP-421A-HOST-REPLACEMENT-BLOCKED-STATE-DIAGNOSTICS.md"
DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-explicit-preflight-inventory-ssot.md"
OPTIONAL_PLAN="docs/development/current/main/design/hako-alloc-host-replacement-optional-ladder-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/host_replacement_explicit_preflight_inventory_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/real_external_provider_api_call_first_pattern_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_host_replacement_explicit_preflight_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-420A host replacement explicit preflight inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_415A" "$CARD_417A" "$CARD_419A" "$CARD" "$NEXT_CARD" "$DESIGN" "$OPTIONAL_PLAN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_415A" "$CARD_417A" "$CARD_419A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-421A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-420A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-420A host replacement explicit preflight inventory' "$OPTIONAL_PLAN" "optional ladder plan must name MIMAP-420A"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-420A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-420A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-420A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-420A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-host-replacement-preflight-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-420A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.host_replacement_explicit_preflight_inventory_box' "$MODULE" "module must export host replacement preflight owner"
guard_expect_in_file "$TAG" 'host_replacement_explicit_preflight_inventory_box.hako' "$MEMORY_README" "memory README must name host replacement preflight owner"
guard_expect_in_file "$TAG" 'record HakoAllocHostReplacementExplicitPreflightInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeHostReplacementExplicitPreflightInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryHostReplacementExplicitPreflight' "$OWNER" "owner must expose preflight inventory route"
guard_expect_in_file "$TAG" 'HakoAllocRealExternalProviderApiCallFirstPatternPilotReport' "$OWNER" "owner must consume real external provider API call pilot report"
guard_expect_in_file "$TAG" 'host_replacement_executed: 0' "$OWNER" "host replacement execution must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-420A owner/app must keep replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'host-replacement-explicit-preflight-inventory-proof|HostReplacementExplicitPreflightInventory|hostReplacementExplicitPreflightInventory|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-420A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap420_host_preflight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap420.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-host-replacement-explicit-preflight-inventory-proof' "$vm_log"
rg -F -q 'hostpreflight=1,0,1,1,0' "$vm_log"
rg -F -q 'realcall=1,1,1,1,1' "$vm_log"
rg -F -q 'inputs=1,1,1,1' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocHostReplacementExplicitPreflightInventory.makeHostReplacementExplicitPreflightInventoryReport/1",
    "HakoAllocHostReplacementExplicitPreflightInventory.inventoryHostReplacementExplicitPreflight/5",
    "HakoAllocHostReplacementExplicitPreflightInventory.reject/6",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocHostReplacementExplicitPreflightInventoryReport")
if report is None:
    raise SystemExit("missing host replacement preflight report typed object plan")
target = "HakoAllocHostReplacementExplicitPreflightInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing host replacement preflight ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "host_replacement_preflight_inventory_present",
    "host_replacement_preflight_ready",
    "host_replacement_executed",
    "hook_installed",
    "backend_matcher_added",
    "global_allocator_installed",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap420a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
