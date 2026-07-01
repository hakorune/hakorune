#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-domain-seed-readiness-resolution-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_domain_seed_readiness_resolution_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2019-MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002.md"
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


token = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarDomainSeedReadinessResolutionV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("id_scalar_cluster_resolution", "").endswith("mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution-v0.json"), "cluster input drift")
need(inputs.get("directability_rerun", "").endswith("mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"), "directability input drift")
need(inputs.get("owner_edge_repair", "").endswith("mirbuilder-id-scalar-domain-owner-edge-repair-v0.json"), "owner repair input drift")

pre = fixture.get("preconditions") or {}
need(pre.get("input_directable_owner_edge_count") == 4, "previous directable owner count drift")
need(pre.get("previous_unique_evidence_quality_tuple_count") == 1, "previous evidence tuple drift")
need(pre.get("owner_edge_repair_unrepaired_row_count") == 0, "owner repair must be complete")
need(pre.get("owner_edge_completeness_required_before_seed_selection") is True, "owner completeness precondition drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("readiness_input_owner_edge_count") == 10, "readiness input count drift")
need(pool.get("owner_edge_repair_required_count") == 0, "repair required count drift")
need(pool.get("seed_materialization_ready_count") == 0, "seed ready count must stay zero")
need(pool.get("missing_seed_evidence_owner_edge_count") == 10, "missing seed evidence count drift")

rows = fixture.get("owner_edge_readiness") or []
need(len(rows) == 10, "owner readiness row count drift")
for row in rows:
    need(row.get("owner_edge_complete") is True, "owner edge must be complete after repair")
    need(row.get("selection_eligible_for_seed_materialization") is False, "seed materialization must remain ineligible")
    blocked = set(row.get("blocked_by") or [])
    for reason in [
        "MissingDerivedArtifactSeedDraftInput",
        "MissingVerifierResultFixture",
        "MissingSourcePlanAndRecipe",
        "DirectabilityOnlyIsNotSeedEvidence",
    ]:
        need(reason in blocked, f"row missing blocker {reason}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoIdScalarSeedMaterializationReadyOwnerEdgeAfterOwnerEdgeRepair", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("selected_owner_edge_id") is None, "owner must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "id_scalar_cluster_resolution_consumed",
    "directability_rerun_consumed",
    "owner_edge_repair_consumed",
]:
    need(claims.get(key) == 1, f"input consumed claim drift: {key}")
for key in [
    "manual_owner_selection",
    "cluster_size_as_proof",
    "directable_row_count_as_proof",
    "lexical_tiebreaker_as_seed_selection_proof",
    "coverage_percentage_as_proof",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
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
    "readiness_input_owner_edge_count = 10",
    "seed_materialization_ready_count = 0",
    "NoIdScalarSeedMaterializationReadyOwnerEdgeAfterOwnerEdgeRepair",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-domain-seed-readiness-resolution-002")
print("readiness_input_owner_edge_count=10")
print("seed_materialization_ready_count=0")
print("selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
