import os
from typing import Any, Dict, Optional

import llvmlite.ir as ir

from instructions.string_fast import string_ptr_for_value
from utils.values import safe_vmap_write

from .primitive_handles import _ensure_module_function


def _stringbox_type(meta: Any) -> bool:
    return bool(
        meta
        and (
            meta.get("kind") == "string"
            or (meta.get("kind") == "handle" and meta.get("box_type") == "StringBox")
        )
    )


def _binop_plus_explicit_route(dst_type: Optional[Any]) -> tuple[bool, bool]:
    explicit_integer = False
    explicit_string = False
    try:
        if dst_type == "i64":
            explicit_integer = True
        elif (
            isinstance(dst_type, dict)
            and dst_type.get("kind") == "handle"
            and dst_type.get("box_type") == "StringBox"
        ):
            explicit_string = True
    except Exception:
        pass
    return explicit_integer, explicit_string


def _binop_plus_operand_is_stringish(resolver, lhs: int, rhs: int, lhs_raw, rhs_raw) -> bool:
    try:
        if resolver is not None and hasattr(resolver, "string_literals"):
            if lhs in resolver.string_literals or rhs in resolver.string_literals:
                return True
        if resolver is not None and hasattr(resolver, "value_types"):
            lhs_type = resolver.value_types.get(lhs)
            rhs_type = resolver.value_types.get(rhs)
            if _stringbox_type(lhs_type) or _stringbox_type(rhs_type):
                return True
        if lhs_raw is not None and hasattr(lhs_raw, "type") and isinstance(lhs_raw.type, ir.PointerType):
            return True
        if rhs_raw is not None and hasattr(rhs_raw, "type") and isinstance(rhs_raw.type, ir.PointerType):
            return True
    except Exception:
        return False
    return False


def _binop_plus_any_tagged_string(resolver, lhs: int, rhs: int) -> bool:
    try:
        if resolver is None:
            return False
        if hasattr(resolver, "string_literals") and (
            lhs in resolver.string_literals or rhs in resolver.string_literals
        ):
            return True
        if hasattr(resolver, "value_types"):
            lhs_ty = resolver.value_types.get(lhs)
            rhs_ty = resolver.value_types.get(rhs)
            return _stringbox_type(lhs_ty) or _stringbox_type(rhs_ty)
    except Exception:
        return False
    return False


def _binop_plus_prefers_string_path(
    resolver,
    lhs: int,
    rhs: int,
    lhs_raw,
    rhs_raw,
    dst_type: Optional[Any],
) -> tuple[bool, bool, bool, bool]:
    explicit_integer, explicit_string = _binop_plus_explicit_route(dst_type)
    operand_is_string = _binop_plus_operand_is_stringish(resolver, lhs, rhs, lhs_raw, rhs_raw)
    is_ptr_side = (hasattr(lhs_raw, "type") and isinstance(lhs_raw.type, ir.PointerType)) or (
        hasattr(rhs_raw, "type") and isinstance(rhs_raw.type, ir.PointerType)
    )

    if operand_is_string:
        if explicit_integer and os.environ.get("NYASH_LLVM_STRICT") == "1":
            raise RuntimeError(
                f"[LLVM_PY/STRICT] Type conflict: dst_type=i64 but operand is string. lhs={lhs} rhs={rhs}"
            )
        return True, explicit_integer, explicit_string, operand_is_string
    if explicit_integer:
        return False, explicit_integer, explicit_string, operand_is_string
    if explicit_string:
        return True, explicit_integer, explicit_string, operand_is_string
    return (
        is_ptr_side or _binop_plus_any_tagged_string(resolver, lhs, rhs),
        explicit_integer,
        explicit_string,
        operand_is_string,
    )


def _binop_plus_string_tags(resolver, lhs: int, rhs: int) -> tuple[bool, bool]:
    lhs_tag = False
    rhs_tag = False
    try:
        if resolver is not None:
            if hasattr(resolver, "is_stringish"):
                lhs_tag = bool(resolver.is_stringish(lhs))
                rhs_tag = bool(resolver.is_stringish(rhs))
            if hasattr(resolver, "string_literals"):
                lhs_tag = lhs_tag or (lhs in resolver.string_literals)
                rhs_tag = rhs_tag or (rhs in resolver.string_literals)
            if hasattr(resolver, "value_types"):
                if not lhs_tag:
                    lhs_tag = _stringbox_type(resolver.value_types.get(lhs))
                if not rhs_tag:
                    rhs_tag = _stringbox_type(resolver.value_types.get(rhs))
    except Exception:
        pass
    return lhs_tag, rhs_tag


