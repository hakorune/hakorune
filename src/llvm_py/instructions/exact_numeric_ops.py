from typing import Any, Dict, Optional

import llvmlite.ir as ir

from instructions.primitive_handles import (
    resolver_value_type,
    unbox_primitive_handle_if_needed,
)
from instructions.typeop import _emit_trap
from utils.values import resolve_i64_strict, safe_vmap_write


_EXACT_NUMERIC_TYPES = {
    "i8": ("signed", 8),
    "i16": ("signed", 16),
    "i32": ("signed", 32),
    "i64": ("signed", 64),
    "isize": ("signed", 64),
    "u8": ("unsigned", 8),
    "u16": ("unsigned", 16),
    "u32": ("unsigned", 32),
    "u64": ("unsigned", 64),
    "usize": ("unsigned", 64),
}


def _normalize_declared_type(raw: Any) -> Optional[str]:
    if not isinstance(raw, str):
        return None
    value = raw.strip().lower()
    return value if value in _EXACT_NUMERIC_TYPES else None


def _type_info(raw: Any) -> Optional[Dict[str, Any]]:
    declared = _normalize_declared_type(raw)
    if declared is None:
        return None
    signedness, bits = _EXACT_NUMERIC_TYPES[declared]
    return {
        "declared_type": declared,
        "signedness": signedness,
        "bits": bits,
    }


def _route_by_dst(resolver, attr: str, dst: int) -> Optional[Dict[str, Any]]:
    routes = getattr(resolver, attr, None)
    if not isinstance(routes, dict):
        return None
    row = routes.get(int(dst))
    return row if isinstance(row, dict) else None


def _route_matches(row: Dict[str, Any], op: str, lhs: int, rhs: int) -> bool:
    return (
        row.get("operation") == op
        and row.get("lhs") == int(lhs)
        and row.get("rhs") == int(rhs)
    )


def _exact_route_type_info(
    row: Dict[str, Any],
    *,
    op: str,
    lhs: int,
    rhs: int,
) -> Dict[str, Any]:
    if not _route_matches(row, op, lhs, rhs):
        raise RuntimeError("[exact-numeric/backend-route-mismatch]")
    info = _type_info(row.get("declared_type"))
    if info is None:
        raise RuntimeError("[exact-numeric/backend-unsupported-type]")
    return info


def _declare(module: ir.Module, name: str, ret_ty, arg_tys):
    for fn in module.functions:
        if fn.name == name:
            return fn
    return ir.Function(module, ir.FunctionType(ret_ty, arg_tys), name=name)


def _resolve_i64_operand(
    builder: ir.IRBuilder,
    resolver,
    value_id: int,
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
    *,
    name_hint: str,
):
    i64 = ir.IntType(64)
    value = vmap.get(value_id)
    if value is None:
        value = resolve_i64_strict(
            resolver,
            value_id,
            current_block,
            preds,
            block_end_values,
            vmap,
            bb_map,
            hot_scope="exact_numeric",
        )
    try:
        value_type = value.type
    except Exception:
        value_type = None
    if isinstance(value_type, ir.PointerType):
        value = builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    elif isinstance(value_type, ir.IntType):
        if value_type.width < 64:
            value = builder.zext(value, i64, name=f"{name_hint}_zext")
        elif value_type.width > 64:
            value = builder.trunc(value, i64, name=f"{name_hint}_trunc")
    value = unbox_primitive_handle_if_needed(
        builder,
        value,
        resolver_value_type(resolver, int(value_id)),
        name_hint=name_hint,
    )
    return value if value is not None else ir.Constant(i64, 0)


def _trap_if_i1(builder: ir.IRBuilder, condition: ir.Value, *, name_hint: str) -> None:
    fn = builder.block.function
    suffix = len(list(fn.blocks))
    ok_bb = fn.append_basic_block(name=f"{name_hint}_{suffix}_ok")
    trap_bb = fn.append_basic_block(name=f"{name_hint}_{suffix}_trap")
    builder.cbranch(condition, trap_bb, ok_bb)
    builder.position_at_end(trap_bb)
    _emit_trap(builder)
    builder.position_at_end(ok_bb)


def _range_invalid(builder: ir.IRBuilder, value: ir.Value, info: Dict[str, Any]):
    i64 = ir.IntType(64)
    signedness = info["signedness"]
    bits = int(info["bits"])
    if signedness == "unsigned":
        if bits >= 64:
            return None
        max_value = (1 << bits) - 1
        return builder.icmp_unsigned(
            ">",
            value,
            ir.Constant(i64, max_value),
            name="exact_range_u_hi",
        )

    min_value = -(1 << (bits - 1))
    max_value = (1 << (bits - 1)) - 1
    invalid = None
    if bits < 64:
        below = builder.icmp_signed(
            "<",
            value,
            ir.Constant(i64, min_value),
            name="exact_range_s_lo",
        )
        above = builder.icmp_signed(
            ">",
            value,
            ir.Constant(i64, max_value),
            name="exact_range_s_hi",
        )
        invalid = builder.or_(below, above, name="exact_range_s_invalid")
    return invalid


