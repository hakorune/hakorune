#!/usr/bin/env python3
"""Explain MIR-owned FastPathPlan coverage from a MIR JSON artifact.

This is a hako_check diagnostic adapter, not an optimizer. It consumes MIR JSON
metadata that the compiler already produced and reports whether direct-memory
plan/fact surfaces are present for selected functions.
"""

from __future__ import annotations

import argparse
import hashlib
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


def file_sha256(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            hasher.update(chunk)
    return "sha256:" + hasher.hexdigest()


def kv_payload(lines: list[str]) -> dict[str, str]:
    payload: dict[str, str] = {}
    for line in lines:
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        payload[key] = value
    return payload


def site_id(row: dict[str, Any]) -> str:
    explicit = row.get("site_id") or row.get("obligation_id")
    if explicit is not None:
        return str(explicit)
    block = row.get("block", "unknown")
    inst = row.get("instruction_index", "unknown")
    return f"block_{block}:inst_{inst}"


def site_record(function: str, kind: str, row: dict[str, Any]) -> dict[str, Any]:
    return {
        "function": function,
        "site_id": site_id(row),
        "kind": kind,
        "block": row.get("block"),
        "instruction_index": row.get("instruction_index"),
        "op": row.get("op"),
        "access_kind": row.get("access_kind", kind),
        "route": row.get("route") or row.get("actual_route"),
        "bounds_policy": row.get("bounds_policy"),
        "proof_kind": row.get("proof_kind"),
        "proof_ids": row.get("proof_ids") if isinstance(row.get("proof_ids"), list) else [],
        "fallback_policy": row.get("fallback_policy"),
        "status": row.get("status") or row.get("summary"),
        "failure_code": row.get("failure_code"),
        "failure_reason": row.get("failure_reason"),
        "source_span": row.get("source_span") if isinstance(row.get("source_span"), dict) else None,
        "source_text": row.get("source_text"),
    }


def render_summary(payload: dict[str, Any]) -> str:
    counts = payload["counts"]
    lines = [
        "output_contract=hako-check-fastpath-summary-v0",
        f"target_method={counts['target_method']}",
        f"clean={counts['clean']}",
        f"fastpath_plan_count={counts['fastpath_plan_count']}",
        f"direct_array_access_plan_count={counts['direct_array_access_plan_count']}",
        f"direct_array_proved_unchecked_plan_count={counts['direct_array_proved_unchecked_plan_count']}",
        f"span_access_plan_count={counts['span_access_plan_count']}",
        f"fastpath_obligation_failed_count={counts['fastpath_obligation_failed_count']}",
        f"direct_exact_plan_lowered_to_fallback_count={counts['direct_exact_plan_lowered_to_fallback_count']}",
        f"generic_method_dispatch_count={counts['generic_method_dispatch_count']}",
        f"dynamic_route_count={counts['dynamic_route_count']}",
        f"boxed_fallback_count={counts['boxed_fallback_count']}",
        "source_rewrite_executed=0",
        f"summary={'ok' if counts['clean'] == '1' else 'failed'}",
    ]
    return "\n".join(lines) + "\n"


def render_markdown(payload: dict[str, Any], topn: int) -> str:
    counts = payload["counts"]
    sites = payload["sites"][: max(0, topn)]
    lines = [
        "# Hakorune FastPath Report",
        "",
        f"- Output contract: `{payload['output_contract']}`",
        f"- Target method: `{counts['target_method']}`",
        f"- MIR hash: `{payload['mir_hash']}`",
        f"- Source hash: `{payload['source_hash']}`",
        "- Source rewrite: `0`",
        f"- Clean: `{counts['clean']}`",
        "",
        "## Summary",
        "",
        "| Metric | Value |",
        "|---|---:|",
        f"| FastPath plans | {counts['fastpath_plan_count']} |",
        f"| DirectArray plans | {counts['direct_array_access_plan_count']} |",
        f"| DirectArray proved unchecked | {counts['direct_array_proved_unchecked_plan_count']} |",
        f"| Span plans | {counts['span_access_plan_count']} |",
        f"| Failed obligations | {counts['fastpath_obligation_failed_count']} |",
        f"| Lowering fallback | {counts['direct_exact_plan_lowered_to_fallback_count']} |",
        f"| Generic method dispatch | {counts['generic_method_dispatch_count']} |",
        "",
        "## Sites",
        "",
    ]
    if not sites:
        lines.append("_No FastPath sites were present in the selected MIR metadata._")
    else:
        lines.extend(
            [
                "| Function | Site | Kind | Op | Route | Bounds | Proof | Status |",
                "|---|---|---|---|---|---|---|---|",
            ]
        )
        for site in sites:
            proof = site.get("proof_kind") or ",".join(site.get("proof_ids") or []) or "unknown"
            lines.append(
                "| {function} | {site_id} | {kind} | {op} | {route} | {bounds} | {proof} | {status} |".format(
                    function=site.get("function", "unknown"),
                    site_id=site.get("site_id", "unknown"),
                    kind=site.get("kind", "unknown"),
                    op=site.get("op") or "unknown",
                    route=site.get("route") or "unknown",
                    bounds=site.get("bounds_policy") or "unknown",
                    proof=proof,
                    status=site.get("status") or "unknown",
                )
            )
    lines.extend(
        [
            "",
            "## Boundary",
            "",
            "This report is generated from MIR metadata. It does not edit `.hako` source files.",
            "Source excerpts are only shown when the compiler emits source-span metadata for a site.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", help="Optional exact MIR function name")
    parser.add_argument("--topn", type=int, default=8)
    parser.add_argument(
        "--format",
        choices=("kv", "json"),
        default="kv",
        help="Output format for the base report. Default: kv.",
    )
    parser.add_argument(
        "--summary",
        action="store_true",
        help="Print a compact human-readable summary.",
    )
    parser.add_argument(
        "--annotated-report",
        choices=("md",),
        help="Generate a source-mapped report without rewriting source files.",
    )
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
    effect_summaries: list[tuple[str, dict[str, Any]]] = []
    receiver_snapshot_plans: list[tuple[str, dict[str, Any]]] = []
    hotcore_summaries: list[tuple[str, dict[str, Any]]] = []
    hotcore_call_plans: list[tuple[str, dict[str, Any]]] = []

    for function in selected:
        name = function_name(function)
        direct_plans.extend((name, row) for row in list_meta(function, "direct_array_access_plans"))
        span_plans.extend((name, row) for row in list_meta(function, "span_access_plans"))
        regions.extend((name, row) for row in list_meta(function, "required_fastpath_regions"))
        obligations.extend((name, row) for row in list_meta(function, "fastpath_obligations"))
        effect_summaries.extend((name, row) for row in list_meta(function, "effect_summaries"))
        receiver_snapshot_plans.extend(
            (name, row) for row in list_meta(function, "receiver_snapshot_publication_plans")
        )
        hotcore_summaries.extend(
            (name, row) for row in list_meta(function, "hotcore_method_summaries")
        )
        hotcore_call_plans.extend(
            (name, row) for row in list_meta(function, "direct_exact_hotcore_call_plans")
        )

    direct_rows = [row for _, row in direct_plans]
    span_rows = [row for _, row in span_plans]
    obligation_rows = [row for _, row in obligations]
    effect_summary_rows = [row for _, row in effect_summaries]
    receiver_snapshot_rows = [row for _, row in receiver_snapshot_plans]
    hotcore_summary_rows = [row for _, row in hotcore_summaries]
    hotcore_call_rows = [row for _, row in hotcore_call_plans]

    direct_op_counts = count_by(direct_rows, "op")
    direct_bounds_counts = count_by(direct_rows, "bounds_policy")
    direct_proof_counts = count_by(direct_rows, "proof_kind")
    span_op_counts = count_by(span_rows, "op")
    span_bounds_counts = count_by(span_rows, "bounds_policy")
    obligation_status_counts = count_by(obligation_rows, "status")
    effect_summary_status_counts = count_by(effect_summary_rows, "summary")
    effect_summary_candidate_counts = count_by(effect_summary_rows, "candidate_kind")
    receiver_snapshot_status_counts = count_by(receiver_snapshot_rows, "summary")
    receiver_snapshot_kind_counts = count_by(receiver_snapshot_rows, "publication_kind")
    receiver_snapshot_barrier_counts = count_by(receiver_snapshot_rows, "barrier_policy")
    receiver_snapshot_lifetime_counts = count_by(receiver_snapshot_rows, "lifetime_policy")
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
        "source_rewrite_executed=0",
        f"mir_hash={file_sha256(args.mir_json)}",
        "source_hash=unavailable",
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
        f"effect_summary_count={len(effect_summaries)}",
        f"effect_summary_ok_count={effect_summary_status_counts['ok']}",
        f"effect_summary_rejected_count={effect_summary_status_counts['rejected']}",
        f"effect_summary_receiver_local_leaf_count={effect_summary_candidate_counts['receiver_local_leaf_candidate']}",
        f"effect_summary_mixed_base_scalar_snapshot_count={effect_summary_candidate_counts['mixed_base_scalar_snapshot_candidate']}",
        f"effect_summary_mixed_base_publication_count={effect_summary_candidate_counts['mixed_base_publication_candidate']}",
        f"effect_summary_rejected_shape_count={effect_summary_candidate_counts['rejected_effect_shape']}",
        f"effect_summary_handle_publication_count={sum(int(row.get('handle_publications', 0) or 0) for row in effect_summary_rows)}",
        f"effect_summary_foreign_read_count={sum(int(row.get('foreign_reads', 0) or 0) for row in effect_summary_rows)}",
        f"effect_summary_foreign_write_count={sum(int(row.get('foreign_writes', 0) or 0) for row in effect_summary_rows)}",
        f"receiver_snapshot_publication_plan_count={len(receiver_snapshot_plans)}",
        f"receiver_snapshot_publication_plan_ok_count={receiver_snapshot_status_counts['ok']}",
        f"receiver_snapshot_publication_plan_rejected_count={receiver_snapshot_status_counts['rejected']}",
        f"receiver_snapshot_scalar_snapshot_count={receiver_snapshot_kind_counts['scalar_snapshot']}",
        f"receiver_snapshot_foreign_handle_publication_count={receiver_snapshot_kind_counts['foreign_handle_publication']}",
        f"receiver_snapshot_barrier_none_count={receiver_snapshot_barrier_counts['none']}",
        f"receiver_snapshot_barrier_unproven_count={receiver_snapshot_barrier_counts['unproven']}",
        f"receiver_snapshot_lifetime_caller_visible_handle_count={receiver_snapshot_lifetime_counts['caller_visible_handle']}",
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

    for idx, (name, summary) in enumerate(effect_summaries[: max(0, args.topn)]):
        prefix = f"effect_summary_{idx}"
        lines.extend(
            [
                f"{prefix}_function={name}",
                f"{prefix}_method={summary.get('method', 'unknown')}",
                f"{prefix}_candidate_kind={summary.get('candidate_kind', 'unknown')}",
                f"{prefix}_receiver_reads={summary.get('receiver_reads', 'unknown')}",
                f"{prefix}_receiver_writes={summary.get('receiver_writes', 'unknown')}",
                f"{prefix}_foreign_reads={summary.get('foreign_reads', 'unknown')}",
                f"{prefix}_foreign_writes={summary.get('foreign_writes', 'unknown')}",
                f"{prefix}_handle_publications={summary.get('handle_publications', 'unknown')}",
                f"{prefix}_status={summary.get('summary', 'unknown')}",
                f"{prefix}_failure_reason={field_text(summary, 'failure_reason', 'none')}",
            ]
        )

    for idx, (name, plan) in enumerate(receiver_snapshot_plans[: max(0, args.topn)]):
        prefix = f"receiver_snapshot_publication_{idx}"
        lines.extend(
            [
                f"{prefix}_function={name}",
                f"{prefix}_method={plan.get('method', 'unknown')}",
                f"{prefix}_publication_kind={plan.get('publication_kind', 'unknown')}",
                f"{prefix}_barrier_policy={plan.get('barrier_policy', 'unknown')}",
                f"{prefix}_handle_publication_proof_kind={field_text(plan, 'handle_publication_proof_kind', 'none')}",
                f"{prefix}_lifetime_policy={plan.get('lifetime_policy', 'unknown')}",
                f"{prefix}_foreign_base_count={plan.get('foreign_base_count', 'unknown')}",
                f"{prefix}_receiver_writes={plan.get('receiver_writes', 'unknown')}",
                f"{prefix}_foreign_reads={plan.get('foreign_reads', 'unknown')}",
                f"{prefix}_handle_publications={plan.get('handle_publications', 'unknown')}",
                f"{prefix}_lowering_consumer_enabled={bool_text(bool(plan.get('lowering_consumer_enabled')))}",
                f"{prefix}_status={plan.get('summary', 'unknown')}",
                f"{prefix}_failure_reason={field_text(plan, 'failure_reason', 'none')}",
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

    counts = kv_payload(lines)
    site_rows: list[dict[str, Any]] = []
    site_rows.extend(site_record(name, "direct_array_access", row) for name, row in direct_plans)
    site_rows.extend(site_record(name, "span_access", row) for name, row in span_plans)
    site_rows.extend(site_record(name, "fastpath_obligation", row) for name, row in obligations)
    report_payload = {
        "output_contract": "hako-check-fastpath-explain-v0",
        "input_kind": "mir_json",
        "tool_surface": "hako_check_fastpath_explain",
        "observation_only": 1,
        "rewrite_executed": 0,
        "source_rewrite_executed": 0,
        "mir_json_path": str(args.mir_json),
        "mir_hash": counts["mir_hash"],
        "source_hash": counts["source_hash"],
        "counts": counts,
        "sites": site_rows,
    }

    if args.annotated_report == "md":
        report = render_markdown(report_payload, args.topn)
    elif args.summary:
        report = render_summary(report_payload)
    elif args.format == "json":
        report = json.dumps(report_payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    else:
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
