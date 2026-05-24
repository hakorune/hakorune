#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-call-modeled-open-pilot"
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
    echo "[$TAG] ERROR: MIMAP-386A defers L3/L4 evidence to provider-call modeled-open closeout or first provider-call execution seam" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-call-modeled-open-pilot-proof/main.hako"
APP_README="apps/hako-alloc-provider-call-modeled-open-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-provider-call-modeled-open-pilot-proof/test.sh"
CARD_384A="docs/development/current/main/phases/phase-293x/293x-1006-MIMAP-384A-PROVIDER-CALL-DRY-RUN-UNSUPPORTED-BEHAVIOR.md"
CARD_385A="docs/development/current/main/phases/phase-293x/293x-1007-MIMAP-385A-POST-PROVIDER-CALL-DRY-RUN-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1008-MIMAP-386A-PROVIDER-CALL-MODELED-OPEN-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1009-MIMAP-387A-POST-PROVIDER-CALL-MODELED-OPEN-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-call-modeled-open-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/provider_call_modeled_open_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_call_dry_run_unsupported_behavior_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_call_modeled_open_pilot_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-386A provider-call modeled open pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_384A" "$CARD_385A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$MODULE_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_384A" "MIMAP-384A dry-run unsupported behavior must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_385A" "MIMAP-385A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-386A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$NEXT_CARD" "MIMAP-387A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-386A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-386A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-386A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-386A"
guard_expect_in_file "$TAG" 'row_kind = "modeled-open-pilot"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-386A must be a modeled-open-pilot row"
guard_expect_in_file "$TAG" 'memory.provider_call_modeled_open_pilot_box' "$MODULE" "module must export provider-call modeled-open owner"
guard_expect_in_file "$TAG" 'provider_call_modeled_open_pilot_box.hako' "$MODULE_INDEX" "memory module index must name provider-call modeled-open owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderCallModeledOpenPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderCallModeledOpenPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'openModeledProviderCall' "$OWNER" "owner must expose modeled provider-call route"
guard_expect_in_file "$TAG" 'HakoAllocProviderCallDryRunUnsupportedBehaviorReport' "$OWNER" "owner must consume provider-call dry-run report"
guard_expect_in_file "$TAG" 'modeled_open_count: usize = 0' "$OWNER" "modeled-open owner-local counters must be exact usize"
guard_expect_in_file "$TAG" 'closed_backend_matcher_reject_count: usize = 0' "$OWNER" "backend matcher owner-local counter must be exact usize"
guard_expect_in_file "$TAG" 'last_reason: i64 = 0' "$OWNER" "modeled-open reason vocabulary must remain signed"
guard_expect_in_file "$TAG" 'provider_call_modeled_open' "$OWNER" "owner must report modeled provider-call open state"
guard_expect_in_file "$TAG" 'provider_call_model_active' "$OWNER" "owner must report modeled provider-call active state"
guard_expect_in_file "$TAG" 'provider_call_execution_closed: dry_run.provider_call_execution_closed' "$OWNER" "actual provider-call execution must remain closed"
guard_expect_in_file "$TAG" 'would_call_provider: would_call' "$OWNER" "modeled-open report must expose would_call_provider"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'callProvider|provider_api|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-386A owner/app must keep provider-call/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-call-modeled-open-pilot-proof|ProviderCallModeledOpen|providerCallModeledOpen' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-386A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap386_provider_call_open.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap386.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-call-modeled-open-pilot-proof' "$vm_log"
rg -F -q 'opened=1,0,1,1,1,0' "$vm_log"
rg -F -q 'dryrun=1,1,0,1,1,1,1' "$vm_log"
rg -F -q 'owner=10,1,9,1,1,1,1,1,1,1,1,1,9' "$vm_log"
rg -F -q 'closed=1,1,1,1,1,1,0,0,0,0' "$vm_log"
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
    "HakoAllocProviderCallModeledOpenPilot.makeProviderCallModeledOpenPilotReport/1",
    "HakoAllocProviderCallModeledOpenPilot.openModeledProviderCall/1",
    "HakoAllocProviderCallModeledOpenPilot.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderCallModeledOpenPilotReport")
if report is None:
    raise SystemExit("missing provider-call modeled-open report typed object plan")
owner = plans.get("HakoAllocProviderCallModeledOpenPilot")
if owner is None:
    raise SystemExit("missing provider-call modeled-open typed object plan")
target = "HakoAllocProviderCallModeledOpenPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider-call modeled-open ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
for name in (
    "modeled_open_count",
    "accepted_count",
    "reject_count",
    "missing_dry_run_reject_count",
    "rejected_dry_run_reject_count",
    "missing_capability_reject_count",
    "invalid_capability_reject_count",
    "unsupported_outcome_reject_count",
    "closed_call_reject_count",
    "closed_host_replacement_reject_count",
    "closed_hook_reject_count",
    "closed_backend_matcher_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"owner-local counter {name} must be exact usize: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
    raise SystemExit(f"last_reason must remain signed: {field}")
for name in (
    "modeled_open_count",
    "accepted_count",
    "reject_count",
    "missing_dry_run_reject_count",
    "rejected_dry_run_reject_count",
    "missing_capability_reject_count",
    "invalid_capability_reject_count",
    "unsupported_outcome_reject_count",
    "closed_call_reject_count",
    "closed_host_replacement_reject_count",
    "closed_hook_reject_count",
    "closed_backend_matcher_reject_count",
    "modeled_open_present",
    "provider_call_unsupported",
    "provider_call_modeled_open",
    "provider_call_model_active",
    "provider_call_inactive",
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
print("[mimap386a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
