#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-boundary-diagnostic-vocabulary"
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
    echo "[$TAG] ERROR: MIMAP-360A defers L3/L4 evidence to a provider-facing closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-boundary-diagnostic-vocabulary-proof/main.hako"
APP_README="apps/hako-alloc-provider-boundary-diagnostic-vocabulary-proof/README.md"
APP_TEST="apps/hako-alloc-provider-boundary-diagnostic-vocabulary-proof/test.sh"
CARD_358A="docs/development/current/main/phases/phase-293x/293x-974-MIMAP-358A-PROVIDER-FACING-LADDER-CLOSED-PLAN.md"
CARD_359A="docs/development/current/main/phases/phase-293x/293x-975-MIMAP-359A-POST-PROVIDER-FACING-LADDER-PLAN-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-976-MIMAP-360A-PROVIDER-BOUNDARY-DIAGNOSTIC-VOCABULARY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-977-MIMAP-361A-POST-PROVIDER-BOUNDARY-DIAGNOSTIC-VOCABULARY-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-boundary-diagnostic-vocabulary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_boundary_diagnostic_vocabulary_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_inactive_boundary_inventory_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_boundary_diagnostic_vocabulary_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-360A provider boundary diagnostic vocabulary\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_358A" "$CARD_359A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_358A" "MIMAP-358A provider-facing closed plan must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_359A" "MIMAP-359A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-360A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-361A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-360A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-360A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-360A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-360A"
guard_expect_in_file "$TAG" 'row_kind = "diagnostic-vocabulary"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-360A must be a diagnostic-vocabulary row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-360A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.provider_boundary_diagnostic_vocabulary_box' "$MODULE" "module must export provider boundary vocabulary owner"
guard_expect_in_file "$TAG" 'provider_boundary_diagnostic_vocabulary_box.hako' "$MEMORY_README" "memory README must name provider boundary vocabulary owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderBoundaryDiagnosticVocabularyReportFields' "$OWNER" "provider vocabulary owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderBoundaryDiagnosticVocabularyReport' "$OWNER" "provider vocabulary owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'inventoryProviderBoundaryDiagnostics' "$OWNER" "provider vocabulary owner must expose inventory route"
guard_expect_in_file "$TAG" 'HakoAllocProviderInactiveBoundaryInventoryReport' "$OWNER" "provider vocabulary owner must consume provider inactive boundary report"
guard_expect_in_file "$TAG" 'reason_count: 8' "$OWNER" "provider vocabulary must publish eight reason codes"
guard_expect_in_file "$TAG" 'provider_request_reason_code: 3' "$OWNER" "provider request reason must be code 3"
guard_expect_in_file "$TAG" 'host_replacement_reason_code: 4' "$OWNER" "host replacement reason must be code 4"
guard_expect_in_file "$TAG" 'hook_request_reason_code: 5' "$OWNER" "hook reason must be code 5"
guard_expect_in_file "$TAG" 'backend_matcher_reason_code: 6' "$OWNER" "backend matcher reason must be code 6"
guard_expect_in_file "$TAG" 'worker_thread_reason_code: 7' "$OWNER" "worker/thread reason must be code 7"
guard_expect_in_file "$TAG" 'would_activate_provider: 0' "$OWNER" "provider activation must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"

if rg -n 'providerActivate|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|selectProvider|activateProvider|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-360A owner/app must keep provider/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-boundary-diagnostic-vocabulary-proof|ProviderBoundaryDiagnosticVocabulary|providerBoundaryDiagnosticVocabulary' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-360A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap360_provider_vocab.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap360.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-boundary-diagnostic-vocabulary-proof' "$vm_log"
rg -F -q 'vocab=1,0,1,8,99019005099' "$vm_log"
rg -F -q 'codes=0,1,2,3,4,5,6,7' "$vm_log"
rg -F -q 'owner=8,1,7,1,1,1,1,1,1,1,7' "$vm_log"
rg -F -q 'inactive=1,1,1,1,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6,7' "$vm_log"
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
    "HakoAllocProviderBoundaryDiagnosticVocabulary.makeProviderBoundaryDiagnosticVocabularyReport/1",
    "HakoAllocProviderBoundaryDiagnosticVocabulary.inventoryProviderBoundaryDiagnostics/1",
    "HakoAllocProviderBoundaryDiagnosticVocabulary.reject/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderBoundaryDiagnosticVocabularyReport")
if report is None:
    raise SystemExit("missing provider boundary diagnostic vocabulary report typed object plan")
target = "HakoAllocProviderBoundaryDiagnosticVocabularyReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider boundary diagnostic vocabulary ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "reason_count",
    "provider_request_reason_code",
    "host_replacement_reason_code",
    "hook_request_reason_code",
    "backend_matcher_reason_code",
    "worker_thread_reason_code",
    "would_activate_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap360a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
