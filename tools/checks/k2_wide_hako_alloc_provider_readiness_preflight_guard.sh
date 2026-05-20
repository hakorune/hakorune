#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-readiness-preflight"
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
    echo "[$TAG] ERROR: MIMAP-362A defers L3/L4 evidence to a provider-facing closeout" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-readiness-preflight-proof/main.hako"
APP_README="apps/hako-alloc-provider-readiness-preflight-proof/README.md"
APP_TEST="apps/hako-alloc-provider-readiness-preflight-proof/test.sh"
CARD_360A="docs/development/current/main/phases/phase-293x/293x-976-MIMAP-360A-PROVIDER-BOUNDARY-DIAGNOSTIC-VOCABULARY.md"
CARD_361A="docs/development/current/main/phases/phase-293x/293x-977-MIMAP-361A-POST-PROVIDER-BOUNDARY-DIAGNOSTIC-VOCABULARY-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-978-MIMAP-362A-PROVIDER-READINESS-PREFLIGHT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-979-MIMAP-363A-POST-PROVIDER-READINESS-PREFLIGHT-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-readiness-preflight-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_readiness_preflight_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/provider_boundary_diagnostic_vocabulary_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_readiness_preflight_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-362A provider readiness preflight\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_360A" "$CARD_361A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_360A" "MIMAP-360A provider boundary diagnostic vocabulary must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_361A" "MIMAP-361A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-362A card must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-363A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-362A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-362A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-362A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-362A"
guard_expect_in_file "$TAG" 'row_kind = "preflight"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-362A must be a preflight row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-362A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.provider_readiness_preflight_box' "$MODULE" "module must export provider readiness preflight owner"
guard_expect_in_file "$TAG" 'provider_readiness_preflight_box.hako' "$MEMORY_README" "memory README must name provider readiness preflight owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderReadinessPreflightReportFields' "$OWNER" "provider readiness preflight owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderReadinessPreflightReport' "$OWNER" "provider readiness preflight owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'preflightProviderReadiness' "$OWNER" "provider readiness preflight owner must expose preflight route"
guard_expect_in_file "$TAG" 'HakoAllocProviderBoundaryDiagnosticVocabularyReport' "$OWNER" "provider readiness preflight owner must consume diagnostic vocabulary report"
guard_expect_in_file "$TAG" 'would_activate_provider: 0' "$OWNER" "provider activation must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"

if rg -n 'providerActivate|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|selectProvider|activateProvider|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-362A owner/app must keep provider/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-readiness-preflight-proof|ProviderReadinessPreflight|providerReadinessPreflight' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-362A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap362_provider_ready.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap362.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-readiness-preflight-proof' "$vm_log"
rg -F -q 'ready=1,0,1,99019005101,1' "$vm_log"
rg -F -q 'vocab=1,1,0,8' "$vm_log"
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
    "HakoAllocProviderReadinessPreflight.makeProviderReadinessPreflightReport/1",
    "HakoAllocProviderReadinessPreflight.preflightProviderReadiness/2",
    "HakoAllocProviderReadinessPreflight.reject/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderReadinessPreflightReport")
if report is None:
    raise SystemExit("missing provider readiness preflight report typed object plan")
target = "HakoAllocProviderReadinessPreflightReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider readiness preflight ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "readiness_token",
    "readiness_token_valid",
    "reason_count",
    "provider_activation_inactive",
    "host_replacement_inactive",
    "hooks_inactive",
    "backend_matcher_inactive",
    "would_activate_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap362a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
