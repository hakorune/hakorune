#!/usr/bin/env python3
"""Resolve mainline readiness for the minimal MirBuilder path.

This is a code-facing readiness resolver, not a new semantic projector. It
consumes the semantic closure report, the composed execution continuation,
the explicit design-stop frontier resolution, the allocation-policy adoption
recheck, the current-state pointer, the task-order pointer, and the role /
adoption SSOT to derive a stable readiness decision without hand-picking a
new semantic owner.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

import tomllib

from shared_family_generator import read_json, sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

REPORT_PATH = FIXTURES / "minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"
CONTINUATION_PATH = FIXTURES / "mirbuilder-minimal-path-composed-execution-continuation-v2.json"
COMPOSED_CLOSURE_ARTIFACT_PATH = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution_closure.artifact.json"
COMPOSED_CLOSURE_VERIFIER_PATH = FIXTURES / "mirbuilder-minimal-path-composed-execution-closure-derived-hako-verifier-result-v0.json"
FRONTIER_PATH = FIXTURES / "mirbuilder-minimal-execution-path-frontier-resolution-v0.json"
ADOPTION_RECHECK_PATH = FIXTURES / "mirbuilder-allocation-policy-hako-adoption-decision-recheck-v1.json"
CURRENT_STATE_PATH = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER_PATH = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ROLE_SSOT_PATH = ROOT / "docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
DESIGN_STOP_CONTRACT_PATH = ROOT / "tools/checks/current_state_design_stop_contract.txt"
OUTPUT_PATH = FIXTURES / "mirbuilder-minimal-path-mainline-readiness-resolution-v0.json"

EXPECTED_STABLE_NEXT_SLICE_TOKEN = "MIRBUILDER-MINIMAL-PATH-COMPOSED-EXECUTION-CLOSURE-003"
EXPECTED_MAINLINE_PILOT_TOKEN = "MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001"


class MainlineReadinessError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise MainlineReadinessError(message)


def read_toml(path: Path) -> dict[str, Any]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def parse_current_state() -> dict[str, Any]:
    state = read_toml(CURRENT_STATE_PATH)
    require(bool(state.get("latest_card")), "latest_card must be present")
    require(bool(state.get("latest_card_path")), "latest_card_path must be present")
    require(state.get("latest_card") in state.get("latest_card_path", ""), "latest_card_path must reference latest_card")
    require(bool(state.get("current_blocker_token")), "current_blocker_token must be present")
    return state


def validate_task_order() -> dict[str, Any]:
    text = TASK_ORDER_PATH.read_text(encoding="utf-8")
    for needle in [
        "mainline_readiness = Ready",
        "mainline_readiness_decision = ReadyForMinimalPathMainlinePilot",
        "mainline_next_unconsumed_edge = Closed",
        "mainline_generated_hako_executable_closure = Closed",
        "mainline_same_state_handoff_observed = 1",
        "ReadyForMinimalPathMainlinePilot",
        "MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001",
        "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    ]:
        require(needle in text, f"task-order missing: {needle}")
    return {
        "path": rel(TASK_ORDER_PATH),
        "sha256": sha256_file(TASK_ORDER_PATH),
    }


def validate_role_ssot() -> dict[str, Any]:
    text = ROLE_SSOT_PATH.read_text(encoding="utf-8")
    for needle in [
        "existing Python SemanticProjector = bootstrap/oracle only",
        "new Python SemanticProjector = forbidden by default",
        "HakoAdopted artifact write by Python = forbidden",
        "PYTHON-SEMANTIC-PROJECTOR-GROWTH-FREEZE-001",
    ]:
        require(needle in text, f"role SSOT missing: {needle}")
    return {
        "path": rel(ROLE_SSOT_PATH),
        "sha256": sha256_file(ROLE_SSOT_PATH),
    }


def validate_design_stop_contract() -> dict[str, Any]:
    text = DESIGN_STOP_CONTRACT_PATH.read_text(encoding="utf-8")
    require("blocker_token_contains=DESIGN-STOP" in text, "design-stop contract must still be active")
    return {
        "path": rel(DESIGN_STOP_CONTRACT_PATH),
        "sha256": sha256_file(DESIGN_STOP_CONTRACT_PATH),
    }


def validate_semantic_closure_report() -> dict[str, Any]:
    report = read_json(REPORT_PATH)
    require(
        report.get("kind") == "MinimalMirBuilderExecutionPathSemanticClosureReportV1",
        "semantic closure report has wrong kind",
    )
    closure = report.get("closure") or {}
    require(closure.get("semantic_plan_closure") == "Closed", "semantic closure must remain closed")
    require(closure.get("rust_smoke_observation") == "Green", "rust smoke must remain green")
    require(closure.get("generated_hako_executable_closure") == "Open", "generated Hako closure must stay open")
    require(closure.get("full_path_mainline_eligible") is False, "mainline eligibility must stay false")
    require(closure.get("source_selfhost_eligible") is False, "source selfhost eligibility must stay false")
    require(closure.get("artifact_selfhost_checkpoint_complete") is False, "artifact selfhost checkpoint must stay open")

    first_gap = report.get("first_executable_materialization_gap") or {}
    require(first_gap.get("edge_id") == "minimal_path.completion_design_stop", "design-stop frontier drift")
    require(
        first_gap.get("required_capability") == "MinimalExecutionPathCompletionDesignReviewRequired",
        "design-stop capability drift",
    )
    require(
        first_gap.get("next_slice_token") == "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001",
        "design-stop next slice drift",
    )

    return {
        "path": rel(REPORT_PATH),
        "sha256": sha256_file(REPORT_PATH),
        "report": report,
        "first_gap": first_gap,
        "closure": closure,
    }


def validate_continuation() -> dict[str, Any]:
    continuation = read_json(CONTINUATION_PATH)
    require(
        continuation.get("kind") == "MinimalMirBuilderExecutionPathComposedExecutionContinuationV2",
        "continuation has wrong kind",
    )
    prefix = continuation.get("continuation") or {}
    require(prefix.get("kind") == "ContinueComposedExecutionPrefix", "continuation kind drift")
    require(prefix.get("prefix_state") == "Green", "prefix must remain green")
    require(prefix.get("first_composition_red_edge") is None, "green continuation must not claim a red edge")
    require(prefix.get("stable_reason_token") == "COMPOSED_PREFIX_REMAINS_GREEN", "stable reason drift")
    require(prefix.get("stable_next_slice_token") == "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001", "stable next slice drift")
    same_state = continuation.get("same_state_handoff") or {}
    require(same_state.get("observed") == 1, "same-state handoff must stay observed")
    require(same_state.get("selected_existing_contracts_consumed") == 1, "existing contracts must be consumed")
    require(same_state.get("fallback_to_standalone_harness") == 0, "standalone harness fallback must stay off")
    return {
        "path": rel(CONTINUATION_PATH),
        "sha256": sha256_file(CONTINUATION_PATH),
        "continuation": continuation,
        "prefix": prefix,
        "same_state": same_state,
    }


def validate_composed_execution_closure() -> dict[str, Any]:
    verifier = read_json(COMPOSED_CLOSURE_VERIFIER_PATH)
    require(
        verifier.get("kind") == "DerivedHakoArtifactVerifierResult",
        "composed execution closure has wrong verifier kind",
    )
    require(
        verifier.get("family_id") == "hakorune_mir_builder::minimal_path_composed_execution_closure",
        "composed execution closure family drift",
    )
    require(verifier.get("pilot_scope") == "MinimalMirBuilderComposedExecutionClosure_prepared_state_only", "composed execution closure scope drift")
    require(verifier.get("result") == "VerifiedHakoFamilyIR", "composed execution closure must remain verified")
    checks = verifier.get("checks") or {}
    require(checks.get("generated_hako_change") == 1, "composed execution closure must claim generated_hako_change")
    require(checks.get("generated_hako_executable_closure_closed") == 1, "composed execution closure must seal executable closure")
    require(checks.get("same_state_handoff_observed") == 1, "composed execution closure must observe same-state handoff")
    require(checks.get("selected_existing_contracts_consumed") == 1, "composed execution closure must consume existing contracts")
    require(checks.get("route_chain_closed") == 1, "composed execution closure must close route chain")
    require(checks.get("source_selfhost_claim") == 0, "composed execution closure must not claim source selfhost")
    require(checks.get("runtime_fallback") == 0, "composed execution closure must keep fallback off")
    require(checks.get("new_backend_route") == 0, "composed execution closure must keep backend route off")
    require(checks.get("new_abi") == 0, "composed execution closure must keep ABI off")
    denied_boundaries = verifier.get("denied_boundaries") or []
    require(
        denied_boundaries
        == [
            "semantic_plan_closure",
            "full_minimal_path_mainline_selected",
            "hako_adopted",
            "rust_bootstrap_retirement",
            "new_backend_route",
            "new_abi",
            "runtime_fallback",
            "source_selfhost_claim",
        ],
        "composed execution closure denied boundaries drift",
    )
    transport_notes = verifier.get("transport_notes") or {}
    require(
        transport_notes.get("generated_hako_executable_closure") == "Closed",
        "composed execution closure transport drift",
    )
    require(
        transport_notes.get("route_chain_closed") == 1,
        "composed execution closure route transport drift",
    )
    artifact = read_json(COMPOSED_CLOSURE_ARTIFACT_PATH)
    require(
        artifact.get("kind") == "RustDerivedHakoArtifact",
        "composed execution closure artifact has wrong kind",
    )
    require(
        artifact.get("family_id") == "hakorune_mir_builder::minimal_path_composed_execution_closure",
        "composed execution closure artifact family drift",
    )
    require(artifact.get("state") == "DerivedMainline", "composed execution closure artifact state drift")
    claims = artifact.get("claims") or {}
    require(claims.get("generated_hako_change") == 1, "composed execution closure artifact must claim generated_hako_change")
    require(claims.get("generated_hako_executable_closure") == 1, "composed execution closure artifact must claim executable closure")
    require(claims.get("same_state_handoff_observed") == 1, "composed execution closure artifact must observe same-state handoff")
    require(claims.get("selected_existing_contracts_consumed") == 1, "composed execution closure artifact must consume existing contracts")
    require(claims.get("route_chain_closed") == 1, "composed execution closure artifact must close route chain")
    require(claims.get("generated_hako_manual_edit") == 0, "composed execution closure artifact must keep manual edit off")
    require(claims.get("mainline_selected") == 1, "composed execution closure artifact must stay mainline selected")
    require(claims.get("runtime_fallback") == 0, "composed execution closure artifact must keep fallback off")
    require(claims.get("new_backend_route") == 0, "composed execution closure artifact must keep backend route off")
    require(claims.get("new_abi") == 0, "composed execution closure artifact must keep ABI off")
    require(claims.get("source_selfhost_claim") == 0, "composed execution closure artifact must not claim source selfhost")
    output = artifact.get("output") or {}
    hako_path = ROOT / output.get("hako_path", "")
    require(output.get("hako_path"), "composed execution closure missing hako output path")
    require(hako_path.exists(), f"composed execution closure hako output missing: {hako_path}")
    require(output.get("hako_sha256") == sha256_file(hako_path), "composed execution closure hako hash stale")
    return {
        "verifier_path": rel(COMPOSED_CLOSURE_VERIFIER_PATH),
        "verifier_sha256": sha256_file(COMPOSED_CLOSURE_VERIFIER_PATH),
        "artifact_path": rel(COMPOSED_CLOSURE_ARTIFACT_PATH),
        "artifact_sha256": sha256_file(COMPOSED_CLOSURE_ARTIFACT_PATH),
        "verifier": verifier,
        "artifact": artifact,
        "checks": checks,
        "transport_notes": transport_notes,
        "output": output,
        "claims": claims,
    }


def validate_frontier_resolution() -> dict[str, Any]:
    resolution = read_json(FRONTIER_PATH)
    require(
        resolution.get("kind") == "MinimalMirBuilderExecutionPathFrontierResolutionV1",
        "frontier resolution has wrong kind",
    )
    require(resolution.get("resolution_scope") == "DesignStopFrontier", "resolution scope drift")
    decision = resolution.get("decision") or {}
    require(decision.get("kind") == "Blocked", "frontier resolution must remain blocked")
    require(decision.get("next_slice_token") == "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001", "frontier next slice drift")
    require(decision.get("owner_scope") == "integration", "owner scope drift")
    return {
        "path": rel(FRONTIER_PATH),
        "sha256": sha256_file(FRONTIER_PATH),
        "resolution": resolution,
        "decision": decision,
    }


def validate_adoption_recheck() -> dict[str, Any]:
    adoption = read_json(ADOPTION_RECHECK_PATH)
    require(
        adoption.get("kind") == "MirBuilderAllocationPolicyHakoAdoptionDecisionV1",
        "adoption recheck has wrong kind",
    )
    require(adoption.get("decision") == "Adopt", "adoption decision must remain Adopt")
    require(adoption.get("reason_token") == "", "adoption reason token drift")
    input_evidence = adoption.get("input_evidence") or {}
    require(input_evidence.get("native_source_owner_present") == 1, "native source owner evidence must stay present")
    require(input_evidence.get("generator_overwrite_guard") == 1, "generator overwrite guard must stay present")
    return {
        "path": rel(ADOPTION_RECHECK_PATH),
        "sha256": sha256_file(ADOPTION_RECHECK_PATH),
        "adoption": adoption,
        "input_evidence": input_evidence,
    }


def classify_readiness(
    report: dict[str, Any],
    continuation: dict[str, Any],
    composed_closure: dict[str, Any],
    frontier: dict[str, Any],
    adoption: dict[str, Any],
) -> dict[str, Any]:
    closure = report["closure"]
    prefix = continuation["prefix"]
    same_state = continuation["same_state"]
    closure_checks = composed_closure["checks"]
    decision = frontier["decision"]
    input_evidence = adoption["input_evidence"]

    readiness_state = "Ready"
    decision_kind = "ReadyForMinimalPathMainlinePilot"
    reason_token = "GeneratedHakoExecutableClosureClosed"
    reason = "generated Hako executable closure is closed and the mainline pilot can proceed"
    next_slice_token = EXPECTED_MAINLINE_PILOT_TOKEN

    if closure.get("generated_hako_executable_closure") == "Open" and closure_checks.get("generated_hako_executable_closure_closed") != 1:
        readiness_state = "NotReady"
        decision_kind = "NeedExecutableClosurePatch"
        reason_token = "GeneratedHakoExecutableClosureOpen"
        reason = "generated Hako executable closure remains open"
        next_slice_token = EXPECTED_STABLE_NEXT_SLICE_TOKEN
    elif adoption["adoption"].get("decision") != "Adopt":
        readiness_state = "NotReady"
        decision_kind = "NeedAdoptionGuardPatch"
        reason_token = "AllocationPolicyAdoptionNotAdopt"
        reason = "allocation policy adoption is not yet Adopt"
        next_slice_token = "MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-003"
    elif decision.get("kind") != "Blocked":
        readiness_state = "Blocked"
        decision_kind = "Blocked"
        reason_token = "FrontierResolutionUnexpected"
        reason = "frontier resolution is no longer on the explicit design-stop blocker"
        next_slice_token = decision.get("next_slice_token") or EXPECTED_STABLE_NEXT_SLICE_TOKEN

    mainline_readiness = {
        "semantic_plan_closure": closure.get("semantic_plan_closure"),
        "composed_prefix_state": prefix.get("prefix_state"),
        "next_unconsumed_edge_classification": "Closed" if decision.get("kind") == "Blocked" else "Unknown",
        "generated_hako_executable_closure": "Closed" if closure_checks.get("generated_hako_executable_closure_closed") == 1 else closure.get("generated_hako_executable_closure"),
        "allocation_policy_adoption": adoption["adoption"].get("decision"),
        "full_path_mainline_eligible": closure.get("full_path_mainline_eligible"),
        "source_selfhost_eligible": closure.get("source_selfhost_eligible"),
        "artifact_selfhost_checkpoint_complete": closure.get("artifact_selfhost_checkpoint_complete"),
        "readiness_state": readiness_state,
    }

    return {
        "kind": "MinimalMirBuilderExecutionPathMainlineReadinessResolutionV1",
        "schema_version": 0,
        "readiness_scope": "MinimalPathMainline",
        "input_profile": {"ast": "ASTNode::Literal(Integer(0))"},
        "source_authority": {
            "semantic_closure_report": {"path": rel(REPORT_PATH), "sha256": sha256_file(REPORT_PATH)},
            "composed_execution_continuation": {"path": rel(CONTINUATION_PATH), "sha256": sha256_file(CONTINUATION_PATH)},
            "composed_execution_closure_verifier": {
                "path": rel(COMPOSED_CLOSURE_VERIFIER_PATH),
                "sha256": sha256_file(COMPOSED_CLOSURE_VERIFIER_PATH),
            },
            "composed_execution_closure_artifact": {
                "path": rel(COMPOSED_CLOSURE_ARTIFACT_PATH),
                "sha256": sha256_file(COMPOSED_CLOSURE_ARTIFACT_PATH),
            },
            "frontier_resolution": {"path": rel(FRONTIER_PATH), "sha256": sha256_file(FRONTIER_PATH)},
            "adoption_recheck": {"path": rel(ADOPTION_RECHECK_PATH), "sha256": sha256_file(ADOPTION_RECHECK_PATH)},
            "current_state": {"path": rel(CURRENT_STATE_PATH), "sha256": sha256_file(CURRENT_STATE_PATH)},
            "task_order_pointer": {"path": rel(TASK_ORDER_PATH), "sha256": sha256_file(TASK_ORDER_PATH)},
            "role_ssot": {"path": rel(ROLE_SSOT_PATH), "sha256": sha256_file(ROLE_SSOT_PATH)},
            "design_stop_contract": {"path": rel(DESIGN_STOP_CONTRACT_PATH), "sha256": sha256_file(DESIGN_STOP_CONTRACT_PATH)},
        },
        "current_state": {
            "latest_card": parse_current_state().get("latest_card"),
            "latest_card_path": parse_current_state().get("latest_card_path"),
            "current_blocker_token": parse_current_state().get("current_blocker_token"),
        },
        "composed_execution": {
            "same_state_handoff_observed": same_state.get("observed"),
            "selected_existing_contracts_consumed": same_state.get("selected_existing_contracts_consumed"),
            "fallback_to_standalone_harness": same_state.get("fallback_to_standalone_harness"),
            "generated_hako_change": same_state.get("generated_hako_change"),
        },
        "composed_execution_closure": {
            "generated_hako_executable_closure_closed": closure_checks.get("generated_hako_executable_closure_closed"),
            "route_chain_closed": closure_checks.get("route_chain_closed"),
        },
        "mainline_readiness": mainline_readiness,
        "decision": {
            "kind": decision_kind,
            "reason": reason,
            "reason_token": reason_token,
            "next_slice_token": next_slice_token,
            "owner_scope": "integration",
        },
        "selected_evidence": [
            {"kind": "semantic_closure_report", "path": rel(REPORT_PATH), "sha256": sha256_file(REPORT_PATH)},
            {"kind": "composed_execution_continuation", "path": rel(CONTINUATION_PATH), "sha256": sha256_file(CONTINUATION_PATH)},
            {
                "kind": "composed_execution_closure_verifier",
                "path": rel(COMPOSED_CLOSURE_VERIFIER_PATH),
                "sha256": sha256_file(COMPOSED_CLOSURE_VERIFIER_PATH),
            },
            {
                "kind": "composed_execution_closure_artifact",
                "path": rel(COMPOSED_CLOSURE_ARTIFACT_PATH),
                "sha256": sha256_file(COMPOSED_CLOSURE_ARTIFACT_PATH),
            },
            {"kind": "frontier_resolution", "path": rel(FRONTIER_PATH), "sha256": sha256_file(FRONTIER_PATH)},
            {"kind": "adoption_recheck", "path": rel(ADOPTION_RECHECK_PATH), "sha256": sha256_file(ADOPTION_RECHECK_PATH)},
            {"kind": "current_state", "path": rel(CURRENT_STATE_PATH), "sha256": sha256_file(CURRENT_STATE_PATH)},
            {"kind": "task_order_pointer", "path": rel(TASK_ORDER_PATH), "sha256": sha256_file(TASK_ORDER_PATH)},
        ],
        "claims": {
            "existing_evidence_consumed": 1,
            "manual_next_owner_selection": 0,
            "semantic_recipe_recopy": 0,
            "new_semantic_projection": 0,
            "same_state_handoff_observed": 1,
            "stable_next_slice_token": 1,
            "first_red_edge_if_any_is_stable": 1,
            "generated_hako_change": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "source_selfhost_claim": 0,
            "hako_adopted": 0,
            "full_minimal_path_mainline_selected": 0,
        },
        "evidence_flags": {
            "generated_hako_executable_closure": closure.get("generated_hako_executable_closure"),
            "allocation_policy_adoption": adoption["adoption"].get("decision"),
            "native_source_owner_present": input_evidence.get("native_source_owner_present"),
            "generator_overwrite_guard": input_evidence.get("generator_overwrite_guard"),
        },
    }


def build_resolution() -> dict[str, Any]:
    state = parse_current_state()
    report_info = validate_semantic_closure_report()
    continuation_info = validate_continuation()
    composed_closure_info = validate_composed_execution_closure()
    frontier_info = validate_frontier_resolution()
    adoption_info = validate_adoption_recheck()
    task_order = validate_task_order()
    role_ssot = validate_role_ssot()
    design_stop = validate_design_stop_contract()

    resolution = classify_readiness(report_info, continuation_info, composed_closure_info, frontier_info, adoption_info)
    resolution["source_authority"]["current_state"]["latest_card"] = state.get("latest_card")
    resolution["source_authority"]["current_state"]["latest_card_path"] = state.get("latest_card_path")
    resolution["source_authority"]["current_state"]["current_blocker_token"] = state.get("current_blocker_token")
    resolution["source_authority"]["task_order_pointer"] = task_order
    resolution["source_authority"]["role_ssot"] = role_ssot
    resolution["source_authority"]["design_stop_contract"] = design_stop
    return resolution


def run(check: bool) -> None:
    resolution = build_resolution()
    resolution_text = stable_json(resolution)
    if check:
        if not OUTPUT_PATH.exists() or OUTPUT_PATH.read_text(encoding="utf-8") != resolution_text:
            raise MainlineReadinessError(f"{rel(OUTPUT_PATH)} is stale")
    else:
        write_if_changed(OUTPUT_PATH, resolution_text)

    print("output_contract=rust-lifecycle-mirbuilder-minimal-path-mainline-readiness-resolution-v0")
    print("readiness_guard=green")
    print(f"decision_kind={resolution['decision']['kind']}")
    print(f"reason_token={resolution['decision']['reason_token']}")
    print(f"next_slice_token={resolution['decision']['next_slice_token']}")
    print(f"readiness_state={resolution['mainline_readiness']['readiness_state']}")
    print(f"generated_hako_executable_closure={resolution['mainline_readiness']['generated_hako_executable_closure']}")
    print(f"allocation_policy_adoption={resolution['mainline_readiness']['allocation_policy_adoption']}")
    print("manual_next_owner_selection=0")
    print("runtime_fallback=0")
    print("new_backend_route=0")
    print("new_abi=0")
    print("source_selfhost_claim=0")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        run(check=args.check)
    except MainlineReadinessError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()
