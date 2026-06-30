#!/usr/bin/env python3
"""Derive the box field initialization HakoShadow promotion decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

PARITY_RESULT_PATH = (
    FIXTURES
    / "mirbuilder-statement-value-construction-box-field-initialization-hako-shadow-result-v0.json"
)
OUTPUT_PATH = FIXTURES / "box-field-initialization-hako-shadow-promotion-decision-v0.json"
STAGE_INVENTORY_GUARD = (
    ROOT / "tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh"
)
PARITY_GUARD = (
    ROOT
    / "tools/checks/"
    / "rust_lifecycle_mirbuilder_statement_value_construction_box_field_initialization_hako_shadow_parity_guard.sh"
)

FAMILY_ID = "hakorune_mir_builder::statement_value_construction_box_field_initialization"
STAGE_ID = "box_field_initialization"
PROMOTION_TOKEN = "BoxFieldInitializationHakoShadowPromotionTokenV1"
RETIREMENT_TOKEN = "BoxFieldInitializationHakoShadowRetirementTokenV1"
NEXT_TOKEN = (
    "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-"
    "HAKO-SHADOW-PROMOTION-DECISION-001"
)


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
    require(
        parity.get("kind")
        == "MirBuilderStatementValueConstructionBoxFieldInitializationHakoShadowResultV1",
        "parity result kind drift",
    )
    result = parity.get("result") or {}
    require(result.get("err") == 0, "parity result must be green")

    shadow_record = result.get("shadow_record") or {}
    require(
        shadow_record.get("kind") == "BoxFieldInitializationShadowCandidateV1",
        "shadow record kind drift",
    )
    require(shadow_record.get("family_id") == FAMILY_ID, "shadow family drift")
    require(shadow_record.get("stage_id") == STAGE_ID, "shadow stage drift")

    source_authority = shadow_record.get("source_authority") or []
    require(len(source_authority) == 2, "shadow source authority must contain the two Rust surfaces")
    source_symbols = sorted(item.get("source_symbol") for item in source_authority)
    require(
        source_symbols == [
            "build_box_field_initializers",
            "build_new_expression_with_field_initializers",
        ],
        "shadow source authority symbol drift",
    )

    frame = shadow_record.get("mutation_frame_contract") or {}
    require(
        frame.get("delegated_mutation_owner") == "build_field_assignment_from_value",
        "delegated mutation owner drift",
    )
    require(
        frame.get("read_only_inputs")
        == ["record constructor classifier", "MirBuilder.comp_ctx.user_defined_boxes"],
        "read-only input drift",
    )
    require(
        frame.get("state_outputs")
        == [
            "dst ValueId",
            "object field assignments through build_field_assignment_from_value",
            "MirBuilder.current_function_state",
            "MirBuilder.type_ctx",
        ],
        "state output drift",
    )
    require(
        frame.get("mutation_order")
        == [
            "RejectRecordConstructorFieldInitializers",
            "CreateDestinationBox",
            "InitializeSeenFieldSet",
            "RejectDuplicateInitializerField",
            "ValidateUserDefinedBoxFieldMembership",
            "DelegateFieldAssignmentForInitializer",
            "ReturnDestinationValue",
        ],
        "mutation order drift",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderStatementValueConstructionBoxFieldInitializationHakoShadowPromotionDecisionV1",
        "output_contract": "rust-lifecycle-box-field-initialization-hako-shadow-promotion-decision-v0",
        "family_id": FAMILY_ID,
        "stage_id": STAGE_ID,
        "current_stage": "HakoShadow",
        "selected_stage": "HakoMainline",
        "decision": {
            "kind": "Promote",
            "owner_scope": "support_lane_projector",
            "reason": "BoxFieldInitialization HakoShadow parity is green and the mutation-frame stage-state tokens are explicit.",
            "reason_token": "BoxFieldInitializationHakoShadowParityGreen",
            "next_slice_token": NEXT_TOKEN,
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
        if OUTPUT_PATH.read_text(encoding="utf-8") != rendered:
            raise PromotionDecisionError("checked-in promotion decision fixture is stale")
    else:
        write_if_changed(OUTPUT_PATH, rendered)
        print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
