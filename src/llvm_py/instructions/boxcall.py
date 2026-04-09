"""
BoxCall instruction lowering
Core of Nyash's "Everything is Box" philosophy
"""

import llvmlite.ir as ir

# Phase 287 P5: Universal slot constants (SSOT)
UNIVERSAL_SLOT_TOSTRING = 0  # toString/stringify/str (all types)
TOSTRING_METHODS = ("toString", "stringify", "str")
from typing import Dict, List, Optional, Any
from instructions.safepoint import insert_automatic_safepoint
from naming_helper import encode_static_method
from console_bridge import emit_console_call  # Phase 133: Console 箱化モジュール
from instructions.stringbox import emit_stringbox_call  # Phase 134-B: StringBox 箱化モジュール
from instructions.mir_call.auto_specialize import (
    prefer_runtime_data_array_i64_key_route,
    receiver_is_arrayish,
    receiver_is_stringish,
)
from instructions.boxcall_runtime_data import try_lower_collection_boxcall
from instructions.direct_box_method import try_lower_known_box_method_call
from instructions.sum_escape import materialize_sum_escape_value_if_needed
from instructions.user_box_local import materialize_user_box_escape_value_if_needed
from instructions.string_fast import literal_string_for_receiver
from instructions.string_result_policy import mark_string_result_if_needed
from utils.values import resolve_i64_strict
from utils.resolver_helpers import get_box_type, mark_as_handle

def _declare(module: ir.Module, name: str, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)

def _ensure_handle(builder: ir.IRBuilder, module: ir.Module, v: ir.Value) -> ir.Value:
    """Coerce a value to i64 handle. If pointer, box via nyash.box.from_i8_string."""
    i64 = ir.IntType(64)
    if hasattr(v, 'type'):
        if isinstance(v.type, ir.IntType) and v.type.width == 64:
            return v
        if isinstance(v.type, ir.PointerType):
            # call nyash.box.from_i8_string(i8*) -> i64
            i8p = ir.IntType(8).as_pointer()
            # If pointer-to-array, GEP to first element
            try:
                if isinstance(v.type.pointee, ir.ArrayType):
                    c0 = ir.IntType(32)(0)
                    v = builder.gep(v, [c0, c0], name="bc_str_gep")
            except Exception:
                pass
            callee = _declare(module, "nyash.box.from_i8_string", i64, [i8p])
            return builder.call(callee, [v], name="str_ptr2h")
        if isinstance(v.type, ir.IntType):
            # extend/trunc to i64
            return builder.zext(v, i64) if v.type.width < 64 else builder.trunc(v, i64)
    return ir.Constant(i64, 0)


def _maybe_seed_string_ptr_from_handle(
    builder: ir.IRBuilder,
    module: ir.Module,
    resolver,
    dst_vid: Optional[int],
    handle_val: ir.Value,
) -> None:
    if resolver is None or dst_vid is None:
        return
    try:
        if not (hasattr(handle_val, "type") and isinstance(handle_val.type, ir.IntType) and handle_val.type.width == 64):
            return
        if not hasattr(resolver, "is_stringish") or not resolver.is_stringish(int(dst_vid)):
            return
        ptr_map = getattr(resolver, "string_ptrs", None)
        if not isinstance(ptr_map, dict) or int(dst_vid) in ptr_map:
            return
        bridge = _declare(module, "nyash.string.to_i8p_h", ir.IntType(8).as_pointer(), [ir.IntType(64)])
        ptr_map[int(dst_vid)] = builder.call(bridge, [handle_val], name=f"boxcall_str_h2p_{dst_vid}")
        if hasattr(resolver, "mark_string"):
            resolver.mark_string(int(dst_vid))
    except Exception:
        pass

