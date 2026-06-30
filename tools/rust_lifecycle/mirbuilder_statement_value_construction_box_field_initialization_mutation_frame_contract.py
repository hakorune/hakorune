#!/usr/bin/env python3
"""Materialize StatementValueConstruction box field initialization mutation-frame contract."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
INPUT = (
    FIXTURES
    / "mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0.json"
)
OUTPUT = (
    FIXTURES
    / "mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json"
)
TOKEN = (
    "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-"
    "MUTATION-FRAME-CONTRACT-001"
)
NEXT_CARD = (
    "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-"
    "HAKO-SHADOW-PARITY-001"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_contract() -> dict[str, Any]:
    input_fixture = read_json(INPUT)
    surfaces = input_fixture["source_surfaces"]

    return {
        "schema_version": 0,
        "kind": "MirBuilderStatementValueConstructionBoxFieldInitializationMutationFrameContractV1",
        "token": TOKEN,
        "input_state": {
            "projection_policy_fixture": rel(INPUT),
            "source_surfaces": [
                {
                    "source_id": surface["source_id"],
                    "source_symbol": surface["symbol"],
                    "source_path": surface["source_path"],
                }
                for surface in surfaces
            ],
        },
        "mutation_frame_contract": {
            "state_inputs": [
                "class",
                "arguments",
                "field_initializers",
                "MirBuilder.comp_ctx.user_defined_boxes",
                "MirBuilder.current_function_state",
                "MirBuilder.type_ctx",
            ],
            "state_outputs": [
                "dst ValueId",
                "object field assignments through build_field_assignment_from_value",
                "MirBuilder.current_function_state",
                "MirBuilder.type_ctx",
            ],
            "read_only_inputs": [
                "record constructor classifier",
                "MirBuilder.comp_ctx.user_defined_boxes",
            ],
            "local_only_state": [
                "seen initializer field set",
            ],
            "mutation_order": [
                "RejectRecordConstructorFieldInitializers",
                "CreateDestinationBox",
                "InitializeSeenFieldSet",
                "RejectDuplicateInitializerField",
                "ValidateUserDefinedBoxFieldMembership",
                "DelegateFieldAssignmentForInitializer",
                "ReturnDestinationValue",
            ],
            "delegated_mutation_owner": "build_field_assignment_from_value",
        },
        "source_order_sections": [
            {
                "source_path": "src/mir/builder/builder_build.rs",
                "markers": [
                    "!field_initializers.is_empty() && self.is_record_constructor_class(&class)",
                    "[box-init/record-reject]",
                    "let dst = self.build_new_expression(class.clone(), arguments)?;",
                    "self.build_box_field_initializers(dst, &class, field_initializers)?;\n        Ok(dst)",
                ],
            },
            {
                "source_path": "src/mir/builder/fields.rs",
                "markers": [
                    "let mut seen = std::collections::BTreeSet::new();",
                    "for (field, value) in field_initializers",
                    "if !seen.insert(field.clone())",
                    "[box-init/duplicate-field]",
                    "self.comp_ctx.user_defined_boxes.contains_key(class)",
                    "let declared = self",
                    "[box-init/unknown-field]",
                    "self.build_field_assignment_from_value(object_value, field, value)?;",
                    "Ok(())",
                ],
            },
        ],
        "decision": {
            "kind": "SelectHakoShadowParity",
            "selected_next_card": NEXT_CARD,
            "reason_token": "BoxFieldInitializationMutationFrameContractReady",
        },
        "claims": {
            "manual_family_selection": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in contract fixture.")
    args = parser.parse_args()

    output = stable_json(build_contract())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
