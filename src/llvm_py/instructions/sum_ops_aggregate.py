from typing import Any, Dict, Optional

import llvmlite.ir as ir

from instructions.field_access import (
    _canonical_bool_i64,
    _canonical_i64,
    _declare,
    _ensure_handle,
    _field_ptr,
    _mark_bool_immediate,
    _mark_float_immediate,
    _mark_integer_immediate,
    _resolve_receiver,
    _resolve_typed_float_value,
)
from instructions.newbox import lower_newbox
from instructions.primitive_handles import resolver_value_type, unbox_primitive_handle_if_needed
from instructions.sum_runtime import ENUM_PAYLOAD_FIELD, ENUM_TAG_FIELD, runtime_box_name
from instructions.typeop import _emit_trap
from type_facts import is_box_handle_fact, make_box_handle_fact
from utils.resolver_helpers import mark_as_handle
from utils.values import safe_vmap_write

_SAFE_SUM_OPS_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)



from instructions.sum_ops_payload import (
    _new_runtime_sum_handle,
    _payload_kind,
    _payload_handle_value,
    _resolve_payload_value,
    _payload_fact_store,
    _record_sum_payload_fact,
    _sum_payload_fact,
    _resolved_payload_fact,
    _project_payload_fact,
    _declared_payload_fact,
    _runtime_payload_fact,
    _storage_kind_from_fact,
    _apply_payload_fact_to_result,
    _set_i64_field,
    _set_bool_field,
    _set_float_field,
    _set_handle_field,
    _get_i64_field,
    _get_bool_field,
    _get_float_field,
    _get_handle_field,
)
def _is_local_sum_aggregate(value: Any) -> bool:
    return isinstance(value, dict) and value.get("kind") == "local_sum_aggregate"


def _sum_local_aggregate_paths(resolver) -> Dict[int, str]:
    paths = getattr(resolver, "sum_local_aggregate_paths", None)
    if isinstance(paths, dict):
        return paths
    paths = {}
    setattr(resolver, "sum_local_aggregate_paths", paths)
    return paths


def _sum_local_aggregate_layouts(resolver) -> Dict[int, str]:
    layouts = getattr(resolver, "sum_local_aggregate_layouts", None)
    if isinstance(layouts, dict):
        return layouts
    layouts = {}
    setattr(resolver, "sum_local_aggregate_layouts", layouts)
    return layouts


def _sum_uses_local_aggregate(resolver, sum_vid: int) -> bool:
    if resolver is None:
        return False
    return _sum_local_aggregate_paths(resolver).get(int(sum_vid)) == "local_aggregate"


def _sum_local_aggregate_layout_name(
    resolver,
    sum_vid: int,
    payload_type: Optional[str],
    has_payload: bool,
) -> str:
    if resolver is not None:
        layout = _sum_local_aggregate_layouts(resolver).get(int(sum_vid))
        if isinstance(layout, str) and layout:
            return layout
    kind = _payload_kind(payload_type)
    if not has_payload or kind == "Void":
        return "tag_only"
    if kind in {"Integer", "Bool"}:
        return "tag_i64_payload"
    if kind == "Float":
        return "tag_f64_payload"
    return "tag_handle_payload"


def _resolve_local_sum_aggregate(value_vid: int, vmap: Dict[int, Any], resolver):
    direct = vmap.get(int(value_vid))
    if _is_local_sum_aggregate(direct):
        return direct

    try:
        global_vmap = getattr(resolver, "global_vmap", None)
        if isinstance(global_vmap, dict):
            global_value = global_vmap.get(int(value_vid))
            if _is_local_sum_aggregate(global_value):
                return global_value
    except _SAFE_SUM_OPS_EXC:
        pass

    try:
        current_bid = getattr(resolver, "current_block_id", None)
        ctx = getattr(resolver, "context", None)
        if current_bid is not None and ctx is not None and hasattr(ctx, "get_block_snapshot"):
            snapshot = ctx.get_block_snapshot(int(current_bid))
            snap_value = snapshot.get(int(value_vid))
            if _is_local_sum_aggregate(snap_value):
                return snap_value
    except _SAFE_SUM_OPS_EXC:
        pass

    return None


def _copy_local_sum_metadata_alias(resolver, src_vid: int, dst_vid: int) -> None:
    if resolver is None:
        return

    facts = _payload_fact_store(resolver)
    if facts is not None and int(src_vid) in facts:
        facts[int(dst_vid)] = facts[int(src_vid)]

    try:
        paths = _sum_local_aggregate_paths(resolver)
        if int(src_vid) in paths:
            paths[int(dst_vid)] = paths[int(src_vid)]
    except _SAFE_SUM_OPS_EXC:
        pass

    try:
        layouts = _sum_local_aggregate_layouts(resolver)
        if int(src_vid) in layouts:
            layouts[int(dst_vid)] = layouts[int(src_vid)]
    except _SAFE_SUM_OPS_EXC:
        pass


