#!/usr/bin/env python3
"""Report lexical/owner-name selector risks and guard the active selector chain."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
TOOLS = ROOT / "tools/rust_lifecycle"
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "semantic-selector-no-lexical-tiebreak-guard-v0.json"

TOKEN = "MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001"

ACTIVE_ENFORCED = [
    TOOLS / "mirbuilder_id_scalar_source_plan_and_recipe_derivability_resolution_003.py",
    TOOLS / "mirbuilder_id_scalar_typed_evidence_index_policy.py",
    TOOLS / "mirbuilder_id_scalar_operation_vocabulary_authority_split.py",
]
SELF = TOOLS / "semantic_selector_no_lexical_tiebreak_guard.py"

FORBIDDEN_NEEDLES = [
    "family_id_lexical",
    "verifier_fixture_lexical",
    "cluster_id_lexical_tiebreaker",
    "lexical_tiebreaker_allowed_for_seed_selection\": True",
    "eligible.sort(key=lambda row: row[\"priority_tuple\"])",
    "sorted(eligible, key=lambda row: row[\"priority_tuple\"])",
    "selected = eligible[0] if eligible else None",
    "selected = eligible_rows[0] if eligible_rows else None",
    "selected_candidate = eligible_tokens[0]",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def line_findings(path: Path, needles: list[str]) -> list[dict[str, Any]]:
    findings: list[dict[str, Any]] = []
    text = path.read_text(encoding="utf-8")
    for lineno, line in enumerate(text.splitlines(), start=1):
        for needle in needles:
            if needle in line:
                findings.append({"path": rel(path), "line": lineno, "pattern": needle})
    return findings


def exactly_one_guarded_selection(path: Path) -> int:
    text = path.read_text(encoding="utf-8")
    return int("len(eligible) == 1" in text and "eligible[0]" in text)


def build_fixture() -> dict[str, Any]:
    active_findings = []
    exact_one_guarded_count = 0
    for path in ACTIVE_ENFORCED:
        active_findings.extend(line_findings(path, FORBIDDEN_NEEDLES))
        exact_one_guarded_count += exactly_one_guarded_selection(path)

    historical_findings = []
    for path in sorted(TOOLS.glob("*.py")):
        if path in ACTIVE_ENFORCED or path == SELF:
            continue
        historical_findings.extend(line_findings(path, FORBIDDEN_NEEDLES))

    return {
        "schema_version": 0,
        "kind": "MirBuilderSemanticSelectorNoLexicalTiebreakGuardV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "active_enforced_files": [rel(path) for path in ACTIVE_ENFORCED],
        },
        "guard_policy": {
            "manual_owner_selection": False,
            "owner_name_as_proof": False,
            "lexical_order_as_proof": False,
            "fixture_name_as_proof": False,
            "manifest_order_as_proof": False,
            "first_eligible_selection_requires_exactly_one_guard": True,
            "historical_findings_are_not_new_authority": True,
        },
        "active_enforcement": {
            "active_file_count": len(ACTIVE_ENFORCED),
            "forbidden_active_finding_count": len(active_findings),
            "exactly_one_guarded_selection_count": exact_one_guarded_count,
            "findings": active_findings,
        },
        "historical_findings": {
            "finding_count": len(historical_findings),
            "findings": historical_findings,
            "next_cleanup_card": "MIRBUILDER-HISTORICAL-SEED-SELECTOR-QUARANTINE-001",
        },
        "decision": {
            "kind": "GuardDefined",
            "reason_token": "SemanticSelectorNoLexicalTiebreakGuardDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "manual_owner_selection": 0,
            "owner_name_as_proof": 0,
            "lexical_order_as_proof": 0,
            "fixture_name_as_proof": 0,
            "manifest_order_as_proof": 0,
            "first_eligible_without_exactly_one_guard": 0,
            "source_plan_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
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
        print("semantic-selector-no-lexical-tiebreak-guard unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
