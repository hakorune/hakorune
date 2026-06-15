from typing import Any, Dict, Optional

import llvmlite.ir as ir

from instructions import flattened_nested_fields as _flattened_nested_fields
from instructions.field_access_helpers import (
    _boxed_field_key,
    _canonical_bool_i64,
    _canonical_i64,
    _current_function_name,
    _declare,
    _declared_type_matches_box_type,
    _direct_slot_nativedirect_selected,
    _direct_slot_nativedirect_storage_supported,
    _exact_field_plan_for_receiver,
    _ensure_handle,
    _field_ptr,
    _floatish_llvm_value,
    _is_bool_immediate_meta,
    _is_exact_slot_u64_storage,
    _is_float_immediate_meta,
    _lower_direct_slot_nativedirect_get,
    _lower_direct_slot_nativedirect_set,
    _lower_exact_object_field_get,
    _lower_exact_object_field_set,
    _lower_typed_bool_field_get,
    _lower_typed_bool_field_set,
    _lower_typed_float_field_get,
    _lower_typed_float_field_set,
    _lower_typed_integer_field_get,
    _lower_typed_integer_field_set,
    _mark_bool_immediate,
    _mark_float_immediate,
    _mark_integer_immediate,
    _receiver_box_type,
    _resolve_receiver,
    _resolve_typed_float_value,
    _typed_bool_field_enabled,
    _typed_bool_field_set_enabled,
    _typed_float_field_enabled,
    _typed_float_field_set_enabled,
    _typed_integer_field_enabled,
    _typed_user_box_field_enabled,
)
from instructions.mir_call.runtime_data_dispatch import lower_runtime_data_field_call
from instructions.primitive_handles import resolver_value_type, unbox_primitive_handle_if_needed
from instructions.thin_entry_selection import thin_entry_prefers_inline_scalar_field
from instructions.typed_object_exact import (
    exact_field_plan_for_box,
    is_handle_storage,
    is_signed_storage,
    is_unsigned_storage,
)
from instructions.user_box_local import (
    lower_local_user_box_field_get,
    lower_local_user_box_field_set,
)
from type_facts import is_box_handle_fact
from utils.resolver_helpers import mark_as_handle
from utils.values import resolve_i64_strict

_EXACT_NUMERIC_RUNTIME_RANGES = {
    "i8": (-128, 127),
    "i16": (-(1 << 15), (1 << 15) - 1),
    "i32": (-(1 << 31), (1 << 31) - 1),
    "i64": (-(1 << 63), (1 << 63) - 1),
    "isize": (-(1 << 63), (1 << 63) - 1),
    "u8": (0, (1 << 8) - 1),
    "u16": (0, (1 << 16) - 1),
    "u32": (0, (1 << 32) - 1),
    "u64": (0, None),
    "usize": (0, None),
}


def _flattened_nested_field_access_route_enabled() -> bool:
    """Enabled route hook for flattened nested field state.

    The passive seam rows kept lowering disabled.  Once the guarded pilot flips
    the backend lowering flag, field access must route through the shared
    flattened state; otherwise the enabled pilot cannot reach generated code.
    """

    return (
        _flattened_nested_fields.FLATTENED_NESTED_FIELD_STATE_SEAM_DEFINED
        and _flattened_nested_fields.FLATTENED_NESTED_FIELD_LOWERING_ENABLED
    )


def _flattened_nested_owner_box_name(resolver, box_vid: Optional[int], field_name: str) -> Optional[str]:
    """Resolve the owner box for the guarded flattened-nested-field route.

    The normal receiver type helper only reads the direct value type.  The
    selected object-lifecycle front often reaches field access through a Copy,
    while the canonical receiver type is still available in the current route
    decision.  Use that metadata only for this ObjectStoragePlan consumer so the
    broader typed exact-slot route remains unchanged.
    """

    owner_box_name = _receiver_box_type(resolver, box_vid)
    if owner_box_name is not None:
        return owner_box_name
    if resolver is None:
        return None
    try:
        block_id = int(getattr(resolver, "current_block_id"))
        instruction_index = int(getattr(resolver, "current_instruction_index"))
    except (AttributeError, TypeError, ValueError):
        return None
    decisions_by_site = getattr(resolver, "route_decisions_by_site", None)
    if not isinstance(decisions_by_site, dict):
        return None
    decisions = decisions_by_site.get((block_id, instruction_index), [])
    if not isinstance(decisions, list):
        return None
    for decision in decisions:
        if not isinstance(decision, dict):
            continue
        if decision.get("field_id") not in (None, field_name):
            continue
        candidate = decision.get("receiver_box_name")
        if isinstance(candidate, str) and candidate:
            return candidate
    return None


