from typing import Any, Dict, List, Optional

from cfg.utils import (
    collect_arrayish_value_ids,
    collect_integerish_value_ids,
    collect_non_negative_value_ids,
    collect_stringish_value_ids,
)
from context import FunctionLowerContext
from instructions.user_box_local import seed_local_user_box_layouts_from_function_data


def _load_value_types_metadata(builder, func_data: Dict[str, Any]) -> None:
    try:
        metadata = func_data.get("metadata", {})
        value_types_json = metadata.get("value_types", {})
        builder.resolver.value_types = {}
        for vid_str, vtype in value_types_json.items():
            try:
                builder.resolver.value_types[int(vid_str)] = vtype
            except (ValueError, TypeError):
                pass
    except Exception:
        builder.resolver.value_types = {}


def _load_thin_entry_selection_metadata(builder, func_data: Dict[str, Any]) -> None:
    try:
        metadata = func_data.get("metadata", {})
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
        for row in placement_effect_routes:
            try:
                normalized = _thin_entry_row_from_placement_effect_route(row)
                if isinstance(normalized, dict):
                    add_row(normalized)
            except Exception:
                pass

        for row in rows:
            try:
                if not isinstance(row, dict):
                    continue
                normalized = dict(row)
                value = normalized.get("value")
                if isinstance(value, int):
                    normalized["value"] = int(value)
                else:
                    normalized["value"] = None
                add_row(normalized)
            except Exception:
                pass

        builder.resolver.thin_entry_selections = normalized_rows
        builder.resolver.thin_entry_selection_by_value = by_value
        builder.resolver.thin_entry_selection_by_subject = by_subject
    except Exception:
        builder.resolver.thin_entry_selections = []
        builder.resolver.thin_entry_selection_by_value = {}
        builder.resolver.thin_entry_selection_by_subject = {}


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
    try:
        metadata = func_data.get("metadata", {})
        placement_effect_routes = metadata.get("placement_effect_routes", [])
        selections = metadata.get("sum_placement_selections", [])
        layouts = metadata.get("sum_placement_layouts", [])

        local_paths = {}
        for row in placement_effect_routes:
            try:
                if not isinstance(row, dict):
                    continue
                if row.get("source") != "sum_placement":
                    continue
                if row.get("decision") != "local_aggregate":
                    continue
                if row.get("detail") != "variant_make.local_aggregate":
                    continue
                value = row.get("value")
                if isinstance(value, int):
                    local_paths[int(value)] = "local_aggregate"
            except Exception:
                pass

        for row in selections:
            try:
                if row.get("surface") != "variant_make":
                    continue
                if row.get("selected_path") != "local_aggregate":
                    continue
                value = row.get("value")
                if isinstance(value, int) and int(value) not in local_paths:
                    local_paths[int(value)] = "local_aggregate"
            except Exception:
                pass

        local_layouts = {}
        for row in placement_effect_routes:
            try:
                if not isinstance(row, dict):
                    continue
                if row.get("source") != "agg_local_scalarization":
                    continue
                if row.get("decision") != "local_aggregate":
                    continue
                detail = row.get("detail")
                layout = _sum_layout_from_placement_effect_detail(detail)
                value = row.get("value")
                if isinstance(value, int) and isinstance(layout, str):
                    local_layouts[int(value)] = layout
            except Exception:
                pass

        for row in layouts:
            try:
                if row.get("surface") != "variant_make":
                    continue
                value = row.get("value")
                layout = row.get("layout")
                if (
                    isinstance(value, int)
                    and isinstance(layout, str)
                    and int(value) not in local_layouts
                ):
                    local_layouts[int(value)] = layout
            except Exception:
                pass

        builder.resolver.sum_local_aggregate_paths = local_paths
        builder.resolver.sum_local_aggregate_layouts = local_layouts
    except Exception:
        builder.resolver.sum_local_aggregate_paths = {}
        builder.resolver.sum_local_aggregate_layouts = {}


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
    try:
        seed_local_user_box_layouts_from_function_data(builder, func_data)
    except Exception:
        builder.resolver.user_box_local_aggregate_layouts = {}


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
    except Exception:
        context.non_negative_value_ids = set()
        builder.resolver.non_negative_ids = context.non_negative_value_ids

    try:
        context.integerish_value_ids = collect_integerish(blocks)
        builder.resolver.integerish_ids = context.integerish_value_ids
    except Exception:
        context.integerish_value_ids = set()
        builder.resolver.integerish_ids = context.integerish_value_ids

    try:
        context.resolver_array_ids = collect_arrayish(blocks)
        builder.resolver.array_ids = context.resolver_array_ids
    except Exception:
        context.resolver_array_ids = set()
        builder.resolver.array_ids = context.resolver_array_ids

    try:
        inferred_stringish = collect_stringish(blocks)
        context.resolver_string_ids.clear()
        context.resolver_string_ids.update(inferred_stringish)
        builder.resolver.string_ids = context.resolver_string_ids
    except Exception:
        context.resolver_string_ids.clear()
        builder.resolver.string_ids = context.resolver_string_ids
