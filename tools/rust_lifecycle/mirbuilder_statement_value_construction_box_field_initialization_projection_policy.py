#!/usr/bin/env python3
"""Resolve StatementValueConstruction box field initialization projection policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
DECOMPOSITION = (
    FIXTURES / "mirbuilder-statement-value-construction-subcluster-decomposition-v0.json"
)
PREVIOUS_POLICY = (
    FIXTURES
    / "mirbuilder-statement-value-construction-block-termination-predicate-projection-policy-v0.json"
)
OUTPUT = (
    FIXTURES
    / "mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0.json"
)
SUBCLUSTER_ID = "BoxFieldInitialization"
TOKEN = "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001"
NEXT_CARD = (
    "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-"
    "MUTATION-FRAME-CONTRACT-001"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def source_markers_for(symbol: str, source_text: str) -> list[str]:
    markers_by_symbol = {
        "build_new_expression_with_field_initializers": [
            "field_initializers.is_empty()",
            "self.is_record_constructor_class(&class)",
            "[box-init/record-reject]",
            "let dst = self.build_new_expression(class.clone(), arguments)?;",
            "self.build_box_field_initializers(dst, &class, field_initializers)?;",
            "Ok(dst)",
        ],
        "build_box_field_initializers": [
            "let mut seen = std::collections::BTreeSet::new();",
            "for (field, value) in field_initializers",
            "[box-init/duplicate-field]",
            "self.comp_ctx.user_defined_boxes.contains_key(class)",
            "[box-init/unknown-field]",
            "self.build_field_assignment_from_value(object_value, field, value)?;",
            "Ok(())",
        ],
    }
    return [marker for marker in markers_by_symbol[symbol] if marker in source_text]


def build_policy() -> dict[str, Any]:
    decomposition = read_json(DECOMPOSITION)
    surfaces = [
        surface
        for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SUBCLUSTER_ID
    ]
    expected_symbols = {
        "build_new_expression_with_field_initializers",
        "build_box_field_initializers",
    }
    if {surface["symbol"] for surface in surfaces} != expected_symbols:
        raise SystemExit(f"unexpected box field initialization surfaces: {surfaces}")

    source_surfaces = []
    for surface in surfaces:
        source_text = read_source(surface["source_path"])
        source_surfaces.append({
            "source_id": surface["source_id"],
            "symbol": surface["symbol"],
            "source_path": surface["source_path"],
            "params": surface["params"],
            "return_type": surface["return_type"],
            "mutation_role": (
                "new_box_with_field_initializers_entry"
                if surface["symbol"] == "build_new_expression_with_field_initializers"
                else "box_field_initializer_assignment_loop"
            ),
            "source_markers": source_markers_for(surface["symbol"], source_text),
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderStatementValueConstructionBoxFieldInitializationProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "subcluster_decomposition": rel(DECOMPOSITION),
            "previous_policy": rel(PREVIOUS_POLICY),
            "selected_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.statement_value_construction",
            "borrow_axis": "NoReturnedBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": source_surfaces,
        "selected_policy": {
            "policy": "MutationFrameContractRequired",
            "owner_edge": "mirbuilder::statement_value_construction_box_field_initialization",
            "projection_surface_selected": False,
            "reason_token": "BoxFieldInitializationMutatesObjectFieldsAndValidatesFieldSet",
        },
        "mutation_frame_evidence": {
            "record_constructor_field_initializers_rejected": True,
            "new_box_value_created_before_field_initializers": True,
            "field_initializer_loop_detected": True,
            "duplicate_field_guard_detected": True,
            "user_defined_box_field_membership_guard_detected": True,
            "field_assignment_delegation_detected": True,
            "object_field_state_mutated_by_delegate": True,
        },
        "decision": {
            "kind": "SelectMutationFrameContract",
            "selected_next_card": NEXT_CARD,
            "reason_token": "MutationFrameMustBeContractedBeforeBoxFieldProjection",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
            "hako_generation": 0,
            "hako_shadow_projector_selected": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-statement-value-construction-box-field-initialization-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
