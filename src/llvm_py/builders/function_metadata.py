from typing import Any, Dict, List, Optional

from cfg.utils import (
    collect_arrayish_value_ids,
    collect_integerish_value_ids,
    collect_non_negative_value_ids,
    collect_stringish_value_ids,
)
from context import FunctionLowerContext
from instructions.user_box_local import seed_local_user_box_layouts_from_function_data


def _safe_metadata(func_data: Dict[str, Any]) -> Dict[str, Any]:
    metadata = func_data.get("metadata", {}) if isinstance(func_data, dict) else {}
    return metadata if isinstance(metadata, dict) else {}


def _as_int_or_none(value: Any) -> Optional[int]:
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def _load_value_types_metadata(builder, func_data: Dict[str, Any]) -> None:
    metadata = _safe_metadata(func_data)
    value_types_json = metadata.get("value_types", {})
    builder.resolver.value_types = {}
    if not isinstance(value_types_json, dict):
        return
    for vid_str, vtype in value_types_json.items():
        vid = _as_int_or_none(vid_str)
        if vid is None:
            continue
        builder.resolver.value_types[vid] = vtype


def _load_thin_entry_selection_metadata(builder, func_data: Dict[str, Any]) -> None:
    metadata = _safe_metadata(func_data)
    placement_effect_routes = metadata.get("placement_effect_routes", [])
    rows = metadata.get("thin_entry_selections", [])

    normalized_rows = []
    by_value = {}
    by_subject = {}

    def add_row(normalized):
        key = (
            normalized.get("surface"),
            normalized.get("subject"),
            normalized.get("value"),
            normalized.get("manifest_row"),
        )
        if key in seen:
            return
        seen.add(key)
        value = normalized.get("value")
        surface = normalized.get("surface")
        subject = normalized.get("subject")
        if isinstance(value, int):
            by_value.setdefault(int(value), []).append(normalized)
        else:
            normalized["value"] = None
        if isinstance(surface, str) and isinstance(subject, str):
            by_subject.setdefault((surface, subject), []).append(normalized)
        normalized_rows.append(normalized)

    seen = set()
    if isinstance(placement_effect_routes, list):
        for row in placement_effect_routes:
            normalized = _thin_entry_row_from_placement_effect_route(row)
            if isinstance(normalized, dict):
                add_row(normalized)

    if isinstance(rows, list):
        for row in rows:
            if not isinstance(row, dict):
                continue
            normalized = dict(row)
            value = _as_int_or_none(normalized.get("value"))
            normalized["value"] = value
            add_row(normalized)

    builder.resolver.thin_entry_selections = normalized_rows
    builder.resolver.thin_entry_selection_by_value = by_value
    builder.resolver.thin_entry_selection_by_subject = by_subject


def _thin_entry_row_from_placement_effect_route(row: Any) -> Optional[Dict[str, Any]]:
    if not isinstance(row, dict):
        return None
    if row.get("source") != "thin_entry":
        return None
    manifest_row = row.get("detail")
    if not isinstance(manifest_row, str) or not manifest_row:
        return None
    surface = _thin_entry_surface_from_manifest_row(manifest_row)
    if not isinstance(surface, str):
        return None
    subject = row.get("subject")
    if not isinstance(subject, str) or not subject:
        return None
    selected_entry = row.get("decision")
    if selected_entry not in ("public_entry", "thin_internal_entry"):
        return None
    value = row.get("value")
    return {
        "surface": surface,
        "subject": subject,
        "manifest_row": manifest_row,
        "selected_entry": selected_entry,
        "state": row.get("state"),
        "value": int(value) if isinstance(value, int) else None,
    }


def _thin_entry_surface_from_manifest_row(manifest_row: Any) -> Optional[str]:
    if not isinstance(manifest_row, str):
        return None
    if "." not in manifest_row:
        return None
    prefix = manifest_row.split(".", 1)[0]
    return prefix if prefix else None


