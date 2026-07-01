#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_domain_seed_candidate_cluster_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2016-MIRBUILDER-ID-SCALAR-DOMAIN-SEED-CANDIDATE-CLUSTER-RESOLUTION-001.md"
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


token = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-CANDIDATE-CLUSTER-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarDomainSeedCandidateClusterResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

summary = fixture.get("summary") or {}
need(summary.get("input_directable_owner_edge_count") == 4, "input owner count drift")
need(summary.get("selection_eligible_cluster_count") == 4, "eligible count drift")
need(summary.get("unique_evidence_quality_tuple_count") == 1, "evidence quality tuple count drift")
need(summary.get("selected_cluster_count") == 0, "selected cluster must stay zero")

rule = fixture.get("selection_rule") or {}
need(rule.get("cluster_size_as_proof") is False, "cluster size must not be proof")
need(rule.get("lexical_tiebreaker_allowed_for_seed_selection") is False, "lexical tiebreaker must not select seed")
need(rule.get("equal_evidence_quality_keeps_stopped") is True, "equal evidence must keep stopped")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "MultipleEqualEvidenceIdScalarOwnerEdgeClusters", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("native_owner_seed_capability_survey_rerun_009_consumed") == 1, "input consumed claim drift")
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
    "input_directable_owner_edge_count = 4",
    "unique_evidence_quality_tuple_count = 1",
    "MultipleEqualEvidenceIdScalarOwnerEdgeClusters",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution")
print("input_directable_owner_edge_count=4")
print("selected_cluster_count=0")
print("reason=MultipleEqualEvidenceIdScalarOwnerEdgeClusters")
print("source_selfhost_claim=0")
print("summary=ok")
PY
