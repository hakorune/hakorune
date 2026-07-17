#!/usr/bin/env python3
"""Build/check the CorePlan Add-result representation M0 inventory."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


FIXTURE = Path(
    "tools/checks/fixtures/coreplan_add_result_representation_i0_inventory_v1.json"
)


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        raise SystemExit(f"[coreplan-add-result-inventory] missing: {relative}")
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
            raise SystemExit(f"[coreplan-add-result-inventory] missing {label}: {anchor}")
        cursor = position


def exact_fixture_tsv_rows(root: Path, fixture: str) -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for path in sorted((root / "tools/smokes/v2/profiles").rglob("*.tsv")):
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.startswith(fixture + "\t"):
                rows.append(
                    {
                        "path": path.relative_to(root).as_posix(),
                        "row": line,
                    }
                )
    return rows


def build(root: Path) -> dict[str, object]:
    decision_path = (
        "src/mir/builder/control_flow/plan/normalizer/"
        "add_result_representation.rs"
    )
    normalizer_path = (
        "src/mir/builder/control_flow/plan/normalizer/helpers_value.rs"
    )
    direct_path = "src/mir/builder/ops/arithmetic.rs"
    pipeline_path = "src/mir/type_propagation/pipeline.rs"
    facts_path = (
        "src/mir/builder/control_flow/plan/facts/loop_array_join_facts.rs"
    )
    join_path = (
        "src/mir/builder/control_flow/plan/steps/join_payload.rs"
    )
    loop_path = (
        "src/mir/builder/control_flow/plan/parts/loop_/loop_v0.rs"
    )
    frame_path = (
        "src/mir/builder/control_flow/plan/features/coreloop_frame.rs"
    )
    runtime_path = "src/backend/mir_interpreter/helpers.rs"
    smoke_path = (
        "tools/smokes/v2/profiles/integration/joinir/"
        "loop_simple_while_stringutils_join_vm.sh"
    )

    decision = read(root, decision_path)
    normalizer = read(root, normalizer_path)
    direct = read(root, direct_path)
    pipeline = read(root, pipeline_path)
    facts = read(root, facts_path)
    join = read(root, join_path)
    loop = read(root, loop_path)
    frame = read(root, frame_path)
    runtime = read(root, runtime_path)
    smoke = read(root, smoke_path)

    require_order(
        normalizer,
        [
            "fn non_add_arithmetic_result_type(",
            "if matches!(lhs_ty, Some(MirType::Float))",
            "MirType::Float",
            "MirType::Integer",
        ],
        "non-Add normalizer decision",
    )
    require_order(
        normalizer,
        [
            "let (lhs, op, rhs, mut consts) =",
            "Self::lower_binop_ast(ast, builder, phi_bindings)?",
            "let result_type = if op == BinaryOp::Add",
            "prepare_coreplan_add_result_representation_v1(",
            "Self::non_add_arithmetic_result_type(builder, lhs, rhs)",
            "let dst = builder.alloc_typed(result_type)",
            "CoreEffectPlan::BinOp",
        ],
        "operand -> Add-only decision -> allocation -> effect timing",
    )
    require_order(
        join,
        [
            "get_type(then_val)",
            "get_type(else_val)",
            "unwrap_or(MirType::Unknown)",
            "let dst = builder.alloc_typed(ty)",
        ],
        "join destination propagation",
    )

    decision_symbol = "prepare_coreplan_add_result_representation_v1("
    symbol_rows = path_counts(root, all_rust(root), decision_symbol)
    fixture = "apps/tests/phase29ap_stringutils_join_min.hako"
    tsv_rows = exact_fixture_tsv_rows(root, fixture)

    return {
        "schema_version": 1,
        "stage": "i0_activation",
        "decision": {
            "definition_count": count(
                decision, "fn prepare_coreplan_add_result_representation_v1("
            ),
            "all_symbol_rows": symbol_rows,
            "production_consumer_count": sum(
                occurrences
                for path, occurrences in symbol_rows.items()
                if path != decision_path
            ),
            "clone_derives": count(
                decision,
                "#[derive(Debug, Clone, PartialEq, Eq)]\n"
                "pub(in crate::mir::builder) struct "
                "PreparedCorePlanAddResultRepresentationV1",
            ),
        },
        "normalizer": {
            "legacy_general_decision_definition_count": count(
                normalizer, "fn arithmetic_result_type("
            ),
            "legacy_general_decision_consumer_count": count(
                normalizer, "Self::arithmetic_result_type(builder, lhs, rhs)"
            ),
            "non_add_decision_definition_count": count(
                normalizer, "fn non_add_arithmetic_result_type("
            ),
            "non_add_decision_consumer_count": count(
                normalizer,
                "Self::non_add_arithmetic_result_type(builder, lhs, rhs)",
            ),
            "add_only_dispatch_count": count(
                normalizer, "if op == BinaryOp::Add"
            ),
            "float_else_integer_branches": count(
                normalizer, "if matches!(lhs_ty, Some(MirType::Float))"
            ),
        },
        "other_consumer_families": {
            "direct_builder_classification_sites": count(
                direct, "let lhs_type = classify_operand_type(builder, lhs)"
            ),
            "direct_builder_string_unknown_no_fact_rows": count(
                direct, "(String, Unknown) | (Unknown, String)"
            ),
            "final_pipeline_add_matrix_count": count(
                pipeline, "let new_type = match (lhs_class, rhs_class)"
            ),
            "final_pipeline_string_string_rows": count(
                pipeline, "(String, String) => Some(MirType::String)"
            ),
        },
        "loop_array_join": {
            "facts_mir_type_mentions": count(facts, "MirType"),
            "result_var_fields": count(facts, "pub result_var: String"),
            "loop_v0_init_type_reads": count(loop, "get_type(init_val)"),
            "coreloop_frame_init_type_reads": count(frame, "get_type(init_val)"),
            "join_then_else_type_precedence_rows": count(
                join, "get_type(then_val)"
            ),
        },
        "runtime_reference": {
            "string_left_concat_rows": count(runtime, "(Add, String(s), other)"),
            "string_right_concat_rows": count(runtime, "(Add, other, String(s))"),
        },
        "fixture": {
            "path": fixture,
            "smoke_mentions": smoke.count(fixture),
            "tsv_rows": tsv_rows,
            "historical_loop_simple_while_expectations": sum(
                "LoopSimpleWhile" in row["row"] for row in tsv_rows
            ),
            "post_i0_expected_result": "a,b,c",
        },
        "nonauthority": {
            "generic_loop_mentions_in_decision": count(decision, "GenericLoop"),
            "loop_array_join_mentions_in_decision": count(decision, "loop_array_join"),
            "hmi_mentions_in_decision": count(decision, "HMI"),
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
            raise SystemExit("[coreplan-add-result-inventory] reference drift")
        print("[coreplan-add-result-inventory] reference=green")
    else:
        print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
