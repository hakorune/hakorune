"""
Box method call lowering for MIR Call instruction.

Handles lowering of box method calls (BoxCall) to LLVM IR.
Implements the "Everything is Box" philosophy with unified method dispatch.
"""

from typing import Dict, Any, Optional
from llvmlite import ir
import os
from instructions.stringbox import emit_stringbox_call
from instructions.string_fast import (
    literal_string_for_receiver,
    llvm_fast_enabled,
    string_ptr_for_value,
)
from .arg_resolver import make_call_arg_resolver
from .auto_specialize import (
    prefer_array_len_h_route,
    prefer_map_len_h_route,
    prefer_string_len_h_route,
)
from .intrinsic_registry import (
    is_length_like_method,
    requires_string_receiver_tag,
)
from .collection_method_call import lower_collection_method_call
from .method_fallback_tail import lower_direct_or_plugin_method_call
from .runtime_data_dispatch import lower_runtime_data_field_call
from .string_console_method_call import (
    lower_string_or_console_method_call,
    lower_string_search_or_slice_method_call,
)
from instructions.string_result_policy import mark_string_result_if_needed


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
    receiver_literal = literal_string_for_receiver(resolver, receiver)
    literal_recv = receiver_literal if fast_on and is_length_like_method(method) else None

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

    def _resolve_string_ptr_for_receiver(receiver_vid: int):
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

    def _box_string_ptr(ptr):
        callee = _declare("nyash.box.from_i8_string", i64, [i8p])
        return builder.call(callee, [ptr], name="unified_str_ptr2h")

    def _store_result_string_ptr(ptr):
        try:
            if dst_vid is not None and resolver is not None and hasattr(resolver, "string_ptrs"):
                resolver.string_ptrs[int(dst_vid)] = ptr
            if dst_vid is not None and resolver is not None and hasattr(resolver, "mark_string"):
                resolver.mark_string(int(dst_vid))
        except Exception:
            pass

    def _mark_receiver_stringish():
        try:
            if resolver is not None and hasattr(resolver, "mark_string"):
                resolver.mark_string(receiver)
        except Exception:
            pass

    len_ptr_hint = _resolve_string_ptr_for_receiver(receiver)

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
    if method in ("toString", "stringify", "str"):
        callee = _declare("nyash.any.toString_h", i64, [i64])
        result = builder.call(callee, [recv_h], name="slot0_tostring")

    elif is_length_like_method(method):
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
        # - prefer string.len_h / array.slot_len_h when receiver facts exist
        # - otherwise keep generic Any.length_h contract.
        if result is None:
            if str(box_name or "") == "ArrayBox" and len(args) == 0:
                callee = _declare("nyash.array.slot_len_h", i64, [i64])
                result = builder.call(callee, [recv_h], name="unified_array_slot_len_h")
            elif str(box_name or "") == "MapBox" and len(args) == 0:
                callee = _declare("nyash.map.entry_count_h", i64, [i64])
                result = builder.call(callee, [recv_h], name="unified_map_entry_count_h")
            elif prefer_string_len_h_route(method, len(args), resolver, receiver):
                callee = _declare("nyash.string.len_h", i64, [i64])
                result = builder.call(callee, [recv_h], name="unified_string_len_h")
            elif prefer_array_len_h_route(method, len(args), resolver, receiver):
                callee = _declare("nyash.array.slot_len_h", i64, [i64])
                result = builder.call(callee, [recv_h], name="unified_array_slot_len_h")
            elif prefer_map_len_h_route(method, len(args), resolver, receiver):
                callee = _declare("nyash.map.entry_count_h", i64, [i64])
                result = builder.call(callee, [recv_h], name="unified_map_entry_count_h")

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

    elif box_name == "ArrayBox" and method == "birth" and not args:
        # `newbox ArrayBox` already materializes the handle; the follow-up
        # `ArrayBox.birth()` in MIR is an initializer marker only.
        result = ir.Constant(i64, 0)

    elif method in {"get", "push", "set", "has"}:
        result = lower_collection_method_call(
            builder=builder,
            declare=_declare,
            box_name=box_name,
            method_name=method,
            recv_h=recv_h,
            arg_ids=args,
            resolve_arg=_resolve_arg,
            resolver=resolver,
            receiver_vid=receiver,
        )
        if method == "get" and dst_vid is not None:
            try:
                if (
                    resolver is not None
                    and hasattr(resolver, "is_stringish")
                    and resolver.is_stringish(int(dst_vid))
                    and hasattr(resolver, "string_ptrs")
                ):
                    ptr_map = resolver.string_ptrs
                    if (
                        isinstance(ptr_map, dict)
                        and int(dst_vid) not in ptr_map
                        and hasattr(result, "type")
                        and isinstance(result.type, ir.IntType)
                        and result.type.width == 64
                    ):
                        bridge = _declare("nyash.string.to_i8p_h", i8p, [i64])
                        ptr_map[int(dst_vid)] = builder.call(
                            bridge, [result], name=f"method_get_str_h2p_{dst_vid}"
                        )
                        if hasattr(resolver, "mark_string"):
                            resolver.mark_string(int(dst_vid))
            except Exception:
                pass

    elif box_name == "RuntimeDataBox" and method in {"getField", "setField"}:
        result = lower_runtime_data_field_call(
            builder=builder,
            declare=_declare,
            box_name=box_name,
            method=method,
            recv_h=recv_h,
            args=args,
            resolve_arg=_resolve_arg,
            ensure_handle=_ensure_handle,
        )

    elif box_name == "StringBox" and method in {"substring", "indexOf", "lastIndexOf"}:
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
        result = vmap.get(dst_vid) if handled and dst_vid is not None else None

    else:
        string_recv_ptr = _resolve_string_ptr_for_receiver(receiver)
        result = lower_string_search_or_slice_method_call(
            builder=builder,
            declare=_declare,
            method_name=method,
            recv_h=recv_h,
            recv_ptr=string_recv_ptr,
            arg_ids=args,
            resolve_arg=_resolve_arg,
            ensure_handle=_ensure_handle,
            needle_ptr_for_value=lambda vid: string_ptr_for_value(resolver, vid),
            mark_receiver_stringish=(
                _mark_receiver_stringish if requires_string_receiver_tag(method) else None
            ),
            box_string_ptr=_box_string_ptr,
            store_result_string_ptr=_store_result_string_ptr,
        )
        if result is None:
            result = lower_string_or_console_method_call(
                builder=builder,
                declare=_declare,
                method_name=method,
                recv_h=recv_h,
                recv_ptr=string_recv_ptr,
                arg_ids=args,
                resolve_arg=_resolve_arg,
                ensure_handle=_ensure_handle,
                needle_ptr_for_value=lambda vid: string_ptr_for_value(resolver, vid),
                mark_receiver_stringish=(
                    _mark_receiver_stringish if requires_string_receiver_tag(method) else None
                ),
                box_string_ptr=_box_string_ptr,
                store_result_string_ptr=_store_result_string_ptr,
            )
        if result is None:
            result = lower_direct_or_plugin_method_call(
                builder=builder,
                module=module,
                box_name=box_name,
                method_name=method,
                recv_h=recv_h,
                args=args,
                resolve_arg=_resolve_arg,
                ensure_handle=_ensure_handle,
                direct_call_name=f"known_box_{method}",
                plugin_call_name="unified_plugin_invoke",
                receiver_literal=receiver_literal,
            )

    # Store result
    if dst_vid is not None:
        vmap[dst_vid] = result
        # Mark string-producing methods
        mark_string_result_if_needed(resolver, dst_vid, method)