def materialize_local_sum_aggregate(
    builder: ir.IRBuilder,
    module: ir.Module,
    local_sum,
    resolver=None,
    *,
    name_hint: str = "sum_escape",
):
    if not _is_local_sum_aggregate(local_sum):
        return local_sum

    enum_name = local_sum.get("enum_name")
    if not isinstance(enum_name, str) or not enum_name:
        raise RuntimeError("[sum_ops] local sum aggregate is missing enum_name")

    recv_h = _new_runtime_sum_handle(builder, module, enum_name, name_hint=name_hint)
    _set_i64_field(builder, module, recv_h, ENUM_TAG_FIELD, local_sum["tag"])

    payload_value = local_sum.get("payload")
    if payload_value is None:
        return recv_h

    if _is_local_sum_aggregate(payload_value):
        payload_value = materialize_local_sum_aggregate(
            builder,
            module,
            payload_value,
            resolver,
            name_hint=f"{name_hint}_nested",
        )
        _set_handle_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, payload_value)
        return recv_h

    payload_fact = local_sum.get("payload_fact")
    storage_kind = _storage_kind_from_fact(payload_fact)
    if storage_kind == "Integer":
        _set_i64_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, payload_value)
    elif storage_kind == "Bool":
        _set_bool_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, payload_value)
    elif storage_kind == "Float":
        _set_float_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, payload_value)
    else:
        _set_handle_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, payload_value)
    return recv_h


def lower_variant_make(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst_vid: Optional[int],
    enum_name: Optional[str],
    variant: Optional[str],
    tag: Optional[int],
    payload_vid: Optional[int],
    payload_type: Optional[str],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> None:
    if dst_vid is None or not enum_name:
        raise RuntimeError("[sum_ops] variant_make requires dst and variant name")
    local_sum = _try_build_local_sum_aggregate(
        builder,
        module,
        int(dst_vid),
        enum_name,
        variant,
        tag,
        payload_vid,
        payload_type,
        vmap,
        resolver,
        preds,
        block_end_values,
        bb_map,
    )
    if local_sum is not None:
        safe_vmap_write(vmap, int(dst_vid), local_sum, "variant_make_local", resolver=resolver)
        payload_fact = local_sum.get("payload_fact")
        if payload_fact is not None:
            _record_sum_payload_fact(resolver, int(dst_vid), payload_fact)
        return
    lower_newbox(builder, module, runtime_box_name(enum_name), [], int(dst_vid), vmap, resolver)
    recv_h = vmap[int(dst_vid)]
    _set_i64_field(
        builder,
        module,
        recv_h,
        ENUM_TAG_FIELD,
        ir.Constant(ir.IntType(64), int(tag or 0)),
    )

    if payload_vid is None:
        return

    payload_value = _resolve_payload_value(
        builder,
        int(payload_vid),
        vmap,
        resolver,
        preds,
        block_end_values,
        bb_map,
    )
    payload_meta = resolver_value_type(resolver, int(payload_vid))
    payload_fallback_kind = _payload_kind(payload_type)
    payload_fact = _resolved_payload_fact(
        resolver,
        int(payload_vid),
        payload_meta,
        payload_value,
        payload_type,
    )
    storage_kind = _storage_kind_from_fact(payload_fact)
    if storage_kind == "Integer":
        stored = unbox_primitive_handle_if_needed(
            builder,
            _canonical_i64(builder, payload_value, name_hint=f"variant_make_{variant}_payload"),
            payload_meta,
            name_hint=f"variant_make_{variant}_{payload_vid}",
        )
        stored = _canonical_i64(builder, stored, name_hint=f"variant_make_{variant}_i64")
        _set_i64_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, stored)
        _record_sum_payload_fact(resolver, int(dst_vid), payload_fact)
        return
    if storage_kind == "Bool":
        stored = unbox_primitive_handle_if_needed(
            builder,
            _canonical_i64(builder, payload_value, name_hint=f"variant_make_{variant}_payload"),
            payload_meta,
            name_hint=f"variant_make_{variant}_{payload_vid}",
        )
        stored = _canonical_bool_i64(builder, stored, name_hint=f"variant_make_{variant}_bool")
        _set_bool_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, stored)
        _record_sum_payload_fact(resolver, int(dst_vid), payload_fact)
        return
    if storage_kind == "Float":
        stored = _resolve_typed_float_value(
            builder,
            payload_value,
            payload_meta,
            name_hint=f"variant_make_{variant}_float",
        )
        _set_float_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, stored)
        _record_sum_payload_fact(resolver, int(dst_vid), payload_fact)
        return

    handle_value = _payload_handle_value(
        builder,
        module,
        int(payload_vid),
        payload_value,
        payload_meta,
        payload_fact,
        payload_fallback_kind,
    )
    _set_handle_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, handle_value)
    _record_sum_payload_fact(resolver, int(dst_vid), payload_fact)


