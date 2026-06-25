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
assert gap["edge_id"] == "lower_root.literal_integer"
assert gap["callsite"] == "MirBuilder::lower_root(ASTNode::Literal(Integer(0)))"
assert gap["required_capability"] == "LiteralIntegerLowering"
assert gap["next_slice_token"] == "MIRBUILDER-LITERAL-INTEGER-DERIVED-HAKO-ARTIFACT-001"
module_edges = [edge for edge in report["edges"] if edge["edge_id"] == "prepare_module.module_new"]
assert len(module_edges) == 1
module_edge = module_edges[0]
assert module_edge["evidence_tier"] == "VerifiedArtifact"
assert module_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert module_edge["route_state"] == "DerivedShadow"
assert module_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json"
function_edges = [edge for edge in report["edges"] if edge["edge_id"] == "prepare_module.function_new"]
assert len(function_edges) == 1
function_edge = function_edges[0]
assert function_edge["evidence_tier"] == "VerifiedArtifact"
assert function_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert function_edge["route_state"] == "DerivedShadow"
assert function_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json"
state_install_edges = [edge for edge in report["edges"] if edge["edge_id"] == "prepare_module.state_install"]
assert len(state_install_edges) == 1
state_install_edge = state_install_edges[0]
assert state_install_edge["evidence_tier"] == "VerifiedArtifact"
assert state_install_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert state_install_edge["route_state"] == "DerivedShadow"
assert state_install_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json"
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
