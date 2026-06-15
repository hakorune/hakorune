"""Passive exact-AOT seams for flattened nested object fields.

This module intentionally does not lower anything yet.  It gives the backend
one place to validate a future ObjectStoragePlan::FlattenedNestedFields payload
and its state-sharing contract before the first guarded pilot enables
execution.
"""

from __future__ import annotations

import hashlib
from typing import Any, Dict, Iterable, List

import llvmlite.ir as ir

from instructions.llvm_decl import declare_function as _declare
from instructions.mir_call.runtime_data_dispatch import lower_runtime_data_field_call
from instructions.primitive_handles import unbox_primitive_handle_if_needed


SUPPORTED_REPRESENTATION = "flatten_nested_fields"
BACKEND_FLATTENED_NESTED_FIELD_CONSUMER = True
FLATTENED_NESTED_FIELD_LOWERING_ENABLED = True
FLATTENED_NESTED_FIELD_STATE_SEAM_DEFINED = True

OWNER_BOX = "HakoAllocObjectLifecycleFacade"
OWNER_FIELD = "alignment_result"
NESTED_OBJECT = "HakoAllocObjectLifecycleAlignmentResult"
NESTED_FIELDS = (
    "last_requested",
    "last_normalized",
    "last_reason",
    "last_supported",
)
READ_METHOD_TO_FIELD = {
    "requested": "last_requested",
    "normalized": "last_normalized",
    "reason": "last_reason",
    "supported": "last_supported",
}
WRITE_METHODS = {
    "recordFailure": ("last_requested", "last_normalized", "last_reason", "last_supported"),
    "recordSuccess": ("last_requested", "last_normalized", "last_reason", "last_supported"),
    "reset": ("last_requested", "last_normalized", "last_reason", "last_supported"),
}


def validate_flattened_nested_field_plan(plan: Dict[str, Any]) -> Dict[str, Any]:
    """Validate passive plan shape without rewriting MIR or emitting LLVM IR."""

    representation = plan.get("representation_choice")
    fields = plan.get("fields")
    valid_representation = representation == SUPPORTED_REPRESENTATION
    valid_fields = isinstance(fields, list) and all(
        isinstance(field, dict)
        and isinstance(field.get("flattened_name"), str)
        and isinstance(field.get("scalar_type"), str)
        for field in fields
    )
    return {
        "backend_flattened_nested_field_consumer": int(
            BACKEND_FLATTENED_NESTED_FIELD_CONSUMER
        ),
        "backend_lowering_enabled": int(FLATTENED_NESTED_FIELD_LOWERING_ENABLED),
        "representation_choice": representation,
        "valid_representation": int(valid_representation),
        "valid_fields": int(valid_fields),
        "flattened_nested_field_count": len(fields) if isinstance(fields, list) else 0,
    }


def build_passive_flattened_nested_field_plan(
    *, owner_field: str, nested_object: str, flattened_names: Iterable[str]
) -> Dict[str, Any]:
    """Build the backend-facing passive plan payload for tests/tools."""

    fields: List[Dict[str, str]] = [
        {"flattened_name": name, "scalar_type": "I64"} for name in flattened_names
    ]
    return {
        "representation_choice": SUPPORTED_REPRESENTATION,
        "owner_field": owner_field,
        "nested_object": nested_object,
        "fields": fields,
    }


def build_passive_flattened_nested_state_plan(
    *,
    owner_box: str,
    owner_field: str,
    nested_object: str,
    flattened_names: Iterable[str],
) -> Dict[str, Any]:
    """Build the shared-state contract for a flattened nested object field.

    The future lowering route has three consumers that must agree on one state
    identity:

    - owner birth writes the nested field through `field_set owner_field`
    - owner methods read the nested object through `field_get owner_field`
    - nested object methods read/write primitive fields on the flattened state

    v0 only names this contract.  It does not rewrite field access, method
    calls, or newbox lowering.
    """

    fields: List[Dict[str, str]] = [
        {"flattened_name": name, "scalar_type": "I64"} for name in flattened_names
    ]
    state_id = f"{owner_box}.{owner_field}->{nested_object}"
    return {
        "representation_choice": SUPPORTED_REPRESENTATION,
        "state_id": state_id,
        "owner_box": owner_box,
        "owner_field": owner_field,
        "nested_object": nested_object,
        "fields": fields,
        "owner_field_set_route": "future_field_access_consumer",
        "owner_field_get_route": "future_field_access_consumer",
        "nested_method_route": "future_method_call_consumer",
        "field_access_route_enabled": False,
        "method_call_route_enabled": False,
        "backend_lowering_enabled": FLATTENED_NESTED_FIELD_LOWERING_ENABLED,
    }


