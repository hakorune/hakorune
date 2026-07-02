#!/usr/bin/env python3
"""Define Source Selfhost local candidate selection policy."""

from __future__ import annotations

import argparse
from pathlib import Path

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-local-candidate-selection-policy-v0.json"

TOKEN = "SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict:
    return {
        "schema_version": 0,
        "kind": "SourceSelfhostLocalCandidateSelectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "latest_design_stop_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011",
        },
        "policy_rule": {
            "name": "SourceSelfhostLocalCandidateSelectionPolicyV1",
            "worker_inventory_first": True,
            "external_consultation_only_for_new_authority": True,
            "local_mechanical_selection_requires_exactly_one": True,
            "zero_or_multiple_keeps_stopped": True,
            "parked_lane_must_record_reentry_condition": True,
            "no_consultation_for_counting": True,
            "two_turn_same_missing_authority_stops_micro_basis": True,
        },
        "local_candidate_selection_protocol": [
            "spawn_read_only_worker_inventory",
            "record_candidate_set",
            "record_selector_rule",
            "record_allowed_proof_axes",
            "record_forbidden_proof_axes",
            "record_proof_tuple_per_candidate",
            "record_selection_eligible_count",
            "select_locally_only_if_exactly_one",
            "keep_stopped_locally_if_zero_or_multiple",
        ],
        "worker_inventory_required_fields": [
            "candidate_set",
            "selector_rule",
            "allowed_proof_axes",
            "forbidden_proof_axes",
            "proof_tuple_per_candidate",
            "selection_eligible_count",
            "zero_or_multiple_reason",
            "reentry_condition_when_parked",
        ],
        "external_consultation_gate": {
            "ask_externally_only_if_any_true": [
                "new_proof_axis_needed",
                "existing_forbidden_axis_needs_reconsideration",
                "new_authority_source_kind_introduced",
                "source_selfhost_or_native_seed_or_hako_adopted_boundary_approached",
                "selector_rule_semantics_change",
                "local_worker_finds_fixture_card_contradiction",
            ],
            "do_not_ask_externally_for": [
                "row_count_differs",
                "cluster_count_differs",
                "candidate_labels_look_important",
                "one_option_feels_more_central",
                "historical_card_exists",
            ],
        },
        "keep_stopped_reentry_contract": {
            "required_fields": [
                "park_reason_token",
                "exact_blocking_counts",
                "forbidden_axes_held_at_zero",
                "new_evidence_that_allows_reentry",
                "selected_next_card_or_design_stop_pointer",
            ]
        },
        "two_turn_rule": {
            "if_two_consecutive_keep_stopped_same_selector_family": True,
            "same_forbidden_axes": True,
            "same_missing_authority_class": True,
            "then_do_not_open_another_micro_basis_in_same_lane": True,
            "allowed_next_actions": [
                "park_lane_and_return_wider",
                "open_explicit_authority_registry_basis",
                "keep_stopped_with_reentry_contract",
            ],
        },
        "decision": {
            "kind": "PolicyDefined",
            "reason_token": "SourceSelfhostLocalCandidateSelectionPolicyDefined",
            "selected_next_card": DESIGN_STOP,
            "selected_lane": None,
        },
        "claims": {
            "semantic_lane_selected": 0,
            "projection_policy_selected": 0,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "manual_lane_selection": 0,
            "row_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "owner_name_as_proof": 0,
            "historical_preference_as_proof": 0,
            "external_consultation_for_counting": 0,
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
        print("source-selfhost-local-candidate-selection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
