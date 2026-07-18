#!/usr/bin/env python3
"""Build/check the GenericLoop carrier TYPE0 producer/slot inventory."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


FIXTURE = Path("tools/checks/fixtures/generic_loop_carrier_type_m0_inventory_v1.json")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        raise SystemExit(f"[generic-loop-carrier-type-inventory] missing: {relative}")
    return path.read_text(encoding="utf-8")


def code_only(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//.*", "", text)


def count(text: str, needle: str) -> int:
    return code_only(text).count(needle)


def all_rust(root: Path) -> list[Path]:
    return sorted((root / "src").rglob("*.rs"))


def path_counts(root: Path, paths: list[Path], needle: str) -> dict[str, int]:
    rows: dict[str, int] = {}
    for path in paths:
        occurrences = count(path.read_text(encoding="utf-8"), needle)
        if occurrences:
            rows[path.relative_to(root).as_posix()] = occurrences
    return rows


def require_order(text: str, anchors: list[str], label: str) -> None:
    cursor = -1
    for anchor in anchors:
        position = text.find(anchor, cursor + 1)
        if position < 0:
            raise SystemExit(
                f"[generic-loop-carrier-type-inventory] missing {label}: {anchor}"
            )
        cursor = position


def build(root: Path) -> dict[str, object]:
    skeleton_path = "src/mir/builder/control_flow/plan/skeletons/generic_loop.rs"
    normalizer_path = "src/mir/builder/control_flow/plan/generic_loop/normalizer.rs"
    composer_path = (
        "src/mir/builder/control_flow/plan/recipe_tree/generic_loop_composer.rs"
    )
    v0_path = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v0.rs"
    v1_path = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs"
    facts_path = "src/mir/builder/control_flow/plan/generic_loop/facts_types.rs"
    decision_path = (
        "src/mir/builder/control_flow/plan/generic_loop/carrier_representation.rs"
    )
    carriers_path = (
        "src/mir/builder/control_flow/plan/features/generic_loop_body/carriers.rs"
    )
    step_path = "src/mir/builder/control_flow/plan/features/generic_loop_step.rs"
    nested_path = (
        "src/mir/builder/control_flow/plan/features/nested_loop_depth1_route.rs"
    )
    handler_path = (
        "src/mir/builder/control_flow/joinir/route_entry/registry/handlers/generic.rs"
    )

    skeleton = read(root, skeleton_path)
    normalizer = read(root, normalizer_path)
    composer = read(root, composer_path)
    v0 = read(root, v0_path)
    v1 = read(root, v1_path)
    facts = read(root, facts_path)
    decision = read(root, decision_path)
    carriers = read(root, carriers_path)
    step = read(root, step_path)
    nested = read(root, nested_path)
    handler = read(root, handler_path)

    require_order(
        skeleton,
        [
            "prepare_generic_loop_carrier_representation_v1(",
            "let blocks = LoopBlocksStandard5::allocate(builder)?",
            "let loop_var_current = builder.alloc_typed(exact_type.clone())",
            "let loop_var_next = builder.alloc_typed(exact_type)",
        ],
        "I0 skeleton timing",
    )

    rust_paths = all_rust(root)
    allocator_symbol = "alloc_generic_loop_v0_skeleton("
    allocator_rows = path_counts(root, rust_paths, allocator_symbol)
    decision_symbol = "prepare_generic_loop_carrier_representation_v1("
    decision_rows = path_counts(root, rust_paths, decision_symbol)

    return {
        "schema_version": 1,
        "stage": "i0_activation",
        "role": {
            "definition_count": count(facts, "enum GenericLoopCarrierRoleV1"),
            "facts_field_count": count(facts, "carrier_role:"),
            "v0_constructor_count": count(v0, "matches.push(GenericLoopV0Facts {"),
            "v1_extraction_constructor_count": count(
                v1, "matches.push(GenericLoopV1ExtractionV1::new("
            ),
            "v1_resolution_disposition_field_count": count(
                v1, "\n    disposition: GenericLoopV1StepDispositionV1,\n"
            ),
            "v1_successful_disposition_definition_count": count(
                facts, "enum GenericLoopV1StepDispositionV1"
            ),
            "v1_role_mapping_count": count(facts, "GenericLoopCarrierRoleV1::"),
        },
        "representation": {
            "decision_definition_count": count(
                decision, "fn prepare_generic_loop_carrier_representation_v1("
            ),
            "all_symbol_rows": decision_rows,
            "production_consumer_count": sum(
                occurrences
                for path, occurrences in decision_rows.items()
                if path != decision_path
            ),
        },
        "skeleton": {
            "definition_and_caller_rows": allocator_rows,
            "direct_caller_count": sum(allocator_rows.values()) - 1,
            "caller_contract": {
                normalizer_path: count(normalizer, allocator_symbol),
                composer_path: count(composer, allocator_symbol),
            },
            "integer_slot_allocations": count(
                skeleton, "builder.alloc_typed(MirType::Integer)"
            ),
            "init_before_block_allocation": True,
        },
        "slots": {
            "v0_next_effect_rows": count(step, "dst: skeleton.loop_var_next"),
            "v0_header_phi_rows": count(
                step, "skeleton.loop_var_next,\n        format!(\"loop_var_"
            ),
            "v1_selected_step_phi_unknown_fallbacks": count(
                carriers[: carriers.index("let mut carrier_infos")],
                ".unwrap_or(MirType::Unknown)",
            ),
            "v1_independent_carrier_unknown_fallbacks": count(
                carriers[carriers.index("let mut carrier_infos") :],
                ".unwrap_or(MirType::Unknown)",
            ),
            "v1_header_phi_rows": count(carriers, "loop_var_current,"),
        },
        "error_propagation": {
            "nested_normalize_ok_swallow_count": count(
                nested, "normalize_generic_loop_v1(builder, &facts, &ctx).ok()"
            ),
            "top_level_compose_error_swallow_count": count(
                handler, "Err(_err) if !env.strict_or_dev => return Ok(None)"
            ),
            "post_compose_verifier_fallback_count": count(
                handler, "if PlanVerifier::verify(&core_plan).is_err()"
            ),
            "post_compose_lowerer_fallback_count": count(
                handler, "Err(_) => Ok(None)"
            ),
        },
        "nonauthority": {
            "test_progression_role_imports_in_v1": count(
                v1, "facts::progression_role"
            ),
            "facts_side_mir_type_mentions": count(facts, "MirType"),
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    report = build(root)
    if args.check_reference:
        expected = json.loads(read(root, FIXTURE.as_posix()))
        if report != expected:
            print(json.dumps(report, indent=2, sort_keys=True))
            raise SystemExit(
                "[generic-loop-carrier-type-inventory] reference drift"
            )
        print("[generic-loop-carrier-type-inventory] reference=green")
    else:
        print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
