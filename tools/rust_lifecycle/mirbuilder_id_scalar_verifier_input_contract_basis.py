#!/usr/bin/env python3
"""Define verifier input contract for ID scalar SourcePlan evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-verifier-input-contract-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003"

EFFECT = FIXTURES / "mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"
MUTATION = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
ID_DOMAIN = FIXTURES / "mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"
ERROR_ORDER = FIXTURES / "mirbuilder-id-scalar-error-and-deterministic-order-basis-v0.json"
FILE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    effect = read_json(EFFECT)
    mutation = read_json(MUTATION)
    id_domain = read_json(ID_DOMAIN)
    error_order = read_json(ERROR_ORDER)
    file_boundary = read_json(FILE_BOUNDARY)

    input_facts = [
        {
            "fact_set": "EffectCoverageRows",
            "source_fixture": rel(EFFECT),
            "row_count": (effect.get("candidate_pool") or {}).get("effect_row_count"),
            "verifier_obligation": "Every bounded owner operation token maps to exactly one effect class",
        },
        {
            "fact_set": "MutationFrameRows",
            "source_fixture": rel(MUTATION),
            "row_count": (mutation.get("candidate_pool") or {}).get("mutation_frame_count"),
            "verifier_obligation": "Every mutating effect references a declared mutation frame",
        },
        {
            "fact_set": "IdDomainBoundaryRows",
            "source_fixture": rel(ID_DOMAIN),
            "row_count": (id_domain.get("candidate_pool") or {}).get("id_domain_boundary_count"),
            "verifier_obligation": "Nominal ID domains are not raw-i64 interchangeable",
        },
        {
            "fact_set": "ErrorSemanticsRows",
            "source_fixture": rel(ERROR_ORDER),
            "row_count": (error_order.get("candidate_pool") or {}).get("error_semantics_count"),
            "verifier_obligation": "Invalid/missing IDs and diagnostics have declared semantics",
        },
        {
            "fact_set": "DeterministicOrderRows",
            "source_fixture": rel(ERROR_ORDER),
            "row_count": (error_order.get("candidate_pool") or {}).get("deterministic_order_count"),
            "verifier_obligation": "Verifier-visible mutation order is declared",
        },
        {
            "fact_set": "NativeSeedFileBoundaryRows",
            "source_fixture": rel(FILE_BOUNDARY),
            "row_count": (file_boundary.get("candidate_pool") or {}).get(
                "native_seed_file_boundary_derivable_count"
            ),
            "verifier_obligation": "Verifier subject is a bounded owner module seed boundary",
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarVerifierInputContractBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "behavior_recipe_effect_coverage_basis": rel(EFFECT),
            "state_mutation_frame_basis": rel(MUTATION),
            "id_domain_boundary_basis": rel(ID_DOMAIN),
            "error_and_deterministic_order_basis": rel(ERROR_ORDER),
            "native_seed_file_boundary_basis": rel(FILE_BOUNDARY),
        },
        "provenance": {
            "behavior_recipe_effect_coverage_basis_hash": sha256_file(EFFECT),
            "state_mutation_frame_basis_hash": sha256_file(MUTATION),
            "id_domain_boundary_basis_hash": sha256_file(ID_DOMAIN),
            "error_and_deterministic_order_basis_hash": sha256_file(ERROR_ORDER),
            "native_seed_file_boundary_basis_hash": sha256_file(FILE_BOUNDARY),
        },
        "contract_policy": {
            "verifier_input_contract_declared": True,
            "verifier_result_materialization": False,
            "source_plan_materialization": False,
            "behavior_recipe_materialization": False,
        },
        "input_fact_sets": input_facts,
        "candidate_pool": {
            "input_fact_set_count": len(input_facts),
            "effect_row_count": input_facts[0]["row_count"],
            "mutation_frame_count": input_facts[1]["row_count"],
            "id_domain_boundary_count": input_facts[2]["row_count"],
            "native_seed_file_boundary_count": input_facts[5]["row_count"],
        },
        "decision": {
            "kind": "VerifierInputContractBasisDefined",
            "reason_token": "IdScalarVerifierInputContractBasisDefined",
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
        print("mirbuilder-id-scalar-verifier-input-contract-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
