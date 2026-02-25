"""
RuntimeDataBox method dispatch helpers for MIR call lowering.

This module keeps RuntimeDataBox method-to-kernel symbol mapping in one place
to avoid drift between method_call and mir_call_legacy lowerers.
Array receiver mono-route (AS-03) is selected here as well.
"""

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


_RUNTIME_DATA_ARRAY_METHODS = {
    "get": ("nyash.array.get_hh", "unified_array_get_hh", 1),
    "push": ("nyash.array.push_hh", "unified_array_push_hh", 1),
    "set": ("nyash.array.set_hhh", "unified_array_set_hhh", 2),
    "has": ("nyash.array.has_hh", "unified_array_has_hh", 1),
}


_RUNTIME_DATA_ARRAY_I64_KEY_METHODS = {
    "get": ("nyash.array.get_hi", "unified_array_get_hi", 1),
    "set": ("nyash.array.set_hih", "unified_array_set_hih", 2),
    "has": ("nyash.array.has_hi", "unified_array_has_hi", 1),
}


_RUNTIME_DATA_ARRAY_I64_KEY_I64_VALUE_METHODS = {
    "set": ("nyash.array.set_hii", "unified_array_set_hii", 2),
}


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
):
    """
    Lower RuntimeDataBox method call to kernel runtime_data exports.

    Returns:
      - LLVM value when handled (including fail-fast zero for arity mismatch)
      - None when call is not a RuntimeDataBox target/method
    """
    if box_name != "RuntimeDataBox":
        return None
    if prefer_runtime_data_array_route(
        method=method,
        box_name=box_name,
        resolver=resolver,
        receiver_vid=receiver_vid,
        arg_vids=arg_vids,
    ):
        if prefer_runtime_data_array_i64_key_route(
            method=method,
            resolver=resolver,
            arg_vids=arg_vids,
        ):
            if prefer_runtime_data_array_i64_key_i64_value_route(
                method=method,
                resolver=resolver,
                arg_vids=arg_vids,
            ):
                spec = _RUNTIME_DATA_ARRAY_I64_KEY_I64_VALUE_METHODS.get(method)
                if spec is None:
                    spec = _RUNTIME_DATA_ARRAY_I64_KEY_METHODS.get(method)
            else:
                spec = _RUNTIME_DATA_ARRAY_I64_KEY_METHODS.get(method)
            if spec is None:
                spec = _RUNTIME_DATA_ARRAY_METHODS.get(method)
        else:
            spec = _RUNTIME_DATA_ARRAY_METHODS.get(method)
    else:
        spec = _RUNTIME_DATA_METHODS.get(method)
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
