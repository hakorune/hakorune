#!/usr/bin/env python3
"""Select a native seed candidate from strict converter emission evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json"

BRIDGE = FIXTURES / "mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"
STRICT_PROBE = FIXTURES / "mirbuilder-strict-converter-emission-probe-v0.json"
FAMILY_MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
TOKEN = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001"

GAP_TOKENS = [
    "borrow",
    "carrier",
    "unsupportedtypetransport",
    "unsupportedkeytransport",
    "unsafeorffi",
    "nontrivialdrop",
    "runtime_fallback",
    "runtime fallback",
    "fallback",
    "new_abi",
    "abi_changed",
    "new_backend",
    "backend_route_changed",
    "unstructuredcontrolflow",
    "loopcarriedstaterequired",
    "phijoinrequired",
]

COMPOSITE_TOKENS = [
    "minimal_path",
    "all_functions",
    "pipeline",
    "composition",
    "publication",
    "finalize",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def slug_family_id(family_id: str) -> str:
    tail = family_id.split("::")[-1]
    out: list[str] = []
    prev_lower = False
    for ch in tail:
        if ch.isupper() and prev_lower:
            out.append("-")
        if ch in "_:.":
            out.append("-")
            prev_lower = False
        elif ch.isalnum():
            out.append(ch.lower())
            prev_lower = ch.islower() or ch.isdigit()
        else:
            out.append("-")
            prev_lower = False
    slug = "-".join(part for part in "".join(out).split("-") if part)
    return slug or "unknown-owner"


def seed_card_for(family_id: str) -> str:
    return f"MIRBUILDER-{slug_family_id(family_id).upper().replace('-', '-')}-HAKO-NATIVE-SOURCE-SEED-001"


def manifest_rows_until(token: str) -> list[dict[str, Any]]:
    manifest = read_json(FAMILY_MANIFEST)
    rows: list[dict[str, Any]] = []
    for row in manifest.get("rows", []):
        if row.get("token") == token:
            break
        rows.append(row)
    return rows


def adopted_family_ids(cutoff_token: str = TOKEN) -> set[str]:
    ids: set[str] = set()
    for row in manifest_rows_until(cutoff_token):
        fixture_path = row.get("fixture") or ""
        if "adoption" not in fixture_path:
            continue
        path = ROOT / fixture_path
        try:
            data = read_json(path)
        except (json.JSONDecodeError, FileNotFoundError):
            continue
        decision = data.get("decision")
        value = decision.get("value") if isinstance(decision, dict) else decision
        if value != "Adopt":
            continue
        family_id = data.get("family_id") or (data.get("selected_surface") or {}).get("family_id")
        if family_id:
            ids.add(family_id)
    return ids


def unscoped_adoption_slugs(cutoff_token: str = TOKEN) -> set[str]:
    slugs: set[str] = set()
    del cutoff_token
    for path in FIXTURES.glob("*adoption*decision-v0.json"):
        try:
            data = read_json(path)
        except (json.JSONDecodeError, FileNotFoundError):
            continue
        decision = data.get("decision")
        value = decision.get("value") if isinstance(decision, dict) else decision
        if value != "Adopt":
            continue
        if data.get("family_id") or (data.get("selected_surface") or {}).get("family_id"):
            continue
        name = path.name.removesuffix("-adoption-decision-v0.json")
        slugs.add(name)
    return slugs


def plan_recipe_present(verifier: dict[str, Any]) -> tuple[bool, str | None, str | None]:
    source_plan = verifier.get("source_plan") or verifier.get("source_facts")
    source_recipe = verifier.get("source_recipe")
    if not source_plan or not source_recipe:
        return False, source_plan, source_recipe
    return (FIXTURES / source_plan).exists() and (FIXTURES / source_recipe).exists(), source_plan, source_recipe


def has_gap(denied_boundaries: list[Any]) -> bool:
    text = " ".join(str(item) for item in denied_boundaries).lower().replace(" ", "")
    spaced = " ".join(str(item) for item in denied_boundaries).lower()
    return any(token in text or token in spaced for token in GAP_TOKENS)


def is_composite_or_integration(family_id: str) -> bool:
    return any(token in family_id for token in COMPOSITE_TOKENS)


def bridge_state_for(
    row: dict[str, str],
    adopted: set[str],
    unscoped_adopted_slugs: set[str],
) -> dict[str, Any]:
    verifier_path = ROOT / row["fixture"]
    verifier = read_json(verifier_path)
    family_id = row["family_id"]
    deterministic, source_plan, source_recipe = plan_recipe_present(verifier)
    family_slug = slug_family_id(family_id)
    denied_boundaries = verifier.get("denied_boundaries") or []
    owner_confidence = "FixtureMapped" if family_id and source_plan and source_recipe else "None"

    blockers: list[str] = []
    if not deterministic:
        blockers.append("MissingDeterministicRegeneration")
    if owner_confidence not in ["ExactSymbol", "FixtureMapped"]:
        blockers.append("MissingOwnerEdgeConfidence")
    if family_id in adopted:
        blockers.append("AlreadyHakoAdopted")
    if any(family_slug in slug or slug in family_slug for slug in unscoped_adopted_slugs):
        blockers.append("AlreadyCoveredByUnscopedAdoptionDecision")
    if is_composite_or_integration(family_id):
        blockers.append("CompositeOrIntegrationOwner")
    if has_gap(denied_boundaries):
        blockers.append("PolicyGapInDeniedBoundaries")

    bridge_state = "BridgeEligible" if not blockers else "BridgeBlocked"
    priority_tuple = [
        0 if owner_confidence == "ExactSymbol" else 1,
        0,
        0,
        0,
        0,
        0 if not is_composite_or_integration(family_id) else 1,
        family_id,
        Path(row["fixture"]).name,
    ]

    return {
        "owner_edge_id": family_id,
        "family_id": family_id,
        "verifier_result_fixture": row["fixture"],
        "owner_edge_confidence": owner_confidence,
        "deterministic_regeneration": deterministic,
        "source_plan": source_plan,
        "source_recipe": source_recipe,
        "provenance_manifest_present": bool(source_plan and source_recipe),
        "borrow_policy_gap": any("borrow" in str(item).lower() for item in denied_boundaries),
        "carrier_type_transport_gap": has_gap(denied_boundaries),
        "verifier_or_oracle_or_guard_present": row.get("result") == "VerifiedHakoFamilyIR",
        "composite_owner": is_composite_or_integration(family_id),
        "already_hako_adopted": family_id in adopted,
        "bridge_state": bridge_state,
        "blocked_by": blockers,
        "priority_tuple": priority_tuple,
        "next_card": seed_card_for(family_id) if bridge_state == "BridgeEligible" else None,
    }


def build_fixture() -> dict[str, Any]:
    bridge = read_json(BRIDGE)
    probe = read_json(STRICT_PROBE)
    adopted = adopted_family_ids()
    unscoped = unscoped_adoption_slugs()

    candidates = [
        bridge_state_for(row, adopted, unscoped)
        for row in probe["verified_hako_family_ir_fixtures"]
    ]
    eligible = [row for row in candidates if row["bridge_state"] == "BridgeEligible"]
    eligible.sort(key=lambda row: row["priority_tuple"])
    selected = eligible[0] if eligible else None

    if selected:
        decision = {
            "kind": "SelectNativeSeedCandidate",
            "selected_owner_edge_id": selected["owner_edge_id"],
            "selected_next_card": selected["next_card"],
            "reason_token": "StrictEmissionBridgeEligibleCandidateSelected",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoBridgeEligibleStrictEmissionNativeSeedCandidate",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "bridge_policy": rel(BRIDGE),
            "strict_converter_emission_probe": rel(STRICT_PROBE),
        },
        "provenance": {
            "bridge_policy_hash": sha256_file(BRIDGE),
            "strict_converter_emission_probe_hash": sha256_file(STRICT_PROBE),
            "source_selfhost_family_guard_manifest": rel(FAMILY_MANIFEST),
            "adoption_evidence_cutoff_token": TOKEN,
        },
        "selection_rule": {
            "manual_family_selection": False,
            "stable_priority": [
                "owner_edge_confidence",
                "verifier_or_oracle_or_guard_present",
                "no_policy_gap",
                "deterministic_regeneration",
                "provenance_manifest_present",
                "not_composite_or_integration_owner",
                "family_id_lexical",
                "verifier_fixture_lexical",
            ],
            "cluster_size_as_proof": False,
            "coverage_percentage_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "candidate_pool": {
            "verified_hako_family_ir_count": len(candidates),
            "bridge_eligible_count": len(eligible),
            "bridge_blocked_count": len(candidates) - len(eligible),
            "already_adopted_count": sum("AlreadyHakoAdopted" in row["blocked_by"] for row in candidates),
            "unscoped_adoption_blocked_count": sum("AlreadyCoveredByUnscopedAdoptionDecision" in row["blocked_by"] for row in candidates),
            "composite_or_integration_owner_count": sum("CompositeOrIntegrationOwner" in row["blocked_by"] for row in candidates),
            "gap_blocked_count": sum("PolicyGapInDeniedBoundaries" in row["blocked_by"] for row in candidates),
        },
        "candidates": candidates,
        "decision": decision,
        "claims": {
            "bridge_policy_consumed": 1,
            "strict_converter_emission_probe_consumed": 1,
            "manual_family_selection": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in candidate selection fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-strict-converter-emission-native-seed-candidate-selection unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