def lower_boxcall(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: int,
    method_name: str,
    args: List[int],
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None,
    method_id: Optional[int] = None,  # Phase 287 P4: Universal slot ID
) -> None:
    # Guard against emitting after a terminator: create continuation block if needed.
    try:
        if builder.block is not None and getattr(builder.block, 'terminator', None) is not None:
            func = builder.block.parent
            cont = func.append_basic_block(name=f"cont_bb_{builder.block.name}")
            builder.position_at_end(cont)
    except Exception:
        pass
    """
    Lower MIR BoxCall instruction
    
    Current implementation uses method_id approach for plugin boxes.
    
    Args:
        builder: Current LLVM IR builder
        module: LLVM module
        box_vid: Box instance value ID (handle)
        method_name: Method name to call
        args: List of argument value IDs
        dst_vid: Optional destination for return value
        vmap: Value map
        resolver: Optional resolver for type handling
    """
    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    # Insert a safepoint around potential heavy boxcall sites (pre-call)
    try:
        import os
        if os.environ.get('NYASH_LLVM_AUTO_SAFEPOINT', '1') == '1':
            insert_automatic_safepoint(builder, module, "boxcall")
    except Exception:
        pass

    # Phase 287 P4: Universal slot #0 handling (toString/stringify/str)
    # SSOT: toString is ALWAYS slot #0, works on ALL types including primitives
    if method_id == UNIVERSAL_SLOT_TOSTRING and method_name in TOSTRING_METHODS:
        import os, sys
        if os.environ.get('NYASH_LLVM_TRACE_SLOT') == '1':
            print(f"[llvm-py/slot] Universal slot #0: {method_name} on box_vid={box_vid}", file=sys.stderr)

        # Get receiver value
        recv_val = vmap.get(box_vid)
        if isinstance(box_vid, int):
            local_sum_value = materialize_sum_escape_value_if_needed(
                builder,
                module,
                int(box_vid),
                vmap,
                resolver,
                name_hint=f"boxcall_recv_{box_vid}",
            )
            if local_sum_value is not None:
                recv_val = local_sum_value
            else:
                local_user_box = materialize_user_box_escape_value_if_needed(
                    builder,
                    module,
                    int(box_vid),
                    vmap,
                    resolver,
                    name_hint=f"boxcall_recv_{box_vid}",
                )
                if local_user_box is not None:
                    recv_val = local_user_box
        if recv_val is None:
            recv_val = ir.Constant(i64, 0)

        # Phase 287 P4: Box primitive i64 first, then call universal slot #0 SSOT
        # SSOT: nyash.any.toString_h(handle) -> StringBox handle (works for ALL types)
        if hasattr(recv_val, 'type') and isinstance(recv_val.type, ir.IntType) and recv_val.type.width == 64:
            # Step 1: Box primitive i64 → IntegerBox handle
            box_i64_fn = _declare(module, "nyash.box.from_i64", i64, [i64])
            boxed_handle = builder.call(box_i64_fn, [recv_val], name="box_prim_i64")
            if os.environ.get('NYASH_LLVM_TRACE_SLOT') == '1':
                print(f"[llvm-py/slot] Boxed primitive i64 to IntegerBox handle", file=sys.stderr)
            recv_h = boxed_handle
        else:
            # Already a handle
            recv_h = _ensure_handle(builder, module, recv_val)

        # Step 2: Call universal slot #0 via nyash.any.toString_h (SSOT - no plugin invoke)
        # This works for IntegerBox, FloatBox, BoolBox, StringBox, ArrayBox, etc.
        tostring_fn = _declare(module, "nyash.any.toString_h", i64, [i64])
        result = builder.call(tostring_fn, [recv_h], name="slot0_tostring")
        if os.environ.get('NYASH_LLVM_TRACE_SLOT') == '1':
            print(f"[llvm-py/slot] Called nyash.any.toString_h (universal slot #0)", file=sys.stderr)

        if dst_vid is not None:
            vmap[dst_vid] = result
            # Mark result as string handle
            try:
                if resolver is not None and hasattr(resolver, 'mark_string'):
                    resolver.mark_string(dst_vid)
            except Exception:
                pass
        return

    # Short-hands with ctx (backward-compatible fallback)
    r = resolver
    p = preds
    bev = block_end_values
    bbm = bb_map
    if ctx is not None:
        try:
            r = getattr(ctx, 'resolver', r)
            p = getattr(ctx, 'preds', p)
            bev = getattr(ctx, 'block_end_values', bev)
            bbm = getattr(ctx, 'bb_map', bbm)
        except Exception:
            pass
    def _res_i64(vid: int):
        local_sum_value = materialize_sum_escape_value_if_needed(
            builder,
            module,
            int(vid),
            vmap,
            r,
            name_hint=f"boxcall_arg_{vid}",
        )
        if local_sum_value is not None:
            return local_sum_value
        local_user_box = materialize_user_box_escape_value_if_needed(
            builder,
            module,
            int(vid),
            vmap,
            r,
            name_hint=f"boxcall_arg_{vid}",
        )
        if local_user_box is not None:
            return local_user_box
        # SSOT: Use the common resolver policy (prefer local SSA, then global vmap, then PHI-localize).
        if r is not None and p is not None and bev is not None:
            try:
                return resolve_i64_strict(r, vid, builder.block, p, bev, vmap, bbm)
            except Exception:
                return None
        return vmap.get(vid)

    # If BuildCtx is provided, prefer its maps for consistency.
    if ctx is not None:
        try:
            if getattr(ctx, 'resolver', None) is not None:
                resolver = ctx.resolver
            if getattr(ctx, 'preds', None) is not None and preds is None:
                preds = ctx.preds
            if getattr(ctx, 'block_end_values', None) is not None and block_end_values is None:
                block_end_values = ctx.block_end_values
            if getattr(ctx, 'bb_map', None) is not None and bb_map is None:
                bb_map = ctx.bb_map
        except Exception:
            pass
    # Receiver value
    recv_val = _res_i64(box_vid)
    if recv_val is None:
        recv_val = vmap.get(box_vid, ir.Constant(i64, 0))

    # Phase 134-B: StringBox 箱化 - StringBox メソッドを stringbox に委譲
    if emit_stringbox_call(builder, module, method_name, recv_val, args, dst_vid, vmap, box_vid, resolver, preds, block_end_values, bb_map, ctx):
        return

    collection_result = try_lower_collection_boxcall(
        builder=builder,
        module=module,
        method_name=method_name,
        recv_val=recv_val,
        box_vid=box_vid,
        args=args,
        resolve_arg=lambda vid: _res_i64(vid) or vmap.get(vid),
        ensure_handle=lambda value: _ensure_handle(builder, module, value),
        declare=_declare,
        resolver=resolver,
    )
    if collection_result is not None:
        if dst_vid is not None:
            vmap[dst_vid] = collection_result
            if method_name == "get":
                _maybe_seed_string_ptr_from_handle(builder, module, resolver, dst_vid, collection_result)
        return

    # Phase 133: Console 箱化 - ConsoleBox メソッドを console_bridge に委譲
    if emit_console_call(builder, module, method_name, args, dst_vid, vmap, resolver, preds, block_end_values, bb_map, ctx):
        return

    # Special: method on `me` (self) or static dispatch to Main.* → direct call to `Main.method/arity`
    try:
        cur_fn_name = str(builder.block.parent.name)
    except Exception:
        cur_fn_name = ''
    # Heuristic: MIR encodes `me` as a string literal "__me__" or sometimes value-id 0.
    is_me = False
    try:
        if box_vid == 0:
            is_me = True
        # Prefer literal marker captured by resolver (from const lowering)
        elif resolver is not None and hasattr(resolver, 'string_literals'):
            lit = resolver.string_literals.get(box_vid)
            if lit == "__me__":
                is_me = True
    except Exception:
        pass
    if is_me and cur_fn_name.startswith('Main.'):
        # NamingBox SSOT: Build target function name with arity
        arity = len(args)
        target = encode_static_method("Main", method_name, arity)
        # If module already has such function, prefer direct call
        callee = None
        for f in module.functions:
            if f.name == target:
                callee = f
                break
        if callee is not None:
            a = []
            for i, aid in enumerate(args):
                raw = vmap.get(aid)
                if raw is not None and hasattr(raw, 'type') and isinstance(raw.type, ir.PointerType):
                    aval = _ensure_handle(builder, module, raw)
                else:
                    if resolver is not None and preds is not None and block_end_values is not None and bb_map is not None:
                        aval = resolver.resolve_i64(aid, builder.block, preds, block_end_values, vmap, bb_map)
                    else:
                        aval = vmap.get(aid, ir.Constant(ir.IntType(64), 0))
                    if hasattr(aval, 'type') and isinstance(aval.type, ir.PointerType):
                        aval = _ensure_handle(builder, module, aval)
                    elif hasattr(aval, 'type') and isinstance(aval.type, ir.IntType) and aval.type.width != 64:
                        aval = builder.zext(aval, ir.IntType(64)) if aval.type.width < 64 else builder.trunc(aval, ir.IntType(64))
                a.append(aval)
            res = builder.call(callee, a, name=f"call_self_{method_name}")
            if dst_vid is not None:
                vmap[dst_vid] = res
                try:
                    if method_name in ("esc_json", "node_json", "dirname", "join", "read_all") and resolver is not None and hasattr(resolver, 'mark_string'):
                        resolver.mark_string(dst_vid)
                except Exception:
                    pass
            return

    known_box_name = get_box_type(resolver, box_vid)
    receiver_literal = literal_string_for_receiver(resolver, box_vid)
    recv_h = _ensure_handle(builder, module, recv_val)
    result = try_lower_known_box_method_call(
        builder=builder,
        module=module,
        box_name=known_box_name,
        method_name=method_name,
        recv_h=recv_h,
        args=args,
        resolve_arg=lambda vid: _res_i64(vid),
        ensure_handle=lambda value: _ensure_handle(builder, module, value),
        call_name=(
            f"call_known_{method_name}" if known_box_name else f"call_stage1_{method_name}"
        ),
        receiver_literal=receiver_literal,
    )
    if result is None:
        raise NotImplementedError(f"Unsupported BoxCall method: {method_name!r}")
    if dst_vid is not None:
        vmap[dst_vid] = result
        # Type tagging: mark handles for downstream consumers (e.g., print)
        try:
            if resolver is not None and hasattr(resolver, 'value_types'):
                # String-returning plugin methods share the MIR call registry SSOT.
                mark_string_result_if_needed(resolver, dst_vid, method_name)

                # Phase 285LLVM-1.5: Tag getField results as handles (unified via mark_as_handle)
                # getField returns a handle to the field value (e.g., handle to IntegerBox(42))
                # This prevents print from boxing the handle itself
                if method_name == "getField":
                    # Mark as generic handle (box_type unknown - could be IntegerBox, StringBox, etc.)
                    mark_as_handle(resolver, dst_vid)
                    # Debug logging: getField tagging
                    import os, sys
                    if os.environ.get('NYASH_CLI_VERBOSE') == '1':
                        print(f"[llvm-py/types] getField dst=%{dst_vid}: tagged as handle", file=sys.stderr)
        except Exception:
            pass
