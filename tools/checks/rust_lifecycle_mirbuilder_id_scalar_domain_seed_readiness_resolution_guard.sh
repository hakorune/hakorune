#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-domain-seed-readiness-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_domain_seed_readiness_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2017-MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-001.md"
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


token = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-001"
next_card = "MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarDomainSeedReadinessResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("id_scalar_cluster_resolution", "").endswith("mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution-v0.json"), "cluster input drift")
need(inputs.get("native_owner_seed_survey_rerun_009", "").endswith("mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-009-v0.json"), "survey input drift")
need(inputs.get("directability_rerun", "").endswith("mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"), "directability input drift")

pre = fixture.get("preconditions") or {}
need(pre.get("input_directable_owner_edge_count") == 4, "directable owner count drift")
need(pre.get("selection_eligible_cluster_count") == 4, "eligible count drift")
need(pre.get("unique_evidence_quality_tuple_count") == 1, "evidence tuple count drift")
need(pre.get("owner_edge_repair_required_count") == 12, "repair count drift")
need(pre.get("owner_edge_completeness_required_before_seed_selection") is True, "owner completeness precondition drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("readiness_input_owner_edge_count") == 4, "readiness input count drift")
need(pool.get("owner_edge_repair_required_count") == 12, "pool repair count drift")
need(pool.get("seed_materialization_ready_count") == 0, "seed ready must be zero")
need(pool.get("ambiguous_ready_count") == 0, "ambiguous ready must be zero")

rows = fixture.get("owner_edge_readiness") or []
need(len(rows) == 4, "owner readiness row count drift")
for row in rows:
    need(row.get("owner_edge_complete") is False, "owner edge must be incomplete before repair")
    need(row.get("selection_eligible_for_seed_materialization") is False, "seed materialization must be ineligible")
    need("OwnerEdgeRepairRequired" in (row.get("blocked_by") or []), "row must be blocked by owner repair")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectOwnerEdgeRepair", "decision kind drift")
need(decision.get("reason_token") == "IdScalarOwnerEdgeRepairRequiredBeforeSeedReadiness", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "id_scalar_cluster_resolution_consumed",
    "native_owner_seed_survey_rerun_009_consumed",
    "directability_rerun_consumed",
]:
    need(claims.get(key) == 1, f"input consumed claim drift: {key}")
for key in [
    "manual_owner_selection",
    "cluster_size_as_proof",
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
    next_card,
    "owner_edge_repair_required_count = 12",
    "seed_materialization_ready_count = 0",
    "IdScalarOwnerEdgeRepairRequiredBeforeSeedReadiness",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-domain-seed-readiness-resolution")
print("owner_edge_repair_required_count=12")
print("seed_materialization_ready_count=0")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
