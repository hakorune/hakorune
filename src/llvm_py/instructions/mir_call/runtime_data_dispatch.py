"""
RuntimeDataBox method dispatch helpers for MIR call lowering.

This module keeps RuntimeDataBox method-to-kernel symbol mapping in one place
to avoid drift between method_call and mir_call_legacy lowerers.
Array receiver specialized routing (AS-03/03b/03c) is selected here as well.
"""

import os
from functools import lru_cache

from llvmlite import ir
from .auto_specialize import (
    prefer_runtime_data_array_i64_key_i64_value_route,
    prefer_runtime_data_array_i64_key_route,
    prefer_runtime_data_array_route,
)


_RUNTIME_DATA_METHODS = {
    "get": ("nyash.runtime_data.get_hh", "unified_runtime_data_get", 1),
    "push": ("nyash.runtime_data.push_hh", "unified_runtime_data_push", 1),
    "set": ("nyash.runtime_data.set_hhh", "unified_runtime_data_set", 2),
    "has": ("nyash.runtime_data.has_hh", "unified_runtime_data_has", 1),
}


_ARRAY_COLLECTION_METHODS = {
    "get": ("nyash.runtime_data.get_hh", "unified_runtime_data_get", 1),
    "push": ("nyash.array.slot_append_hh", "unified_array_slot_append_hh", 1),
    "set": ("nyash.runtime_data.set_hhh", "unified_runtime_data_set", 2),
    "has": ("nyash.runtime_data.has_hh", "unified_runtime_data_has", 1),
}


_ARRAY_COLLECTION_I64_KEY_METHODS = {
    "get": ("nyash.array.slot_load_hi", "unified_array_slot_load_hi", 1),
    "set": ("nyash.array.slot_store_hih", "unified_array_slot_store_hih", 2),
    "has": ("nyash.runtime_data.has_hh", "unified_runtime_data_has", 1),
}


_ARRAY_COLLECTION_I64_KEY_I64_VALUE_METHODS = {
    "set": ("nyash.array.slot_store_hii", "unified_array_slot_store_hii", 2),
}


_RUNTIME_DATA_FIELD_METHODS = {
    "getField": ("nyash.map.slot_load_hh", "unified_runtime_data_getField", 1),
    "setField": ("nyash.map.slot_store_hhh", "unified_runtime_data_setField", 2),
}


@lru_cache(maxsize=1)
def _runtime_data_array_route_policy():
    """
    RuntimeDataBox array-route policy SSOT.

    - default (`array_mono`): allow current array-specialized route
      (`push -> slot_append_hh`, integer-key `get/set -> slot_load_hi/slot_store_hih|slot_store_hii`,
      `has -> runtime_data.has_hh`)
    - `runtime_data_only`: force `nyash.runtime_data.*` even when array hints match
    """
    raw = str(os.getenv("NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY", "array_mono") or "array_mono")
    policy = raw.strip().lower()
    if policy in ("array_mono", "array", "default"):
        return "array_mono"
    if policy in ("runtime_data_only", "runtime_data"):
        return "runtime_data_only"
    raise RuntimeError(
        "unsupported NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY="
        f"{raw!r} (expected: array_mono|runtime_data_only)"
    )


def _prefer_array_mono_route_default():
    return _runtime_data_array_route_policy() == "array_mono"


def _reset_runtime_data_array_route_policy_cache_for_tests():
    _runtime_data_array_route_policy.cache_clear()


def _select_array_collection_call_spec(*, method_name, resolver=None, arg_vids=None):
    if prefer_runtime_data_array_i64_key_route(
        method=method_name,
        resolver=resolver,
        arg_vids=arg_vids,
    ):
        if prefer_runtime_data_array_i64_key_i64_value_route(
            method=method_name,
            resolver=resolver,
            arg_vids=arg_vids,
        ):
            spec = _ARRAY_COLLECTION_I64_KEY_I64_VALUE_METHODS.get(method_name)
            if spec is None:
                spec = _ARRAY_COLLECTION_I64_KEY_METHODS.get(method_name)
        else:
            spec = _ARRAY_COLLECTION_I64_KEY_METHODS.get(method_name)
        if spec is None:
            spec = _ARRAY_COLLECTION_METHODS.get(method_name)
    else:
        spec = _ARRAY_COLLECTION_METHODS.get(method_name)
    return spec