def _emit_exact_numeric_runtime_range_check(
    builder: ir.IRBuilder,
    module: ir.Module,
    value_vid: Optional[int],
    runtime_check: Any,
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> None:
    if not isinstance(runtime_check, dict):
        return
    if runtime_check.get("kind") != "dynamic_integer_range":
        return
    declared_type = runtime_check.get("declared_type")
    if not isinstance(declared_type, str):
        raise RuntimeError("[exact-numeric/runtime-check] missing declared_type")
    range_key = declared_type.strip().lower()
    if range_key not in _EXACT_NUMERIC_RUNTIME_RANGES:
        raise RuntimeError(
            f"[exact-numeric/runtime-check] unsupported declared_type={declared_type}"
        )
    if not isinstance(value_vid, int):
        raise RuntimeError("[exact-numeric/runtime-check] missing value id")

    i64 = ir.IntType(64)
    value_val = _resolve_receiver(
        builder, value_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    value_val = unbox_primitive_handle_if_needed(
        builder,
        _canonical_i64(builder, value_val, name_hint="exact_runtime_range_value"),
        resolver_value_type(resolver, int(value_vid)),
        name_hint=f"exact_runtime_range_{value_vid}",
    )
    value_val = _canonical_i64(builder, value_val, name_hint="exact_runtime_range_final")

    min_value, max_value = _EXACT_NUMERIC_RUNTIME_RANGES[range_key]
    if min_value == -(1 << 63) and (max_value is None or max_value == (1 << 63) - 1):
        return
    if max_value is None or max_value == (1 << 63) - 1:
        callee = _declare(
            module,
            "nyash.exact_numeric.assert_i64_min_ii",
            i64,
            [i64, i64],
        )
        builder.call(callee, [value_val, ir.Constant(i64, int(min_value))])
        return

    callee = _declare(
        module,
        "nyash.exact_numeric.assert_i64_range_iii",
        i64,
        [i64, i64, i64],
    )
    builder.call(
        callee,
        [
            value_val,
            ir.Constant(i64, int(min_value)),
            ir.Constant(i64, int(max_value)),
        ],
    )


def lower_field_get(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    dst_vid: Optional[int],
    declared_type: Any,
    user_box_decls: Any,
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    owner_box_name = _flattened_nested_owner_box_name(resolver, box_vid, field_name)
    if _flattened_nested_field_access_route_enabled():
        owner_val = _resolve_receiver(
            builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
        )
        owner_h = _ensure_handle(builder, module, owner_val)
        flattened_result = _flattened_nested_fields.try_lower_owner_field_get(
            owner_box_name=owner_box_name,
            field_name=field_name,
            owner_handle=owner_h,
        )
        if flattened_result is not None:
            if dst_vid is not None:
                vmap[int(dst_vid)] = flattened_result
            return flattened_result
    local_result = lower_local_user_box_field_get(
        builder,
        box_vid,
        field_name,
        dst_vid,
        vmap,
        resolver,
        _mark_integer_immediate,
        _mark_bool_immediate,
        _mark_float_immediate,
    )
    if local_result is not None:
        return local_result
    exact_field_plan = _exact_field_plan_for_receiver(resolver, box_vid, field_name)
    if exact_field_plan is not None:
        exact_result = _lower_exact_object_field_get(
            builder,
            module,
            box_vid,
            exact_field_plan,
            dst_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
        if exact_result is not None:
            return exact_result
    if _typed_float_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        thin_entry_surface="user_box_field_get",
        selection_value_id=dst_vid,
    ):
        return _lower_typed_float_field_get(
            builder,
            module,
            box_vid,
            field_name,
            dst_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
    if _typed_bool_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        thin_entry_surface="user_box_field_get",
        selection_value_id=dst_vid,
    ):
        return _lower_typed_bool_field_get(
            builder,
            module,
            box_vid,
            field_name,
            dst_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
    if _typed_integer_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        thin_entry_surface="user_box_field_get",
        selection_value_id=dst_vid,
    ):
        return _lower_typed_integer_field_get(
            builder,
            module,
            box_vid,
            field_name,
            dst_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )

    i64 = ir.IntType(64)
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    key_h = _boxed_field_key(builder, module, field_name)
    result = lower_runtime_data_field_call(
        builder=builder,
        declare=lambda name, ret, args: _declare(module, name, ret, args),
        box_name="RuntimeDataBox",
        method="getField",
        recv_h=recv_h,
        args=[key_h],
        resolve_arg=None,
        ensure_handle=None,
    )
    if result is None:
        result = ir.Constant(i64, 0)
    if dst_vid is not None:
        vmap[dst_vid] = result
        mark_as_handle(resolver, int(dst_vid))
    return result


def lower_field_set(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    declared_type: Any,
    user_box_decls: Any,
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
    exact_numeric_runtime_check: Any = None,
) -> ir.Value:
    _emit_exact_numeric_runtime_range_check(
        builder,
        module,
        value_vid,
        exact_numeric_runtime_check,
        vmap,
        resolver,
        preds,
        block_end_values,
        bb_map,
    )
    owner_box_name = _flattened_nested_owner_box_name(resolver, box_vid, field_name)
    if _flattened_nested_field_access_route_enabled():
        owner_val = _resolve_receiver(
            builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
        )
        owner_h = _ensure_handle(builder, module, owner_val)
        flattened_result = _flattened_nested_fields.try_lower_owner_field_set(
            builder=builder,
            module=module,
            owner_box_name=owner_box_name,
            field_name=field_name,
            owner_handle=owner_h,
        )
        if flattened_result is not None:
            return flattened_result
    if lower_local_user_box_field_set(
        builder,
        box_vid,
        field_name,
        value_vid,
        vmap,
        resolver,
        preds,
        block_end_values,
        bb_map,
    ):
        return ir.Constant(ir.IntType(64), 0)
    exact_field_plan = _exact_field_plan_for_receiver(resolver, box_vid, field_name)
    if exact_field_plan is not None:
        exact_result = _lower_exact_object_field_set(
            builder,
            module,
            box_vid,
            value_vid,
            exact_field_plan,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
        if exact_result is not None:
            return exact_result
    if _typed_float_field_set_enabled(
        box_vid=box_vid,
        field_name=field_name,
        value_vid=value_vid,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        vmap=vmap,
        resolver=resolver,
    ):
        return _lower_typed_float_field_set(
            builder,
            module,
            box_vid,
            field_name,
            value_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
    if _typed_bool_field_set_enabled(
        box_vid=box_vid,
        field_name=field_name,
        value_vid=value_vid,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        vmap=vmap,
        resolver=resolver,
    ):
        return _lower_typed_bool_field_set(
            builder,
            module,
            box_vid,
            field_name,
            value_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
    if _typed_integer_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        thin_entry_surface="user_box_field_set",
    ):
        return _lower_typed_integer_field_set(
            builder,
            module,
            box_vid,
            field_name,
            value_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )

    i64 = ir.IntType(64)
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    key_h = _boxed_field_key(builder, module, field_name)
    value_val = _resolve_receiver(
        builder, value_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    value_h = _ensure_handle(builder, module, value_val)
    result = lower_runtime_data_field_call(
        builder=builder,
        declare=lambda name, ret, args: _declare(module, name, ret, args),
        box_name="RuntimeDataBox",
        method="setField",
        recv_h=recv_h,
        args=[key_h, value_h],
        resolve_arg=None,
        ensure_handle=None,
    )
    if result is None:
        result = ir.Constant(i64, 0)
    return result