def _binop_to_string_handle(builder: ir.IRBuilder, raw, value, tag: str, dst: int, i64, i8p):
    if raw is not None and hasattr(raw, "type") and isinstance(raw.type, ir.IntType) and raw.type.width == 64:
        return raw
    if raw is not None and hasattr(raw, "type") and isinstance(raw.type, ir.PointerType):
        try:
            if isinstance(raw.type.pointee, ir.ArrayType):
                c0 = ir.Constant(ir.IntType(32), 0)
                raw = builder.gep(raw, [c0, c0], name=f"bin_gep_{tag}_{dst}")
        except Exception:
            pass
        callee = _ensure_module_function(
            builder.module,
            "nyash.box.from_i8_string",
            i64,
            [i8p],
        )
        return builder.call(callee, [raw], name=f"str_ptr2h_{tag}_{dst}")
    if value is not None and hasattr(value, "type") and isinstance(value.type, ir.IntType) and value.type.width == 64:
        return value
    return ir.Constant(i64, 0)


def _binop_to_string_ptr(builder: ir.IRBuilder, resolver, value_id: int, raw, value):
    ptr = None
    if raw is not None and hasattr(raw, "type") and isinstance(raw.type, ir.PointerType):
        ptr = raw
        try:
            if isinstance(raw.type.pointee, ir.ArrayType):
                c0 = ir.Constant(ir.IntType(32), 0)
                ptr = builder.gep(raw, [c0, c0], name="bin_str_gep")
        except Exception:
            pass
    if ptr is None:
        ptr = string_ptr_for_value(resolver, value_id)
    return ptr


def _binop_any_to_string_handle(builder: ir.IRBuilder, handle_val, tag: str, dst: int, i64):
    callee = _ensure_module_function(
        builder.module,
        "nyash.any.toString_h",
        i64,
        [i64],
    )
    return builder.call(callee, [handle_val], name=f"any_tostr_h_{tag}_{dst}")


def _binop_needs_stringify_bridge(tagged: bool, raw, value) -> bool:
    if tagged:
        return False
    try:
        candidate = raw if isinstance(raw, ir.Constant) else value
        return bool(
            isinstance(candidate, ir.Constant)
            and isinstance(candidate.type, ir.IntType)
            and candidate.type.width == 64
        )
    except Exception:
        return False


def _materialize_string_concat_handles(
    builder: ir.IRBuilder,
    resolver,
    lhs: int,
    rhs: int,
    dst: int,
    lhs_raw,
    rhs_raw,
    lhs_val,
    rhs_val,
):
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    lhs_tag, rhs_tag = _binop_plus_string_tags(resolver, lhs, rhs)
    if os.environ.get("NYASH_BINOP_DEBUG") == "1":
        print(f"  [concat path] lhs_tag={lhs_tag} rhs_tag={rhs_tag}")
    hl = _binop_to_string_handle(builder, lhs_raw, lhs_val, "l", dst, i64, i8p)
    hr = _binop_to_string_handle(builder, rhs_raw, rhs_val, "r", dst, i64, i8p)
    if _binop_needs_stringify_bridge(lhs_tag, lhs_raw, lhs_val):
        hl = _binop_any_to_string_handle(builder, hl, "l", dst, i64)
    if _binop_needs_stringify_bridge(rhs_tag, rhs_raw, rhs_val):
        hr = _binop_any_to_string_handle(builder, hr, "r", dst, i64)
    return hl, hr


def _try_lower_string_concat_fast(
    builder: ir.IRBuilder,
    resolver,
    lhs: int,
    rhs: int,
    dst: int,
    lhs_raw,
    rhs_raw,
    lhs_val,
    rhs_val,
):
    if os.environ.get("NYASH_LLVM_FAST") != "1" or resolver is None:
        return None

    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    lhs_ptr = _binop_to_string_ptr(builder, resolver, lhs, lhs_raw, lhs_val)
    rhs_ptr = _binop_to_string_ptr(builder, resolver, rhs, rhs_raw, rhs_val)
    if lhs_ptr is None or rhs_ptr is None:
        return None

    concat = _ensure_module_function(
        builder.module,
        "nyash.string.concat_ss",
        i8p,
        [i8p, i8p],
    )
    out_ptr = builder.call(concat, [lhs_ptr, rhs_ptr], name=f"concat_ss_{dst}")
    boxer = _ensure_module_function(
        builder.module,
        "nyash.box.from_i8_string",
        i64,
        [i8p],
    )
    out_h = builder.call(boxer, [out_ptr], name=f"str_ptr2h_concat_{dst}")

    try:
        if hasattr(resolver, "string_ptrs"):
            resolver.string_ptrs[int(dst)] = out_ptr
        if hasattr(resolver, "mark_string"):
            resolver.mark_string(dst)
    except Exception:
        pass

    return out_h


