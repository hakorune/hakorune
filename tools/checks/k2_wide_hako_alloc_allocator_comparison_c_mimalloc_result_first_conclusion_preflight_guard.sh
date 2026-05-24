#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight"
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
    echo "[$TAG] ERROR: MIMAP-464A keeps final conclusions deferred beyond preflight" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight-proof/test.sh"
CARD_461A="docs/development/current/main/phases/phase-293x/293x-1091-MIMAP-461A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-REPORTING-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1094-MIMAP-464A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-FIRST-CONCLUSION-PREFLIGHT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight-ssot.md"
DESIGN_461A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-reporting-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_first_conclusion_preflight_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_reporting_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_preflight_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-464A allocator comparison C mimalloc result first conclusion preflight\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_461A" "$CARD" "$DESIGN" "$DESIGN_461A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MODULE_INDEX" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_461A" "MIMAP-461A must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-464A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-464A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_461A" "MIMAP-461A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-464A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-464A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-464A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-464A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-c-mimalloc-result-conclusion-row-selection"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-464A must defer final conclusion beyond preflight"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_result_first_conclusion_preflight_box' "$MODULE" "module must export first conclusion preflight owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_result_first_conclusion_preflight_box.hako' "$MODULE_INDEX" "memory module index must name first conclusion preflight owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocResultFirstConclusionPreflightReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocResultFirstConclusionPreflightReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'preflightAllocatorComparisonCMimallocResultFirstConclusion' "$OWNER" "owner must expose first conclusion preflight route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocResultReportingDiagnosticReport' "$OWNER" "owner must consume MIMAP-461A reporting diagnostic report"
guard_expect_in_file "$TAG" 'allocation_count_delta: report.allocation_count_delta' "$OWNER" "preflight must preserve allocation delta field"
guard_expect_in_file "$TAG" 'requested_bytes_delta: report.requested_bytes_delta' "$OWNER" "preflight must preserve requested bytes delta field"
guard_expect_in_file "$TAG" 'performance_conclusion_made: report.performance_conclusion_made' "$OWNER" "preflight must preserve performance conclusion field"
guard_expect_in_file "$TAG" 'provider_package_generated: report.provider_package_generated' "$OWNER" "preflight must preserve provider package field"

if rg -n 'run_benchmark[[:space:]]*\(|bash[[:space:]]+tools/allocator/c_mimalloc_explicit_runner|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-464A owner/app must keep benchmark/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocResultFirstConclusionPreflight|allocator-comparison-c-mimalloc-result-first-conclusion-preflight-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-464A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap464_c_mimalloc_result_first_conclusion_preflight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap464.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight-proof' "$vm_log"
rg -F -q 'preflight=1,0,1,1,1,0,1,1,1,1,1' "$vm_log"
rg -F -q 'metrics=3,72,64,33254,4096,61,33182' "$vm_log"
rg -F -q 'owner=6,1,5,1,1,1,1,1,8' "$vm_log"
rg -F -q 'blocked=0,0,0,0,0' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,6,7,8' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocResultFirstConclusionPreflight.makeAllocatorComparisonCMimallocResultFirstConclusionPreflightReport/1",
    "HakoAllocAllocatorComparisonCMimallocResultFirstConclusionPreflight.preflightAllocatorComparisonCMimallocResultFirstConclusion/1",
    "HakoAllocAllocatorComparisonCMimallocResultFirstConclusionPreflight.reasonFrom/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonCMimallocResultFirstConclusionPreflightReport")
if report is None:
    raise SystemExit("missing C mimalloc result first conclusion preflight report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocResultFirstConclusionPreflightReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing C mimalloc result first conclusion preflight ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "preflight_present",
    "conclusion_ready",
    "comparison_available",
    "hako_ready_execution_present",
    "c_ready_evidence_present",
    "memory_evidence_present",
    "allocation_count_delta",
    "requested_bytes_delta",
    "performance_conclusion_made",
    "memory_conclusion_made",
    "repeated_benchmark_executed",
    "provider_package_generated",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap464a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
