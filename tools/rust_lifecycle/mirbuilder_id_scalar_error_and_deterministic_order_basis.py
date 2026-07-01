#!/usr/bin/env python3
"""Define error semantics and deterministic order basis for ID scalar SourcePlan."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-error-and-deterministic-order-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001"

ID_DOMAIN = FIXTURES / "mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"
MUTATION_FRAME = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
STATE_TARGETS = FIXTURES / "mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    id_domain = read_json(ID_DOMAIN)
    mutation_frame = read_json(MUTATION_FRAME)
    state_targets = read_json(STATE_TARGETS)

    error_rows = [
        {
            "error_semantics_id": f"id_domain::{row['canonical_id_type']}::invalid_or_missing",
            "subject": row["canonical_id_type"],
            "source": "IdDomainBoundary",
            "invalid_or_missing_id_behavior": row["invalid_or_missing_id_behavior"],
            "sentinel_policy": row["sentinel_policy"],
            "reserved_id_policy": row["reserved_id_policy"],
            "diagnostic_prefix_required": False,
            "runtime_fallback": False,
        }
        for row in id_domain.get("domain_boundaries") or []
    ]

    diagnostic_targets = []
    for owner in state_targets.get("owner_edge_targets") or []:
        for target in owner.get("state_targets") or []:
            if target.get("target_kind") == "DiagnosticState":
                diagnostic_targets.append(target)
    for target in diagnostic_targets:
        error_rows.append(
            {
                "error_semantics_id": target["state_target_id"] + "::diagnostic",
                "subject": target["semantic_resource"],
                "source": "DiagnosticStateTarget",
                "invalid_or_missing_id_behavior": "NotApplicable",
                "sentinel_policy": "NotApplicable",
                "reserved_id_policy": "NotApplicable",
                "diagnostic_prefix_required": True,
                "runtime_fallback": False,
            }
        )

    order_rows = [
        {
            "order_semantics_id": frame["mutation_frame_id"] + "::order",
            "subject": frame["mutation_frame_id"],
            "source_order_preservation": True,
            "stable_iteration": False,
            "verifier_observable_order": True,
            "order_basis": frame["mutation_order"],
        }
        for frame in mutation_frame.get("mutation_frames") or []
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarErrorAndDeterministicOrderBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "id_domain_boundary_basis": rel(ID_DOMAIN),
            "state_mutation_frame_basis": rel(MUTATION_FRAME),
            "state_target_enumeration_basis": rel(STATE_TARGETS),
        },
        "provenance": {
            "id_domain_boundary_basis_hash": sha256_file(ID_DOMAIN),
            "state_mutation_frame_basis_hash": sha256_file(MUTATION_FRAME),
            "state_target_enumeration_basis_hash": sha256_file(STATE_TARGETS),
        },
        "basis_policy": {
            "error_semantics_declared": True,
            "deterministic_order_declared": True,
            "runtime_fallback": False,
            "source_order_preservation_declared": True,
            "stable_iteration_declared": True,
            "verifier_observable_order_declared": True,
        },
        "error_semantics": error_rows,
        "deterministic_order": order_rows,
        "candidate_pool": {
            "error_semantics_count": len(error_rows),
            "deterministic_order_count": len(order_rows),
            "runtime_fallback_count": 0,
            "diagnostic_prefix_required_count": len(
                [row for row in error_rows if row["diagnostic_prefix_required"]]
            ),
        },
        "decision": {
            "kind": "ErrorAndDeterministicOrderBasisDefined",
            "reason_token": "IdScalarErrorAndDeterministicOrderBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-id-scalar-error-and-deterministic-order-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