def _load_sum_placement_metadata(builder, func_data: Dict[str, Any]) -> None:
    metadata = _safe_metadata(func_data)
    placement_effect_routes = metadata.get("placement_effect_routes", [])
    selections = metadata.get("sum_placement_selections", [])
    layouts = metadata.get("sum_placement_layouts", [])

    local_paths = {}
    if isinstance(placement_effect_routes, list):
        for row in placement_effect_routes:
            if not isinstance(row, dict):
                continue
            if row.get("source") != "sum_placement":
                continue
            if row.get("decision") != "local_aggregate":
                continue
            if row.get("detail") != "variant_make.local_aggregate":
                continue
            value = _as_int_or_none(row.get("value"))
            if value is not None:
                local_paths[value] = "local_aggregate"

    if isinstance(selections, list):
        for row in selections:
            if not isinstance(row, dict):
                continue
            if row.get("surface") != "variant_make":
                continue
            if row.get("selected_path") != "local_aggregate":
                continue
            value = _as_int_or_none(row.get("value"))
            if value is not None and value not in local_paths:
                local_paths[value] = "local_aggregate"

    local_layouts = {}
    if isinstance(placement_effect_routes, list):
        for row in placement_effect_routes:
            if not isinstance(row, dict):
                continue
            if row.get("source") != "agg_local_scalarization":
                continue
            if row.get("decision") != "local_aggregate":
                continue
            detail = row.get("detail")
            layout = _sum_layout_from_placement_effect_detail(detail)
            value = _as_int_or_none(row.get("value"))
            if value is not None and isinstance(layout, str):
                local_layouts[value] = layout

    if isinstance(layouts, list):
        for row in layouts:
            if not isinstance(row, dict):
                continue
            if row.get("surface") != "variant_make":
                continue
            value = _as_int_or_none(row.get("value"))
            layout = row.get("layout")
            if value is not None and isinstance(layout, str) and value not in local_layouts:
                local_layouts[value] = layout

    builder.resolver.sum_local_aggregate_paths = local_paths
    builder.resolver.sum_local_aggregate_layouts = local_layouts


def _sum_layout_from_placement_effect_detail(detail: Any) -> Any:
    if not isinstance(detail, str):
        return None
    prefix = "sum_local_layout("
    suffix = ")"
    if not detail.startswith(prefix) or not detail.endswith(suffix):
        return None
    layout = detail[len(prefix) : -len(suffix)]
    return layout if layout else None


def _load_user_box_local_aggregate_metadata(builder, func_data: Dict[str, Any]) -> None:
    seed_local_user_box_layouts_from_function_data(builder, func_data)


def _load_exact_numeric_route_metadata(builder, func_data: Dict[str, Any]) -> None:
    metadata = func_data.get("metadata", {}) if isinstance(func_data, dict) else {}

    def routes_by_dst(key: str) -> Dict[int, Dict[str, Any]]:
        rows = metadata.get(key, []) if isinstance(metadata, dict) else []
        result: Dict[int, Dict[str, Any]] = {}
        if not isinstance(rows, list):
            return result
        for row in rows:
            if not isinstance(row, dict):
                continue
            try:
                dst = int(row.get("dst"))
            except (TypeError, ValueError):
                continue
            normalized = dict(row)
            normalized["dst"] = dst
            for value_key in ("lhs", "rhs", "block", "instruction_index"):
                try:
                    normalized[value_key] = int(normalized[value_key])
                except (KeyError, TypeError, ValueError):
                    pass
            result[dst] = normalized
        return result

    builder.resolver.exact_numeric_binary_op_routes_by_dst = routes_by_dst(
        "exact_numeric_binary_op_routes"
    )
    builder.resolver.exact_numeric_compare_routes_by_dst = routes_by_dst(
        "exact_numeric_compare_routes"
    )
    builder.resolver.exact_numeric_shift_routes_by_dst = routes_by_dst(
        "exact_numeric_shift_routes"
    )


