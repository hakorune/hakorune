#!/usr/bin/env python3
"""Generate the minimal MirBuilder execution path semantic closure report.

The report intentionally separates semantic closure from executable Hako
materialization. It proves that all selected source edges before the design
stop are available, but keeps generated-Hako executable closure, full-path
mainline eligibility, and source-selfhost eligibility open/denied.
"""

from __future__ import annotations

import argparse
import hashlib
from collections import Counter
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PLAN_PATH = FIXTURES / "minimal-mirbuilder-execution-path-plan-v0.json"
RESULT_PATH = FIXTURES / "minimal-mirbuilder-first-red-edge-result-v0.json"
FIXTURE = FIXTURES / "minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"

EXECUTABLE_ARTIFACT_STATES = {"DerivedShadow", "DerivedMainline"}
EXECUTABLE_ARTIFACT_MANIFESTS = {
    "MirModuleMinimalShellTransport": ROOT
    / "lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json",
    "MirFunctionConstructorTransport": ROOT
    / "lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json",
    "PreparedStateInstall": ROOT
    / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json",
}


def _read_json(path: Path) -> dict[str, Any]:
    import json

    return json.loads(path.read_text())


def _artifact_materialization(artifact_state: str | None) -> str:
    if artifact_state == "PlanOnly":
        return "Missing"
    if artifact_state in EXECUTABLE_ARTIFACT_STATES:
        return "ExecutableArtifactPresent"
    return "NotApplicable"


def _semantic_evidence(artifact_state: str | None, status: str) -> str:
    if artifact_state == "PlanOnly":
        return "PlanOnly"
    if artifact_state == "Observed":
        return "Observed"
    if artifact_state in EXECUTABLE_ARTIFACT_STATES:
        return "VerifiedArtifact"
    if status == "ProfileExcluded":
        return "ProfileExcluded"
    return "SourceOrderOrProfile"


def _route_state(artifact_state: str | None) -> str:
    if artifact_state == "DerivedMainline":
        return "DerivedMainline"
    if artifact_state == "DerivedShadow":
        return "DerivedShadow"
    if artifact_state == "PlanOnly":
        return "Unselected"
    return "NotApplicable"


def _sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _executable_artifact_contracts() -> dict[str, dict[str, Any]]:
    contracts: dict[str, dict[str, Any]] = {}
    for capability, manifest_path in EXECUTABLE_ARTIFACT_MANIFESTS.items():
        if not manifest_path.exists():
            continue
        manifest = _read_json(manifest_path)
        require(manifest.get("kind") == "RustDerivedHakoArtifact", f"wrong artifact kind: {manifest_path}")
        state = manifest.get("state")
        require(state in EXECUTABLE_ARTIFACT_STATES, f"artifact state is not executable: {manifest_path}")
        output = manifest.get("output") or {}
        hako_path = ROOT / output.get("hako_path", "")
        require(hako_path.exists(), f"artifact hako output missing: {hako_path}")
        require(
            output.get("hako_sha256") == _sha256_file(hako_path),
            f"artifact hako hash stale: {manifest_path}",
        )
        claims = manifest.get("claims") or {}
        require(claims.get("runtime_fallback") == 0, f"artifact may not claim runtime fallback: {manifest_path}")
        require(claims.get("new_backend_route") == 0, f"artifact may not claim backend route: {manifest_path}")
        contracts[capability] = {
            "capability": capability,
            "contract_kind": "VerifiedFamilyArtifactContractV1",
            "family_id": manifest.get("family_id"),
            "manifest_path": str(manifest_path.relative_to(ROOT)),
            "artifact_state": state,
        }
    return contracts


def _classify_edge(row: dict[str, Any], artifact_contracts: dict[str, dict[str, Any]]) -> dict[str, Any]:
    contract = dict(row.get("contract_reference") or {})
    capability = contract.get("capability") or row.get("required_capability")
    if contract.get("artifact_state") == "PlanOnly" and capability in artifact_contracts:
        contract = dict(artifact_contracts[capability])
    artifact_state = contract.get("artifact_state")
    classified = {
        "edge_id": row["edge_id"],
        "callsite": row["callsite"],
        "status": row["status"],
        "required_capability": row.get("required_capability"),
        "semantic_status": "Available" if row["status"] in {"Available", "ProfileExcluded"} else row["status"],
        "evidence_tier": _semantic_evidence(artifact_state, row["status"]),
        "artifact_materialization": _artifact_materialization(artifact_state),
        "route_state": _route_state(artifact_state),
    }
    if contract:
        classified["provider_reference"] = {
            "capability": contract.get("capability"),
            "contract_kind": contract.get("contract_kind"),
            "family_id": contract.get("family_id"),
            "manifest_path": contract.get("manifest_path"),
            "artifact_state": artifact_state,
        }
    elif row["status"] == "ProfileExcluded":
        classified["provider_reference"] = {
            "contract_kind": "ExecutionProfileExclusion",
            "profile_key": row.get("profile_key"),
            "profile_value": row.get("profile_value"),
        }
    elif "provider" in row:
        classified["provider_reference"] = row["provider"]
    return classified