def validate_flattened_nested_state_plan(plan: Dict[str, Any]) -> Dict[str, Any]:
    """Validate the passive shared-state contract without enabling lowering."""

    fields = plan.get("fields")
    valid_fields = isinstance(fields, list) and all(
        isinstance(field, dict)
        and isinstance(field.get("flattened_name"), str)
        and isinstance(field.get("scalar_type"), str)
        for field in fields
    )
    valid_state = (
        plan.get("representation_choice") == SUPPORTED_REPRESENTATION
        and isinstance(plan.get("state_id"), str)
        and isinstance(plan.get("owner_box"), str)
        and isinstance(plan.get("owner_field"), str)
        and isinstance(plan.get("nested_object"), str)
        and valid_fields
    )
    return {
        "state_sharing_seam_defined": int(
            FLATTENED_NESTED_FIELD_STATE_SEAM_DEFINED and valid_state
        ),
        "backend_lowering_enabled": int(FLATTENED_NESTED_FIELD_LOWERING_ENABLED),
        "field_access_flattened_nested_route_enabled": int(
            bool(plan.get("field_access_route_enabled"))
        ),
        "method_call_flattened_nested_route_enabled": int(
            bool(plan.get("method_call_route_enabled"))
        ),
        "representation_choice": plan.get("representation_choice"),
        "valid_state": int(valid_state),
        "valid_fields": int(valid_fields),
        "flattened_nested_field_count": len(fields) if isinstance(fields, list) else 0,
    }


def _field_ptr(builder: ir.IRBuilder, module: ir.Module, field_name: str) -> ir.Value:
    i8 = ir.IntType(8)
    i32 = ir.IntType(32)
    text = str(field_name or "")
    digest = hashlib.sha1(text.encode("utf-8")).hexdigest()[:12]
    global_name = f".flattened_field_lit_{digest}"
    data = (text + "\0").encode("utf-8")
    arr_ty = ir.ArrayType(i8, len(data))

    existing = None
    for global_value in module.global_values:
        if global_value.name == global_name:
            existing = global_value
            break

    if existing is None:
        global_var = ir.GlobalVariable(module, arr_ty, name=global_name)
        global_var.linkage = "private"
        global_var.global_constant = True
        global_var.initializer = ir.Constant(arr_ty, bytearray(data))
    else:
        global_var = existing

    c0 = ir.Constant(i32, 0)
    return builder.gep(global_var, [c0, c0], inbounds=True)


def _boxed_field_key(builder: ir.IRBuilder, module: ir.Module, field_name: str) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    callee = _declare(module, "nyash.box.from_i8_string", i64, [i8p])
    return builder.call(
        callee,
        [_field_ptr(builder, module, field_name)],
        name="flattened_field_name_h",
    )


def _synthetic_field_name(nested_field: str) -> str:
    return f"{OWNER_FIELD}.{nested_field}"


def _ensure_i64(builder: ir.IRBuilder, value: Any) -> ir.Value:
    i64 = ir.IntType(64)
    if value is None:
        return ir.Constant(i64, 0)
    vtype = getattr(value, "type", None)
    if isinstance(vtype, ir.IntType):
        if vtype.width < 64:
            return builder.zext(value, i64, name="flattened_i64_zext")
        if vtype.width > 64:
            return builder.trunc(value, i64, name="flattened_i64_trunc")
        return value
    if isinstance(vtype, ir.PointerType):
        return builder.ptrtoint(value, i64, name="flattened_i64_p2i")
    return value


def _runtime_get_field(builder: ir.IRBuilder, module: ir.Module, owner_h: ir.Value, field_name: str):
    result = lower_runtime_data_field_call(
        builder=builder,
        declare=lambda name, ret, args: _declare(module, name, ret, args),
        box_name="RuntimeDataBox",
        method="getField",
        recv_h=owner_h,
        args=[_boxed_field_key(builder, module, field_name)],
        resolve_arg=None,
        ensure_handle=None,
    )
    return result if result is not None else ir.Constant(ir.IntType(64), 0)


