#!/usr/bin/env python3
"""Select a narrow MIR owner for field_get result-chain cleanup."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str, default: int = 0) -> int:
    text = values.get(key)
    if text is None or text == "":
        return default
    try:
        return int(text)
    except ValueError:
        return default


def require_contract(values: dict[str, str], path: Path) -> None:
    contract = values.get("output_contract", "")
    expected = "hako-mimalloc-expression-materialization-owner-selection-v0"
    if contract != expected:
        raise SystemExit(f"{path}: expected {expected}, got {contract!r}")
    if values.get("summary") != "ok":
        raise SystemExit(f"{path}: expected summary=ok")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--expression-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    report = read_kv(args.expression_report)
    require_contract(report, args.expression_report)

    field_get_count = as_int(report, "field_get_result_chain_copy_count")
    expression_count = as_int(report, "expression_materialization_copy_count")
    selected_owner = report.get("selected_owner", "none")

    if selected_owner == "field_get_result_chain" and field_get_count > 0:
        mir_owner = "mir_builder_field_access_pin_to_slot_cleanup"
        owner_confidence = "medium"
        next_row = "field_get_result_chain_cleanup_implementation"
    else:
        mir_owner = "expression_materialization_refresh"
        owner_confidence = "low"
        next_row = "expression_materialization_reprobe"

    lines = [
        "output_contract=hako-mimalloc-field-get-result-chain-cleanup-selection-v0",
        "input_contract=hako-mimalloc-expression-materialization-owner-selection-v0",
        f"target_method={report.get('target_method', '')}",
        f"expression_materialization_copy_count={expression_count}",
        f"field_get_result_chain_copy_count={field_get_count}",
        f"selected_expression_owner={selected_owner}",
        f"selected_mir_owner={mir_owner}",
        "selected_file=src/mir/builder/fields.rs",
        "selected_function=MirBuilder::build_field_access",
        "rejected_owner=PlanLowerer::emit_effect(CoreEffectPlan::FieldGet)",
        "rejected_reason=core_effect_field_get_already_emits_selected_dst_directly",
        f"owner_confidence={owner_confidence}",
        f"next_row={next_row}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
