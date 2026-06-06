from typing import Any, Dict, List, Optional

import llvmlite.ir as ir

from utils.values import resolve_i64_strict, safe_vmap_write

_SAFE_MEMOP_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)


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
            hot_scope="fastmem_memop",
        )
    try:
        value_type = value.type
    except _SAFE_MEMOP_EXC:
        value_type = None
    if isinstance(value_type, ir.PointerType):
        value = builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    elif isinstance(value_type, ir.IntType):
        if value_type.width < 64:
            value = builder.zext(value, i64, name=f"{name_hint}_zext")
        elif value_type.width > 64:
            value = builder.trunc(value, i64, name=f"{name_hint}_trunc")
    return value if value is not None else ir.Constant(i64, 0)


def _require_operands(kind: str, operands: List[Any], expected: int) -> None:
    if len(operands) != expected:
        raise RuntimeError(
            f"[llvm/fastmem:arity] kind={kind} expected={expected} actual={len(operands)}"
        )


def lower_memop(
    builder: ir.IRBuilder,
    inst: Dict[str, Any],
    vmap: Dict[int, ir.Value],
    resolver,
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    kind = str(inst.get("kind") or "")
    dst: Optional[int] = inst.get("dst")
    operands = list(inst.get("operands") or [])
    i64 = ir.IntType(64)

    result = None
    if kind == "addr_of":
        _require_operands(kind, operands, 1)
        result = _resolve_i64_operand(
            builder,
            resolver,
            int(operands[0]),
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
            name_hint=f"fastmem_addr_{dst}",
        )
    elif kind in ("logical_shr", "bit_and", "add", "sub", "owner_eq"):
        _require_operands(kind, operands, 2)
        lhs = _resolve_i64_operand(
            builder,
            resolver,
            int(operands[0]),
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
            name_hint=f"fastmem_{kind}_lhs_{dst}",
        )
        rhs = _resolve_i64_operand(
            builder,
            resolver,
            int(operands[1]),
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
            name_hint=f"fastmem_{kind}_rhs_{dst}",
        )
        if kind == "logical_shr":
            result = builder.lshr(lhs, rhs, name=f"fastmem_lshr_{dst}")
        elif kind == "bit_and":
            result = builder.and_(lhs, rhs, name=f"fastmem_and_{dst}")
        elif kind == "add":
            result = builder.add(lhs, rhs, name=f"fastmem_add_{dst}")
        elif kind == "sub":
            result = builder.sub(lhs, rhs, name=f"fastmem_sub_{dst}")
        else:
            result = builder.icmp_unsigned("==", lhs, rhs, name=f"fastmem_owner_eq_{dst}")
    else:
        raise RuntimeError(f"[llvm/fastmem:unsupported-kind] {kind}")

    if dst is None:
        return
    if result is None:
        result = ir.Constant(i64, 0)
    safe_vmap_write(vmap, int(dst), result, f"fastmem_{kind}", resolver=resolver)
