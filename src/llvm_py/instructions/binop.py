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


def _prune_dead_chain_call(builder: ir.IRBuilder, folded_call) -> None:
    """Remove folded concat_hh call when it became dead after concat3 rewrite."""
    if folded_call is None:
        return
    parent = getattr(folded_call, "parent", None)
    if parent is None:
        return
    func = getattr(parent, "parent", None)
    if _value_has_users_in_function(func, folded_call):
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

    # Resolve operands as i64
    fast_int = os.environ.get('NYASH_LLVM_FAST_INT') == '1'
    lhs_val = None
    rhs_val = None
    if fast_int:
        # Prefer same-block SSA directly to avoid resolver/PHI materialization cost in hot loops
        lhs_val = vmap.get(lhs)
        rhs_val = vmap.get(rhs)
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
    # Normalize operation aliases (textual -> symbolic)
    op_raw = op or ''
    op_l = op_raw.lower()
    alias = {
        'add': '+', 'plus': '+',
        'sub': '-', 'minus': '-',
        'mul': '*', 'times': '*',
        'div': '/',
        'mod': '%', 'rem': '%',
        'band': '&', 'bitand': '&',
        'bor': '|', 'bitor': '|',
        'bxor': '^', 'xor': '^',
        'shl': '<<',
        'shr': '>>', 'ashr': '>>',
    }
    op = alias.get(op_l, op_raw)
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
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        lhs_raw = vmap.get(lhs)
        rhs_raw = vmap.get(rhs)
        # Prefer handle pipeline to keep handles consistent across blocks/ret
        # pointer present?
        is_ptr_side = (hasattr(lhs_raw, 'type') and isinstance(lhs_raw.type, ir.PointerType)) or \
                      (hasattr(rhs_raw, 'type') and isinstance(rhs_raw.type, ir.PointerType))

        # Phase 131-11-E: Use dst_type as authoritative hint
        # After Phase 131-11-E, dst_type is correctly set by BinOp re-propagation
        # - "i64" means integer arithmetic (even if operands are unknown)
        # - {"kind": "handle", "box_type": "StringBox"} means string concat
        # - None/missing means fallback to operand analysis
        explicit_integer = False
        explicit_string = False
        try:
            if dst_type == "i64":
                explicit_integer = True
            elif isinstance(dst_type, dict) and dst_type.get('kind') == 'handle' and dst_type.get('box_type') == 'StringBox':
                explicit_string = True
        except Exception:
            pass

        # Phase 131-15-P1: TypeFacts > dst_type hint
        # Check operand facts before applying dst_type hint
        operand_is_string = False
        try:
            # Check 1: string literals
            if resolver is not None and hasattr(resolver, 'string_literals'):
                if lhs in resolver.string_literals or rhs in resolver.string_literals:
                    operand_is_string = True
            # Check 2: value_types
            if not operand_is_string and resolver is not None and hasattr(resolver, 'value_types'):
                lhs_type = resolver.value_types.get(lhs)
                rhs_type = resolver.value_types.get(rhs)
                if lhs_type and (lhs_type.get('kind') == 'string' or
                                (lhs_type.get('kind') == 'handle' and lhs_type.get('box_type') == 'StringBox')):
                    operand_is_string = True
                if rhs_type and (rhs_type.get('kind') == 'string' or
                                (rhs_type.get('kind') == 'handle' and rhs_type.get('box_type') == 'StringBox')):
                    operand_is_string = True
            # Check 3: pointer type
            if not operand_is_string:
                if lhs_raw is not None and hasattr(lhs_raw, 'type') and isinstance(lhs_raw.type, ir.PointerType):
                    operand_is_string = True
                if rhs_raw is not None and hasattr(rhs_raw, 'type') and isinstance(rhs_raw.type, ir.PointerType):
                    operand_is_string = True
        except Exception:
            pass

        # Phase 131-15-P1: Operand facts take priority over dst_type hint
        if operand_is_string:
            # Operand is string: MUST use string concat
            if explicit_integer and os.environ.get('NYASH_LLVM_STRICT') == '1':
                # Fail-Fast in STRICT mode
                raise RuntimeError(
                    f"[LLVM_PY/STRICT] Type conflict: dst_type=i64 but operand is string. "
                    f"lhs={lhs} rhs={rhs}"
                )
            # Force string concatenation when operand facts say so
            is_str = True
        elif explicit_integer:
            # No string operands + explicit i64 hint: integer arithmetic
            is_str = False
        elif explicit_string:
            # Explicit string hint: string concat
            is_str = True
        else:
            # Phase 196: TypeFacts SSOT - Only check for actual string types (not use-site demands)
            # Check if BOTH operands are known to be strings from their definition
            any_tagged = False
            try:
                if resolver is not None:
                    # Only check string_literals (TypeFacts), NOT is_stringish (TypeDemands)
                    if hasattr(resolver, 'string_literals'):
                        any_tagged = (lhs in resolver.string_literals) or (rhs in resolver.string_literals)
                    # Check if resolver has explicit type information (MirType::String or StringBox)
                    if not any_tagged and hasattr(resolver, 'value_types'):
                        lhs_ty = resolver.value_types.get(lhs)
                        rhs_ty = resolver.value_types.get(rhs)
                        lhs_str = lhs_ty and (lhs_ty.get('kind') == 'string' or
                                             (lhs_ty.get('kind') == 'handle' and lhs_ty.get('box_type') == 'StringBox'))
                        rhs_str = rhs_ty and (rhs_ty.get('kind') == 'string' or
                                             (rhs_ty.get('kind') == 'handle' and rhs_ty.get('box_type') == 'StringBox'))
                        any_tagged = lhs_str or rhs_str
            except Exception:
                pass
            is_str = is_ptr_side or any_tagged

        # Phase 131-11-E DEBUG
        if os.environ.get('NYASH_BINOP_DEBUG') == '1':
            print(f"[binop +] lhs={lhs} rhs={rhs} dst={dst}")
            print(f"  dst_type={dst_type} explicit_integer={explicit_integer} explicit_string={explicit_string}")
            print(f"  operand_is_string={operand_is_string} is_ptr_side={is_ptr_side} is_str={is_str}")
            if hasattr(resolver, 'value_types'):
                lhs_vt = resolver.value_types.get(lhs)
                rhs_vt = resolver.value_types.get(rhs)
                print(f"  value_types: lhs={lhs_vt} rhs={rhs_vt}")
        if is_str:
            # Helper: convert raw or resolved value to string handle
            def to_handle(raw, val, tag: str, vid: int):
                # If we already have an i64 SSA (handle) in vmap/raw or resolved val, prefer pass-through.
                if raw is not None and hasattr(raw, 'type') and isinstance(raw.type, ir.IntType) and raw.type.width == 64:
                    return raw
                if raw is not None and hasattr(raw, 'type') and isinstance(raw.type, ir.PointerType):
                    # pointer-to-array -> GEP
                    try:
                        if isinstance(raw.type.pointee, ir.ArrayType):
                            c0 = ir.Constant(ir.IntType(32), 0)
                            raw = builder.gep(raw, [c0, c0], name=f"bin_gep_{tag}_{dst}")
                    except Exception:
                        pass
                    cal = None
                    for f in builder.module.functions:
                        if f.name == 'nyash.box.from_i8_string':
                            cal = f; break
                    if cal is None:
                        cal = ir.Function(builder.module, ir.FunctionType(i64, [i8p]), name='nyash.box.from_i8_string')
                    return builder.call(cal, [raw], name=f"str_ptr2h_{tag}_{dst}")
                # if already i64
                if val is not None and hasattr(val, 'type') and isinstance(val.type, ir.IntType) and val.type.width == 64:
                    # Treat resolved i64 as a handle in string domain（never box numeric here）
                    return val
                return ir.Constant(i64, 0)

            def any_to_string_handle(handle_val, tag: str):
                callee = None
                for f in builder.module.functions:
                    if f.name == 'nyash.any.toString_h':
                        callee = f
                        break
                if callee is None:
                    callee = ir.Function(
                        builder.module,
                        ir.FunctionType(i64, [i64]),
                        name='nyash.any.toString_h',
                    )
                return builder.call(callee, [handle_val], name=f"any_tostr_h_{tag}_{dst}")

            def needs_stringify_bridge(vid: int, tagged: bool, raw, val) -> bool:
                if tagged:
                    return False
                # Plain i64 constants are likely primitive numerics.
                try:
                    c = raw if isinstance(raw, ir.Constant) else val
                    if isinstance(c, ir.Constant) and isinstance(c.type, ir.IntType) and c.type.width == 64:
                        return True
                except Exception:
                    pass
                return False

            # Phase 196: TypeFacts/Resolver SSOT - Use handle+handle only when BOTH are strings.
            # Root cause (Phase 102): loop-carried string accumulator may be i64-handle but not present in value_types;
            # tag lookup MUST consult resolver.is_stringish()/string_ids.
            lhs_tag = False
            rhs_tag = False
            try:
                if resolver is not None:
                    # SSOT: resolver's stringish tag (propagated via Copy/PHI)
                    if hasattr(resolver, 'is_stringish'):
                        lhs_tag = bool(resolver.is_stringish(lhs))
                        rhs_tag = bool(resolver.is_stringish(rhs))
                    # Legacy: actual string constants by ValueId
                    if hasattr(resolver, 'string_literals'):
                        lhs_tag = lhs_tag or (lhs in resolver.string_literals)
                        rhs_tag = rhs_tag or (rhs in resolver.string_literals)
                    # Legacy: value_types hints (best-effort)
                    if hasattr(resolver, 'value_types'):
                        if not lhs_tag:
                            lhs_ty = resolver.value_types.get(lhs)
                            if lhs_ty and (lhs_ty.get('kind') == 'string' or
                                          (lhs_ty.get('kind') == 'handle' and lhs_ty.get('box_type') == 'StringBox')):
                                lhs_tag = True
                        if not rhs_tag:
                            rhs_ty = resolver.value_types.get(rhs)
                            if rhs_ty and (rhs_ty.get('kind') == 'string' or
                                          (rhs_ty.get('kind') == 'handle' and rhs_ty.get('box_type') == 'StringBox')):
                                rhs_tag = True
            except Exception:
                pass
            # Phase 131-15-P1 DEBUG
            if os.environ.get('NYASH_BINOP_DEBUG') == '1':
                print(f"  [concat path] lhs_tag={lhs_tag} rhs_tag={rhs_tag}")
            # Always materialize concat result in string path.
            # If tags are partial/missing, bridge through any.toString_h to avoid
            # treating raw numeric i64 as a host-handle.
            hl = to_handle(lhs_raw, lhs_val, 'l', lhs)
            hr = to_handle(rhs_raw, rhs_val, 'r', rhs)
            if needs_stringify_bridge(lhs, lhs_tag, lhs_raw, lhs_val):
                hl = any_to_string_handle(hl, 'l')
            if needs_stringify_bridge(rhs, rhs_tag, rhs_raw, rhs_val):
                hr = any_to_string_handle(hr, 'r')

            concat3_info = _concat3_chain_args(lhs_raw, rhs_raw, hl, hr)
            if concat3_info is not None:
                concat3_args, folded_call = concat3_info
                hhh_fnty = ir.FunctionType(i64, [i64, i64, i64])
                callee3 = None
                for f in builder.module.functions:
                    if f.name == 'nyash.string.concat3_hhh':
                        callee3 = f
                        break
                if callee3 is None:
                    callee3 = ir.Function(builder.module, hhh_fnty, name='nyash.string.concat3_hhh')
                res = builder.call(callee3, list(concat3_args), name=f"concat3_hhh_{dst}")
                # Keep folded concat_hh alive for now.
                # Lowering is single-pass, so future instructions/blocks may still reference it
                # via copy/release/phi, and eager pruning can create undefined IR values.
            else:
                hh_fnty = ir.FunctionType(i64, [i64, i64])
                callee = None
                for f in builder.module.functions:
                    if f.name == 'nyash.string.concat_hh':
                        callee = f
                        break
                if callee is None:
                    callee = ir.Function(builder.module, hh_fnty, name='nyash.string.concat_hh')
                res = builder.call(callee, [hl, hr], name=f"concat_hh_{dst}")
            safe_vmap_write(vmap, dst, res, "binop_concat", resolver=resolver)
            # Phase 275 C2: String+String only - mixed concat removed
            # Tag result as string handle so subsequent '+' stays in string domain
            try:
                if resolver is not None and hasattr(resolver, 'mark_string'):
                    resolver.mark_string(dst)
            except Exception:
                pass
            return

    # Phase 275 C2: Int+Float promotion (number-only)
    # Type discrimination BEFORE resolve (to avoid premature i64 conversion)
    if op == '+':
        lhs_type = None
        rhs_type = None
        if resolver is not None and hasattr(resolver, 'value_types') and isinstance(resolver.value_types, dict):
            lhs_meta = resolver.value_types.get(lhs)
            rhs_meta = resolver.value_types.get(rhs)
            # Normalize type metadata
            if lhs_meta == 'i64' or lhs_meta == 'Integer' or (isinstance(lhs_meta, dict) and lhs_meta.get('kind') in ('i64', 'Integer')):
                lhs_type = 'Integer'
            elif lhs_meta == 'f64' or lhs_meta == 'Float' or (isinstance(lhs_meta, dict) and lhs_meta.get('kind') in ('f64', 'Float')):
                lhs_type = 'Float'
            if rhs_meta == 'i64' or rhs_meta == 'Integer' or (isinstance(rhs_meta, dict) and rhs_meta.get('kind') in ('i64', 'Integer')):
                rhs_type = 'Integer'
            elif rhs_meta == 'f64' or rhs_meta == 'Float' or (isinstance(rhs_meta, dict) and rhs_meta.get('kind') in ('f64', 'Float')):
                rhs_type = 'Float'

        # Int+Float or Float+Int detected: promote Int to Float, use fadd
        if (lhs_type == 'Integer' and rhs_type == 'Float') or (lhs_type == 'Float' and rhs_type == 'Integer'):
            from trace import values as trace_values
            trace_values(f"[binop] Int+Float addition: lhs={lhs}({lhs_type}) rhs={rhs}({rhs_type})")

            i64 = ir.IntType(64)
            f64 = ir.DoubleType()

            # Get values from vmap (as handles/values, don't resolve to i64 yet)
            lhs_val = vmap.get(lhs)
            rhs_val = vmap.get(rhs)
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
                ) if resolver else ir.Constant(i64, 0)
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
                ) if resolver else ir.Constant(i64, 0)

            # Determine which is Integer (i64 value) and which is Float (handle or double constant)
            if lhs_type == 'Integer' and rhs_type == 'Float':
                int_val = lhs_val  # i64 integer value
                float_val_or_handle = rhs_val  # Could be double constant or i64 handle
            else:  # lhs_type == 'Float' and rhs_type == 'Integer'
                float_val_or_handle = lhs_val  # Could be double constant or i64 handle
                int_val = rhs_val  # i64 integer value

            # Convert Integer (i64) to Float (f64) using sitofp
            int_as_float = builder.sitofp(int_val, f64, name='int_to_f64')

            # Phase 275 C2: Float might be double constant (from const.py) or i64 handle
            # Check actual LLVM type to determine which path
            float_val = None
            try:
                val_type = float_val_or_handle.type
                if isinstance(val_type, ir.DoubleType):
                    # Already a double constant - use directly
                    float_val = float_val_or_handle
                    trace_values(f"[binop] Float is double constant, using directly")
                elif isinstance(val_type, ir.IntType) and val_type.width == 64:
                    # i64 handle - needs unboxing via kernel helper
                    trace_values(f"[binop] Float is i64 handle, unboxing")
                    unbox_func_name = 'nyash.float.unbox_to_f64'
                    unbox_func = None
                    for f in builder.module.functions:
                        if f.name == unbox_func_name:
                            unbox_func = f
                            break
                    if not unbox_func:
                        unbox_func = ir.Function(builder.module, ir.FunctionType(f64, [i64]), name=unbox_func_name)
                    float_val = builder.call(unbox_func, [float_val_or_handle], name='unbox_float')
            except Exception as e:
                trace_values(f"[binop] Exception checking Float type: {e}, assuming constant")
                # Fallback: assume constant
                float_val = float_val_or_handle

            if float_val is None:
                float_val = ir.Constant(f64, 0.0)

            # Add using fadd (double + double)
            if lhs_type == 'Integer':
                result = builder.fadd(int_as_float, float_val, name='int_float_add')
            else:  # lhs was Float
                result = builder.fadd(float_val, int_as_float, name='float_int_add')

            # Phase 275 P0: Store double directly in vmap (no boxing)
            # vmap can hold both i64 and double values
            safe_vmap_write(vmap, dst, result, "binop_int_float_add", resolver=resolver)
            return  # Don't fall through to integer add

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
