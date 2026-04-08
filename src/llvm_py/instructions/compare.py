"""
Compare instruction lowering
Handles comparison operations (<, >, <=, >=, ==, !=)
"""

import llvmlite.ir as ir
from typing import Dict, Optional, Any
from utils.values import resolve_i64_strict, safe_vmap_write
import os
from .externcall import lower_externcall
from .primitive_handles import (
    primitive_numeric_meta_kind,
    resolver_value_type,
    unbox_primitive_handle_if_needed,
)
from .string_fast import llvm_fast_enabled
from trace import values as trace_values
from trace import hot_count as trace_hot_count

_COMPARE_COMMUTATIVE_PREDS = {"==", "!="}


def _canonicalize_i64(builder: ir.IRBuilder, value, vid, vmap: Dict[int, ir.Value], hint: str):
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


def _compare_expr_cache_key(current_block, pred: str, lhs_val, rhs_val, keep_i1: bool):
    if current_block is None:
        return None
    if lhs_val is None or rhs_val is None:
        return None
    lhs_key = id(lhs_val)
    rhs_key = id(rhs_val)
    if pred in _COMPARE_COMMUTATIVE_PREDS and lhs_key > rhs_key:
        lhs_key, rhs_key = rhs_key, lhs_key
    mode = "i1" if keep_i1 else "i64"
    return (current_block.name, mode, pred, lhs_key, rhs_key)

