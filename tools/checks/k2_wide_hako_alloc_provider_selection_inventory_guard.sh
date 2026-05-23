#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-selection-inventory"
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
    echo "[$TAG] ERROR: MIMAP-364A defers L3/L4 evidence to a provider-facing closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-selection-inventory-proof/main.hako"
APP_README="apps/hako-alloc-provider-selection-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-provider-selection-inventory-proof/test.sh"
CARD_362A="docs/development/current/main/phases/phase-293x/293x-978-MIMAP-362A-PROVIDER-READINESS-PREFLIGHT.md"
CARD_363A="docs/development/current/main/phases/phase-293x/293x-979-MIMAP-363A-POST-PROVIDER-READINESS-PREFLIGHT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-980-MIMAP-364A-PROVIDER-SELECTION-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-981-MIMAP-365A-POST-PROVIDER-SELECTION-INVENTORY-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-selection-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_selection_inventory_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_readiness_preflight_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-364A provider selection inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_362A" "$CARD_363A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_362A" "MIMAP-362A provider readiness preflight must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_363A" "MIMAP-363A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-364A card must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-365A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-364A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-364A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-364A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-364A"
guard_expect_in_file "$TAG" 'row_kind = "inventory"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-364A must be an inventory row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-364A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.provider_selection_inventory_box' "$MODULE" "module must export provider selection inventory owner"
guard_expect_in_file "$TAG" 'provider_selection_inventory_box.hako' "$MEMORY_README" "memory README must name provider selection inventory owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderSelectionInventoryReportFields' "$OWNER" "provider selection owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderSelectionInventoryReport' "$OWNER" "provider selection owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryProviderSelection' "$OWNER" "provider selection owner must expose inventory route"
guard_expect_in_file "$TAG" 'HakoAllocProviderReadinessPreflightReport' "$OWNER" "provider selection owner must consume readiness report"
guard_expect_in_file "$TAG" 'selection_count: usize = 0' "$OWNER" "provider selection owner-local counters must be exact usize"
guard_expect_in_file "$TAG" 'closed_execution_reject_count: usize = 0' "$OWNER" "provider selection closed-execution reject counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "provider selection reason vocabulary must remain signed"
guard_expect_in_file "$TAG" 'would_select_provider: accepted' "$OWNER" "provider selection must stay inventory-only"
guard_expect_in_file "$TAG" 'would_activate_provider: 0' "$OWNER" "provider activation must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"

if rg -n 'providerActivate|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|activateProvider|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-364A owner/app must keep activation/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-selection-inventory-proof|ProviderSelectionInventory|providerSelectionInventory' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-364A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap364_provider_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap364.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-selection-inventory-proof' "$vm_log"
rg -F -q 'selection=1,0,1,99019005111,1,1,1' "$vm_log"
rg -F -q 'ready=1,1,0,99019005101,1' "$vm_log"
rg -F -q 'owner=7,1,6,1,1,1,1,1,1,6' "$vm_log"
rg -F -q 'inactive=1,1,1,1,1,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6' "$vm_log"
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
    "HakoAllocProviderSelectionInventory.makeProviderSelectionInventoryReport/1",
    "HakoAllocProviderSelectionInventory.inventoryProviderSelection/3",
    "HakoAllocProviderSelectionInventory.reject/4",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderSelectionInventoryReport")
if report is None:
    raise SystemExit("missing provider selection inventory report typed object plan")
owner = plans.get("HakoAllocProviderSelectionInventory")
if owner is None:
    raise SystemExit("missing provider selection inventory typed object plan")
target = "HakoAllocProviderSelectionInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider selection inventory ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "selection_count",
    "accepted_count",
    "reject_count",
    "missing_readiness_reject_count",
    "rejected_readiness_reject_count",
    "invalid_readiness_token_reject_count",
    "invalid_candidate_token_reject_count",
    "invalid_provider_kind_reject_count",
    "closed_execution_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"owner-local counter {name} must be exact usize: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"last_reason must remain signed: {field}")
for name in (
    "provider_candidate_token",
    "provider_candidate_token_valid",
    "provider_kind",
    "provider_kind_valid",
    "would_select_provider",
    "would_activate_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap364a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
