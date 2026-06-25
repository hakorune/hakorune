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
assert gap["edge_id"] == "finalize_module.type_propagation"
assert gap["callsite"] == "MirBuilder::finalize_module -> TypePropagationPipeline::run"
assert gap["required_capability"] == "TypePropagationPipelineExecution"
assert gap["next_slice_token"] == "MIRBUILDER-TYPE-PROPAGATION-PIPELINE-DERIVED-HAKO-ARTIFACT-001"
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
literal_edges = [edge for edge in report["edges"] if edge["edge_id"] == "lower_root.literal_integer"]
assert len(literal_edges) == 1
literal_edge = literal_edges[0]
assert literal_edge["evidence_tier"] == "VerifiedArtifact"
assert literal_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert literal_edge["route_state"] == "DerivedShadow"
assert literal_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.artifact.json"
finalize_edges = [edge for edge in report["edges"] if edge["edge_id"] == "finalize_module.composition"]
assert len(finalize_edges) == 1
finalize_edge = finalize_edges[0]
assert finalize_edge["evidence_tier"] == "VerifiedArtifact"
assert finalize_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert finalize_edge["route_state"] == "DerivedShadow"
assert finalize_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.artifact.json"
return_emission_edges = [edge for edge in report["edges"] if edge["edge_id"] == "finalize_module.return_emission"]
assert len(return_emission_edges) == 1
return_emission_edge = return_emission_edges[0]
assert return_emission_edge["evidence_tier"] == "VerifiedArtifact"
assert return_emission_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert return_emission_edge["route_state"] == "DerivedShadow"
assert return_emission_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.artifact.json"
return_type_edges = [edge for edge in report["edges"] if edge["edge_id"] == "finalize_module.return_type_publication"]
assert len(return_type_edges) == 1
return_type_edge = return_type_edges[0]
assert return_type_edge["evidence_tier"] == "VerifiedArtifact"
assert return_type_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert return_type_edge["route_state"] == "DerivedShadow"
assert return_type_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.artifact.json"
current_module_edges = [edge for edge in report["edges"] if edge["edge_id"] == "finalize_module.take_module"]
assert len(current_module_edges) == 1
current_module_edge = current_module_edges[0]
assert current_module_edge["evidence_tier"] == "VerifiedArtifact"
assert current_module_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert current_module_edge["route_state"] == "DerivedShadow"
assert current_module_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.artifact.json"
typed_value_edges = [edge for edge in report["edges"] if edge["edge_id"] == "finalize_module.verify_typed_values"]
assert len(typed_value_edges) == 1
typed_value_edge = typed_value_edges[0]
assert typed_value_edge["evidence_tier"] == "VerifiedArtifact"
assert typed_value_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert typed_value_edge["route_state"] == "DerivedShadow"
assert typed_value_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.artifact.json"
current_function_edges = [edge for edge in report["edges"] if edge["edge_id"] == "finalize_module.take_current_function"]
assert len(current_function_edges) == 1
current_function_edge = current_function_edges[0]
assert current_function_edge["evidence_tier"] == "VerifiedArtifact"
assert current_function_edge["artifact_materialization"] == "ExecutableArtifactPresent"
assert current_function_edge["route_state"] == "DerivedShadow"
assert current_function_edge["provider_reference"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.artifact.json"
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