def _first_executable_gap(edges: list[dict[str, Any]]) -> dict[str, Any]:
    for edge in edges:
        if edge["semantic_status"] != "Available":
            continue
        if edge["artifact_materialization"] != "Missing":
            continue
        if edge["evidence_tier"] != "PlanOnly":
            continue
        return {
            "edge_id": edge["edge_id"],
            "callsite": edge["callsite"],
            "required_capability": edge["required_capability"],
            "next_slice_token": _materialization_slice_for(edge["required_capability"]),
            "reason": "Earliest source-order PlanOnly edge without executable Hako artifact",
        }
    raise AssertionError("no executable materialization gap found")


def _materialization_slice_for(capability: str | None) -> str:
    if capability == "MirModuleMinimalShellTransport":
        return "MIR-MODULE-MINIMAL-SHELL-DERIVED-HAKO-ARTIFACT-001"
    if capability == "MirFunctionConstructorTransport":
        return "MIR-FUNCTION-CONSTRUCTOR-DERIVED-HAKO-ARTIFACT-001"
    if capability == "PreparedStateInstall":
        return "MIRBUILDER-PREPARED-STATE-INSTALL-DERIVED-HAKO-ARTIFACT-001"
    if capability == "LiteralIntegerLowering":
        return "MIRBUILDER-LITERAL-INTEGER-DERIVED-HAKO-ARTIFACT-001"
    return f"{capability or 'UNKNOWN'}-DERIVED-HAKO-ARTIFACT-001"


def build_report() -> dict[str, Any]:
    plan = _read_json(PLAN_PATH)
    result = _read_json(RESULT_PATH)
    require(plan["kind"] == "MinimalMirBuilderExecutionPathPlanV1", "wrong plan kind")
    require(result["kind"] == "MinimalMirBuilderFirstRedEdgeResultV1", "wrong result kind")
    first = result["first_unsupported_edge"]
    require(
        first["next_slice_token"]
        == "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001",
        "semantic closure report requires the completion design stop frontier",
    )
    reached = result["reached_prefix"]
    require(reached, "reached prefix is empty")
    require(reached[-1]["edge_id"] == "minimal_path.completion_design_stop", "missing design stop edge")
    require(reached[-1]["status"] == "Unsupported", "design stop must remain unsupported")
    selected_edges = reached[:-1]
    for edge in selected_edges:
        require(
            edge["status"] in {"Available", "ProfileExcluded"},
            f"selected source edge is not available/profile-excluded: {edge['edge_id']}",
        )
    require(result.get("not_reached_edges") == [], "semantic closure report expects no not-reached edges")

    artifact_contracts = _executable_artifact_contracts()
    edges = [_classify_edge(edge, artifact_contracts) for edge in selected_edges]
    first_gap = _first_executable_gap(edges)
    counts = {
        "evidence_tier": dict(sorted(Counter(edge["evidence_tier"] for edge in edges).items())),
        "artifact_materialization": dict(
            sorted(Counter(edge["artifact_materialization"] for edge in edges).items())
        ),
        "route_state": dict(sorted(Counter(edge["route_state"] for edge in edges).items())),
    }
    return {
        "schema_version": 0,
        "kind": "MinimalMirBuilderExecutionPathSemanticClosureReportV1",
        "completion_scope": "SelectedSourceEdgeSemanticClosure",
        "input_profile": plan["input_profile"],
        "execution_profile": plan["execution_profile"],
        "closure": {
            "all_selected_source_edges_available": True,
            "semantic_plan_closure": "Closed",
            "rust_smoke_observation": "Green",
            "generated_hako_executable_closure": "Open",
            "full_path_mainline_eligible": False,
            "source_selfhost_eligible": False,
            "artifact_selfhost_checkpoint_complete": False,
        },
        "design_stop": {
            "edge_id": first["edge_id"],
            "callsite": first["callsite"],
            "deny_detail": first["deny_detail"],
            "next_slice_token": first["next_slice_token"],
        },
        "edge_counts": counts,
        "edges": edges,
        "first_executable_materialization_gap": first_gap,
        "claims": {
            "semantic_closure_report": 1,
            "generated_hako_change": 0,
            "full_build_module_generated_hako_execution": 0,
            "full_path_mainline_selected": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "coverage_percentage_as_proof": 0,
            "bundle_size_as_proof": 0,
        },
    }


