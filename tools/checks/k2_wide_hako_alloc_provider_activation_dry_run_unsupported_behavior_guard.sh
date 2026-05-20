#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-activation-dry-run-unsupported-behavior"
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
    echo "[$TAG] ERROR: MIMAP-378A defers L3/L4 evidence to dry-run closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-activation-dry-run-unsupported-behavior-proof/main.hako"
APP_README="apps/hako-alloc-provider-activation-dry-run-unsupported-behavior-proof/README.md"
APP_TEST="apps/hako-alloc-provider-activation-dry-run-unsupported-behavior-proof/test.sh"
CARD_376A="docs/development/current/main/phases/phase-293x/293x-997-MIMAP-376A-PROVIDER-ACTIVATION-INPUT-BUNDLE-INVENTORY.md"
CARD_377A="docs/development/current/main/phases/phase-293x/293x-998-MIMAP-377A-POST-PROVIDER-ACTIVATION-INPUT-BUNDLE-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-999-MIMAP-378A-PROVIDER-ACTIVATION-DRY-RUN-UNSUPPORTED-BEHAVIOR.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1000-MIMAP-379A-POST-PROVIDER-ACTIVATION-DRY-RUN-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-activation-dry-run-unsupported-behavior-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_activation_dry_run_unsupported_behavior_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_activation_input_bundle_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_activation_dry_run_unsupported_behavior_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-378A provider activation dry-run unsupported behavior\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_376A" "$CARD_377A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_376A" "MIMAP-376A input bundle inventory must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_377A" "MIMAP-377A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-378A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-379A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-378A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-378A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-378A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-378A"
guard_expect_in_file "$TAG" 'row_kind = "dry-run-unsupported-behavior"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-378A must be a dry-run-unsupported-behavior row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-378A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.provider_activation_dry_run_unsupported_behavior_box' "$MODULE" "module must export provider activation dry-run owner"
guard_expect_in_file "$TAG" 'provider_activation_dry_run_unsupported_behavior_box.hako' "$MEMORY_README" "memory README must name provider activation dry-run owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderActivationDryRunUnsupportedBehaviorReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderActivationDryRunUnsupportedBehaviorReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'dryRunProviderActivationUnsupported' "$OWNER" "owner must expose dry-run route"
guard_expect_in_file "$TAG" 'HakoAllocProviderActivationInputBundleInventoryReport' "$OWNER" "owner must consume input bundle report"
guard_expect_in_file "$TAG" 'dry_run_attempted' "$OWNER" "owner must report dry-run attempt"
guard_expect_in_file "$TAG" 'unsupported_outcome_present' "$OWNER" "owner must report unsupported outcome"
guard_expect_in_file "$TAG" 'provider_activation_unsupported: i64 = 1' "$OWNER" "activation must stay unsupported by default"
guard_expect_in_file "$TAG" 'would_activate_provider: 0' "$OWNER" "provider activation must not execute"
guard_expect_in_file "$TAG" 'would_call_provider: 0' "$OWNER" "provider calls must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'providerActivate|callProvider|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|activateProvider|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-378A owner/app must keep activation/provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-activation-dry-run-unsupported-behavior-proof|ProviderActivationDryRunUnsupported|providerActivationDryRunUnsupported' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-378A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap378_provider_dryrun.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap378.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-activation-dry-run-unsupported-behavior-proof' "$vm_log"
rg -F -q 'dryrun=1,0,1,1' "$vm_log"
rg -F -q 'bundle=1,1,0,99019005301,1,1' "$vm_log"
rg -F -q 'owner=7,1,6,1,1,1,1,1,1,6' "$vm_log"
rg -F -q 'inactive=1,1,1,1,1,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderActivationDryRunUnsupportedBehavior.makeProviderActivationDryRunUnsupportedBehaviorReport/1",
    "HakoAllocProviderActivationDryRunUnsupportedBehavior.dryRunProviderActivationUnsupported/1",
    "HakoAllocProviderActivationDryRunUnsupportedBehavior.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderActivationDryRunUnsupportedBehaviorReport")
if report is None:
    raise SystemExit("missing provider activation dry-run report typed object plan")
target = "HakoAllocProviderActivationDryRunUnsupportedBehaviorReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider activation dry-run ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "dry_run_attempted",
    "unsupported_outcome_present",
    "activation_request_token",
    "activation_mode",
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
print("[mimap378a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