def lower_variant_tag(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst_vid: Optional[int],
    value_vid: Optional[int],
    enum_name: Optional[str],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> None:
    if dst_vid is None or value_vid is None or not enum_name:
        raise RuntimeError("[sum_ops] variant_tag requires dst, value, and variant name")
    local_sum = _resolve_local_sum_aggregate(int(value_vid), vmap, resolver)
    if _is_local_sum_aggregate(local_sum):
        if local_sum.get("enum_name") != enum_name:
            raise RuntimeError(
                f"[sum_ops] local sum aggregate enum mismatch: expected {enum_name}, got {local_sum.get('enum_name')}"
            )
        safe_vmap_write(vmap, int(dst_vid), local_sum["tag"], "variant_tag_local", resolver=resolver)
        _mark_integer_immediate(resolver, int(dst_vid))
        return
    recv_val = _resolve_receiver(
        builder,
        int(value_vid),
        vmap,
        resolver,
        preds,
        block_end_values,
        bb_map,
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    result = _get_i64_field(builder, module, recv_h, ENUM_TAG_FIELD, name_hint="variant_tag")
    safe_vmap_write(vmap, int(dst_vid), result, "variant_tag")
    _mark_integer_immediate(resolver, int(dst_vid))


def lower_variant_project(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst_vid: Optional[int],
    value_vid: Optional[int],
    enum_name: Optional[str],
    variant: Optional[str],
    tag: Optional[int],
    payload_type: Optional[str],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> None:
    if dst_vid is None or value_vid is None or not enum_name:
        raise RuntimeError("[sum_ops] variant_project requires dst, value, and variant name")
    local_sum = _resolve_local_sum_aggregate(int(value_vid), vmap, resolver)
    if _is_local_sum_aggregate(local_sum):
        if local_sum.get("enum_name") != enum_name:
            raise RuntimeError(
                f"[sum_ops] local sum aggregate enum mismatch: expected {enum_name}, got {local_sum.get('enum_name')}"
            )
        actual_tag = local_sum["tag"]
        expected_tag = ir.Constant(ir.IntType(64), int(tag or 0))
        is_match = builder.icmp_unsigned("==", actual_tag, expected_tag, name=f"variant_tag_match_{dst_vid}")

        fn = builder.function
        trap_bb = fn.append_basic_block(name=f"variant_project_fail_{dst_vid}")
        ok_bb = fn.append_basic_block(name=f"variant_project_ok_{dst_vid}")
        builder.cbranch(is_match, ok_bb, trap_bb)

        builder.position_at_end(trap_bb)
        _emit_trap(builder)

        builder.position_at_end(ok_bb)
        payload_fact = _declared_payload_fact(payload_type) or local_sum.get("payload_fact")
        payload_value = local_sum.get("payload")
        storage_kind = _storage_kind_from_fact(payload_fact)
        if storage_kind in {"Integer", "Bool", "Float"}:
            if payload_value is None:
                raise RuntimeError("[sum_ops] selected local sum payload is missing")
            safe_vmap_write(vmap, int(dst_vid), payload_value, f"variant_project_local_{storage_kind.lower()}", resolver=resolver)
            _apply_payload_fact_to_result(resolver, int(dst_vid), payload_fact)
            return
        if payload_value is None:
            raise RuntimeError("[sum_ops] selected local sum handle payload is missing")
        safe_vmap_write(vmap, int(dst_vid), payload_value, "variant_project_local_handle", resolver=resolver)
        _apply_payload_fact_to_result(resolver, int(dst_vid), payload_fact)
        return
    recv_val = _resolve_receiver(
        builder,
        int(value_vid),
        vmap,
        resolver,
        preds,
        block_end_values,
        bb_map,
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    actual_tag = _get_i64_field(builder, module, recv_h, ENUM_TAG_FIELD, name_hint="variant_project_tag")
    expected_tag = ir.Constant(ir.IntType(64), int(tag or 0))
    is_match = builder.icmp_unsigned("==", actual_tag, expected_tag, name=f"variant_tag_match_{dst_vid}")

    fn = builder.function
    trap_bb = fn.append_basic_block(name=f"variant_project_fail_{dst_vid}")
    ok_bb = fn.append_basic_block(name=f"variant_project_ok_{dst_vid}")
    builder.cbranch(is_match, ok_bb, trap_bb)

    builder.position_at_end(trap_bb)
    _emit_trap(builder)

    builder.position_at_end(ok_bb)
    payload_fact = _project_payload_fact(resolver, int(value_vid), payload_type)
    storage_kind = _storage_kind_from_fact(payload_fact)
    if storage_kind == "Integer":
        result = _get_i64_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, name_hint="variant_project_i64")
        safe_vmap_write(vmap, int(dst_vid), result, "variant_project_i64")
        _apply_payload_fact_to_result(resolver, int(dst_vid), payload_fact)
        return
    if storage_kind == "Bool":
        result = _get_bool_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, name_hint="variant_project_bool")
        safe_vmap_write(vmap, int(dst_vid), result, "variant_project_bool")
        _apply_payload_fact_to_result(resolver, int(dst_vid), payload_fact)
        return
    if storage_kind == "Float":
        result = _get_float_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, name_hint="variant_project_float")
        safe_vmap_write(vmap, int(dst_vid), result, "variant_project_float")
        _apply_payload_fact_to_result(resolver, int(dst_vid), payload_fact)
        return

    result = _get_handle_field(builder, module, recv_h, ENUM_PAYLOAD_FIELD, name_hint="variant_project_handle")
    safe_vmap_write(vmap, int(dst_vid), result, "variant_project_handle")
    _apply_payload_fact_to_result(resolver, int(dst_vid), payload_fact)


def _try_build_local_sum_aggregate(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst_vid: int,
    enum_name: str,
    variant: Optional[str],
    tag: Optional[int],
    payload_vid: Optional[int],
    payload_type: Optional[str],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
):
    if not _sum_uses_local_aggregate(resolver, int(dst_vid)):
        return None

    layout = _sum_local_aggregate_layout_name(
        resolver,
        int(dst_vid),
        payload_type,
        payload_vid is not None,
    )
    tag_value = ir.Constant(ir.IntType(64), int(tag or 0))
    payload_value = None
    payload_fact = None

    if payload_vid is not None:
        raw_payload = _resolve_payload_value(
            builder,
            int(payload_vid),
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
        payload_meta = resolver_value_type(resolver, int(payload_vid))
        payload_fallback_kind = _payload_kind(payload_type)
        payload_fact = _resolved_payload_fact(
            resolver,
            int(payload_vid),
            payload_meta,
            raw_payload,
            payload_type,
        )
        storage_kind = _storage_kind_from_fact(payload_fact)

        if layout == "tag_i64_payload":
            if storage_kind == "Integer":
                payload_value = unbox_primitive_handle_if_needed(
                    builder,
                    _canonical_i64(builder, raw_payload, name_hint=f"variant_make_{variant}_payload"),
                    payload_meta,
                    name_hint=f"variant_make_{variant}_{payload_vid}",
                )
                payload_value = _canonical_i64(
                    builder, payload_value, name_hint=f"variant_make_{variant}_i64_local"
                )
            elif storage_kind == "Bool":
                payload_value = unbox_primitive_handle_if_needed(
                    builder,
                    _canonical_i64(builder, raw_payload, name_hint=f"variant_make_{variant}_payload"),
                    payload_meta,
                    name_hint=f"variant_make_{variant}_{payload_vid}",
                )
                payload_value = _canonical_bool_i64(
                    builder, payload_value, name_hint=f"variant_make_{variant}_bool_local"
                )
            else:
                return None
        elif layout == "tag_f64_payload":
            if storage_kind != "Float":
                return None
            payload_value = _resolve_typed_float_value(
                builder,
                raw_payload,
                payload_meta,
                name_hint=f"variant_make_{variant}_float_local",
            )
        elif layout == "tag_handle_payload":
            if _is_local_sum_aggregate(raw_payload):
                return None
            payload_value = _payload_handle_value(
                builder,
                module,
                int(payload_vid),
                raw_payload,
                payload_meta,
                payload_fact,
                payload_fallback_kind,
            )
        elif layout == "tag_only":
            payload_value = None
        else:
            return None

    return {
        "kind": "local_sum_aggregate",
        "enum_name": enum_name,
        "tag": tag_value,
        "payload": payload_value,
        "payload_fact": payload_fact,
        "layout": layout,
    }