def validate_report(report: dict[str, Any]) -> None:
    require(
        report["kind"] == "MinimalMirBuilderExecutionPathSemanticClosureReportV1",
        "wrong report kind",
    )
    closure = report["closure"]
    require(closure["semantic_plan_closure"] == "Closed", "semantic closure not closed")
    require(closure["generated_hako_executable_closure"] == "Open", "executable closure claim drift")
    require(closure["full_path_mainline_eligible"] is False, "mainline eligibility claim drift")
    require(closure["source_selfhost_eligible"] is False, "source selfhost eligibility drift")
    first_gap = report["first_executable_materialization_gap"]
    require(first_gap["edge_id"] == "lower_root.literal_integer", "first materialization gap drift")
    require(
        first_gap["required_capability"] == "LiteralIntegerLowering",
        "first materialization gap capability drift",
    )
    require(
        first_gap["next_slice_token"] == "MIRBUILDER-LITERAL-INTEGER-DERIVED-HAKO-ARTIFACT-001",
        "first materialization gap next slice drift",
    )
    for edge in report["edges"]:
        require(edge["semantic_status"] in {"Available", "ProfileExcluded"}, "non-closed edge in report")
        if edge["evidence_tier"] == "PlanOnly":
            require(
                edge["artifact_materialization"] == "Missing",
                f"PlanOnly edge cannot be executable in this report: {edge['edge_id']}",
            )
        if edge["evidence_tier"] == "Observed":
            require(
                edge["artifact_materialization"] == "NotApplicable",
                f"Observed smoke cannot be executable artifact evidence: {edge['edge_id']}",
            )
            require(
                edge["route_state"] == "NotApplicable",
                f"Observed smoke cannot select a route: {edge['edge_id']}",
            )
        if edge["artifact_materialization"] == "ExecutableArtifactPresent":
            require(
                edge["route_state"] in {"DerivedShadow", "DerivedMainline"},
                f"executable artifact lacks route state: {edge['edge_id']}",
            )
    for key, value in report["claims"].items():
        if key == "semantic_closure_report":
            require(value == 1, "semantic closure report claim must be 1")
        else:
            require(value == 0, f"claim must remain 0: {key}")


def run_drift_probes(report: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("smoke treated as executable", ["edges"], None),
        ("first gap hand-edited", ["first_executable_materialization_gap", "edge_id"], "prepare_module.state_install"),
        ("full path mainline claim", ["closure", "full_path_mainline_eligible"], True),
        ("source selfhost claim", ["claims", "source_selfhost_claim"], 1),
    ]
    for label, path, value in probes:
        mutated = deepcopy(report)
        if label == "smoke treated as executable":
            for edge in mutated["edges"]:
                if edge["evidence_tier"] == "Observed":
                    edge["artifact_materialization"] = "ExecutableArtifactPresent"
                    edge["route_state"] = "DerivedMainline"
                    break
        else:
            cursor: Any = mutated
            for key in path[:-1]:
                cursor = cursor[key]
            cursor[path[-1]] = value
        try:
            validate_report(mutated)
        except AssertionError:
            continue
        raise AssertionError(f"drift probe did not fail: {label}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reference", type=Path, default=FIXTURE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    parser.add_argument("--drift-probes", action="store_true")
    args = parser.parse_args()

    report = build_report()
    validate_report(report)
    if args.drift_probes:
        run_drift_probes(report)

    return report_or_emit(
        facts=report,
        reference=args.reference,
        check_reference=args.check_reference,
        emit_json=args.emit_json,
        report=[
            ("output_contract", "rust-lifecycle-minimal-mirbuilder-semantic-closure-report-v0"),
            ("semantic_plan_closure", report["closure"]["semantic_plan_closure"]),
            ("generated_hako_executable_closure", report["closure"]["generated_hako_executable_closure"]),
            ("full_path_mainline_eligible", "0"),
            ("first_executable_materialization_gap", report["first_executable_materialization_gap"]["edge_id"]),
            ("next_slice_token", report["first_executable_materialization_gap"]["next_slice_token"]),
            ("generated_hako_change", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
