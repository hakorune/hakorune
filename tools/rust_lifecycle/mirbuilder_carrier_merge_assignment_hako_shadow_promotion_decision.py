#!/usr/bin/env python3
"""Derive the CarrierMergeAssignment HakoShadow promotion decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

PARITY_RESULT_PATH = FIXTURES / "mirbuilder-carrier-merge-assignment-hako-shadow-result-v0.json"
STAGE_INVENTORY_GUARD = ROOT / "tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh"
PARITY_GUARD = ROOT / "tools/checks/rust_lifecycle_mirbuilder_carrier_merge_assignment_statement_hako_shadow_parity_guard.sh"
OUTPUT_PATH = FIXTURES / "carrier-merge-assignment-hako-shadow-promotion-decision-v0.json"

FAMILY_ID = "hakorune_mir_builder::carrier_merge_assignment"
STAGE_ID = "carrier_merge_assignment"
PROMOTION_TOKEN = "CarrierMergeAssignmentHakoShadowPromotionTokenV1"
RETIREMENT_TOKEN = "CarrierMergeAssignmentHakoShadowRetirementTokenV1"


class PromotionDecisionError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PromotionDecisionError(message)


def build_result() -> dict[str, Any]:
    parity = read_json(PARITY_RESULT_PATH)
    require(parity.get("kind") == "MirBuilderCarrierMergeAssignmentHakoShadowResultV1", "parity result kind drift")
    result = parity.get("result") or {}
    require(result.get("err") == 0, "parity result must be green")
    shadow_record = result.get("shadow_record") or {}
    require(shadow_record.get("kind") == "CarrierMergeAssignmentShadowCandidateV1", "shadow record kind drift")
    require(shadow_record.get("family_id") == FAMILY_ID, "shadow record family drift")
    require(shadow_record.get("stage_id") == STAGE_ID, "shadow record stage drift")
    require(shadow_record.get("source_authority") == "src/mir/builder/control_flow/plan/features/carrier_merge.rs::lower_assignment_stmt:L9", "shadow source authority drift")

    frame = shadow_record.get("mutation_frame_contract") or {}
    require(frame.get("read_only_inputs") == ["carrier_phis"], "shadow read-only inputs drift")
    require(frame.get("state_outputs") == ["current_bindings", "carrier_updates", "builder.variable_ctx.variable_map"], "shadow state outputs drift")
    require(len(frame.get("mutation_order") or []) == 6, "shadow mutation order drift")

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierMergeAssignmentHakoShadowPromotionDecisionV1",
        "output_contract": "rust-lifecycle-carrier-merge-assignment-hako-shadow-promotion-decision-v0",
        "family_id": FAMILY_ID,
        "stage_id": STAGE_ID,
        "current_stage": "HakoShadow",
        "selected_stage": "HakoMainline",
        "decision": {
            "kind": "Promote",
            "owner_scope": "integration",
            "reason": "CarrierMergeAssignment HakoShadow parity is green and mutation-frame stage-state tokens are explicit.",
            "reason_token": "CarrierMergeAssignmentHakoShadowParityGreen",
            "next_slice_token": "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PROMOTION-DECISION-001",
        },
        "input_evidence": {
            "hako_shadow_parity_result": rel(PARITY_RESULT_PATH),
            "parity_guard": rel(PARITY_GUARD),
            "stage_state_inventory_guard": rel(STAGE_INVENTORY_GUARD),
        },
        "python_oracle_retained": 1,
        "hako_shadow_retained": 1,
        "promotion_token": PROMOTION_TOKEN,
        "retirement_token": RETIREMENT_TOKEN,
        "claims": {
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "hako_adopted": 0,
            "python_semantic_projector_growth": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Validate the checked-in output fixture.")
    args = parser.parse_args()

    result = build_result()
    rendered = stable_json(result)

    if args.check:
        existing = OUTPUT_PATH.read_text(encoding="utf-8")
        if existing != rendered:
            raise PromotionDecisionError("checked-in promotion decision fixture is stale")
    else:
        write_if_changed(OUTPUT_PATH, rendered)
        print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