def lower_compare(
    builder: ir.IRBuilder,
    op: str,
    lhs: int,
    rhs: int,
    dst: int,
    vmap: Dict[int, ir.Value],
    resolver=None,
    current_block=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    meta: Optional[Dict[str, Any]] = None,
    ctx: Optional[Any] = None,
) -> None:
    """
    Lower MIR Compare instruction
    
    Args:
        builder: Current LLVM IR builder
        op: Comparison operation (<, >, <=, >=, ==, !=)
        lhs: Left operand value ID
        rhs: Right operand value ID
        dst: Destination value ID
        vmap: Value map
    """
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
    trace_hot_count(resolver, "compare_total")
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()

    # Phase 275 B2: Type discrimination BEFORE resolve (to avoid premature i64 conversion)
    # Check for Int↔Float equality first
    if op in ('==', '!='):
        lhs_type = None
        rhs_type = None
        lhs_meta = None
        rhs_meta = None
        if resolver is not None and hasattr(resolver, 'value_types') and isinstance(resolver.value_types, dict):
            lhs_meta = resolver.value_types.get(lhs)
            rhs_meta = resolver.value_types.get(rhs)
            lhs_type = primitive_numeric_meta_kind(lhs_meta)
            rhs_type = primitive_numeric_meta_kind(rhs_meta)

        # Int↔Float equality: use kernel helper (avoid Float→i64 conversion)
        if (lhs_type == 'Integer' and rhs_type == 'Float') or (lhs_type == 'Float' and rhs_type == 'Integer'):
            trace_values(f"[compare] Int↔Float equality: lhs={lhs}({lhs_type}) rhs={rhs}({rhs_type})")

            # Get values from vmap (as handles, don't resolve to i64 yet)
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
                    hot_scope="compare",
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
                    hot_scope="compare",
                ) if resolver else ir.Constant(i64, 0)

            # Determine which is int and which is float
            if lhs_type == 'Integer' and rhs_type == 'Float':
                int_val = unbox_primitive_handle_if_needed(
                    builder,
                    lhs_val,
                    lhs_meta,
                    name_hint=f"cmp_int_float_lhs_{lhs}",
                )
                float_handle = rhs_val
            else:
                float_handle = lhs_val
                int_val = unbox_primitive_handle_if_needed(
                    builder,
                    rhs_val,
                    rhs_meta,
                    name_hint=f"cmp_int_float_rhs_{rhs}",
                )

            # Call kernel helper: i64 nyash.cmp.int_float_eq(i64 int_val, f64 float_val)
            # Phase 275 P0: Float as double (LLVM harness emits Float constants as double, not handle)
            eq_func_name = 'nyash.cmp.int_float_eq'
            eq_func = None
            for f in builder.module.functions:
                if f.name == eq_func_name:
                    eq_func = f
                    break
            if not eq_func:
                f64 = ir.DoubleType()
                eq_func = ir.Function(builder.module, ir.FunctionType(i64, [i64, f64]), name=eq_func_name)

            result_i64 = builder.call(eq_func, [int_val, float_handle], name='int_float_eq')

            # Phase 275 P0: Store i64 bool (0/1) directly to vmap (PHI expects i64)
            # Kernel helper returns i64(0/1), no need to convert to i1
            if op == '==':
                safe_vmap_write(vmap, dst, result_i64, "compare_int_float_eq", resolver=resolver)
            else:  # op == '!='
                zero = ir.Constant(i64, 0)
                one = ir.Constant(i64, 1)
                # i64 bool negation: 1 - result
                result_ne = builder.sub(one, result_i64, name='int_float_ne')
                safe_vmap_write(vmap, dst, result_ne, "compare_int_float_ne", resolver=resolver)
            return

    # Phase 275 P0: Float==Float comparison (both operands are Float)
    # Check type metadata BEFORE resolve, OR check actual LLVM type from vmap
    if op in ('==', '!=', '<', '>', '<=', '>='):
        lhs_type = None
        rhs_type = None
        lhs_meta = None
        rhs_meta = None

        # First, check MIR type metadata
        if resolver is not None and hasattr(resolver, 'value_types') and isinstance(resolver.value_types, dict):
            lhs_meta = resolver.value_types.get(lhs)
            rhs_meta = resolver.value_types.get(rhs)
            lhs_type = primitive_numeric_meta_kind(lhs_meta)
            rhs_type = primitive_numeric_meta_kind(rhs_meta)

        # Phase 275 P0 FIX: Also check actual LLVM type from vmap (fallback for missing metadata)
        # If vmap has a double value, treat it as Float
        lh_val = vmap.get(lhs)
        rh_val = vmap.get(rhs)
        if lhs_type is None and lh_val is not None:
            try:
                if isinstance(lh_val.type, ir.DoubleType):
                    lhs_type = 'Float'
            except Exception:
                pass
        if rhs_type is None and rh_val is not None:
            try:
                if isinstance(rh_val.type, ir.DoubleType):
                    rhs_type = 'Float'
            except Exception:
                pass

        # Float==Float (or either side is Float): use fcmp (not icmp)
        if lhs_type == 'Float' or rhs_type == 'Float':
            trace_values(f"[compare] Float==Float path: lhs={lhs}({lhs_type}) rhs={rhs}({rhs_type})")

            # Get Float values from vmap (might be double constants or i64 handles needing unboxing)
            lh = vmap.get(lhs)
            rh = vmap.get(rhs)

            # Fallback for values not in vmap
            if lh is None:
                lh = ir.Constant(ir.DoubleType(), 0.0)
            if rh is None:
                rh = ir.Constant(ir.DoubleType(), 0.0)

            # Phase 275 P0: Unbox i64 handles to double for fcmp
            # If type metadata says Float but LLVM type is i64, it's a boxed Float handle
            f64 = ir.DoubleType()
            def ensure_double(val, is_float_type):
                """Convert i64 Float handle to double if needed."""
                try:
                    if isinstance(val.type, ir.DoubleType):
                        return val  # Already double
                    if isinstance(val.type, ir.IntType) and val.type.width == 64 and is_float_type:
                        # i64 handle - unbox to double
                        unbox_func_name = 'nyash.float.unbox_to_f64'
                        unbox_func = None
                        for f in builder.module.functions:
                            if f.name == unbox_func_name:
                                unbox_func = f
                                break
                        if not unbox_func:
                            unbox_func = ir.Function(builder.module, ir.FunctionType(f64, [ir.IntType(64)]), name=unbox_func_name)
                        return builder.call(unbox_func, [val], name='unbox_for_fcmp')
                except Exception:
                    pass
                return val

            lh = ensure_double(lh, lhs_type == 'Float')
            rh = ensure_double(rh, rhs_type == 'Float')

            # Both should be double now
            # Perform ordered fcmp
            # Canonicalize operator
            def _canon(o: str) -> str:
                mapping = {
                    'Lt': '<', 'Le': '<=', 'Gt': '>', 'Ge': '>=', 'Eq': '==', 'Ne': '!='
                }
                return mapping.get(o, o)
            pred = _canon(op)
            if pred not in ('<','>','<=','>=','==','!='):
                pred = '=='
            fcmp_result = builder.fcmp_ordered(pred, lh, rh, name=f"fcmp_{dst}")

            # Normalize to i64 bool for vmap (PHI expects i64)
            i64 = ir.IntType(64)
            fcmp_i64 = builder.zext(fcmp_result, i64, name=f"fcmp_{dst}_i64")
            safe_vmap_write(vmap, dst, fcmp_i64, "compare_float_float", resolver=resolver)
            return

    # Get operands (standard path for non-Int↔Float/Float↔Float comparisons)
    # Prefer same-block SSA from vmap; fallback to resolver for cross-block dominance
    fast_int = os.environ.get('NYASH_LLVM_FAST_INT') == '1'
    lhs_val = None
    rhs_val = None
    if fast_int:
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
            prefer_local=False,
            hot_scope="compare",
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
            prefer_local=False,
            hot_scope="compare",
        )
    lhs_val = _canonicalize_i64(builder, lhs_val, lhs, vmap, "cmp_lhs")
    rhs_val = _canonicalize_i64(builder, rhs_val, rhs, vmap, "cmp_rhs")
    lhs_val = unbox_primitive_handle_if_needed(
        builder,
        lhs_val,
        resolver_value_type(resolver, lhs),
        name_hint=f"cmp_lhs_{lhs}",
    )
    rhs_val = unbox_primitive_handle_if_needed(
        builder,
        rhs_val,
        resolver_value_type(resolver, rhs),
        name_hint=f"cmp_rhs_{rhs}",
    )

    # String-aware equality: if meta marks string or either side is tagged string-ish,
    # compare handles directly via nyash.string.eq_hh
    if op in ('==','!='):
        force_string = False
        try:
            if isinstance(meta, dict) and meta.get('cmp_kind') == 'string':
                force_string = True
        except Exception:
            pass
        lhs_tag = False
        rhs_tag = False
        try:
            if resolver is not None and hasattr(resolver, 'is_stringish'):
                lhs_tag = resolver.is_stringish(lhs)
                rhs_tag = resolver.is_stringish(rhs)
        except Exception:
            pass
        if force_string or lhs_tag or rhs_tag:
            try:
                fn_name = getattr(getattr(builder, "block", None), "parent", None)
                fn_name = getattr(fn_name, "name", "?")
            except Exception:
                fn_name = "?"
            trace_values(
                f"[compare] string-eq path: fn={fn_name} lhs={lhs} rhs={rhs} "
                f"force={force_string} tagL={lhs_tag} tagR={rhs_tag}"
            )
            # String handles are often produced in-place in the same block. Even when
            # fast_int is disabled, prefer the local SSA handle before falling back to
            # dominance-based resolution so we do not accidentally compare against 0.
            local_lh = _canonicalize_i64(builder, vmap.get(lhs), lhs, vmap, "cmp_str_lhs_local")
            local_rh = _canonicalize_i64(builder, vmap.get(rhs), rhs, vmap, "cmp_str_rhs_local")
            # Prefer same-block SSA (vmap) since string handles are produced in-place; fallback to resolver
            lh = local_lh if local_lh is not None else (
                lhs_val if lhs_val is not None else (
                resolve_i64_strict(
                    resolver,
                    lhs,
                    current_block,
                    preds,
                    block_end_values,
                    vmap,
                    bb_map,
                    prefer_local=False,
                    hot_scope="compare",
                )
                if (resolver is not None and preds is not None and block_end_values is not None and current_block is not None) else ir.Constant(i64, 0)
            ))
            rh = local_rh if local_rh is not None else (
                rhs_val if rhs_val is not None else (
                resolve_i64_strict(
                    resolver,
                    rhs,
                    current_block,
                    preds,
                    block_end_values,
                    vmap,
                    bb_map,
                    prefer_local=False,
                    hot_scope="compare",
                )
                if (resolver is not None and preds is not None and block_end_values is not None and current_block is not None) else ir.Constant(i64, 0)
            ))
            trace_values(
                f"[compare] string-eq args: fn={fn_name} "
                f"local_lh={local_lh is not None} local_rh={local_rh is not None} "
                f"lhs_val={lhs_val is not None} rhs_val={rhs_val is not None} "
                f"lh_is_const={isinstance(lh, ir.Constant)} rh_is_const={isinstance(rh, ir.Constant)}"
            )
            eqf = None
            for f in builder.module.functions:
                if f.name == 'nyash.string.eq_hh':
                    eqf = f
                    break
            if not eqf:
                eqf = ir.Function(builder.module, ir.FunctionType(i64, [i64, i64]), name='nyash.string.eq_hh')
            eq = builder.call(eqf, [lh, rh], name='str_eq')
            if op == '==':
                safe_vmap_write(vmap, dst, eq, "compare_str_eq", resolver=resolver)
            else:
                one = ir.Constant(i64, 1)
                ne = builder.sub(one, eq, name='str_ne')
                safe_vmap_write(vmap, dst, ne, "compare_str_ne", resolver=resolver)
            return

    # Default integer compare path
    if lhs_val is None:
        lhs_val = ir.Constant(i64, 0)
    if rhs_val is None:
        rhs_val = ir.Constant(i64, 0)

    # Ensure both are i64
    if hasattr(lhs_val, 'type') and isinstance(lhs_val.type, ir.PointerType):
        lhs_val = builder.ptrtoint(lhs_val, i64)
    if hasattr(rhs_val, 'type') and isinstance(rhs_val.type, ir.PointerType):
        rhs_val = builder.ptrtoint(rhs_val, i64)
    
    # Perform signed comparison using canonical predicates ('<','>','<=','>=','==','!=')
    def _canon(o: str) -> str:
        mapping = {
            'Lt': '<', 'Le': '<=', 'Gt': '>', 'Ge': '>=', 'Eq': '==', 'Ne': '!='
        }
        return mapping.get(o, o)
    pred = _canon(op)
    if pred not in ('<','>','<=','>=','==','!='):
        pred = '=='
    # FAST path: branch-only compare results can stay i1 (avoids i64 round-trip
    # and redundant cond compare in hot loops). Other compare results remain i64.
    keep_i1 = False
    if llvm_fast_enabled():
        try:
            fast_set = getattr(resolver, "fast_branch_only_compare_dsts", None)
            keep_i1 = isinstance(fast_set, set) and int(dst) in fast_set
        except Exception:
            keep_i1 = False

    expr_cache = None
    expr_key = None
    if os.environ.get('NYASH_LLVM_FAST') == '1' and resolver is not None:
        cache_candidate = getattr(resolver, "compare_expr_cache", None)
        if isinstance(cache_candidate, dict):
            expr_cache = cache_candidate
            expr_key = _compare_expr_cache_key(current_block, pred, lhs_val, rhs_val, keep_i1)
            if expr_key is not None:
                cached = expr_cache.get(expr_key)
                if cached is not None:
                    trace_hot_count(resolver, "compare_expr_cache_hit")
                    safe_vmap_write(vmap, dst, cached, f"compare_{pred}_expr_cache_hit", resolver=resolver)
                    return
                trace_hot_count(resolver, "compare_expr_cache_miss")

    cmp_result_i1 = builder.icmp_signed(pred, lhs_val, rhs_val, name=f"cmp_{dst}")

    if keep_i1:
        trace_hot_count(resolver, "compare_keep_i1")
        if expr_cache is not None and expr_key is not None:
            expr_cache[expr_key] = cmp_result_i1
        safe_vmap_write(vmap, dst, cmp_result_i1, f"compare_{pred}_i1_fast", resolver=resolver)
        return

    # Default: normalize compare result to i64 bool (0/1) for vmap.
    trace_hot_count(resolver, "compare_to_i64")
    i64 = ir.IntType(64)
    cmp_result_i64 = builder.zext(cmp_result_i1, i64, name=f"cmp_{dst}_i64")
    if expr_cache is not None and expr_key is not None:
        expr_cache[expr_key] = cmp_result_i64
    safe_vmap_write(vmap, dst, cmp_result_i64, f"compare_{pred}")

def lower_fcmp(
    builder: ir.IRBuilder,
    op: str,
    lhs: int,
    rhs: int,
    dst: int,
    vmap: Dict[int, ir.Value]
) -> None:
    """
    Lower floating point comparison
    
    Args:
        builder: Current LLVM IR builder
        op: Comparison operation
        lhs: Left operand value ID
        rhs: Right operand value ID
        dst: Destination value ID
        vmap: Value map
    """
    # Get operands as f64
    f64 = ir.DoubleType()
    lhs_val = vmap.get(lhs, ir.Constant(f64, 0.0))
    rhs_val = vmap.get(rhs, ir.Constant(f64, 0.0))
    
    # Perform ordered comparison using canonical predicates
    pred = op if op in ('<','>','<=','>=','==','!=') else '=='
    cmp_result = builder.fcmp_ordered(pred, lhs_val, rhs_val, name=f"fcmp_{dst}")

    # Convert i1 to i64
    i64 = ir.IntType(64)
    result = builder.zext(cmp_result, i64, name=f"fcmp_i64_{dst}")

    # Store result
    safe_vmap_write(vmap, dst, result, f"fcmp_{pred}")
