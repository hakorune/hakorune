#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-activation-unsupported-outcome-ledger"
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
    echo "[$TAG] ERROR: MIMAP-370A defers L3/L4 evidence to an unsupported-outcome closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-activation-unsupported-outcome-ledger-proof/main.hako"
APP_README="apps/hako-alloc-provider-activation-unsupported-outcome-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-provider-activation-unsupported-outcome-ledger-proof/test.sh"
CARD_368A="docs/development/current/main/phases/phase-293x/293x-984-MIMAP-368A-PROVIDER-ACTIVATION-FIRST-PATTERN-PLAN.md"
CARD_369A="docs/development/current/main/phases/phase-293x/293x-985-MIMAP-369A-POST-PROVIDER-ACTIVATION-FIRST-PATTERN-PLAN-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-986-MIMAP-370A-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-LEDGER.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-987-MIMAP-371A-POST-PROVIDER-ACTIVATION-UNSUPPORTED-OUTCOME-LEDGER-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-activation-unsupported-outcome-ledger-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_activation_unsupported_outcome_ledger_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_selection_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_ledger_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-370A provider activation unsupported outcome ledger\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_368A" "$CARD_369A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_368A" "MIMAP-368A provider activation first-pattern plan must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_369A" "MIMAP-369A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-370A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-371A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-370A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-370A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-370A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-370A"
guard_expect_in_file "$TAG" 'row_kind = "unsupported-outcome-ledger"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-370A must be an unsupported-outcome-ledger row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-370A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.provider_activation_unsupported_outcome_ledger_box' "$MODULE" "module must export provider activation unsupported outcome ledger owner"
guard_expect_in_file "$TAG" 'provider_activation_unsupported_outcome_ledger_box.hako' "$MEMORY_README" "memory README must name provider activation unsupported outcome ledger owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderActivationUnsupportedOutcomeLedgerReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderActivationUnsupportedOutcomeLedgerReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'ledgerProviderUnsupportedOutcome' "$OWNER" "owner must expose unsupported outcome ledger route"
guard_expect_in_file "$TAG" 'HakoAllocProviderSelectionInventoryReport' "$OWNER" "owner must consume provider selection inventory report"
guard_expect_in_file "$TAG" 'provider_activation_unsupported: 1' "$OWNER" "activation must be represented as unsupported"
guard_expect_in_file "$TAG" 'would_activate_provider: 0' "$OWNER" "provider activation must not execute"
guard_expect_in_file "$TAG" 'would_call_provider: 0' "$OWNER" "provider calls must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"

if rg -n 'providerActivate|callProvider|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|activateProvider|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-370A owner/app must keep activation/provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-activation-unsupported-outcome-ledger-proof|ProviderActivationUnsupportedOutcome|providerActivationUnsupportedOutcome' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-370A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap370_provider_unsupported.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap370.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-activation-unsupported-outcome-ledger-proof' "$vm_log"
rg -F -q 'outcome=1,0,1,1' "$vm_log"
rg -F -q 'selection=1,1,0,99019005201,1,1,1' "$vm_log"
rg -F -q 'owner=6,1,5,1,1,1,1,1,5' "$vm_log"
rg -F -q 'inactive=1,1,1,1,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderActivationUnsupportedOutcomeLedger.makeProviderActivationUnsupportedOutcomeLedgerReport/1",
    "HakoAllocProviderActivationUnsupportedOutcomeLedger.ledgerProviderUnsupportedOutcome/1",
    "HakoAllocProviderActivationUnsupportedOutcomeLedger.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderActivationUnsupportedOutcomeLedgerReport")
if report is None:
    raise SystemExit("missing provider activation unsupported outcome ledger report typed object plan")
target = "HakoAllocProviderActivationUnsupportedOutcomeLedgerReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider activation unsupported outcome ledger ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "provider_activation_unsupported",
    "would_activate_provider",
    "would_call_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap370a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
