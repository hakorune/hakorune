#!/usr/bin/env python3
"""Explain MIR-owned FastPathPlan coverage from a MIR JSON artifact.

This is a hako_check diagnostic adapter, not an optimizer. It consumes MIR JSON
metadata that the compiler already produced and reports whether direct-memory
plan/fact surfaces are present for selected functions.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def functions(data: dict[str, Any]) -> list[dict[str, Any]]:
    rows = data.get("functions")
    if not isinstance(rows, list):
        raise SystemExit("MIR JSON missing functions[]")
    return [row for row in rows if isinstance(row, dict)]


def metadata(function: dict[str, Any]) -> dict[str, Any]:
    row = function.get("metadata")
    return row if isinstance(row, dict) else {}


def list_meta(function: dict[str, Any], key: str) -> list[dict[str, Any]]:
    value = metadata(function).get(key)
    if not isinstance(value, list):
        return []
    return [row for row in value if isinstance(row, dict)]


def function_name(function: dict[str, Any]) -> str:
    return str(function.get("name", "unknown"))


def selected_functions(
    all_functions: list[dict[str, Any]],
    method_filter: str | None,
) -> list[dict[str, Any]]:
    if method_filter is None:
        return all_functions
    selected = [function for function in all_functions if function_name(function) == method_filter]
    if not selected:
        raise SystemExit(f"selected method not found: {method_filter}")
    return selected


def count_by(rows: list[dict[str, Any]], key: str) -> Counter[str]:
    out: Counter[str] = Counter()
    for row in rows:
        out[str(row.get(key, "unknown"))] += 1
    return out


def bool_text(value: bool) -> str:
    return "1" if value else "0"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", help="Optional exact MIR function name")
    parser.add_argument("--topn", type=int, default=8)
    parser.add_argument(
        "--require-clean",
        action="store_true",
        help="Return non-zero if any FastPathObligation failed",
    )
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    all_functions = functions(load_json(args.mir_json))
    selected = selected_functions(all_functions, args.method)

    direct_plans: list[tuple[str, dict[str, Any]]] = []
    span_plans: list[tuple[str, dict[str, Any]]] = []
    regions: list[tuple[str, dict[str, Any]]] = []
    obligations: list[tuple[str, dict[str, Any]]] = []

    for function in selected:
        name = function_name(function)
        direct_plans.extend((name, row) for row in list_meta(function, "direct_array_access_plans"))
        span_plans.extend((name, row) for row in list_meta(function, "span_access_plans"))
        regions.extend((name, row) for row in list_meta(function, "required_fastpath_regions"))
        obligations.extend((name, row) for row in list_meta(function, "fastpath_obligations"))

    direct_rows = [row for _, row in direct_plans]
    span_rows = [row for _, row in span_plans]
    obligation_rows = [row for _, row in obligations]

    direct_op_counts = count_by(direct_rows, "op")
    direct_bounds_counts = count_by(direct_rows, "bounds_policy")
    direct_proof_counts = count_by(direct_rows, "proof_kind")
    span_op_counts = count_by(span_rows, "op")
    span_bounds_counts = count_by(span_rows, "bounds_policy")
    obligation_status_counts = count_by(obligation_rows, "status")
    failure_code_counts = count_by(
        [row for row in obligation_rows if str(row.get("status", "")) != "passed"],
        "failure_code",
    )

    fastpath_plan_count = len(direct_plans) + len(span_plans)
    failed_obligation_count = obligation_status_counts["failed"]
    clean = failed_obligation_count == 0

    function_plan_counts: Counter[str] = Counter()
    for name, _ in direct_plans:
        function_plan_counts[name] += 1
    for name, _ in span_plans:
        function_plan_counts[name] += 1

    lines = [
        "output_contract=hako-check-fastpath-explain-v0",
        "input_kind=mir_json",
        "tool_surface=hako_check_fastpath_explain",
        "observation_only=1",
        "rewrite_executed=0",
        f"target_method={args.method or 'all'}",
        f"function_count={len(all_functions)}",
        f"selected_function_count={len(selected)}",
        f"fastpath_plan_count={fastpath_plan_count}",
        f"direct_array_access_plan_count={len(direct_plans)}",
        f"direct_array_load_plan_count={direct_op_counts['load']}",
        f"direct_array_store_plan_count={direct_op_counts['store']}",
        f"direct_array_checked_plan_count={direct_bounds_counts['checked']}",
        f"direct_array_proved_unchecked_plan_count={direct_bounds_counts['proved_unchecked']}",
        f"direct_array_exact_front_contract_count={direct_proof_counts['exact_front_contract']}",
        f"direct_array_range_index_count={direct_proof_counts['range_index']}",
        f"direct_array_stack_top_pop_count={direct_proof_counts['stack_top_pop']}",
        f"direct_array_caller_precondition_count={direct_proof_counts['caller_precondition']}",
        f"span_access_plan_count={len(span_plans)}",
        f"span_load_plan_count={span_op_counts['load']}",
        f"span_store_plan_count={span_op_counts['store']}",
        f"span_checked_plan_count={span_bounds_counts['checked']}",
        f"span_proved_unchecked_plan_count={span_bounds_counts['proved_unchecked']}",
        f"required_fastpath_region_count={len(regions)}",
        f"fastpath_obligation_count={len(obligations)}",
        f"fastpath_obligation_passed_count={obligation_status_counts['passed']}",
        f"fastpath_obligation_failed_count={failed_obligation_count}",
        f"missing_fastpath_plan_count={failure_code_counts['DM006001']}",
        f"require_clean={bool_text(args.require_clean)}",
        f"clean={bool_text(clean)}",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]

    for idx, (name, count) in enumerate(function_plan_counts.most_common(max(0, args.topn))):
        lines.append(f"top_function_{idx}_name={name}")
        lines.append(f"top_function_{idx}_fastpath_plan_count={count}")

    for idx, (name, obligation) in enumerate(
        [(name, row) for name, row in obligations if str(row.get("status", "")) != "passed"][
            : max(0, args.topn)
        ]
    ):
        prefix = f"failed_obligation_{idx}"
        lines.extend(
            [
                f"{prefix}_function={name}",
                f"{prefix}_region_id={obligation.get('region_id', 'unknown')}",
                f"{prefix}_block=block_{obligation.get('block', 'unknown')}",
                f"{prefix}_inst_index={obligation.get('instruction_index', 'unknown')}",
                f"{prefix}_access_kind={obligation.get('access_kind', 'unknown')}",
                f"{prefix}_op={obligation.get('op', 'unknown')}",
                f"{prefix}_failure_code={obligation.get('failure_code', 'unknown')}",
                f"{prefix}_failure_reason={obligation.get('failure_reason', 'unknown')}",
            ]
        )

    lines.append("summary=ok" if clean or not args.require_clean else "summary=failed")
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")

    if args.require_clean and not clean:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
