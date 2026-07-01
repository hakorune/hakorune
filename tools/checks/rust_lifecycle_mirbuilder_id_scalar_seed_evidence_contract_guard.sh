#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-seed-evidence-contract-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_seed_evidence_contract.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2020-MIRBUILDER-ID-SCALAR-SEED-EVIDENCE-CONTRACT-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-ID-SCALAR-SEED-EVIDENCE-CONTRACT-001"
next_card = "MIRBUILDER-ID-SCALAR-SEED-PACKET-CANDIDATE-SELECTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarSeedEvidenceContractV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("seed_readiness_resolution", "").endswith("mirbuilder-id-scalar-domain-seed-readiness-resolution-002-v0.json"), "readiness input drift")
need(inputs.get("owner_edge_repair", "").endswith("mirbuilder-id-scalar-domain-owner-edge-repair-v0.json"), "owner repair input drift")
need(inputs.get("directability_rerun", "").endswith("mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"), "directability input drift")
need(inputs.get("nominal_id_scalar_transport_policy", "").endswith("mirbuilder-id-scalar-domain-transport-policy-v0.json"), "transport policy input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("readiness_input_owner_edge_count") == 10, "readiness count drift")
need(previous.get("owner_edge_repair_required_count") == 0, "repair required drift")
need(previous.get("seed_materialization_ready_count") == 0, "seed ready drift")
need(previous.get("missing_seed_evidence_owner_edge_count") == 10, "missing seed evidence drift")
need(previous.get("previous_reason_token") == "NoIdScalarSeedMaterializationReadyOwnerEdgeAfterOwnerEdgeRepair", "previous reason drift")

contract = fixture.get("contract") or {}
need(contract.get("directability_only_is_seed_evidence") is False, "directability-only must not be seed evidence")
need(contract.get("directability_may_feed_seed_packet_generation") is True, "directability must feed packet generation")
need(contract.get("seed_evidence_packet_id") == "IdScalarSeedEvidencePacketV1", "packet id drift")
need(contract.get("required_packet_components") == [
    "SourcePlanAndRecipe",
    "VerifierResultFixture",
    "DerivedArtifactSeedDraftInput",
], "required packet components drift")
for invariant in [
    "NominalIdDomainPreserved",
    "NoRawI64Interchangeability",
    "NoBorrowPolicyGap",
    "NoCarrierTypeTransportGap",
    "NoRuntimeFallback",
    "NoNewBackendRoute",
    "NoNewAbi",
    "NoNewPythonSemanticProjector",
]:
    need(invariant in contract.get("required_invariants", []), f"missing invariant {invariant}")

rows = fixture.get("owner_edge_contract_rows") or []
need(len(rows) == 10, "owner edge contract row count drift")
for row in rows:
    need(row.get("owner_edge_complete") is True, "owner edge must be complete")
    need(row.get("nominal_id_domain_isolation") == "Preserved", "nominal domain must be preserved")
    need(row.get("directability_evidence_present") is True, "directability evidence must be present")
    need(row.get("seed_packet_state") == "MissingPacket", "seed packet must be missing")
    need(row.get("selection_eligible_for_seed_materialization") is False, "must not be seed materialization eligible")
    need(row.get("next_owner_kind") == "SeedPacketMaterializationCandidate", "next owner kind drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "PolicyDefined", "decision kind drift")
need(decision.get("reason_token") == "IdScalarSeedEvidencePacketContractDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")
need(decision.get("selected_owner_edge_id") is None, "owner must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "seed_readiness_resolution_consumed",
    "owner_edge_repair_consumed",
    "directability_rerun_consumed",
    "nominal_id_scalar_transport_policy_consumed",
    "directability_may_feed_seed_packet_generation",
    "seed_packet_contract_defined",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "directability_only_is_seed_evidence",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "manual_owner_selection",
    "cluster_size_as_proof",
    "directable_row_count_as_proof",
    "lexical_tiebreaker_as_seed_selection_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    "contract_id = IdScalarSeedEvidencePacketV1",
    "directability_only_is_seed_evidence = 0",
    next_card,
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-seed-evidence-contract")
print("contract_id=IdScalarSeedEvidencePacketV1")
print("directability_only_is_seed_evidence=0")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
