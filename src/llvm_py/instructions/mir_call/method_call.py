"""
Box method call lowering for MIR Call instruction.

Handles lowering of box method calls (BoxCall) to LLVM IR.
Implements the "Everything is Box" philosophy with unified method dispatch.
"""

from typing import Dict, Any, Optional
from llvmlite import ir
import os
from instructions.stringbox import emit_stringbox_call
from instructions.string_fast import literal_string_for_receiver, llvm_fast_enabled
from .arg_resolver import make_call_arg_resolver
from .auto_specialize import prefer_array_len_h_route, prefer_string_len_h_route
from .intrinsic_registry import (
    is_length_like_method,
    produces_string_result,
    requires_string_receiver_tag,
)
from .runtime_data_dispatch import lower_runtime_data_method_call


def lower_method_call(builder, module, box_name, method, receiver, args, dst_vid, vmap, resolver, owner):
    """
    Lower box method call - TRUE UNIFIED IMPLEMENTATION

    Args:
        builder: LLVM IR builder
        module: LLVM Module
        box_name: Name of the box type (e.g., "ConsoleBox", "StringBox")
        method: Method name to call
        receiver: Value ID of the receiver object
        args: List of argument value IDs
        dst_vid: Destination value ID (register)
        vmap: Value mapping dict
        resolver: Value resolver instance
        owner: NyashLLVMBuilder instance

    Effects:
        - Inserts automatic safepoint
        - Emits specialized LLVM calls for known methods
        - Falls back to generic plugin invocation for unknown methods
        - Stores result in vmap[dst_vid]
        - Marks string-producing methods for type tracking
    """
    from instructions.safepoint import insert_automatic_safepoint

    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    fast_on = llvm_fast_enabled()
    literal_recv = (
        literal_string_for_receiver(resolver, receiver)
        if fast_on and is_length_like_method(method)
        else None
    )

    # Helper to declare function
    def _declare(name: str, ret, args_types):
        for f in module.functions:
            if f.name == name:
                return f
        fnty = ir.FunctionType(ret, args_types)
        return ir.Function(module, fnty, name=name)

    # Helper to ensure i64 handle
    def _ensure_handle(v):
        if isinstance(v.type, ir.IntType) and v.type.width == 64:
            return v
        if v.type.is_pointer:
            callee = _declare("nyash.box.from_i8_string", i64, [i8p])
            return builder.call(callee, [v], name="unified_str_ptr2h")
        if isinstance(v.type, ir.IntType):
            return builder.zext(v, i64) if v.type.width < 64 else builder.trunc(v, i64)
        return v

    # Resolve receiver and arguments
    _resolve_arg = make_call_arg_resolver(builder, vmap, resolver, owner)

    def _resolve_string_ptr_for_len(receiver_vid: int):
        if resolver is None:
            return None
        string_ptrs = getattr(resolver, 'string_ptrs', None)
        if string_ptrs is None:
            return None
        try:
            receiver_key = int(receiver_vid)
        except (TypeError, ValueError):
            return None

        try:
            ptr = string_ptrs.get(receiver_key)
        except AttributeError:
            ptr = None
        if ptr is not None:
            return ptr

        newbox_string_args = getattr(resolver, 'newbox_string_args', None)
        if newbox_string_args is None:
            return None
        try:
            arg_vid = newbox_string_args.get(receiver_key)
            if arg_vid is None:
                return None
            return string_ptrs.get(int(arg_vid))
        except (AttributeError, TypeError, ValueError):
            return None

    def _mark_receiver_stringish():
        try:
            if resolver is not None and hasattr(resolver, "mark_string"):
                resolver.mark_string(receiver)
        except Exception:
            pass

    len_ptr_hint = _resolve_string_ptr_for_len(receiver)

    # FAST length/size path can skip safepoint:
    # - literal fold (const i64)
    # - direct strlen helper from known i8* (no plugin dispatch path)
    skip_auto_safepoint = False
    if fast_on and is_length_like_method(method) and not args:
        if isinstance(literal_recv, str) or len_ptr_hint is not None:
            skip_auto_safepoint = True
        elif resolver is not None and hasattr(resolver, "is_stringish"):
            try:
                if resolver.is_stringish(int(receiver)):
                    skip_auto_safepoint = True
            except (TypeError, ValueError):
                pass

    # Insert automatic safepoint
    if os.environ.get('NYASH_LLVM_AUTO_SAFEPOINT', '1') == '1' and not skip_auto_safepoint:
        insert_automatic_safepoint(builder, module, "boxcall")

    recv_val = _resolve_arg(receiver)
    if recv_val is None:
        recv_val = ir.Constant(i64, 0)
    recv_h = _ensure_handle(recv_val)

    # TRUE UNIFIED METHOD DISPATCH - Everything is Box philosophy
    if is_length_like_method(method):
        result = None
        # Ultra-fast literal fold in MIRCall route.
        if fast_on and not args and isinstance(literal_recv, str):
            use_cp = os.environ.get('NYASH_STR_CP') == '1'
            n = len(literal_recv) if use_cp else len(literal_recv.encode('utf-8'))
            result = ir.Constant(i64, n)

        # SSOT route: delegate StringBox length/len lowering to stringbox module.
        if result is None and method in ("length", "len"):
            try:
                handled = emit_stringbox_call(
                    builder=builder,
                    module=module,
                    method_name=method,
                    recv_val=recv_val,
                    args=args,
                    dst_vid=dst_vid,
                    vmap=vmap,
                    box_vid=receiver,
                    resolver=resolver,
                    preds=getattr(owner, "preds", None),
                    block_end_values=getattr(owner, "block_end_values", None),
                    bb_map=getattr(owner, "bb_map", None),
                )
                if handled:
                    if dst_vid is not None:
                        result = vmap.get(dst_vid)
                    if result is None:
                        result = ir.Constant(i64, 0)
            except Exception:
                result = None

        # size/length/len:
        # - prefer string.len_h / array.len_h when receiver facts exist
        # - otherwise keep generic Any.length_h contract.
        if result is None:
            if prefer_string_len_h_route(method, len(args), resolver, receiver):
                callee = _declare("nyash.string.len_h", i64, [i64])
                result = builder.call(callee, [recv_h], name="unified_string_len_h")
            elif prefer_array_len_h_route(method, len(args), resolver, receiver):
                callee = _declare("nyash.array.len_h", i64, [i64])
                result = builder.call(callee, [recv_h], name="unified_array_len_h")

            if method == "size" and fast_on:
                mode = ir.Constant(i64, 1 if os.environ.get('NYASH_STR_CP') == '1' else 0)
                fast_strlen = _declare("nyrt_string_length", i64, [i8p, i64])
                ptr = len_ptr_hint
                if ptr is None and resolver is not None and hasattr(resolver, "is_stringish"):
                    try:
                        if resolver.is_stringish(int(receiver)):
                            to_i8p = _declare("nyash.string.to_i8p_h", i8p, [i64])
                            ptr = builder.call(to_i8p, [recv_h], name="unified_strlen_h2p")
                    except (TypeError, ValueError):
                        pass
                if ptr is not None:
                    result = builder.call(fast_strlen, [ptr, mode], name="unified_strlen_si")

            if result is None:
                callee = _declare("nyash.any.length_h", i64, [i64])
                call_name = "unified_size" if method == "size" else "unified_length"
                result = builder.call(callee, [recv_h], name=call_name)

    elif method == "substring":
        if len(args) >= 2:
            if requires_string_receiver_tag(method):
                _mark_receiver_stringish()
            s = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            e = _resolve_arg(args[1]) or ir.Constant(i64, 0)
            callee = _declare("nyash.string.substring_hii", i64, [i64, i64, i64])
            result = builder.call(callee, [recv_h, s, e], name="unified_substring")
        else:
            result = recv_h

    elif method == "lastIndexOf":
        if args:
            if requires_string_receiver_tag(method):
                _mark_receiver_stringish()
            needle = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            needle_h = _ensure_handle(needle)
            callee = _declare("nyash.string.lastIndexOf_hh", i64, [i64, i64])
            result = builder.call(callee, [recv_h, needle_h], name="unified_lastIndexOf")
        else:
            result = ir.Constant(i64, -1)

    elif method == "indexOf" and len(args) == 1:
        if requires_string_receiver_tag(method):
            _mark_receiver_stringish()
        needle = _resolve_arg(args[0]) or ir.Constant(i64, 0)
        needle_h = _ensure_handle(needle)
        callee = _declare("nyash.string.indexOf_hh", i64, [i64, i64])
        result = builder.call(callee, [recv_h, needle_h], name="unified_indexOf")

    elif method == "get":
        if args:
            k = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            runtime_result = lower_runtime_data_method_call(
                builder=builder,
                declare=_declare,
                box_name=box_name,
                method=method,
                recv_h=recv_h,
                args=[k],
                resolver=resolver,
                receiver_vid=receiver,
                arg_vids=args,
            )
            if runtime_result is not None:
                result = runtime_result
            else:
                callee = _declare("nyash.map.get_hh", i64, [i64, i64])
                result = builder.call(callee, [recv_h, k], name="unified_map_get")
        else:
            result = ir.Constant(i64, 0)

    elif method == "push":
        if args:
            v = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            runtime_result = lower_runtime_data_method_call(
                builder=builder,
                declare=_declare,
                box_name=box_name,
                method=method,
                recv_h=recv_h,
                args=[v],
                resolver=resolver,
                receiver_vid=receiver,
                arg_vids=args,
            )
            if runtime_result is not None:
                result = runtime_result
            else:
                callee = _declare("nyash.array.push_h", i64, [i64, i64])
                result = builder.call(callee, [recv_h, v], name="unified_array_push")
        else:
            result = recv_h

    elif method == "set":
        if len(args) >= 2:
            k = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            v = _resolve_arg(args[1]) or ir.Constant(i64, 0)
            runtime_result = lower_runtime_data_method_call(
                builder=builder,
                declare=_declare,
                box_name=box_name,
                method=method,
                recv_h=recv_h,
                args=[k, v],
                resolver=resolver,
                receiver_vid=receiver,
                arg_vids=args,
            )
            if runtime_result is not None:
                result = runtime_result
            else:
                callee = _declare("nyash.map.set_hh", i64, [i64, i64, i64])
                result = builder.call(callee, [recv_h, k, v], name="unified_map_set")
        else:
            result = recv_h

    elif method == "has":
        if args:
            k = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            runtime_result = lower_runtime_data_method_call(
                builder=builder,
                declare=_declare,
                box_name=box_name,
                method=method,
                recv_h=recv_h,
                args=[k],
                resolver=resolver,
                receiver_vid=receiver,
                arg_vids=args,
            )
            if runtime_result is not None:
                result = runtime_result
            else:
                callee = _declare("nyash.map.has_hh", i64, [i64, i64])
                result = builder.call(callee, [recv_h, k], name="unified_map_has")
        else:
            result = ir.Constant(i64, 0)

    elif method == "log":
        if args:
            arg0 = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            if isinstance(arg0.type, ir.IntType) and arg0.type.width == 64:
                bridge = _declare("nyash.string.to_i8p_h", i8p, [i64])
                p = builder.call(bridge, [arg0], name="unified_str_h2p")
                callee = _declare("nyash.console.log", i64, [i8p])
                result = builder.call(callee, [p], name="unified_console_log")
            else:
                callee = _declare("nyash.console.log", i64, [i8p])
                result = builder.call(callee, [arg0], name="unified_console_log")
        else:
            result = ir.Constant(i64, 0)

    else:
        # Generic plugin method invocation
        method_str = method.encode('utf-8') + b'\0'
        method_gname = f"unified_method_{method}"
        if method_gname in module.globals:
            method_global = module.get_global(method_gname)
        else:
            method_global = ir.GlobalVariable(
                module, ir.ArrayType(i8, len(method_str)), name=method_gname
            )
            method_global.initializer = ir.Constant(
                ir.ArrayType(i8, len(method_str)), bytearray(method_str)
            )
            method_global.global_constant = True
        mptr = builder.gep(method_global, [ir.Constant(ir.IntType(32), 0), ir.Constant(ir.IntType(32), 0)])

        argc = ir.Constant(i64, len(args))
        a1 = _resolve_arg(args[0]) if args else ir.Constant(i64, 0)
        a2 = _resolve_arg(args[1]) if len(args) > 1 else ir.Constant(i64, 0)

        callee = _declare("nyash.plugin.invoke_by_name_i64", i64, [i64, i8p, i64, i64, i64])
        result = builder.call(callee, [recv_h, mptr, argc, a1, a2], name="unified_plugin_invoke")

    # Store result
    if dst_vid is not None:
        vmap[dst_vid] = result
        # Mark string-producing methods
        if resolver and hasattr(resolver, 'mark_string'):
            if produces_string_result(method):
                resolver.mark_string(dst_vid)
