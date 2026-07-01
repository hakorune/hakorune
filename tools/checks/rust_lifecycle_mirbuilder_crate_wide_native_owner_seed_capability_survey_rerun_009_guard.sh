#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-009-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_009.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2015-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009.md"
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


token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009"
next_card = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-CANDIDATE-CLUSTER-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV9", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("id_scalar_domain_transport_directability_rerun", "").endswith("mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"), "input drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_id_scalar_row_count") == 31, "input count drift")
need(pool.get("directable_row_count") == 19, "directable count drift")
need(pool.get("owner_edge_repair_required_count") == 12, "repair count drift")
need(pool.get("directable_owner_edge_count") == 4, "owner edge count drift")
need(pool.get("native_seed_candidate_count") == 0, "seed candidate must remain zero")

owners = (fixture.get("summary") or {}).get("directable_owner_edge_counts") or {}
for key, value in {
    "mirbuilder::context_registry": 5,
    "mirbuilder::emission_ssa_phi": 6,
    "mirbuilder::join_i_r_plan": 7,
    "mirbuilder::join_i_r_route_verify": 1,
}.items():
    need(owners.get(key) == value, f"owner count drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectIdScalarDomainSeedCandidateClusterResolution", "decision kind drift")
need(decision.get("reason_token") == "MultipleIdScalarDirectableOwnerEdgesRequireClusterResolution", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("id_scalar_domain_transport_directability_rerun_consumed") == 1, "input consumed claim drift")
for key in [
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "manual_family_selection",
    "manual_owner_selection",
    "cluster_size_as_proof",
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
    next_card,
    "directable_owner_edge_count = 4",
    "native_seed_candidate_count = 0",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-native-owner-seed-capability-survey-rerun-009")
print("directable_row_count=19")
print("directable_owner_edge_count=4")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
