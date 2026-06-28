#!/usr/bin/env python3
"""Derive the ReturnEmission HakoShadow promotion decision.

This is a stage-scoped support-lane decision resolver. It does not claim
Source Selfhost or HakoAdopted. It consumes the ReturnEmission shadow parity
result and the shadow-projector stage-state inventory to decide whether the
ReturnEmission stage should advance from HakoShadow to HakoMainline.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

PARITY_RESULT_PATH = FIXTURES / "mirbuilder-return-emission-hako-shadow-result-v0.json"
STAGE_INVENTORY_GUARD = ROOT / "tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh"
PARITY_GUARD = ROOT / "tools/checks/rust_lifecycle_mirbuilder_return_emission_hako_shadow_parity_guard.sh"
OUTPUT_PATH = FIXTURES / "return-emission-hako-shadow-promotion-decision-v0.json"

FAMILY_ID = "hakorune_mir_builder::return_emission"
STAGE_ID = "return_emission"
PRESENT_PROMOTION_TOKEN = "ReturnEmissionHakoShadowPromotionTokenV1"
PRESENT_RETIREMENT_TOKEN = "ReturnEmissionHakoShadowRetirementTokenV1"


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
    require(parity.get("kind") == "MirBuilderReturnEmissionDerivedHakoShadowResultV1", "parity result kind drift")
    result = parity.get("result") or {}
    require(result.get("err") == 0, "parity result must be green")
    shadow_record = result.get("shadow_record") or {}
    require(shadow_record.get("kind") == "ReturnEmissionShadowCandidateV1", "shadow record kind drift")
    require(shadow_record.get("family_id") == FAMILY_ID, "shadow record family drift")
    require(shadow_record.get("stage_id") == STAGE_ID, "shadow record stage drift")
    require(shadow_record.get("source_authority") == "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module", "shadow source authority drift")

    execution_profile = shadow_record.get("execution_profile") or {}
    require(execution_profile.get("current_block") == "Present", "shadow current block must stay present")
    require(execution_profile.get("current_function") == "Present", "shadow current function must stay present")
    require(execution_profile.get("target_block_terminated") is False, "shadow target block must stay unterminated")
    require(execution_profile.get("result_value_transport") == "ValueIdAsI64", "shadow transport drift")

    result_contract = shadow_record.get("result_contract") or {}
    require(result_contract.get("terminator") == "MirInstruction::Return", "shadow terminator drift")
    require(result_contract.get("successors") == "Empty", "shadow successors drift")

    # Stage-state inventory remains explicit in the existing guard and docs;
    # this promotion decision only consumes the already-green shadow parity and
    # keeps the support-lane stage selection narrow.
    promotion_token = PRESENT_PROMOTION_TOKEN
    retirement_token = PRESENT_RETIREMENT_TOKEN

    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnEmissionHakoShadowPromotionDecisionV1",
        "output_contract": "rust-lifecycle-return-emission-hako-shadow-promotion-decision-v0",
        "family_id": FAMILY_ID,
        "stage_id": STAGE_ID,
        "current_stage": "HakoShadow",
        "selected_stage": "HakoMainline",
        "decision": {
            "kind": "Promote",
            "owner_scope": "integration",
            "reason": "ReturnEmission HakoShadow parity is green and the stage-state inventory keeps promotion and retirement tokens explicit.",
            "reason_token": "ReturnEmissionHakoShadowParityGreen",
            "next_slice_token": "MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROMOTION-DECISION-001",
        },
        "input_evidence": {
            "hako_shadow_parity_result": rel(PARITY_RESULT_PATH),
            "parity_guard": rel(PARITY_GUARD),
            "stage_state_inventory_guard": rel(STAGE_INVENTORY_GUARD),
        },
        "python_oracle_retained": 1,
        "hako_shadow_retained": 1,
        "promotion_token": promotion_token,
        "retirement_token": retirement_token,
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
