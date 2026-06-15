from typing import Any, Dict, List, Optional, Tuple

import llvmlite.ir as ir

from .fastmem_plan_validation import _require_operands
from .memop_layout_ref import (
    _lower_current_alloc_owner_id,
    _lower_field_load_from_layout_ref,
    _lower_field_store_from_layout_ref,
    _lower_table_index_to_layout_ref,
    _resolve_i64_operand,
)
from .memop_queue_ops import (
    _lower_atomic_remote_head_drain,
    _lower_atomic_remote_head_push,
    _lower_drain_remote_list_to_local,
    _lower_free_head_pop,
    _lower_free_head_push,
    _lower_local_free_pop,
    _lower_local_free_push,
)
from utils.values import safe_vmap_write


def _require_dst(kind: str, dst: Optional[int]) -> int:
    if dst is None:
        raise RuntimeError(f"[llvm/fastmem:{kind.replace('_', '-')}-missing-dst]")
    return int(dst)


def _reject_dst(kind: str, dst: Optional[int]) -> None:
    if dst is not None:
        raise RuntimeError(f"[llvm/fastmem:{kind.replace('_', '-')}-has-dst]")


def _lower_table_index_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    dst_id = _require_dst("table_index", dst)
    _lower_table_index_to_layout_ref(
        builder,
        resolver,
        dst_id,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )
    return None


def _lower_field_load_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    del current_block, preds, block_end_values, bb_map
    _lower_field_load_from_layout_ref(
        builder, resolver, _require_dst("field_load", dst), operands, vmap
    )
    return None


def _lower_field_store_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    _reject_dst("field_store", dst)
    _lower_field_store_from_layout_ref(
        builder,
        resolver,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )
    return None


def _lower_local_free_push_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    _reject_dst("local_free_push", dst)
    _lower_local_free_push(
        builder,
        resolver,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )
    return None


def _lower_local_free_pop_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    del vmap, current_block, preds, block_end_values, bb_map
    dst_id = _require_dst("local_free_pop", dst)
    return (
        _lower_local_free_pop(builder, resolver, dst_id, operands),
        "fastmem_local_free_pop",
    )


def _lower_free_head_pop_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    del vmap, current_block, preds, block_end_values, bb_map
    dst_id = _require_dst("free_head_pop", dst)
    return (
        _lower_free_head_pop(builder, resolver, dst_id, operands),
        "fastmem_free_head_pop",
    )


def _lower_free_head_push_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    _reject_dst("free_head_push", dst)
    _lower_free_head_push(
        builder,
        resolver,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )
    return None


def _lower_atomic_remote_head_push_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    _reject_dst("atomic_remote_head_push", dst)
    _lower_atomic_remote_head_push(
        builder,
        resolver,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )
    return None


def _lower_atomic_remote_head_drain_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    del vmap, current_block, preds, block_end_values, bb_map
    dst_id = _require_dst("atomic_remote_head_drain", dst)
    return (
        _lower_atomic_remote_head_drain(builder, resolver, dst_id, operands),
        "fastmem_atomic_remote_head_drain",
    )


def _lower_drain_remote_list_to_local_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    _reject_dst("drain_remote_list_to_local", dst)
    _lower_drain_remote_list_to_local(
        builder,
        resolver,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )
    return None


def _lower_current_alloc_owner_id_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    del resolver, vmap, current_block, preds, block_end_values, bb_map
    _require_operands("current_alloc_owner_id", operands, 0)
    dst_id = _require_dst("current_alloc_owner_id", dst)
    return (_lower_current_alloc_owner_id(builder, dst_id), "fastmem_current_alloc_owner_id")


def _lower_addr_of_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
):
    _require_operands("addr_of", operands, 1)
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
    return (result, "fastmem_addr_of")


def _lower_binary_i64_memop(
    builder,
    resolver,
    dst,
    operands,
    vmap,
    current_block,
    preds,
    block_end_values,
    bb_map,
    *,
    kind: str,
):
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
    return (result, f"fastmem_{kind}")


def _make_binary_i64_memop_lowerer(kind: str):
    def lowerer(
        builder,
        resolver,
        dst,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    ):
        return _lower_binary_i64_memop(
            builder,
            resolver,
            dst,
            operands,
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
            kind=kind,
        )

    return lowerer


_MEMOP_LOWERERS = {
    "table_index": _lower_table_index_memop,
    "field_load": _lower_field_load_memop,
    "field_store": _lower_field_store_memop,
    "local_free_push": _lower_local_free_push_memop,
    "local_free_pop": _lower_local_free_pop_memop,
    "free_head_pop": _lower_free_head_pop_memop,
    "free_head_push": _lower_free_head_push_memop,
    "atomic_remote_head_push": _lower_atomic_remote_head_push_memop,
    "atomic_remote_head_drain": _lower_atomic_remote_head_drain_memop,
    "drain_remote_list_to_local": _lower_drain_remote_list_to_local_memop,
    "current_alloc_owner_id": _lower_current_alloc_owner_id_memop,
    "addr_of": _lower_addr_of_memop,
    "logical_shr": _make_binary_i64_memop_lowerer("logical_shr"),
    "bit_and": _make_binary_i64_memop_lowerer("bit_and"),
    "add": _make_binary_i64_memop_lowerer("add"),
    "sub": _make_binary_i64_memop_lowerer("sub"),
    "owner_eq": _make_binary_i64_memop_lowerer("owner_eq"),
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

    lowerer = _MEMOP_LOWERERS.get(kind)
    if lowerer is None:
        raise RuntimeError(f"[llvm/fastmem:unsupported-kind] {kind}")

    lowered: Optional[Tuple[ir.Value, str]] = lowerer(
        builder,
        resolver,
        dst,
        operands,
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
    )
    if lowered is None:
        return
    if dst is None:
        return
    result, context = lowered
    safe_vmap_write(vmap, int(dst), result, context, resolver=resolver)
