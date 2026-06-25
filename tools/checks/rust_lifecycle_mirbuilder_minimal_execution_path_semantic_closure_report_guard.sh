#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_minimal_execution_path_semantic_closure_report.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json")
report = json.loads(path.read_text())

assert report["kind"] == "MinimalMirBuilderExecutionPathSemanticClosureReportV1"
closure = report["closure"]
assert closure["all_selected_source_edges_available"] is True
assert closure["semantic_plan_closure"] == "Closed"
assert closure["rust_smoke_observation"] == "Green"
assert closure["generated_hako_executable_closure"] == "Open"
assert closure["full_path_mainline_eligible"] is False
assert closure["source_selfhost_eligible"] is False
gap = report["first_executable_materialization_gap"]
assert gap["edge_id"] == "prepare_module.module_new"
assert gap["callsite"] == "MirBuilder::prepare_module -> MirModule::new"
assert gap["required_capability"] == "MirModuleMinimalShellTransport"
assert gap["next_slice_token"] == "MIR-MODULE-MINIMAL-SHELL-DERIVED-HAKO-ARTIFACT-001"
for edge in report["edges"]:
    if edge["evidence_tier"] == "PlanOnly":
        assert edge["artifact_materialization"] == "Missing", edge["edge_id"]
    if edge["evidence_tier"] == "Observed":
        assert edge["artifact_materialization"] == "NotApplicable", edge["edge_id"]
        assert edge["route_state"] == "NotApplicable", edge["edge_id"]
claims = report["claims"]
assert claims["semantic_closure_report"] == 1
for key in [
    "generated_hako_change",
    "full_build_module_generated_hako_execution",
    "full_path_mainline_selected",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "coverage_percentage_as_proof",
    "bundle_size_as_proof",
]:
    assert claims[key] == 0, key

print("output_contract=rust-lifecycle-minimal-mirbuilder-semantic-closure-report-guard-v0")
print("semantic_closure_report_guard=green")
print("semantic_plan_closure=Closed")
print("generated_hako_executable_closure=Open")
print(f"first_executable_materialization_gap={gap['edge_id']}")
print(f"next_slice_token={gap['next_slice_token']}")
print("full_path_mainline_eligible=0")
print("runtime_fallback=0")
print("summary=ok")
PY
