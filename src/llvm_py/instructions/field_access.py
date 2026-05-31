from typing import Any, Dict, Optional

import llvmlite.ir as ir

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
    _exact_slot_helper_enabled,
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
from instructions.typeop import _emit_trap
from type_facts import is_box_handle_fact
from utils.resolver_helpers import mark_as_handle
from utils.values import resolve_i64_strict

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
) -> ir.Value:
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
