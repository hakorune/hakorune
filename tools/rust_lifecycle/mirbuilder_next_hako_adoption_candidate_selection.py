#!/usr/bin/env python3
"""Derive the next HakoAdopted candidate from the route matrix closeout."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import tomllib

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

ROUTE_MANIFEST_PATH = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
CLOSEOUT_FIXTURE_PATH = FIXTURES / "variable-context-route-matrix-closeout-v0.json"
ROADMAP_PATH = ROOT / "docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md"
CURRENT_STATE_PATH = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
OUTPUT_PATH = FIXTURES / "next-hako-adoption-candidate-selection-v0.json"

EXPECTED_CARD = "MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001"
EXPECTED_CONTRACT = "rust-lifecycle-next-hako-adoption-candidate-selection-v0"
EXCLUDED_POOL_HEADER = "Explicitly excluded from this pool:"


class CandidateSelectionError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_toml(path: Path) -> dict[str, Any]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise CandidateSelectionError(message)


def normalize_token(route_family_id: str) -> str:
    return route_family_id.rsplit("::", 1)[-1]


def extract_selected_route_tokens(route_manifest: dict[str, Any]) -> list[str]:
    rows = route_manifest.get("routes") or []
    tokens: list[str] = []
    for row in rows:
        if row.get("selected_on_mainline") is not True:
            continue
        token = normalize_token(row["family_id"])
        if token not in tokens:
            tokens.append(token)
    return tokens


def extract_excluded_tokens(roadmap_text: str) -> list[str]:
    block = roadmap_text.split(EXCLUDED_POOL_HEADER, 1)
    require(len(block) == 2, "roadmap missing excluded pool block")
    tail = block[1]
    tail = tail.split("This inventory is a consultation aid", 1)[0]
    tokens: list[str] = []
    for raw_line in tail.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("```") or line.startswith("## "):
            continue
        if raw_line.startswith("  "):
            continue
        tokens.append(line)
    require(bool(tokens), "roadmap excluded pool block produced no tokens")
    return tokens


def parse_current_state() -> dict[str, Any]:
    state = read_toml(CURRENT_STATE_PATH)
    require(state.get("latest_card") == EXPECTED_CARD, "current state latest_card must point at candidate selection")
    require(
        state.get("latest_card_path", "").endswith("1775-MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001.md"),
        "current state latest_card_path must point at candidate selection card",
    )
    require(state.get("current_blocker_token") == EXPECTED_CARD, "current state blocker token drift")
    return state


def build_result() -> dict[str, Any]:
    state = parse_current_state()
    route_manifest = read_json(ROUTE_MANIFEST_PATH)
    closeout = read_json(CLOSEOUT_FIXTURE_PATH)
    roadmap_text = ROADMAP_PATH.read_text(encoding="utf-8")

    require(route_manifest.get("kind") == "RustDerivedHakoFamilyRouteManifest", "route manifest kind drift")
    require(route_manifest.get("crate") == "hakorune_mir_builder", "route manifest crate drift")
    require(closeout.get("kind") == "VariableContextRouteMatrixCloseoutV1", "closeout fixture kind drift")
    require(closeout.get("next_action") == EXPECTED_CARD, "closeout next action drift")
    require(closeout.get("family_state") == "Parked", "closeout family state drift")
    require(closeout.get("selected_on_mainline_count") == 4, "closeout selected count drift")
    require(closeout.get("denied_route_count") == 1, "closeout denied count drift")

    selected_tokens = extract_selected_route_tokens(route_manifest)
    excluded_tokens = extract_excluded_tokens(roadmap_text)
    eligible_tokens = [token for token in selected_tokens if token not in set(excluded_tokens)]

    if eligible_tokens:
        selected_candidate = eligible_tokens[0]
        decision = "CandidateSelected"
        reason_token = "DerivedMainlineRouteCandidateAvailable"
        next_action = selected_candidate
    else:
        selected_candidate = None
        decision = "Blocked"
        reason_token = "NoEligibleDerivedMainlineRouteCandidate"
        next_action = "none"

    return {
        "schema_version": 0,
        "kind": "MirBuilderNextHakoAdoptionCandidateSelectionV1",
        "output_contract": EXPECTED_CONTRACT,
        "route_manifest": rel(ROUTE_MANIFEST_PATH),
        "route_matrix_closeout": rel(CLOSEOUT_FIXTURE_PATH),
        "roadmap": rel(ROADMAP_PATH),
        "current_state": {
            "path": rel(CURRENT_STATE_PATH),
            "latest_card": state.get("latest_card"),
            "current_blocker_token": state.get("current_blocker_token"),
        },
        "selected_mainline_route_tokens": selected_tokens,
        "excluded_route_tokens": sorted(set(excluded_tokens)),
        "eligible_route_tokens": eligible_tokens,
        "eligible_candidate_count": len(eligible_tokens),
        "candidate_pool_state": "Blocked" if not eligible_tokens else "CandidateEligible",
        "decision": {
            "kind": decision,
            "owner_scope": "integration",
            "reason": (
                "No selected DerivedMainline route row remains after applying the route-matrix closeout exclusions."
                if not eligible_tokens
                else f"Selected candidate {selected_candidate} remains after applying the route-matrix closeout exclusions."
            ),
            "reason_token": reason_token,
            "next_slice_token": EXPECTED_CARD,
        },
        "next_action": next_action,
        "claims": {
            "manual_next_owner_selection": 0,
            "support_lane_projection_as_candidate": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "hako_adopted": 0,
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
            raise CandidateSelectionError("checked-in candidate selection fixture is stale")
    else:
        write_if_changed(OUTPUT_PATH, rendered)
        print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