def _trap_if_range_invalid(builder: ir.IRBuilder, value: ir.Value, info: Dict[str, Any]) -> None:
    invalid = _range_invalid(builder, value, info)
    if invalid is not None:
        _trap_if_i1(builder, invalid, name_hint="exact_range")


def _overflow_intrinsic(builder: ir.IRBuilder, op: str, info: Dict[str, Any]):
    i64 = ir.IntType(64)
    i1 = ir.IntType(1)
    pair_ty = ir.LiteralStructType([i64, i1])
    prefix = "u" if info["signedness"] == "unsigned" else "s"
    intrinsic = {
        "+": "add",
        "-": "sub",
        "*": "mul",
    }.get(op)
    if intrinsic is None:
        return None
    return _declare(
        builder.module,
        f"llvm.{prefix}{intrinsic}.with.overflow.i64",
        pair_ty,
        [i64, i64],
    )


def _op_name(op: str) -> str:
    return {
        "+": "add",
        "-": "sub",
        "*": "mul",
        ">>": "shr",
    }.get(op, "op")


def _normalize_compare_op(op: str) -> str:
    return {
        "Lt": "<",
        "Le": "<=",
        "Gt": ">",
        "Ge": ">=",
        "Eq": "==",
        "Ne": "!=",
    }.get(op, op)


def lower_exact_numeric_binop_route(
    builder: ir.IRBuilder,
    resolver,
    op: str,
    lhs: int,
    rhs: int,
    dst: int,
    vmap: Dict[int, ir.Value],
    current_block,
    preds=None,
    block_end_values=None,
    bb_map=None,
) -> bool:
    route_attr = (
        "exact_numeric_shift_routes_by_dst"
        if op == ">>"
        else "exact_numeric_binary_op_routes_by_dst"
    )
    row = _route_by_dst(resolver, route_attr, dst)
    if row is None:
        return False
    info = _exact_route_type_info(row, op=op, lhs=lhs, rhs=rhs)
    lhs_val = _resolve_i64_operand(
        builder,
        resolver,
        lhs,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"exact_lhs_{lhs}",
    )
    rhs_val = _resolve_i64_operand(
        builder,
        resolver,
        rhs,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"exact_rhs_{rhs}",
    )

    if op == ">>":
        if info["signedness"] != "unsigned":
            raise RuntimeError("[exact-numeric/backend-signed-logical-shift]")
        i64 = ir.IntType(64)
        too_wide = builder.icmp_unsigned(
            ">=",
            rhs_val,
            ir.Constant(i64, int(info["bits"])),
            name=f"exact_shr_count_oob_{dst}",
        )
        _trap_if_i1(builder, too_wide, name_hint="exact_shr_count")
        result = builder.lshr(lhs_val, rhs_val, name=f"exact_lshr_{dst}")
        safe_vmap_write(vmap, dst, result, "exact_numeric_lshr", resolver=resolver)
        return True

    callee = _overflow_intrinsic(builder, op, info)
    if callee is None:
        return False
    opname = _op_name(op)
    pair = builder.call(callee, [lhs_val, rhs_val], name=f"exact_{opname}_pair_{dst}")
    result = builder.extract_value(pair, 0, name=f"exact_{opname}_value_{dst}")
    overflow = builder.extract_value(pair, 1, name=f"exact_{opname}_overflow_{dst}")
    _trap_if_i1(builder, overflow, name_hint="exact_overflow")
    _trap_if_range_invalid(builder, result, info)
    safe_vmap_write(vmap, dst, result, "exact_numeric_binop", resolver=resolver)
    return True


def lower_exact_numeric_compare_route(
    builder: ir.IRBuilder,
    resolver,
    op: str,
    lhs: int,
    rhs: int,
    dst: int,
    vmap: Dict[int, ir.Value],
    current_block,
    preds=None,
    block_end_values=None,
    bb_map=None,
) -> bool:
    op = _normalize_compare_op(op)
    row = _route_by_dst(resolver, "exact_numeric_compare_routes_by_dst", dst)
    if row is None:
        return False
    info = _exact_route_type_info(row, op=op, lhs=lhs, rhs=rhs)
    lhs_val = _resolve_i64_operand(
        builder,
        resolver,
        lhs,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"exact_cmp_lhs_{lhs}",
    )
    rhs_val = _resolve_i64_operand(
        builder,
        resolver,
        rhs,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"exact_cmp_rhs_{rhs}",
    )
    predicate = {
        "==": "==",
        "!=": "!=",
        "<": "<",
        "<=": "<=",
        ">": ">",
        ">=": ">=",
    }.get(op)
    if predicate is None:
        return False
    cmp_fn = (
        builder.icmp_unsigned
        if info["signedness"] == "unsigned"
        else builder.icmp_signed
    )
    i1_result = cmp_fn(predicate, lhs_val, rhs_val, name=f"exact_cmp_{dst}")
    i64_result = builder.zext(i1_result, ir.IntType(64), name=f"exact_cmp_{dst}_i64")
    safe_vmap_write(vmap, dst, i64_result, "exact_numeric_compare", resolver=resolver)
    return True