def _dispatch_string_concat(builder: ir.IRBuilder, dst: int, lhs_raw, rhs_raw, hl, hr):
    i64 = ir.IntType(64)
    concat3_info = _concat3_chain_args(lhs_raw, rhs_raw, hl, hr)
    if concat3_info is not None:
        concat3_args, folded_call = concat3_info
        callee3 = _ensure_module_function(
            builder.module,
            "nyash.string.concat3_hhh",
            i64,
            [i64, i64, i64],
        )
        result = builder.call(callee3, list(concat3_args), name=f"concat3_hhh_{dst}")
        _prune_dead_chain_call(builder, folded_call, replacement_call=result)
        return result

    callee = _ensure_module_function(
        builder.module,
        "nyash.string.concat_hh",
        i64,
        [i64, i64],
    )
    return builder.call(callee, [hl, hr], name=f"concat_hh_{dst}")


def _finalize_string_concat_result(resolver, vmap: Dict[int, ir.Value], dst: int, result, reason: str):
    safe_vmap_write(vmap, dst, result, reason, resolver=resolver)
    try:
        if resolver is not None and hasattr(resolver, "mark_string"):
            resolver.mark_string(dst)
    except Exception:
        pass


def _lower_string_concat(
    builder: ir.IRBuilder,
    resolver,
    lhs: int,
    rhs: int,
    dst: int,
    lhs_raw,
    rhs_raw,
    lhs_val,
    rhs_val,
    vmap: Dict[int, ir.Value],
):
    fast_res = _try_lower_string_concat_fast(
        builder,
        resolver,
        lhs,
        rhs,
        dst,
        lhs_raw,
        rhs_raw,
        lhs_val,
        rhs_val,
    )
    if fast_res is not None:
        _finalize_string_concat_result(resolver, vmap, dst, fast_res, "binop_concat_fast")
        return

    hl, hr = _materialize_string_concat_handles(
        builder,
        resolver,
        lhs,
        rhs,
        dst,
        lhs_raw,
        rhs_raw,
        lhs_val,
        rhs_val,
    )
    result = _dispatch_string_concat(builder, dst, lhs_raw, rhs_raw, hl, hr)
    _finalize_string_concat_result(resolver, vmap, dst, result, "binop_concat")


def _extract_concat_hh_args(raw) -> Optional[tuple]:
    """Return (a, b) when raw is call nyash.string.concat_hh(a, b); otherwise None."""
    if raw is None:
        return None
    called = getattr(raw, "called_function", None)
    if getattr(called, "name", None) != "nyash.string.concat_hh":
        return None
    try:
        operands = list(getattr(raw, "operands", []))
    except Exception:
        return None
    if len(operands) < 3:
        return None
    a = operands[1]
    b = operands[2]
    if not (hasattr(a, "type") and isinstance(a.type, ir.IntType) and a.type.width == 64):
        return None
    if not (hasattr(b, "type") and isinstance(b.type, ir.IntType) and b.type.width == 64):
        return None
    return (a, b)


def _concat3_chain_args(lhs_raw, rhs_raw, hl, hr) -> Optional[tuple]:
    """Detect one-level concat chain and return (args, folded_call)."""
    lhs_chain = _extract_concat_hh_args(lhs_raw)
    if lhs_chain is not None:
        return ((lhs_chain[0], lhs_chain[1], hr), lhs_raw)
    rhs_chain = _extract_concat_hh_args(rhs_raw)
    if rhs_chain is not None:
        return ((hl, rhs_chain[0], rhs_chain[1]), rhs_raw)
    return None


def _value_has_users_in_function(func, target, *, ignore=None) -> bool:
    if func is None or target is None:
        return True
    for bb in getattr(func, "blocks", []):
        for inst in getattr(bb, "instructions", []):
            if inst is target or inst is ignore:
                continue
            for opnd in getattr(inst, "operands", []):
                if opnd is target:
                    return True
    return False


def _prune_dead_chain_call(builder: ir.IRBuilder, folded_call, replacement_call=None) -> None:
    """
    Remove folded concat_hh call only when it is proven dead in the current function.
    `replacement_call` is ignored when checking uses (concat3 naturally references
    original operands, not folded_call).
    """
    if folded_call is None:
        return
    parent = getattr(folded_call, "parent", None)
    if parent is None:
        return
    func = getattr(parent, "parent", None)
    if _value_has_users_in_function(func, folded_call, ignore=replacement_call):
        return
    try:
        parent.instructions.remove(folded_call)
    except Exception:
        pass
