from typing import Any, Dict, List, Optional

import llvmlite.ir as ir

from utils.values import resolve_i64_strict, safe_vmap_write

_SAFE_MEMOP_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)
_TABLE_INDEX_REQUIRED_PROOF_FLAGS = (
    "table_length_resolved",
    "bounds_proof_valid",
    "stride_resolved",
    "field_offset_resolved",
    "overflow_proof_valid",
    "alignment_valid",
    "element_layout_verified",
)


def _is_fastmem_layout_ref(resolver, value_id: int) -> bool:
    refs = getattr(resolver, "fastmem_layout_refs", None)
    return isinstance(refs, dict) and int(value_id) in refs


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
    if _is_fastmem_layout_ref(resolver, int(value_id)):
        raise RuntimeError(
            f"[llvm/fastmem:layout-ref-as-ordinary-value] v{int(value_id)}"
        )
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


def _current_fastmem_access_plan(resolver, kind: str, dst, operands: List[Any]) -> Optional[Dict[str, Any]]:
    if resolver is None:
        return None
    try:
        block_id = int(getattr(resolver, "current_block_id"))
        instruction_index = int(getattr(resolver, "current_instruction_index"))
    except (TypeError, ValueError):
        return None
    plans_by_site = getattr(resolver, "fastmem_access_plans_by_site", None)
    if not isinstance(plans_by_site, dict):
        return None
    plans = plans_by_site.get((block_id, instruction_index), [])
    for plan in plans:
        if not isinstance(plan, dict):
            continue
        if plan.get("kind") != kind:
            continue
        if plan.get("verified") is not True or plan.get("status") != "verified":
            continue
        result = plan.get("result")
        if dst is not None and result is not None and int(result) != int(dst):
            continue
        if kind == "table_index":
            if len(operands) < 2:
                continue
            if plan.get("table") != int(operands[0]) or plan.get("index") != int(operands[1]):
                continue
        return plan
    return None


def _require_complete_table_index_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-table-plan]")
    missing = [flag for flag in _TABLE_INDEX_REQUIRED_PROOF_FLAGS if plan.get(flag) is not True]
    if missing:
        raise RuntimeError(
            f"[llvm/fastmem:incomplete-table-plan] missing={','.join(missing)}"
        )
    if plan.get("element_repr") != "pointer_to_element":
        raise RuntimeError(
            f"[llvm/fastmem:unsupported-table-element-repr] {plan.get('element_repr')}"
        )
    for key in ("element_stride", "element_layout_id", "table_id", "region"):
        if plan.get(key) is None:
            raise RuntimeError(f"[llvm/fastmem:missing-table-plan-field] {key}")
    return plan


def _layout_refs(resolver) -> Dict[int, Dict[str, Any]]:
    refs = getattr(resolver, "fastmem_layout_refs", None)
    if not isinstance(refs, dict):
        refs = {}
        setattr(resolver, "fastmem_layout_refs", refs)
    return refs


def _lower_table_index_to_layout_ref(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("table_index", operands, 2)
    plan = _require_complete_table_index_plan(
        _current_fastmem_access_plan(resolver, "table_index", dst, operands)
    )
    i64 = ir.IntType(64)
    table_addr = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[0]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_table_base_{dst}",
    )
    index = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_table_index_{dst}",
    )
    stride = int(plan["element_stride"])
    byte_index = builder.mul(index, ir.Constant(i64, stride), name=f"fastmem_table_index_bytes_{dst}")
    slot_addr = builder.add(table_addr, byte_index, name=f"fastmem_table_slot_addr_{dst}")
    slot_ptr = builder.inttoptr(slot_addr, i64.as_pointer(), name=f"fastmem_table_slot_ptr_{dst}")
    element_addr = builder.load(slot_ptr, name=f"fastmem_layout_ref_addr_{dst}")
    element_ptr = builder.inttoptr(element_addr, ir.IntType(8).as_pointer(), name=f"fastmem_layout_ref_ptr_{dst}")
    _layout_refs(resolver)[int(dst)] = {
        "ptr": element_ptr,
        "layout_id": plan.get("element_layout_id"),
        "table_id": plan.get("table_id"),
        "region": plan.get("region"),
        "source_site": (plan.get("block"), plan.get("instruction_index")),
    }


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
    if kind == "table_index":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:table-index-missing-dst]")
        _lower_table_index_to_layout_ref(
            builder,
            resolver,
            int(dst),
            operands,
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
        )
        return
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
