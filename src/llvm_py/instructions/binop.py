"""
BinOp (Binary Operation) instruction lowering
Handles +, -, *, /, %, &, |, ^, <<, >>
"""

import llvmlite.ir as ir
from typing import Dict, Optional, Any
from utils.values import resolve_i64_strict, safe_vmap_write
import os
from .compare import lower_compare
from trace import hot_count as trace_hot_count

_BINOP_COMMUTATIVE_OPS = {"+", "*", "&", "|", "^"}
_BINOP_EXPR_CACHEABLE_OPS = _BINOP_COMMUTATIVE_OPS | {"-", "/", "%", "<<", ">>"}


def _canonicalize_i64(builder: ir.IRBuilder, value, vid, vmap: Dict[int, ir.Value], hint: str):
    """Normalize integers/pointers to i64 and cache per value id for FAST_INT paths."""
    if value is None:
        return None
    target = ir.IntType(64)
    try:
        vtype = value.type
    except Exception:
        vtype = None
    if isinstance(vtype, ir.PointerType):
        value = builder.ptrtoint(value, target, name=f"{hint}_p2i_{vid}")
    elif isinstance(vtype, ir.IntType):
        width = vtype.width
        if width < 64:
            value = builder.zext(value, target, name=f"{hint}_zext_{vid}")
        elif width > 64:
            value = builder.trunc(value, target, name=f"{hint}_trunc_{vid}")
    if isinstance(vid, int):
        safe_vmap_write(vmap, vid, value, f"canonicalize_{hint}")
    return value


