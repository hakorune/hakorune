#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-host-replacement-blocked-state-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-421A defers L3/L4 to the host replacement preflight closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-host-replacement-blocked-state-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-host-replacement-blocked-state-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-host-replacement-blocked-state-diagnostics-proof/test.sh"
CARD_420A="docs/development/current/main/phases/phase-293x/293x-1042-MIMAP-420A-HOST-REPLACEMENT-EXPLICIT-PREFLIGHT-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1043-MIMAP-421A-HOST-REPLACEMENT-BLOCKED-STATE-DIAGNOSTICS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1044-MIMAP-422A-HOST-REPLACEMENT-PREFLIGHT-CLOSEOUT.md"
DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-blocked-state-diagnostics-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-explicit-preflight-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/host_replacement_blocked_state_diagnostic_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/host_replacement_explicit_preflight_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_host_replacement_blocked_state_diagnostics_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-421A host replacement blocked-state diagnostics\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_420A" "$CARD" "$NEXT_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

for card in "$CARD_420A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-422A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-421A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-420A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-421A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-421A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-421A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-421A must be scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-host-replacement-preflight-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-421A must defer EXE to closeout"
guard_expect_in_file "$TAG" 'memory.host_replacement_blocked_state_diagnostic_box' "$MODULE" "module must export blocked-state diagnostic owner"
guard_expect_in_file "$TAG" 'host_replacement_blocked_state_diagnostic_box.hako' "$MEMORY_README" "memory README must name blocked-state diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocHostReplacementBlockedStateDiagnosticReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeHostReplacementBlockedStateDiagnosticReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'diagnoseHostReplacementBlockedState' "$OWNER" "owner must expose diagnostic route"
guard_expect_in_file "$TAG" 'HakoAllocHostReplacementExplicitPreflightInventoryReport' "$OWNER" "owner must consume MIMAP-420A preflight report"
guard_expect_in_file "$TAG" 'host_replacement_executed: 0' "$OWNER" "host replacement execution must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook install must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator install must stay closed"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "thread execution must not execute"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-421A owner/app must keep replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'host-replacement-blocked-state-diagnostics-proof|HostReplacementBlockedStateDiagnostic|hostReplacementBlockedStateDiagnostic|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-421A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap421_host_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap421.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-host-replacement-blocked-state-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,1,1,1,0' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'blocked=1,1,1,1,1,1,1,1' "$vm_log"
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
    "HakoAllocHostReplacementBlockedStateDiagnostic.makeHostReplacementBlockedStateDiagnosticReport/1",
    "HakoAllocHostReplacementBlockedStateDiagnostic.diagnoseHostReplacementBlockedState/1",
    "HakoAllocHostReplacementBlockedStateDiagnostic.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocHostReplacementBlockedStateDiagnosticReport")
if report is None:
    raise SystemExit("missing host replacement blocked-state diagnostic report typed object plan")
target = "HakoAllocHostReplacementBlockedStateDiagnosticReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing host replacement blocked-state diagnostic ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "diagnostic_present",
    "blocked_state_present",
    "missing_real_call_blocked",
    "missing_explicit_request_blocked",
    "closed_seam_blocked",
    "host_replacement_executed",
    "hook_installed",
    "backend_matcher_added",
    "global_allocator_installed",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap421a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
