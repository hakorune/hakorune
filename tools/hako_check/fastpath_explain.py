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


def field_text(row: dict[str, Any], key: str, default: str = "unknown") -> str:
    value = row.get(key)
    if value is None:
        return default
    return str(value)


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
    hotcore_summaries: list[tuple[str, dict[str, Any]]] = []
    hotcore_call_plans: list[tuple[str, dict[str, Any]]] = []

    for function in selected:
        name = function_name(function)
        direct_plans.extend((name, row) for row in list_meta(function, "direct_array_access_plans"))
        span_plans.extend((name, row) for row in list_meta(function, "span_access_plans"))
        regions.extend((name, row) for row in list_meta(function, "required_fastpath_regions"))
        obligations.extend((name, row) for row in list_meta(function, "fastpath_obligations"))
        hotcore_summaries.extend(
            (name, row) for row in list_meta(function, "hotcore_method_summaries")
        )
        hotcore_call_plans.extend(
            (name, row) for row in list_meta(function, "direct_exact_hotcore_call_plans")
        )

    direct_rows = [row for _, row in direct_plans]
    span_rows = [row for _, row in span_plans]
    obligation_rows = [row for _, row in obligations]
    hotcore_summary_rows = [row for _, row in hotcore_summaries]
    hotcore_call_rows = [row for _, row in hotcore_call_plans]

    direct_op_counts = count_by(direct_rows, "op")
    direct_bounds_counts = count_by(direct_rows, "bounds_policy")
    direct_proof_counts = count_by(direct_rows, "proof_kind")
    span_op_counts = count_by(span_rows, "op")
    span_bounds_counts = count_by(span_rows, "bounds_policy")
    obligation_status_counts = count_by(obligation_rows, "status")
    hotcore_summary_status_counts = count_by(hotcore_summary_rows, "summary")
    hotcore_call_status_counts = count_by(hotcore_call_rows, "summary")
    hotcore_call_dispatch_counts = count_by(hotcore_call_rows, "dispatch_policy")
    failure_code_counts = count_by(
        [row for row in obligation_rows if str(row.get("status", "")) != "passed"],
        "failure_code",
    )

    fastpath_plan_count = len(direct_plans) + len(span_plans)
    failed_obligation_count = obligation_status_counts["failed"]
    hotcore_plan_failure_count = hotcore_summary_status_counts["failed"] + hotcore_call_status_counts["failed"]
    direct_exact_static_call_lowered_count = sum(
        1 for row in hotcore_call_rows if bool(row.get("lowering_consumer_enabled"))
    )
    direct_exact_plan_lowered_to_fallback_count = sum(
        1
        for row in hotcore_call_rows
        if bool(row.get("lowering_consumer_enabled"))
        and (
            bool(row.get("generic_method_dispatch"))
            or bool(row.get("dynamic_route"))
            or bool(row.get("boxed_fallback"))
        )
    )
    generic_method_dispatch_count = sum(
        1 for row in hotcore_call_rows if bool(row.get("generic_method_dispatch"))
    )
    dynamic_route_count = sum(1 for row in hotcore_call_rows if bool(row.get("dynamic_route")))
    boxed_fallback_count = sum(1 for row in hotcore_call_rows if bool(row.get("boxed_fallback")))
    clean = (
        failed_obligation_count == 0
        and hotcore_plan_failure_count == 0
        and direct_exact_plan_lowered_to_fallback_count == 0
    )

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
        f"hotcore_method_summary_count={len(hotcore_summaries)}",
        f"hotcore_method_summary_ok_count={hotcore_summary_status_counts['ok']}",
        f"hotcore_method_summary_failed_count={hotcore_summary_status_counts['failed']}",
        f"direct_exact_hotcore_call_plan_count={len(hotcore_call_plans)}",
        f"direct_exact_hotcore_call_plan_ok_count={hotcore_call_status_counts['ok']}",
        f"direct_exact_hotcore_call_plan_failed_count={hotcore_call_status_counts['failed']}",
        f"direct_exact_static_exact_dispatch_count={hotcore_call_dispatch_counts['static_exact']}",
        f"direct_exact_static_call_lowered_count={direct_exact_static_call_lowered_count}",
        f"direct_exact_plan_lowered_to_fallback_count={direct_exact_plan_lowered_to_fallback_count}",
        f"generic_method_dispatch_count={generic_method_dispatch_count}",
        f"dynamic_route_count={dynamic_route_count}",
        f"boxed_fallback_count={boxed_fallback_count}",
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

    for idx, (name, summary) in enumerate(hotcore_summaries[: max(0, args.topn)]):
        prefix = f"hotcore_summary_{idx}"
        lines.extend(
            [
                f"{prefix}_function={name}",
                f"{prefix}_method={summary.get('method', 'unknown')}",
                f"{prefix}_block_count={summary.get('block_count', 'unknown')}",
                f"{prefix}_return_kind={summary.get('return_kind', 'unknown')}",
                f"{prefix}_status={summary.get('summary', 'unknown')}",
                f"{prefix}_failure_reason={field_text(summary, 'failure_reason', 'none')}",
            ]
        )

    for idx, (name, plan) in enumerate(hotcore_call_plans[: max(0, args.topn)]):
        prefix = f"direct_exact_hotcore_call_{idx}"
        lines.extend(
            [
                f"{prefix}_caller={name}",
                f"{prefix}_callee={plan.get('callee', 'unknown')}",
                f"{prefix}_dispatch_policy={plan.get('dispatch_policy', 'unknown')}",
                f"{prefix}_callee_summary_status={plan.get('callee_summary_status', 'unknown')}",
                f"{prefix}_lowering_consumer_enabled={bool_text(bool(plan.get('lowering_consumer_enabled')))}",
                f"{prefix}_status={plan.get('summary', 'unknown')}",
                f"{prefix}_failure_reason={field_text(plan, 'failure_reason', 'none')}",
            ]
        )

    fallback_rows = [
        (name, row)
        for name, row in hotcore_call_plans
        if bool(row.get("lowering_consumer_enabled"))
        and (
            bool(row.get("generic_method_dispatch"))
            or bool(row.get("dynamic_route"))
            or bool(row.get("boxed_fallback"))
        )
    ]
    for idx, (name, plan) in enumerate(fallback_rows[: max(0, args.topn)]):
        prefix = f"direct_exact_plan_fallback_{idx}"
        lines.extend(
            [
                f"{prefix}_caller={name}",
                f"{prefix}_callee={plan.get('callee', 'unknown')}",
                f"{prefix}_generic_method_dispatch={bool_text(bool(plan.get('generic_method_dispatch')))}",
                f"{prefix}_dynamic_route={bool_text(bool(plan.get('dynamic_route')))}",
                f"{prefix}_boxed_fallback={bool_text(bool(plan.get('boxed_fallback')))}",
                f"{prefix}_failure_reason=direct_exact_plan_lowered_to_fallback",
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
