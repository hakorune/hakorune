#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot"
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
    echo "[$TAG] ERROR: MIMAP-451A defines an explicit C runner pilot, not a repeated L3/L4 benchmark pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-proof/main.hako"
APP_README="apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-proof/test.sh"
CARD_448A="docs/development/current/main/phases/phase-293x/293x-1070-MIMAP-448A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-INVENTORY.md"
CARD_449A="docs/development/current/main/phases/phase-293x/293x-1071-MIMAP-449A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-DIAGNOSTICS.md"
CARD_450A="docs/development/current/main/phases/phase-293x/293x-1072-MIMAP-450A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1073-MIMAP-451A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1074-MIMAP-452A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTICS.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-ssot.md"
DESIGN_449A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-ssot.md"
PROVIDER_PACKAGE_SSOT="docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako"
PREV_OWNER="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh"
C_RUNNER_SH="tools/allocator/c_mimalloc_explicit_runner.sh"
C_RUNNER_C="tools/allocator/c_mimalloc_explicit_runner.c"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-451A explicit C mimalloc runner execution pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_448A" "$CARD_449A" "$CARD_450A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_449A" "$PROVIDER_PACKAGE_SSOT" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$PREV_OWNER" "$SELF_SCRIPT" "$C_RUNNER_SH" "$C_RUNNER_C" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$C_RUNNER_SH" "$RUN_PROOF"

for card in "$CARD_448A" "$CARD_449A" "$CARD_450A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-452A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-451A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_449A" "MIMAP-449A design must remain accepted"
guard_expect_in_file "$TAG" 'MIMAP-451A should continue to build the C mimalloc explicit runner execution' "$PROVIDER_PACKAGE_SSOT" "provider package SSOT must keep MIMAP-451A distinct from DLL generation"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-451A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-451A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-451A"
guard_expect_in_file "$TAG" 'validation_profile = "external-runner-pilot"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-451A must use external-runner-pilot validation"
guard_expect_in_file "$TAG" 'exe = "explicit-c-mimalloc-runner"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-451A must record explicit C mimalloc runner execution"
guard_expect_in_file "$TAG" 'memory.allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box' "$MODULE" "module must export explicit C mimalloc runner pilot owner"
guard_expect_in_file "$TAG" 'allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako' "$MEMORY_README" "memory README must name explicit C mimalloc runner pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReportFields' "$OWNER" "owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocExplicitRunnerRunEvidence' "$OWNER" "owner must group runner evidence in a context record"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocExplicitRunnerMemoryEvidence' "$OWNER" "owner must group memory evidence in a context record"
guard_expect_in_file "$TAG" 'record HakoAllocAllocatorComparisonCMimallocExplicitRunnerStopLineEvidence' "$OWNER" "owner must group stop-line evidence in a context record"
guard_expect_in_file "$TAG" 'makeAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReport' "$OWNER" "owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordAllocatorComparisonCMimallocExplicitRunnerExecution' "$OWNER" "owner must expose explicit runner evidence route"
guard_expect_in_file "$TAG" 'HakoAllocAllocatorComparisonCMimallocExecutionDiagnosticReport' "$OWNER" "owner must consume MIMAP-449A diagnostic report"
guard_expect_in_file "$TAG" 'c_mimalloc_executed: executed' "$OWNER" "accepted report must record C mimalloc execution evidence"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "process replacement must stay closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "hook installation must stay closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER" "backend matcher addition must stay closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "global allocator installation must stay closed"
guard_expect_in_file "$TAG" 'provider_package_generated: 0' "$OWNER" "provider package generation must stay closed"
guard_expect_in_file "$TAG" 'output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0' "$C_RUNNER_C" "C runner must emit stable output contract"
guard_expect_in_file "$TAG" 'dlopen\(config.library_path' "$C_RUNNER_C" "C runner must load an explicit mimalloc path"
guard_expect_in_file "$TAG" 'mi_malloc' "$C_RUNNER_C" "C runner must call mimalloc allocation symbols"
guard_expect_in_file "$TAG" 'hidden_discovery_used=0' "$C_RUNNER_C" "C runner evidence must report hidden discovery inactive"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" "$C_RUNNER_C" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-451A owner/app/tool must keep replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocExplicitRunnerExecutionPilot|allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-proof|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-451A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap451_c_mimalloc_explicit_runner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
runner_log="$tmp_dir/c_mimalloc_runner.out"
mir_json="$tmp_dir/mimap451.mir.json"
vm_log="$tmp_dir/vm.log"

bash "$C_RUNNER_SH" --out "$runner_log" --allow-ldconfig-discovery >/tmp/"$TAG".runner_stdout
cat /tmp/"$TAG".runner_stdout
rm -f /tmp/"$TAG".runner_stdout

python3 - "$runner_log" <<'PY'
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = {}
for line in path.read_text(encoding="utf-8").splitlines():
    if "=" in line:
        key, value = line.split("=", 1)
        data[key] = value

required = {
    "c_mimalloc_runner": "1",
    "output_contract": "allocator-comparison-c-mimalloc-explicit-runner-v0",
    "workload": "representative-small-block-v0",
    "result_code": "0",
    "run_count": "1",
    "memory_usage_evidence": "1",
    "process_replacement_executed": "0",
    "hook_installed": "0",
    "backend_matcher_added": "0",
    "global_allocator_installed": "0",
    "hidden_discovery_used": "0",
    "provider_package_generated": "0",
    "summary": "ok",
}
for key, expected in required.items():
    actual = data.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")
for key in ("allocation_count", "free_count", "requested_bytes", "peak_rss_bytes"):
    value = int(data.get(key, "0"))
    if value < 1:
        raise SystemExit(f"{key} must be positive")
if data["allocation_count"] != data["free_count"]:
    raise SystemExit("allocation_count and free_count must match")
if not data.get("library_path", "").endswith("libmimalloc.so.2"):
    raise SystemExit(f"unexpected library_path: {data.get('library_path')!r}")
print("[mimap451a-c-runner] ok")
PY

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-proof' "$vm_log"
rg -F -q 'execution=1,0,1,1,1,1,1,1,0,1,64,64,33254,4096' "$vm_log"
rg -F -q 'owner=9,1,8,1,1,1,1,1,1,1,1,8' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot.makeAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReport/1",
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot.makeReport/5",
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot.recordAllocatorComparisonCMimallocExplicitRunnerExecution/11",
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot.reject/5",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReport")
if report is None:
    raise SystemExit("missing explicit C mimalloc runner pilot report typed object plan")
target = "HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing explicit C mimalloc runner pilot ReportFields record")
for target in (
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerRunEvidence",
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerMemoryEvidence",
    "HakoAllocAllocatorComparisonCMimallocExplicitRunnerStopLineEvidence",
):
    if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
        raise SystemExit(f"missing explicit C mimalloc runner context record: {target}")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "execution_pilot_present",
    "diagnostic_ready",
    "explicit_runner_invoked",
    "explicit_runner_output_present",
    "c_mimalloc_execution_evidence_present",
    "memory_usage_evidence_present",
    "stable_output_contract_present",
    "allocation_count",
    "free_count",
    "requested_bytes",
    "peak_rss_bytes",
    "c_mimalloc_executed",
    "process_replacement_executed",
    "global_allocator_installed",
    "provider_package_generated",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap451a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