def _load_direct_array_access_plan_metadata(builder, func_data: Dict[str, Any]) -> None:
    metadata = func_data.get("metadata", {}) if isinstance(func_data, dict) else {}
    rows = metadata.get("direct_array_access_plans", []) if isinstance(metadata, dict) else []
    by_site: Dict[tuple[int, int], List[Dict[str, Any]]] = {}
    if not isinstance(rows, list):
        builder.resolver.direct_array_access_plans_by_site = {}
        return

    int_keys = (
        "block",
        "instruction_index",
        "receiver_value",
        "index_value",
        "value_value",
        "result_value",
    )
    for row in rows:
        if not isinstance(row, dict):
            continue
        normalized = dict(row)
        try:
            block = int(normalized.get("block"))
            instruction_index = int(normalized.get("instruction_index"))
        except (TypeError, ValueError):
            continue
        for key in int_keys:
            value = normalized.get(key)
            if value is None:
                normalized[key] = None
                continue
            try:
                normalized[key] = int(value)
            except (TypeError, ValueError):
                pass
        proof_ids = normalized.get("proof_ids")
        if isinstance(proof_ids, list):
            normalized["proof_ids"] = [
                item for item in proof_ids if isinstance(item, str) and item
            ]
        elif isinstance(normalized.get("proof_kind"), str):
            normalized["proof_ids"] = [normalized["proof_kind"]]
        else:
            normalized["proof_ids"] = []
        by_site.setdefault((block, instruction_index), []).append(normalized)
    builder.resolver.direct_array_access_plans_by_site = by_site

    decision_rows = metadata.get("route_decisions", []) if isinstance(metadata, dict) else []
    builder.resolver.route_decisions_metadata_present = (
        isinstance(metadata, dict)
        and "route_decisions" in metadata
        and isinstance(decision_rows, list)
    )
    decisions_by_site: Dict[tuple[int, int], List[Dict[str, Any]]] = {}
    if isinstance(decision_rows, list):
        for row in decision_rows:
            if not isinstance(row, dict):
                continue
            normalized = dict(row)
            try:
                block = int(normalized.get("block"))
                instruction_index = int(normalized.get("instruction_index"))
            except (TypeError, ValueError):
                continue
            normalized["block"] = block
            normalized["instruction_index"] = instruction_index
            decisions_by_site.setdefault((block, instruction_index), []).append(normalized)
    builder.resolver.route_decisions_by_site = decisions_by_site


def _load_fastmem_access_plan_metadata(builder, func_data: Dict[str, Any]) -> None:
    metadata = _safe_metadata(func_data)
    rows = metadata.get("fastmem_access_plans", [])
    by_site: Dict[tuple[int, int], List[Dict[str, Any]]] = {}
    if not isinstance(rows, list):
        builder.resolver.fastmem_access_plans_by_site = {}
        return

    int_keys = (
        "block",
        "instruction_index",
        "region",
        "base",
        "value",
        "result",
        "table",
        "index",
        "byte_offset",
        "alignment",
        "element_stride",
        "length",
    )
    for row in rows:
        if not isinstance(row, dict):
            continue
        normalized = dict(row)
        try:
            block = int(normalized.get("block"))
            instruction_index = int(normalized.get("instruction_index"))
        except (TypeError, ValueError):
            continue
        for key in int_keys:
            value = normalized.get(key)
            if value is None:
                normalized[key] = None
                continue
            try:
                normalized[key] = int(value)
            except (TypeError, ValueError):
                pass
        by_site.setdefault((block, instruction_index), []).append(normalized)
    builder.resolver.fastmem_access_plans_by_site = by_site


def _seed_resolver_fact_sets(
    builder,
    context: FunctionLowerContext,
    blocks: List[Dict[str, Any]],
    *,
    collect_non_negative=collect_non_negative_value_ids,
    collect_integerish=collect_integerish_value_ids,
    collect_arrayish=collect_arrayish_value_ids,
    collect_stringish=collect_stringish_value_ids,
) -> None:
    try:
        context.non_negative_value_ids = collect_non_negative(blocks)
        builder.resolver.non_negative_ids = context.non_negative_value_ids
    except (TypeError, ValueError):
        context.non_negative_value_ids = set()
        builder.resolver.non_negative_ids = context.non_negative_value_ids

    try:
        context.integerish_value_ids = collect_integerish(blocks)
        builder.resolver.integerish_ids = context.integerish_value_ids
    except (TypeError, ValueError):
        context.integerish_value_ids = set()
        builder.resolver.integerish_ids = context.integerish_value_ids

    try:
        context.resolver_array_ids = collect_arrayish(blocks)
        builder.resolver.array_ids = context.resolver_array_ids
    except (TypeError, ValueError):
        context.resolver_array_ids = set()
        builder.resolver.array_ids = context.resolver_array_ids

    try:
        inferred_stringish = collect_stringish(blocks)
        context.resolver_string_ids.clear()
        context.resolver_string_ids.update(inferred_stringish)
        builder.resolver.string_ids = context.resolver_string_ids
    except (TypeError, ValueError):
        context.resolver_string_ids.clear()
        builder.resolver.string_ids = context.resolver_string_ids