def _block_id_from_name(current_block) -> Optional[int]:
    if current_block is None:
        return None
    name = getattr(current_block, "name", "")
    if not isinstance(name, str) or not name.startswith("bb"):
        return None
    try:
        return int(name[2:])
    except Exception:
        return None


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
        elif isinstance(dst_type, dict) and dst_type.get("kind") == "handle" and dst_type.get("box_type") == "StringBox":
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
    is_ptr_side = (
        (hasattr(lhs_raw, "type") and isinstance(lhs_raw.type, ir.PointerType))
        or (hasattr(rhs_raw, "type") and isinstance(rhs_raw.type, ir.PointerType))
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
    return is_ptr_side or _binop_plus_any_tagged_string(resolver, lhs, rhs), explicit_integer, explicit_string, operand_is_string


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


def _ensure_module_function(module, name: str, return_type, arg_types):
    for func in module.functions:
        if func.name == name:
            return func
    return ir.Function(module, ir.FunctionType(return_type, arg_types), name=name)


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


def _value_sig(val, fallback_vid: int):
    # Prefer literal identity for constants so repeated const ValueIds share cache key.
    try:
        if isinstance(val, ir.Constant):
            vtype = getattr(val, "type", None)
            if isinstance(vtype, ir.IntType):
                return ("c_i", vtype.width, int(val.constant))
            if isinstance(vtype, ir.DoubleType):
                return ("c_f", float(val.constant))
    except Exception:
        pass
    if val is not None:
        return ("obj", id(val))
    return ("v", int(fallback_vid))


def _binop_expr_cache_key(op: str, lhs_val, rhs_val, lhs_vid: int, rhs_vid: int):
    if op not in _BINOP_EXPR_CACHEABLE_OPS:
        return None
    if lhs_val is None or rhs_val is None:
        return None
    lhs_key = _value_sig(lhs_val, lhs_vid)
    rhs_key = _value_sig(rhs_val, rhs_vid)
    if op in _BINOP_COMMUTATIVE_OPS and lhs_key > rhs_key:
        lhs_key, rhs_key = rhs_key, lhs_key
    return ("i64", op, lhs_key, rhs_key)


def _cache_entry_reusable(resolver, current_bid: Optional[int], def_bid: Optional[int]) -> bool:
    # Conservative fallback: only allow same-block reuse when dominance metadata is absent.
    if current_bid is None:
        return def_bid is None
    if def_bid is None:
        return False
    if current_bid == def_bid:
        return True
    ctx = getattr(resolver, "context", None) if resolver is not None else None
    if ctx is None:
        return False
    try:
        return bool(ctx.dominates(def_bid, current_bid))
    except Exception:
        return False


def _normalize_binop_op(op: str) -> str:
    op_raw = op or ""
    op_l = op_raw.lower()
    alias = {
        "add": "+",
        "plus": "+",
        "sub": "-",
        "minus": "-",
        "mul": "*",
        "times": "*",
        "div": "/",
        "mod": "%",
        "rem": "%",
        "band": "&",
        "bitand": "&",
        "bor": "|",
        "bitor": "|",
        "bxor": "^",
        "xor": "^",
        "shl": "<<",
        "shr": ">>",
        "ashr": ">>",
    }
    return alias.get(op_l, op_raw)


def _resolve_binop_i64_operands(
    builder: ir.IRBuilder,
    resolver,
    lhs: int,
    rhs: int,
    vmap: Dict[int, ir.Value],
    current_block: ir.Block,
    preds=None,
    block_end_values=None,
    bb_map=None,
):
    fast_int = os.environ.get("NYASH_LLVM_FAST_INT") == "1"
    lhs_val = vmap.get(lhs) if fast_int else None
    rhs_val = vmap.get(rhs) if fast_int else None
    if lhs_val is None:
        lhs_val = resolve_i64_strict(
            resolver,
            lhs,
            current_block,
            preds,
            block_end_values,
            vmap,
            bb_map,
            hot_scope="binop",
        )
    if rhs_val is None:
        rhs_val = resolve_i64_strict(
            resolver,
            rhs,
            current_block,
            preds,
            block_end_values,
            vmap,
            bb_map,
            hot_scope="binop",
        )
    lhs_val = _canonicalize_i64(builder, lhs_val, lhs, vmap, "bin_lhs")
    rhs_val = _canonicalize_i64(builder, rhs_val, rhs, vmap, "bin_rhs")
    if lhs_val is None:
        lhs_val = ir.Constant(ir.IntType(64), 0)
    if rhs_val is None:
        rhs_val = ir.Constant(ir.IntType(64), 0)
    return lhs_val, rhs_val


def _binop_numeric_meta_kind(meta) -> Optional[str]:
    if meta == "i64" or meta == "Integer" or (
        isinstance(meta, dict) and meta.get("kind") in ("i64", "Integer")
    ):
        return "Integer"
    if meta == "f64" or meta == "Float" or (
        isinstance(meta, dict) and meta.get("kind") in ("f64", "Float")
    ):
        return "Float"
    return None


def _binop_plus_numeric_types(resolver, lhs: int, rhs: int) -> tuple[Optional[str], Optional[str]]:
    if resolver is None or not hasattr(resolver, "value_types") or not isinstance(resolver.value_types, dict):
        return None, None
    return (
        _binop_numeric_meta_kind(resolver.value_types.get(lhs)),
        _binop_numeric_meta_kind(resolver.value_types.get(rhs)),
    )


def _resolve_binop_value(
    resolver,
    value_id: int,
    vmap: Dict[int, ir.Value],
    current_block: ir.Block,
    preds=None,
    block_end_values=None,
    bb_map=None,
):
    value = vmap.get(value_id)
    if value is not None:
        return value
    if resolver is None:
        return ir.Constant(ir.IntType(64), 0)
    return resolve_i64_strict(
        resolver,
        value_id,
        current_block,
        preds,
        block_end_values,
        vmap,
        bb_map,
        hot_scope="binop",
    )


def _coerce_float_operand_to_f64(builder: ir.IRBuilder, operand, *, trace_values=None):
    i64 = ir.IntType(64)
    f64 = ir.DoubleType()
    if operand is None:
        return ir.Constant(f64, 0.0)
    try:
        val_type = operand.type
        if isinstance(val_type, ir.DoubleType):
            if trace_values is not None:
                trace_values("[binop] Float is double constant, using directly")
            return operand
        if isinstance(val_type, ir.IntType) and val_type.width == 64:
            if trace_values is not None:
                trace_values("[binop] Float is i64 handle, unboxing")
            callee = _ensure_module_function(
                builder.module,
                "nyash.float.unbox_to_f64",
                f64,
                [i64],
            )
            return builder.call(callee, [operand], name="unbox_float")
    except Exception as exc:
        if trace_values is not None:
            trace_values(f"[binop] Exception checking Float type: {exc}, assuming constant")
        return operand
    return ir.Constant(f64, 0.0)


def _lower_int_float_addition(
    builder: ir.IRBuilder,
    resolver,
    lhs: int,
    rhs: int,
    dst: int,
    vmap: Dict[int, ir.Value],
    current_block: ir.Block,
    preds=None,
    block_end_values=None,
    bb_map=None,
) -> bool:
    lhs_type, rhs_type = _binop_plus_numeric_types(resolver, lhs, rhs)
    if (lhs_type, rhs_type) not in {("Integer", "Float"), ("Float", "Integer")}:
        return False

    from trace import values as trace_values

    trace_values(f"[binop] Int+Float addition: lhs={lhs}({lhs_type}) rhs={rhs}({rhs_type})")
    f64 = ir.DoubleType()
    lhs_val = _resolve_binop_value(resolver, lhs, vmap, current_block, preds, block_end_values, bb_map)
    rhs_val = _resolve_binop_value(resolver, rhs, vmap, current_block, preds, block_end_values, bb_map)

    if lhs_type == "Integer":
        int_val = lhs_val
        float_val_or_handle = rhs_val
    else:
        float_val_or_handle = lhs_val
        int_val = rhs_val

    int_as_float = builder.sitofp(int_val, f64, name="int_to_f64")
    float_val = _coerce_float_operand_to_f64(builder, float_val_or_handle, trace_values=trace_values)

    if lhs_type == "Integer":
        result = builder.fadd(int_as_float, float_val, name="int_float_add")
    else:
        result = builder.fadd(float_val, int_as_float, name="float_int_add")

    safe_vmap_write(vmap, dst, result, "binop_int_float_add", resolver=resolver)
    return True


def _const_i64_literal(val):
    try:
        if isinstance(val, ir.Constant):
            vtype = getattr(val, "type", None)
            if isinstance(vtype, ir.IntType):
                return int(val.constant)
    except Exception:
        pass
    return None


def _is_positive_power_of_two(n: int) -> bool:
    return n > 0 and (n & (n - 1)) == 0


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


def _lower_mod_op(builder: ir.IRBuilder, resolver, lhs_val, rhs_val, lhs_vid: int, dst: int, i64):
    """Lower signed remainder with a power-of-two fast path."""
    rhs_const = _const_i64_literal(rhs_val)
    lhs_nonneg = False
    try:
        nonneg_ids = getattr(resolver, "non_negative_ids", None)
        lhs_nonneg = isinstance(nonneg_ids, set) and int(lhs_vid) in nonneg_ids
    except Exception:
        lhs_nonneg = False

    if isinstance(rhs_const, int) and _is_positive_power_of_two(rhs_const):
        rhs_pow2 = ir.Constant(i64, rhs_const)
        mask = ir.Constant(i64, rhs_const - 1)
        masked = builder.and_(lhs_val, mask, name=f"rem2k_mask_{dst}")
        if lhs_nonneg:
            trace_hot_count(resolver, "binop_mod_pow2_and")
            return masked

        zero = ir.Constant(i64, 0)
        lhs_neg = builder.icmp_signed('<', lhs_val, zero, name=f"rem2k_neg_{dst}")
        masked_nonzero = builder.icmp_signed('!=', masked, zero, name=f"rem2k_nz_{dst}")
        need_adjust = builder.and_(lhs_neg, masked_nonzero, name=f"rem2k_needadj_{dst}")
        adjusted = builder.sub(masked, rhs_pow2, name=f"rem2k_adj_{dst}")
        trace_hot_count(resolver, "binop_mod_pow2_signed")
        return builder.select(need_adjust, adjusted, masked, name=f"rem2k_sel_{dst}")

    # Signed remainder fallback
    return builder.srem(lhs_val, rhs_val, name=f"rem_{dst}")

def lower_binop(
    builder: ir.IRBuilder,
    resolver,  # Resolver instance
    op: str,
    lhs: int,
    rhs: int,
    dst: int,
    vmap: Dict[int, ir.Value],
    current_block: ir.Block,
    preds=None,
    block_end_values=None,
    bb_map=None,
    *,
    dst_type: Optional[Any] = None,
) -> None:
    """
    Lower MIR BinOp instruction
    
    Args:
        builder: Current LLVM IR builder
        resolver: Resolver for value resolution
        op: Operation string (+, -, *, /, etc.)
        lhs: Left operand value ID
        rhs: Right operand value ID
        dst: Destination value ID
        vmap: Value map
        current_block: Current basic block
    """
    trace_hot_count(resolver, "binop_total")
    op = _normalize_binop_op(op)
    if op == '%':
        trace_hot_count(resolver, "binop_mod")

    # Relational/equality operators delegate to compare
    if op in ('==','!=','<','>','<=','>='):
        # Delegate to compare with resolver/preds context to maintain dominance via localization
        lower_compare(
            builder,
            op,
            lhs,
            rhs,
            dst,
            vmap,
            resolver=resolver,
            current_block=current_block,
            preds=preds,
            block_end_values=block_end_values,
            bb_map=bb_map,
            ctx=getattr(resolver, 'ctx', None),
        )
        return

    # String-aware concatenation unified to handles (i64).
    # Use concat_hh when either side is a pointer string OR either side is tagged as string handle
    # (including literal strings and PHI-propagated tags).
    if op == '+':
        lhs_raw = vmap.get(lhs)
        rhs_raw = vmap.get(rhs)
        lhs_val = _resolve_binop_value(
            resolver,
            lhs,
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
        )
        rhs_val = _resolve_binop_value(
            resolver,
            rhs,
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
        )
        is_str, explicit_integer, explicit_string, operand_is_string = _binop_plus_prefers_string_path(
            resolver,
            lhs,
            rhs,
            lhs_raw,
            rhs_raw,
            dst_type,
        )

        # Phase 131-11-E DEBUG
        if os.environ.get('NYASH_BINOP_DEBUG') == '1':
            print(f"[binop +] lhs={lhs} rhs={rhs} dst={dst}")
            print(f"  dst_type={dst_type} explicit_integer={explicit_integer} explicit_string={explicit_string}")
            print(f"  operand_is_string={operand_is_string} is_str={is_str}")
            if hasattr(resolver, 'value_types'):
                lhs_vt = resolver.value_types.get(lhs)
                rhs_vt = resolver.value_types.get(rhs)
                print(f"  value_types: lhs={lhs_vt} rhs={rhs_vt}")
        if is_str:
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
            res = _dispatch_string_concat(builder, dst, lhs_raw, rhs_raw, hl, hr)
            safe_vmap_write(vmap, dst, res, "binop_concat", resolver=resolver)
            # Phase 275 C2: String+String only - mixed concat removed
            # Tag result as string handle so subsequent '+' stays in string domain
            try:
                if resolver is not None and hasattr(resolver, 'mark_string'):
                    resolver.mark_string(dst)
            except Exception:
                pass
            return

    if op == "+" and _lower_int_float_addition(
        builder,
        resolver,
        lhs,
        rhs,
        dst,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    ):
        return

    lhs_val, rhs_val = _resolve_binop_i64_operands(
        builder,
        resolver,
        lhs,
        rhs,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )

    # Ensure both are i64
    i64 = ir.IntType(64)
    if hasattr(lhs_val, 'type') and lhs_val.type != i64:
        # Type conversion if needed
        if lhs_val.type.is_pointer:
            lhs_val = builder.ptrtoint(lhs_val, i64, name=f"binop_lhs_p2i_{dst}")
    if hasattr(rhs_val, 'type') and rhs_val.type != i64:
        if rhs_val.type.is_pointer:
            rhs_val = builder.ptrtoint(rhs_val, i64, name=f"binop_rhs_p2i_{dst}")

    current_bid = _block_id_from_name(current_block)
    expr_cache = None
    expr_key = None
    if os.environ.get('NYASH_LLVM_FAST') == '1' and resolver is not None:
        cache_candidate = getattr(resolver, "binop_expr_cache", None)
        if isinstance(cache_candidate, dict):
            expr_cache = cache_candidate
            expr_key = _binop_expr_cache_key(op, lhs_val, rhs_val, lhs, rhs)
            if expr_key is not None:
                cached_entry = expr_cache.get(expr_key)
                cached = None
                cached_def_bid = current_bid
                if isinstance(cached_entry, tuple) and len(cached_entry) == 2:
                    cached = cached_entry[0]
                    try:
                        cached_def_bid = int(cached_entry[1])
                    except Exception:
                        cached_def_bid = None
                else:
                    cached = cached_entry
                if cached is not None and _cache_entry_reusable(resolver, current_bid, cached_def_bid):
                    trace_hot_count(resolver, "binop_expr_cache_hit")
                    safe_vmap_write(vmap, dst, cached, f"binop_{op}_expr_cache_hit")
                    return
                trace_hot_count(resolver, "binop_expr_cache_miss")
    
    # Perform operation
    if op == '+':
        result = builder.add(lhs_val, rhs_val, name=f"add_{dst}")
    elif op == '-':
        result = builder.sub(lhs_val, rhs_val, name=f"sub_{dst}")
    elif op == '*':
        result = builder.mul(lhs_val, rhs_val, name=f"mul_{dst}")
    elif op == '/':
        # Signed division
        result = builder.sdiv(lhs_val, rhs_val, name=f"div_{dst}")
    elif op == '%':
        result = _lower_mod_op(builder, resolver, lhs_val, rhs_val, lhs, dst, i64)
    elif op == '&':
        result = builder.and_(lhs_val, rhs_val, name=f"and_{dst}")
    elif op == '|':
        result = builder.or_(lhs_val, rhs_val, name=f"or_{dst}")
    elif op == '^':
        result = builder.xor(lhs_val, rhs_val, name=f"xor_{dst}")
    elif op == '<<':
        result = builder.shl(lhs_val, rhs_val, name=f"shl_{dst}")
    elif op == '>>':
        # Arithmetic shift right
        result = builder.ashr(lhs_val, rhs_val, name=f"ashr_{dst}")
    else:
        # Unknown op - return zero
        result = ir.Constant(i64, 0)

    # Phase 131-12-P1: Object identity trace before write
    import sys  # os already imported at module level
    if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
        print(f"[vmap/id] binop op={op} dst={dst} vmap id={id(vmap)} before_write", file=sys.stderr)

    if expr_cache is not None and expr_key is not None:
        expr_cache[expr_key] = (result, current_bid)

    # Store result
    safe_vmap_write(vmap, dst, result, f"binop_{op}")
