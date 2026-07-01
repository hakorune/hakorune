#!/usr/bin/env python3
"""Define the ID scalar seed evidence packet contract."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-seed-evidence-contract-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SEED-EVIDENCE-CONTRACT-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-SEED-PACKET-CANDIDATE-SELECTION-001"

READINESS_002 = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-002-v0.json"
OWNER_EDGE_REPAIR = FIXTURES / "mirbuilder-id-scalar-domain-owner-edge-repair-v0.json"
DIRECTABILITY = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"
TRANSPORT_POLICY = FIXTURES / "mirbuilder-id-scalar-domain-transport-policy-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    readiness = read_json(READINESS_002)
    previous_pool = readiness.get("candidate_pool") or {}

    owner_rows = []
    for row in readiness.get("owner_edge_readiness", []):
        owner_rows.append(
            {
                "owner_edge_id": row["owner_edge_id"],
                "owner_edge_complete": row.get("owner_edge_complete"),
                "owner_edge_confidence_set": row.get("owner_edge_confidence_set") or [],
                "nominal_id_domain_isolation": row.get("nominal_id_domain_isolation"),
                "directability_evidence_present": True,
                "source_plan_and_recipe_required": True,
                "verifier_result_fixture_required": True,
                "derived_artifact_seed_draft_input_required": True,
                "seed_packet_state": "MissingPacket",
                "selection_eligible_for_seed_materialization": False,
                "next_owner_kind": "SeedPacketMaterializationCandidate",
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSeedEvidenceContractV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "seed_readiness_resolution": rel(READINESS_002),
            "owner_edge_repair": rel(OWNER_EDGE_REPAIR),
            "directability_rerun": rel(DIRECTABILITY),
            "nominal_id_scalar_transport_policy": rel(TRANSPORT_POLICY),
        },
        "provenance": {
            "seed_readiness_resolution_hash": sha256_file(READINESS_002),
            "owner_edge_repair_hash": sha256_file(OWNER_EDGE_REPAIR),
            "directability_rerun_hash": sha256_file(DIRECTABILITY),
            "nominal_id_scalar_transport_policy_hash": sha256_file(TRANSPORT_POLICY),
        },
        "previous_state": {
            "readiness_input_owner_edge_count": previous_pool.get("readiness_input_owner_edge_count"),
            "owner_edge_repair_required_count": previous_pool.get("owner_edge_repair_required_count"),
            "seed_materialization_ready_count": previous_pool.get("seed_materialization_ready_count"),
            "missing_seed_evidence_owner_edge_count": previous_pool.get("missing_seed_evidence_owner_edge_count"),
            "previous_reason_token": (readiness.get("decision") or {}).get("reason_token"),
        },
        "contract": {
            "directability_only_is_seed_evidence": False,
            "directability_may_feed_seed_packet_generation": True,
            "seed_evidence_packet_id": "IdScalarSeedEvidencePacketV1",
            "required_packet_components": [
                "SourcePlanAndRecipe",
                "VerifierResultFixture",
                "DerivedArtifactSeedDraftInput",
            ],
            "required_invariants": [
                "NominalIdDomainPreserved",
                "NoRawI64Interchangeability",
                "NoBorrowPolicyGap",
                "NoCarrierTypeTransportGap",
                "NoRuntimeFallback",
                "NoNewBackendRoute",
                "NoNewAbi",
                "NoNewPythonSemanticProjector",
            ],
        },
        "owner_edge_contract_rows": owner_rows,
        "decision": {
            "kind": "PolicyDefined",
            "reason_token": "IdScalarSeedEvidencePacketContractDefined",
            "selected_next_card": NEXT_CARD,
            "selected_owner_edge_id": None,
        },
        "claims": {
            "seed_readiness_resolution_consumed": 1,
            "owner_edge_repair_consumed": 1,
            "directability_rerun_consumed": 1,
            "nominal_id_scalar_transport_policy_consumed": 1,
            "directability_only_is_seed_evidence": 0,
            "directability_may_feed_seed_packet_generation": 1,
            "seed_packet_contract_defined": 1,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "directable_row_count_as_proof": 0,
            "lexical_tiebreaker_as_seed_selection_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
        print("mirbuilder-id-scalar-seed-evidence-contract unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