def select_array_collection_call_spec(*, method, resolver=None, arg_vids=None):
    """
    Canonical daily ArrayBox symbol table for the K2-core RawArray seam.

    This keeps ArrayBox lowering and RuntimeDataBox(array-specialized) lowering on
    the same `nyash.array.slot_*` contract while leaving fallback on the
    `nyash.runtime_data.*` facade when proof is insufficient.
    """
    return _select_array_collection_call_spec(
        method_name=str(method or ""),
        resolver=resolver,
        arg_vids=arg_vids,
    )


def select_runtime_data_call_spec(
    *,
    method,
    box_name,
    resolver=None,
    receiver_vid=None,
    arg_vids=None,
    prefer_array_mono_route=True,
):
    """
    Select RuntimeDataBox call target symbol in one place.

    RZ-ARRAY-min1:
    - default behavior stays unchanged (`prefer_array_mono_route=True`)
    - callers can explicitly choose runtime_data-only route without touching
      lowerer internals.
    """
    method_name = str(method or "")
    if str(box_name or "") != "RuntimeDataBox":
        return None

    if not prefer_array_mono_route:
        return _RUNTIME_DATA_METHODS.get(method_name)

    if prefer_runtime_data_array_route(
        method=method_name,
        box_name=box_name,
        resolver=resolver,
        receiver_vid=receiver_vid,
        arg_vids=arg_vids,
    ):
        spec = _select_array_collection_call_spec(
            method_name=method_name,
            resolver=resolver,
            arg_vids=arg_vids,
        )
    else:
        spec = _RUNTIME_DATA_METHODS.get(method_name)
    return spec


def lower_runtime_data_method_call(
    builder,
    declare,
    box_name,
    method,
    recv_h,
    args,
    *,
    resolver=None,
    receiver_vid=None,
    arg_vids=None,
    prefer_array_mono_route=None,
):
    """
    Lower RuntimeDataBox method call to kernel runtime_data exports.

    Returns:
      - LLVM value when handled (including fail-fast zero for arity mismatch)
      - None when call is not a RuntimeDataBox target/method
    """
    if prefer_array_mono_route is None:
        prefer_array_mono_route = _prefer_array_mono_route_default()

    spec = select_runtime_data_call_spec(
        method=method,
        box_name=box_name,
        resolver=resolver,
        receiver_vid=receiver_vid,
        arg_vids=arg_vids,
        prefer_array_mono_route=prefer_array_mono_route,
    )
    if spec is None:
        return None

    symbol, call_name, arity = spec
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    call_args = args if isinstance(args, list) else []

    if arity == 1:
        if len(call_args) < 1:
            return zero
        callee = declare(symbol, i64, [i64, i64])
        return builder.call(callee, [recv_h, call_args[0] or zero], name=call_name)

    if arity == 2:
        if len(call_args) < 2:
            return zero
        callee = declare(symbol, i64, [i64, i64, i64])
        return builder.call(
            callee,
            [recv_h, call_args[0] or zero, call_args[1] or zero],
            name=call_name,
        )

    return None


def lower_runtime_data_field_call(
    builder,
    declare,
    box_name,
    method,
    recv_h,
    args,
    *,
    resolve_arg=None,
    ensure_handle=None,
):
    """
    Lower RuntimeDataBox field-store access to the map ABI.

    RuntimeDataCoreBox uses a string-backed MiniMap for register state, so the
    `getField/setField` surface is the register-store read/write path. Keep the
    lowering explicit here so the direct route remains fail-fast for other boxes.
    """
    if str(box_name or "") != "RuntimeDataBox":
        return None

    method_name = str(method or "")
    spec = _RUNTIME_DATA_FIELD_METHODS.get(method_name)
    if spec is None:
        return None

    symbol, call_name, arity = spec
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    call_args = args if isinstance(args, list) else []

    def _maybe_handle(value):
        if ensure_handle is None or value is None:
            return value
        try:
            return ensure_handle(value)
        except Exception:
            return value

    if arity == 1:
        if len(call_args) < 1:
            return zero
        key = resolve_arg(call_args[0]) if resolve_arg is not None else call_args[0]
        if key is None:
            key = zero
        key = _maybe_handle(key)
        callee = declare(symbol, i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name=call_name)

    if arity == 2:
        if len(call_args) < 2:
            return zero
        key = resolve_arg(call_args[0]) if resolve_arg is not None else call_args[0]
        value = resolve_arg(call_args[1]) if resolve_arg is not None else call_args[1]
        if key is None:
            key = zero
        if value is None:
            value = zero
        key = _maybe_handle(key)
        value = _maybe_handle(value)
        callee = declare(symbol, i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, key, value], name=call_name)

    return None
