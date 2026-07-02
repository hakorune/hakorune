#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-local-candidate-selection-policy-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_local_candidate_selection_policy.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2096-SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "SourceSelfhostLocalCandidateSelectionPolicyV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("latest_design_stop_card") == "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011", "latest design stop drift")

rule = fixture.get("policy_rule") or {}
for key in [
    "worker_inventory_first",
    "external_consultation_only_for_new_authority",
    "local_mechanical_selection_requires_exactly_one",
    "zero_or_multiple_keeps_stopped",
    "parked_lane_must_record_reentry_condition",
    "no_consultation_for_counting",
    "two_turn_same_missing_authority_stops_micro_basis",
]:
    need(rule.get(key) is True, f"policy flag drift: {key}")

required_worker_fields = set(fixture.get("worker_inventory_required_fields") or [])
for key in [
    "candidate_set",
    "selector_rule",
    "allowed_proof_axes",
    "forbidden_proof_axes",
    "proof_tuple_per_candidate",
    "selection_eligible_count",
    "zero_or_multiple_reason",
    "reentry_condition_when_parked",
]:
    need(key in required_worker_fields, f"worker field missing: {key}")

gate = fixture.get("external_consultation_gate") or {}
ask = set(gate.get("ask_externally_only_if_any_true") or [])
for key in [
    "new_proof_axis_needed",
    "existing_forbidden_axis_needs_reconsideration",
    "new_authority_source_kind_introduced",
    "source_selfhost_or_native_seed_or_hako_adopted_boundary_approached",
    "selector_rule_semantics_change",
    "local_worker_finds_fixture_card_contradiction",
]:
    need(key in ask, f"external ask gate missing: {key}")
no_ask = set(gate.get("do_not_ask_externally_for") or [])
for key in [
    "row_count_differs",
    "cluster_count_differs",
    "candidate_labels_look_important",
    "one_option_feels_more_central",
    "historical_card_exists",
]:
    need(key in no_ask, f"external no-ask gate missing: {key}")

reentry = set((fixture.get("keep_stopped_reentry_contract") or {}).get("required_fields") or [])
for key in [
    "park_reason_token",
    "exact_blocking_counts",
    "forbidden_axes_held_at_zero",
    "new_evidence_that_allows_reentry",
    "selected_next_card_or_design_stop_pointer",
]:
    need(key in reentry, f"reentry field missing: {key}")

two_turn = fixture.get("two_turn_rule") or {}
need(two_turn.get("then_do_not_open_another_micro_basis_in_same_lane") is True, "two-turn stop drift")
need("park_lane_and_return_wider" in (two_turn.get("allowed_next_actions") or []), "two-turn action missing")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "PolicyDefined", "decision kind drift")
need(decision.get("reason_token") == "SourceSelfhostLocalCandidateSelectionPolicyDefined", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next card drift")
need(decision.get("selected_lane") is None, "policy must not select lane")

claims = fixture.get("claims") or {}
for key in [
    "semantic_lane_selected",
    "projection_policy_selected",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "manual_lane_selection",
    "row_count_as_proof",
    "cluster_size_as_proof",
    "owner_name_as_proof",
    "historical_preference_as_proof",
    "external_consultation_for_counting",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows = {row.get("token"): row for row in manifest.get("rows") or []}
row = rows.get(token) or {}
need(row.get("card", "").endswith("2096-SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001.md"), "manifest card drift")
need(row.get("fixture", "").endswith("source-selfhost-local-candidate-selection-policy-v0.json"), "manifest fixture drift")
need(row.get("legacy_guard", "").endswith("rust_lifecycle_source_selfhost_local_candidate_selection_policy_guard.sh"), "manifest guard drift")

print("output_contract=rust-lifecycle-source-selfhost-local-candidate-selection-policy")
print("worker_inventory_first=1")
print("external_consultation_only_for_new_authority=1")
print("zero_or_multiple_keeps_stopped=1")
print("semantic_lane_selected=0")
PY
