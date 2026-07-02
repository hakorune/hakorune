#!/usr/bin/env python3
"""Split ID scalar operation vocabulary authority from diagnostic suggestions."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-authority-split-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001"

TYPED_INDEX = FIXTURES / "mirbuilder-id-scalar-typed-evidence-index-policy-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"
DERIVABILITY = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    typed_index = read_json(TYPED_INDEX)
    operations = read_json(OPERATIONS)
    derivability = read_json(DERIVABILITY)

    tied_owners = {
        row["owner_edge_id"]
        for row in derivability.get("candidates") or []
        if row.get("selection_eligible")
    }

    owner_rows = []
    total_counts: Counter[str] = Counter()
    tied_semantic_complete_count = 0
    for candidate in operations.get("candidates") or []:
        owner = candidate["owner_edge_id"]
        counts = Counter(row.get("classification_authority") for row in candidate.get("operation_rows") or [])
        total_counts.update(counts)
        semantic_count = counts.get("RoleMapped", 0)
        diagnostic_count = counts.get("SymbolReturnTypeMapped", 0)
        unknown_count = counts.get("Unknown", 0)
        semantic_complete = semantic_count > 0 and diagnostic_count == 0 and unknown_count == 0
        if owner in tied_owners and semantic_complete:
            tied_semantic_complete_count += 1
        owner_rows.append(
            {
                "owner_edge_id": owner,
                "is_tied_derivable_owner": owner in tied_owners,
                "semantic_role_mapped_count": semantic_count,
                "diagnostic_suggestion_count": diagnostic_count,
                "unknown_operation_count": unknown_count,
                "semantic_operation_authority_complete": semantic_complete,
                "selection_eligible": False,
                "blocked_by": [] if semantic_complete else ["OperationVocabularyHasDiagnosticOnlyRows"],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarOperationVocabularyAuthoritySplitV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "typed_evidence_index_policy": rel(TYPED_INDEX),
            "operation_vocabulary_inventory": rel(OPERATIONS),
            "derivability_rerun_003": rel(DERIVABILITY),
        },
        "provenance": {
            "typed_evidence_index_policy_hash": sha256_file(TYPED_INDEX),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
            "derivability_rerun_003_hash": sha256_file(DERIVABILITY),
        },
        "authority_policy": {
            "semantic_operation_authority": "FixtureDeclaredRoleMapped",
            "diagnostic_operation_suggestion": "SymbolReturnTypeMapped",
            "symbol_return_type_fallback_is_semantic_authority": False,
            "diagnostic_suggestion_may_select_source_plan_owner": False,
            "fixture_declared_role_required_for_semantic_operation_mapping": True,
            "operation_name_fallback_is_diagnostic_only": True,
            "source_plan_materialization": False,
        },
        "owner_rows": owner_rows,
        "candidate_pool": {
            "input_owner_count": len(owner_rows),
            "tied_derivable_owner_count": len(tied_owners),
            "semantic_role_mapped_operation_count": total_counts.get("RoleMapped", 0),
            "diagnostic_suggestion_operation_count": total_counts.get("SymbolReturnTypeMapped", 0),
            "unknown_operation_count": total_counts.get("Unknown", 0),
            "tied_semantic_authority_complete_owner_count": tied_semantic_complete_count,
            "selection_eligible_count": 0,
        },
        "decision": {
            "kind": "PolicyDefined",
            "reason_token": "IdScalarOperationVocabularyAuthoritySplitDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "symbol_return_type_fallback_as_semantic_authority": 0,
            "diagnostic_suggestion_as_source_plan_selection_proof": 0,
            "manual_owner_selection": 0,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
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
        print("mirbuilder-id-scalar-operation-vocabulary-authority-split unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
