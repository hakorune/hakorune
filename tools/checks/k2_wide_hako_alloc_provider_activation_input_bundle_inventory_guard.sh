#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-activation-input-bundle-inventory"
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
    echo "[$TAG] ERROR: MIMAP-376A defers L3/L4 evidence to an explicit first-pattern provider activation row" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-activation-input-bundle-inventory-proof/main.hako"
APP_README="apps/hako-alloc-provider-activation-input-bundle-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-provider-activation-input-bundle-inventory-proof/test.sh"
CARD_374A="docs/development/current/main/phases/phase-293x/293x-990-MIMAP-374A-PROVIDER-ACTIVATION-EXPLICIT-INPUT-CONTRACT.md"
CARD_375A="docs/development/current/main/phases/phase-293x/293x-991-MIMAP-375A-POST-PROVIDER-ACTIVATION-EXPLICIT-INPUT-CONTRACT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-997-MIMAP-376A-PROVIDER-ACTIVATION-INPUT-BUNDLE-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-998-MIMAP-377A-POST-PROVIDER-ACTIVATION-INPUT-BUNDLE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-activation-input-bundle-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_activation_input_bundle_inventory_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_activation_unsupported_outcome_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_activation_input_bundle_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-376A provider activation input bundle inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_374A" "$CARD_375A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_374A" "MIMAP-374A explicit-input contract must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD_375A" "MIMAP-375A row-selection card must be selected current or landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-376A card must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-377A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-376A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-376A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-376A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-376A"
guard_expect_in_file "$TAG" 'row_kind = "input-bundle-inventory"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-376A must be an input-bundle-inventory row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-376A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-first-pattern"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-376A must defer EXE to first-pattern"
guard_expect_in_file "$TAG" 'memory.provider_activation_input_bundle_inventory_box' "$MODULE" "module must export provider activation input bundle inventory owner"
guard_expect_in_file "$TAG" 'provider_activation_input_bundle_inventory_box.hako' "$MEMORY_README" "memory README must name provider activation input bundle owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderActivationInputBundleInventoryReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderActivationInputBundleInventoryReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryProviderActivationInputBundle' "$OWNER" "owner must expose input bundle inventory route"
guard_expect_in_file "$TAG" 'HakoAllocProviderActivationUnsupportedOutcomeLedgerReport' "$OWNER" "owner must consume unsupported-outcome ledger report"
guard_expect_in_file "$TAG" 'bundle_count: usize = 0' "$OWNER" "input bundle owner-local counters must be exact usize"
guard_expect_in_file "$TAG" 'closed_execution_reject_count: usize = 0' "$OWNER" "closed-execution owner-local counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "input bundle reason vocabulary must remain signed"
guard_expect_in_file "$TAG" 'activation_request_token' "$OWNER" "owner must require explicit activation request token"
guard_expect_in_file "$TAG" 'activation_mode' "$OWNER" "owner must require explicit activation mode"
guard_expect_in_file "$TAG" 'provider_activation_unsupported: i64 = 1' "$OWNER" "activation must stay unsupported by default"
guard_expect_in_file "$TAG" 'would_activate_provider: 0' "$OWNER" "provider activation must not execute"
guard_expect_in_file "$TAG" 'would_call_provider: 0' "$OWNER" "provider calls must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'providerActivate|callProvider|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|activateProvider|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-376A owner/app must keep activation/provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-activation-input-bundle-inventory-proof|ProviderActivationInputBundle|providerActivationInputBundle' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-376A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap376_provider_input_bundle.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap376.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-activation-input-bundle-inventory-proof' "$vm_log"
rg -F -q 'bundle=1,0,1,1,99019005301,1,1' "$vm_log"
rg -F -q 'outcome=1,1,0,99019005201,1,1,1' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'inactive=1,1,1,1,1,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderActivationInputBundleInventory.makeProviderActivationInputBundleInventoryReport/1",
    "HakoAllocProviderActivationInputBundleInventory.inventoryProviderActivationInputBundle/3",
    "HakoAllocProviderActivationInputBundleInventory.reject/4",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderActivationInputBundleInventoryReport")
if report is None:
    raise SystemExit("missing provider activation input bundle inventory report typed object plan")
owner = plans.get("HakoAllocProviderActivationInputBundleInventory")
if owner is None:
    raise SystemExit("missing provider activation input bundle inventory typed object plan")
target = "HakoAllocProviderActivationInputBundleInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider activation input bundle ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "bundle_count",
    "accepted_count",
    "reject_count",
    "missing_outcome_reject_count",
    "rejected_outcome_reject_count",
    "invalid_candidate_reject_count",
    "invalid_kind_reject_count",
    "invalid_request_token_reject_count",
    "invalid_mode_reject_count",
    "unsupported_evidence_reject_count",
    "closed_execution_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"owner-local counter {name} must be exact usize: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"last_reason must remain signed: {field}")
for name in (
    "bundle_count",
    "accepted_count",
    "reject_count",
    "missing_outcome_reject_count",
    "rejected_outcome_reject_count",
    "invalid_candidate_reject_count",
    "invalid_kind_reject_count",
    "invalid_request_token_reject_count",
    "invalid_mode_reject_count",
    "unsupported_evidence_reject_count",
    "closed_execution_reject_count",
    "activation_request_token",
    "activation_request_token_valid",
    "activation_mode",
    "activation_mode_valid",
    "provider_activation_unsupported",
    "would_activate_provider",
    "would_call_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "would_run_thread",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap376a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
