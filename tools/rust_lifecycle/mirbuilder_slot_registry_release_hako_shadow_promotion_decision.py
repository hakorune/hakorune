#!/usr/bin/env python3
"""Derive the SlotRegistryRelease HakoShadow promotion decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

VERIFIER_PATH = FIXTURES / "mirbuilder-slot-registry-release-derived-hako-verifier-result-v0.json"
STAGE_INVENTORY_GUARD = ROOT / "tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh"
DERIVED_ARTIFACT_GUARD = ROOT / "tools/checks/rust_lifecycle_mirbuilder_slot_registry_release_derived_artifact_guard.sh"
OUTPUT_PATH = FIXTURES / "slot-registry-release-hako-shadow-promotion-decision-v0.json"

FAMILY_ID = "hakorune_mir_builder::slot_registry_release"
STAGE_ID = "slot_registry_release"
PRESENT_PROMOTION_TOKEN = "SlotRegistryReleaseHakoShadowPromotionTokenV1"
PRESENT_RETIREMENT_TOKEN = "SlotRegistryReleaseHakoShadowRetirementTokenV1"


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
    verifier = read_json(VERIFIER_PATH)
    require(verifier.get("kind") == "DerivedHakoArtifactVerifierResult", "verifier result kind drift")
    require(verifier.get("result") == "VerifiedHakoFamilyIR", "verifier result must be verified")
    require(verifier.get("family_id") == FAMILY_ID, "verifier family drift")
    require(verifier.get("pilot_scope") == "SlotRegistryRelease_prepared_slot_registry_only", "verifier scope drift")

    checks = verifier.get("checks") or {}
    required_checks = {
        "slot_registry_release_only": 1,
        "current_slot_registry_cleared": 1,
        "released_registry_present": 1,
        "slot_registry_released": 1,
        "host_env_lookup": 0,
        "module_metadata_publication": 0,
        "metadata_publication": 0,
        "semantic_refresh": 0,
        "all_functions_phi_materialization": 0,
        "full_finalize_module": 0,
        "runtime_fallback": 0,
    }
    for key, expected in required_checks.items():
        if checks.get(key) != expected:
            raise PromotionDecisionError(f"verifier check drift: {key}={checks.get(key)}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderSlotRegistryReleaseHakoShadowPromotionDecisionV1",
        "output_contract": "rust-lifecycle-slot-registry-release-hako-shadow-promotion-decision-v0",
        "family_id": FAMILY_ID,
        "stage_id": STAGE_ID,
        "current_stage": "HakoShadow",
        "selected_stage": "HakoMainline",
        "decision": {
            "kind": "Promote",
            "owner_scope": "integration",
            "reason": "SlotRegistryRelease HakoShadow parity is green and the stage-state inventory keeps promotion and retirement tokens explicit.",
            "reason_token": "SlotRegistryReleaseHakoShadowParityGreen",
            "next_slice_token": "MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-SHADOW-PROMOTION-DECISION-001",
        },
        "input_evidence": {
            "derived_hako_verifier_result": rel(VERIFIER_PATH),
            "derived_artifact_guard": rel(DERIVED_ARTIFACT_GUARD),
            "stage_state_inventory_guard": rel(STAGE_INVENTORY_GUARD),
        },
        "python_oracle_retained": 1,
        "hako_shadow_retained": 1,
        "promotion_token": PRESENT_PROMOTION_TOKEN,
        "retirement_token": PRESENT_RETIREMENT_TOKEN,
        "claims": {
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "host_env_lookup": 0,
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
