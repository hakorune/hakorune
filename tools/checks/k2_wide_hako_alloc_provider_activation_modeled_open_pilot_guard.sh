#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-activation-modeled-open-pilot"
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
    echo "[$TAG] ERROR: MIMAP-380A defers L3/L4 evidence to modeled-open closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-activation-modeled-open-pilot-proof/main.hako"
APP_README="apps/hako-alloc-provider-activation-modeled-open-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-provider-activation-modeled-open-pilot-proof/test.sh"
CARD_378A="docs/development/current/main/phases/phase-293x/293x-999-MIMAP-378A-PROVIDER-ACTIVATION-DRY-RUN-UNSUPPORTED-BEHAVIOR.md"
CARD_379A="docs/development/current/main/phases/phase-293x/293x-1000-MIMAP-379A-POST-PROVIDER-ACTIVATION-DRY-RUN-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1001-MIMAP-380A-PROVIDER-ACTIVATION-MODELED-OPEN-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1002-MIMAP-381A-POST-PROVIDER-ACTIVATION-MODELED-OPEN-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-activation-modeled-open-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_activation_modeled_open_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_activation_dry_run_unsupported_behavior_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_activation_modeled_open_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-380A provider activation modeled open pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_378A" "$CARD_379A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_378A" "MIMAP-378A dry-run unsupported behavior must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_379A" "MIMAP-379A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-380A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-381A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-380A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-380A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-380A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-380A"
guard_expect_in_file "$TAG" 'row_kind = "modeled-open-pilot"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-380A must be a modeled-open-pilot row"
guard_expect_in_file "$TAG" 'first_pattern = true' "$PROOF_MANIFEST_INCLUDE" "MIMAP-380A must be marked first-pattern"
guard_expect_in_file "$TAG" 'memory.provider_activation_modeled_open_pilot_box' "$MODULE" "module must export provider activation modeled-open owner"
guard_expect_in_file "$TAG" 'provider_activation_modeled_open_pilot_box.hako' "$MEMORY_README" "memory README must name provider activation modeled-open owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderActivationModeledOpenPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderActivationModeledOpenPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'openModeledProviderActivation' "$OWNER" "owner must expose modeled activation-open route"
guard_expect_in_file "$TAG" 'HakoAllocProviderActivationDryRunUnsupportedBehaviorReport' "$OWNER" "owner must consume dry-run unsupported report"
guard_expect_in_file "$TAG" 'provider_activation_modeled_open' "$OWNER" "owner must report modeled activation open state"
guard_expect_in_file "$TAG" 'provider_activation_model_active' "$OWNER" "owner must report modeled activation active state"
guard_expect_in_file "$TAG" 'would_activate_provider: would_activate' "$OWNER" "modeled-open report must expose would_activate_provider"
guard_expect_in_file "$TAG" 'would_call_provider: 0' "$OWNER" "provider calls must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-380A owner/app must keep provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-activation-modeled-open-pilot-proof|ProviderActivationModeledOpen|providerActivationModeledOpen' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-380A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap380_provider_modeled_open.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap380.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-activation-modeled-open-pilot-proof' "$vm_log"
rg -F -q 'opened=1,0,1,1,1,0' "$vm_log"
rg -F -q 'dryrun=1,1,0,99019005301,1,1' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'closed=1,1,1,1,1,0,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderActivationModeledOpenPilot.makeProviderActivationModeledOpenPilotReport/1",
    "HakoAllocProviderActivationModeledOpenPilot.openModeledProviderActivation/1",
    "HakoAllocProviderActivationModeledOpenPilot.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderActivationModeledOpenPilotReport")
if report is None:
    raise SystemExit("missing provider activation modeled-open report typed object plan")
target = "HakoAllocProviderActivationModeledOpenPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider activation modeled-open ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "provider_activation_modeled_open",
    "provider_activation_model_active",
    "provider_activation_inactive",
    "provider_call_inactive",
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
print("[mimap380a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