def _runtime_set_field(
    builder: ir.IRBuilder,
    module: ir.Module,
    owner_h: ir.Value,
    field_name: str,
    value: Any,
):
    result = lower_runtime_data_field_call(
        builder=builder,
        declare=lambda name, ret, args: _declare(module, name, ret, args),
        box_name="RuntimeDataBox",
        method="setField",
        recv_h=owner_h,
        args=[_boxed_field_key(builder, module, field_name), _ensure_i64(builder, value)],
        resolve_arg=None,
        ensure_handle=None,
    )
    return result if result is not None else ir.Constant(ir.IntType(64), 0)


def is_flattened_nested_view(value: Any) -> bool:
    return isinstance(value, dict) and value.get("kind") == "flattened_nested_field_state"


def try_lower_owner_field_get(
    *,
    owner_box_name: Any,
    field_name: str,
    owner_handle: ir.Value,
):
    if not FLATTENED_NESTED_FIELD_LOWERING_ENABLED:
        return None
    if owner_box_name != OWNER_BOX or field_name != OWNER_FIELD:
        return None
    return {
        "kind": "flattened_nested_field_state",
        "state_id": f"{OWNER_BOX}.{OWNER_FIELD}->{NESTED_OBJECT}",
        "owner_box": OWNER_BOX,
        "owner_field": OWNER_FIELD,
        "nested_object": NESTED_OBJECT,
        "owner_handle": owner_handle,
    }


def try_lower_owner_field_set(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    owner_box_name: Any,
    field_name: str,
    owner_handle: ir.Value,
):
    if not FLATTENED_NESTED_FIELD_LOWERING_ENABLED:
        return None
    if owner_box_name != OWNER_BOX or field_name != OWNER_FIELD:
        return None
    zero = ir.Constant(ir.IntType(64), 0)
    result = zero
    for nested_field in NESTED_FIELDS:
        result = _runtime_set_field(
            builder,
            module,
            owner_handle,
            _synthetic_field_name(nested_field),
            zero,
        )
    return result


def try_lower_nested_method_call(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    receiver: Any,
    method_name: Any,
    args: List[int],
    resolve_arg,
    dst_vid: Any,
    vmap: Dict[int, Any],
    resolver,
):
    if not FLATTENED_NESTED_FIELD_LOWERING_ENABLED:
        return None
    if not is_flattened_nested_view(receiver):
        return None
    if receiver.get("nested_object") != NESTED_OBJECT:
        return None
    owner_h = receiver.get("owner_handle")
    if owner_h is None:
        return None

    if method_name in READ_METHOD_TO_FIELD and len(args) == 0:
        result = _runtime_get_field(
            builder,
            module,
            owner_h,
            _synthetic_field_name(READ_METHOD_TO_FIELD[str(method_name)]),
        )
        if isinstance(dst_vid, int):
            vmap[int(dst_vid)] = result
            try:
                if hasattr(resolver, "mark_integer"):
                    resolver.mark_integer(int(dst_vid))
            except (AttributeError, TypeError, ValueError):
                pass
        return result

    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    if method_name == "reset" and len(args) == 0:
        result = zero
        for nested_field in WRITE_METHODS["reset"]:
            result = _runtime_set_field(
                builder,
                module,
                owner_h,
                _synthetic_field_name(nested_field),
                zero,
            )
        return result

    if method_name == "recordFailure" and len(args) == 2:
        requested = _ensure_i64(builder, resolve_arg(args[0]))
        normalized = _ensure_i64(builder, resolve_arg(args[1]))
        reason = ir.Constant(i64, 1)
        supported = zero
    elif method_name == "recordSuccess" and len(args) == 2:
        requested = _ensure_i64(builder, resolve_arg(args[0]))
        normalized = _ensure_i64(builder, resolve_arg(args[1]))
        reason = zero
        supported = ir.Constant(i64, 1)
    else:
        return None

    result = _runtime_set_field(
        builder,
        module,
        owner_h,
        _synthetic_field_name("last_requested"),
        requested,
    )
    result = _runtime_set_field(
        builder,
        module,
        owner_h,
        _synthetic_field_name("last_normalized"),
        normalized,
    )
    result = _runtime_set_field(
        builder,
        module,
        owner_h,
        _synthetic_field_name("last_reason"),
        reason,
    )
    result = _runtime_set_field(
        builder,
        module,
        owner_h,
        _synthetic_field_name("last_supported"),
        supported,
    )
    return result
