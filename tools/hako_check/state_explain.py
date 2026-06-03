#!/usr/bin/env python3
"""Explain MIR-owned state/residence metadata from a MIR JSON artifact.

This is a hako_check diagnostic adapter, not an optimizer. It consumes MIR JSON
metadata that the compiler already produced and reports user-box field buckets,
DirectState candidates, and the current absence/presence of record-state
residence plans.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


PRIMITIVE_TYPES = {
    "i8",
    "i16",
    "i32",
    "i64",
    "isize",
    "u8",
    "u16",
    "u32",
    "u64",
    "usize",
    "bool",
}

PUBLIC_SEMANTIC_FIELDS = {
    "page_id",
    "block_size",
    "capacity",
    "reserved",
}

PUBLIC_PROOF_FIELDS = {"requested_bytes"}

PRIMITIVE_HOT_FIELDS = {
    "used",
    "free_top",
    "local_free_top",
    "retired",
    "decommitted",
    "peak_used",
    "last_selected_index",
    "last_selected_page_id",
    "last_selected_kind",
    "last_alloc_page_index",
    "last_alloc_page_id",
}

DIRECT_ARRAY_FIELD_NAMES = {
    "free",
    "local_free",
    "block_used",
}

DIAGNOSTIC_NAME_PARTS = (
    "reject",
    "skip",
    "fallback",
    "miss",
    "decommit",
    "recommit",
    "reactivate",
    "collect",
)


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def root_list(data: dict[str, Any], key: str) -> list[dict[str, Any]]:
    value = data.get(key)
    if not isinstance(value, list):
        return []
    return [row for row in value if isinstance(row, dict)]


def typed_object_field_names(plan: dict[str, Any]) -> set[str]:
    values = plan.get("fields")
    if not isinstance(values, list):
        return set()
    names: set[str] = set()
    for field in values:
        if isinstance(field, dict) and isinstance(field.get("name"), str):
            names.add(str(field["name"]))
    return names


def function_metadata_rows(data: dict[str, Any], key: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for function in root_list(data, "functions"):
        metadata = function.get("metadata")
        if not isinstance(metadata, dict):
            continue
        values = metadata.get(key)
        if not isinstance(values, list):
            continue
        for row in values:
            if isinstance(row, dict):
                copied = dict(row)
                copied.setdefault("function", function.get("name", "unknown"))
                rows.append(copied)
    return rows


def field_name(row: dict[str, Any]) -> str:
    return str(row.get("name", "unknown"))


def field_type(row: dict[str, Any]) -> str:
    value = row.get("declared_type")
    return "unknown" if value is None else str(value)


def box_name(row: dict[str, Any]) -> str:
    return str(row.get("name") or row.get("box_name") or "unknown")


def is_primitive_type(type_name: str) -> bool:
    return type_name in PRIMITIVE_TYPES


def is_handle_like(type_name: str) -> bool:
    if type_name in ("unknown", ""):
        return False
    if is_primitive_type(type_name):
        return False
    return type_name not in ("ArrayBox", "DirectArrayI64")


def classify_field(owner: str, field: dict[str, Any]) -> str:
    name = field_name(field)
    ty = field_type(field)

    if name in DIRECT_ARRAY_FIELD_NAMES or ty == "DirectArrayI64":
        return "direct_array_owner"
    if name == "pages":
        return "handle_cache"
    if "result" in name and is_handle_like(ty):
        return "result_capsule"
    if is_handle_like(ty) and ("page" in name or name.endswith("_queue")):
        return "handle_cache"
    if owner.endswith("Result"):
        return "result_capsule"
    if name in PUBLIC_PROOF_FIELDS:
        return "public_semantics_proof_evidence"
    if name in PUBLIC_SEMANTIC_FIELDS:
        return "public_semantics"
    if name in PRIMITIVE_HOT_FIELDS:
        return "primitive_hot_state"
    if any(part in name for part in DIAGNOSTIC_NAME_PARTS):
        return "diagnostic_only"
    if name.endswith("_count"):
        return "observer_boundary"
    if is_primitive_type(ty):
        return "primitive_hot_state"
    return "escape_unknown"


def iter_user_box_fields(
    user_box_decls: list[dict[str, Any]],
) -> list[tuple[str, dict[str, Any], str]]:
    rows: list[tuple[str, dict[str, Any], str]] = []
    for box in user_box_decls:
        owner = box_name(box)
        fields = box.get("field_decls")
        if not isinstance(fields, list):
            continue
        for field in fields:
            if not isinstance(field, dict):
                continue
            rows.append((owner, field, classify_field(owner, field)))
    return rows


def selected_boxes(
    rows: list[tuple[str, dict[str, Any], str]],
    box_filter: str | None,
) -> list[tuple[str, dict[str, Any], str]]:
    if box_filter is None:
        return rows
    selected = [row for row in rows if row[0] == box_filter]
    if not selected:
        raise SystemExit(f"selected box not found or has no field decls: {box_filter}")
    return selected


def bool_text(value: bool) -> str:
    return "1" if value else "0"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--box", dest="box_filter", help="Optional exact box name")
    parser.add_argument("--topn", type=int, default=8)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    data = load_json(args.mir_json)
    user_box_decls = root_list(data, "user_box_decls")
    record_decls = root_list(data, "record_decls")
    typed_object_plans = root_list(data, "typed_object_plans")
    direct_state_plans = root_list(data, "direct_state_plans")
    record_layout_plans = root_list(data, "record_layout_plans")
    record_state_residence_plans = root_list(data, "record_state_residence_plans")
    record_state_field_access_plans = function_metadata_rows(
        data, "record_state_field_access_plans"
    )

    all_field_rows = iter_user_box_fields(user_box_decls)
    field_rows = selected_boxes(all_field_rows, args.box_filter)

    bucket_counts: Counter[str] = Counter(bucket for _, _, bucket in field_rows)
    box_bucket_counts: Counter[tuple[str, str]] = Counter(
        (owner, bucket) for owner, _, bucket in field_rows
    )
    direct_state_by_box = {str(plan.get("box_name", "unknown")): plan for plan in direct_state_plans}
    typed_object_by_box = {
        str(plan.get("box_name", "unknown")): plan for plan in typed_object_plans
    }
    selected_typed_object_plans = [
        plan
        for plan in typed_object_plans
        if args.box_filter is None or str(plan.get("box_name", "unknown")) == args.box_filter
    ]
    selected_direct_state_plans = [
        plan
        for plan in direct_state_plans
        if args.box_filter is None or str(plan.get("box_name", "unknown")) == args.box_filter
    ]
    positive_direct_state = [
        plan for plan in direct_state_plans if bool(plan.get("positive_net_expected"))
    ]
    mixed_direct_state = [
        plan
        for plan in direct_state_plans
        if int(plan.get("unsupported_field_count") or 0) > 0
    ]
    selected_positive_direct_state = [
        plan for plan in selected_direct_state_plans if bool(plan.get("positive_net_expected"))
    ]
    selected_mixed_direct_state = [
        plan
        for plan in selected_direct_state_plans
        if int(plan.get("unsupported_field_count") or 0) > 0
    ]

    record_state_candidate_fields = [
        (owner, field)
        for owner, field, bucket in field_rows
        if bucket == "primitive_hot_state"
    ]
    handle_reject_fields = [
        (owner, field)
        for owner, field, bucket in field_rows
        if bucket in {"handle_cache", "direct_array_owner", "escape_unknown"}
    ]
    record_state_field_access_lowering_enabled = sum(
        1 for plan in record_state_field_access_plans if bool(plan.get("lowering_enabled"))
    )
    selected_record_state_plans = [
        plan
        for plan in record_state_residence_plans
        if args.box_filter is None
        or str(plan.get("owner_box", "unknown")) == args.box_filter
    ]
    selected_record_state_access_plans = [
        plan
        for plan in record_state_field_access_plans
        if args.box_filter is None
        or str(plan.get("owner_box", "unknown")) == args.box_filter
    ]
    record_state_access_exact_slot_covered_count = 0
    record_state_access_exact_slot_missing_count = 0
    for plan in selected_record_state_access_plans:
        owner = str(plan.get("owner_box", "unknown"))
        field = str(plan.get("field_name", "unknown"))
        typed_object_field_set = typed_object_field_names(typed_object_by_box.get(owner, {}))
        if field in typed_object_field_set:
            record_state_access_exact_slot_covered_count += 1
        else:
            record_state_access_exact_slot_missing_count += 1
    record_state_lowering_owner_selection_enabled = int(
        bool(selected_record_state_plans) and bool(selected_record_state_access_plans)
    )
    if record_state_lowering_owner_selection_enabled:
        if record_state_access_exact_slot_missing_count == 0:
            record_state_lowering_owner_selected = "typed_object_exact_slot_existing"
            record_state_lowering_owner_reason = (
                "record_state_access_sites_already_have_typed_object_slot_storage"
            )
            record_state_lowering_owner_next_bridge = (
                "measure_representation_delta_before_record_state_lowering"
            )
        else:
            record_state_lowering_owner_selected = "typed_object_exact_slot_gap"
            record_state_lowering_owner_reason = "record_state_access_sites_missing_typed_slots"
            record_state_lowering_owner_next_bridge = "fix_typed_object_slot_coverage_first"
    else:
        record_state_lowering_owner_selected = "none"
        record_state_lowering_owner_reason = "record_state_access_sites_not_ready"
        record_state_lowering_owner_next_bridge = "record_state_field_access_site_measurement"

    lines = [
        "output_contract=hako-check-state-explain-v0",
        "input_kind=mir_json",
        "tool_surface=hako_check_state_explain",
        "observation_only=1",
        "rewrite_executed=0",
        "keeper_selection=0",
        f"target_box={args.box_filter or 'all'}",
        f"user_box_decl_count={len(user_box_decls)}",
        f"selected_field_count={len(field_rows)}",
        f"record_decl_count={len(record_decls)}",
        f"record_layout_plan_count={len(record_layout_plans)}",
        f"typed_object_plan_count={len(typed_object_plans)}",
        f"selected_typed_object_plan_count={len(selected_typed_object_plans)}",
        f"direct_state_plan_count={len(direct_state_plans)}",
        f"direct_state_positive_candidate_count={len(positive_direct_state)}",
        f"direct_state_mixed_candidate_count={len(mixed_direct_state)}",
        f"selected_direct_state_plan_count={len(selected_direct_state_plans)}",
        f"selected_direct_state_positive_candidate_count={len(selected_positive_direct_state)}",
        f"selected_direct_state_mixed_candidate_count={len(selected_mixed_direct_state)}",
        f"record_state_residence_plan_count={len(record_state_residence_plans)}",
        f"record_state_field_access_plan_count={len(record_state_field_access_plans)}",
        f"record_state_field_access_lowering_enabled={record_state_field_access_lowering_enabled}",
        "record_state_route_decision_enabled=0",
        "record_state_lowering_owner_selection_plan_v0="
        f"{record_state_lowering_owner_selection_enabled}",
        f"record_state_access_exact_slot_covered_count={record_state_access_exact_slot_covered_count}",
        f"record_state_access_exact_slot_missing_count={record_state_access_exact_slot_missing_count}",
        f"record_state_lowering_owner_selected={record_state_lowering_owner_selected}",
        f"record_state_lowering_owner_reason={record_state_lowering_owner_reason}",
        f"record_state_lowering_owner_next_bridge={record_state_lowering_owner_next_bridge}",
        f"record_state_residence_candidate_field_count={len(record_state_candidate_fields)}",
        f"record_state_handle_reject_field_count={len(handle_reject_fields)}",
        "record_state_source_migration_selected=0",
        "whole_record_abi_enabled=0",
        "public_materialization_enabled=0",
        "ordinary_box_auto_recordification=0",
        "record_to_box_conversion=0",
    ]

    for bucket in (
        "primitive_hot_state",
        "public_semantics",
        "public_semantics_proof_evidence",
        "proof_evidence",
        "diagnostic_only",
        "observer_boundary",
        "handle_cache",
        "result_capsule",
        "direct_array_owner",
        "escape_unknown",
    ):
        lines.append(f"bucket_{bucket}_field_count={bucket_counts[bucket]}")

    for idx, ((owner, bucket), count) in enumerate(
        box_bucket_counts.most_common(max(0, args.topn))
    ):
        lines.append(f"top_bucket_{idx}_box={owner}")
        lines.append(f"top_bucket_{idx}_bucket={bucket}")
        lines.append(f"top_bucket_{idx}_field_count={count}")

    for idx, (owner, field) in enumerate(record_state_candidate_fields[: max(0, args.topn)]):
        prefix = f"record_state_candidate_{idx}"
        direct_plan = direct_state_by_box.get(owner)
        lines.extend(
            [
                f"{prefix}_box={owner}",
                f"{prefix}_field={field_name(field)}",
                f"{prefix}_declared_type={field_type(field)}",
                f"{prefix}_direct_state_plan_present={bool_text(direct_plan is not None)}",
                f"{prefix}_accepted_shape=box_private_primitive_subfield",
            ]
        )

    for idx, plan in enumerate(selected_direct_state_plans[: max(0, args.topn)]):
        prefix = f"direct_state_plan_{idx}"
        lines.extend(
            [
                f"{prefix}_box={plan.get('box_name', 'unknown')}",
                f"{prefix}_state_repr={plan.get('state_repr', 'unknown')}",
                f"{prefix}_selected_field_count={plan.get('selected_field_count', 'unknown')}",
                f"{prefix}_unsupported_field_count={plan.get('unsupported_field_count', 'unknown')}",
                f"{prefix}_materialization_boundary_known={bool_text(bool(plan.get('materialization_boundary_known')))}",
                f"{prefix}_positive_net_expected={bool_text(bool(plan.get('positive_net_expected')))}",
            ]
        )

    for idx, plan in enumerate(selected_record_state_plans[: max(0, args.topn)]):
        prefix = f"record_state_residence_plan_{idx}"
        lines.extend(
            [
                f"{prefix}_owner_box={plan.get('owner_box', 'unknown')}",
                f"{prefix}_candidate_record={plan.get('candidate_record', 'unknown')}",
                f"{prefix}_residence={plan.get('residence', 'unknown')}",
                f"{prefix}_report_only={bool_text(bool(plan.get('report_only')))}",
                f"{prefix}_source_migration_allowed={bool_text(bool(plan.get('source_migration_allowed')))}",
                f"{prefix}_selected_field_count={plan.get('selected_field_count', 'unknown')}",
                f"{prefix}_rejected_field_count={plan.get('rejected_field_count', 'unknown')}",
                f"{prefix}_summary={plan.get('summary', 'unknown')}",
            ]
        )

    for idx, plan in enumerate(selected_record_state_access_plans[: max(0, args.topn)]):
        prefix = f"record_state_field_access_plan_{idx}"
        lines.extend(
            [
                f"{prefix}_function={plan.get('function', 'unknown')}",
                f"{prefix}_owner_box={plan.get('owner_box', 'unknown')}",
                f"{prefix}_candidate_record={plan.get('candidate_record', 'unknown')}",
                f"{prefix}_field={plan.get('field_name', 'unknown')}",
                f"{prefix}_op={plan.get('op', 'unknown')}",
                f"{prefix}_route={plan.get('route', 'unknown')}",
                f"{prefix}_lowering_enabled={bool_text(bool(plan.get('lowering_enabled')))}",
                f"{prefix}_fallback_policy={plan.get('fallback_policy', 'unknown')}",
            ]
        )

    clean = len(record_state_residence_plans) == 0 or all(
        bool(row.get("summary", "ok") == "ok") for row in record_state_residence_plans
    )
    lines.append(f"clean={bool_text(clean)}")
    lines.append("summary=ok")

    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
