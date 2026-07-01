#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-seed-packet-candidate-selection-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_seed_packet_candidate_selection.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2021-MIRBUILDER-ID-SCALAR-SEED-PACKET-CANDIDATE-SELECTION-001.md"
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


token = "MIRBUILDER-ID-SCALAR-SEED-PACKET-CANDIDATE-SELECTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarSeedPacketCandidateSelectionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")
need((fixture.get("input_state") or {}).get("seed_evidence_contract", "").endswith("mirbuilder-id-scalar-seed-evidence-contract-v0.json"), "contract input drift")

rule = fixture.get("selection_rule") or {}
for proof in ["cluster_size", "directable_row_count", "lexical_order", "coverage_percentage", "route_membership_alone", "manual_owner_preference"]:
    need(proof in rule.get("forbidden_proofs", []), f"missing forbidden proof {proof}")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_owner_edge_count") == 10, "input owner count drift")
need(pool.get("packet_generation_candidate_count") == 10, "candidate count drift")
need(pool.get("selected_candidate_count") == 0, "selected count must be zero")
need(pool.get("ambiguous_candidate_count") == 4, "ambiguous count drift")

fixture_mapped = [row for row in fixture.get("candidate_rows", []) if row.get("owner_edge_confidence") == "FixtureMapped"]
need(len(fixture_mapped) == 4, "FixtureMapped candidate count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "MultipleEqualIdScalarSeedPacketCandidates", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("selected_owner_edge_id") is None, "owner must not be selected")

claims = fixture.get("claims") or {}
need(claims.get("seed_evidence_contract_consumed") == 1, "contract consumed drift")
for key in [
    "manual_owner_selection",
    "cluster_size_as_proof",
    "directable_row_count_as_proof",
    "lexical_tiebreaker_as_seed_selection_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "source_plan_and_recipe_materialization",
    "verifier_result_fixture_materialization",
    "derived_artifact_seed_draft_input_materialization",
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
    "packet_generation_candidate_count = 10",
    "ambiguous_candidate_count = 4",
    "MultipleEqualIdScalarSeedPacketCandidates",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-seed-packet-candidate-selection")
print("packet_generation_candidate_count=10")
print("ambiguous_candidate_count=4")
print("selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
