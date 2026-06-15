from typing import Any, Dict, List

import llvmlite.ir as ir

from instructions.fastmem_plan_validation import (
    _current_fastmem_access_plan,
    _require_complete_atomic_remote_head_drain_plan,
    _require_complete_atomic_remote_head_push_plan,
    _require_complete_drain_remote_list_to_local_plan,
    _require_complete_free_head_pop_plan,
    _require_complete_free_head_push_plan,
    _require_complete_local_free_pop_plan,
    _require_complete_local_free_push_plan,
    _require_operands,
)

from .memop_layout_ref import _SAFE_MEMOP_EXC, _get_layout_ref, _resolve_i64_operand


def _gep_i64_field_ptr(
    builder: ir.IRBuilder,
    base_ptr,
    byte_offset: int,
    *,
    name_prefix: str,
):
    i64 = ir.IntType(64)
    i8_ptr = ir.IntType(8).as_pointer()
    try:
        base_type = base_ptr.type
    except _SAFE_MEMOP_EXC:
        base_type = None
    if base_type != i8_ptr:
        base_ptr = builder.bitcast(base_ptr, i8_ptr, name=f"{name_prefix}_base")
    field_addr = builder.gep(
        base_ptr,
        [ir.Constant(i64, byte_offset)],
        name=f"{name_prefix}_addr",
    )
    return builder.bitcast(field_addr, i64.as_pointer(), name=f"{name_prefix}_ptr")


def _lower_local_free_push(
    builder: ir.IRBuilder,
    resolver,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("local_free_push", operands, 2)
    plan = _require_complete_local_free_push_plan(
        _current_fastmem_access_plan(resolver, "local_free_push", None, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["local_free_head_layout_id"]),
    )
    i64 = ir.IntType(64)
    block_addr = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_local_free_block_{operands[1]}",
    )
    block_ptr = builder.inttoptr(
        block_addr,
        ir.IntType(8).as_pointer(),
        name=f"fastmem_local_free_block_ptr_{operands[1]}",
    )
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["local_free_head_byte_offset"]),
        name_prefix="fastmem_local_free_head",
    )
    old_head = builder.load(head_ptr, name="fastmem_local_free_old_head")
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_local_free_block_next",
    )
    builder.store(old_head, block_next_ptr)
    builder.store(block_addr, head_ptr)


def _lower_local_free_pop(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
) -> ir.Value:
    _require_operands("local_free_pop", operands, 1)
    plan = _require_complete_local_free_pop_plan(
        _current_fastmem_access_plan(resolver, "local_free_pop", dst, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["local_free_head_layout_id"]),
    )
    i8_ptr = ir.IntType(8).as_pointer()
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["local_free_head_byte_offset"]),
        name_prefix="fastmem_local_free_head",
    )
    old_head = builder.load(head_ptr, name="fastmem_local_free_pop_old_head")
    block_ptr = builder.inttoptr(
        old_head,
        i8_ptr,
        name=f"fastmem_local_free_pop_block_ptr_{dst}",
    )
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_local_free_pop_block_next",
    )
    next_head = builder.load(block_next_ptr, name="fastmem_local_free_pop_next_head")
    builder.store(next_head, head_ptr)
    return old_head


def _lower_free_head_pop(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
) -> ir.Value:
    _require_operands("free_head_pop", operands, 1)
    plan = _require_complete_free_head_pop_plan(
        _current_fastmem_access_plan(resolver, "free_head_pop", dst, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["free_head_layout_id"]),
    )
    i8_ptr = ir.IntType(8).as_pointer()
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["free_head_byte_offset"]),
        name_prefix="fastmem_free_head",
    )
    old_head = builder.load(head_ptr, name="fastmem_free_head_pop_old_head")
    block_ptr = builder.inttoptr(
        old_head,
        i8_ptr,
        name=f"fastmem_free_head_pop_block_ptr_{dst}",
    )
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_free_head_pop_block_next",
    )
    next_head = builder.load(block_next_ptr, name="fastmem_free_head_pop_next_head")
    builder.store(next_head, head_ptr)
    return old_head


def _lower_free_head_push(
    builder: ir.IRBuilder,
    resolver,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("free_head_push", operands, 2)
    plan = _require_complete_free_head_push_plan(
        _current_fastmem_access_plan(resolver, "free_head_push", None, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["free_head_layout_id"]),
    )
    block_addr = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_free_head_push_block_{operands[1]}",
    )
    block_ptr = builder.inttoptr(
        block_addr,
        ir.IntType(8).as_pointer(),
        name=f"fastmem_free_head_push_block_ptr_{operands[1]}",
    )
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["free_head_byte_offset"]),
        name_prefix="fastmem_free_head_push_head",
    )
    old_head = builder.load(head_ptr, name="fastmem_free_head_push_old_head")
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_free_head_push_block_next",
    )
    builder.store(old_head, block_next_ptr)
    builder.store(block_addr, head_ptr)


def _lower_atomic_remote_head_push(
    builder: ir.IRBuilder,
    resolver,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("atomic_remote_head_push", operands, 2)
    plan = _require_complete_atomic_remote_head_push_plan(
        _current_fastmem_access_plan(resolver, "atomic_remote_head_push", None, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["remote_head_layout_id"]),
    )
    block_addr = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_atomic_remote_block_{operands[1]}",
    )
    block_ptr = builder.inttoptr(
        block_addr,
        ir.IntType(8).as_pointer(),
        name=f"fastmem_atomic_remote_block_ptr_{operands[1]}",
    )
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["remote_head_byte_offset"]),
        name_prefix="fastmem_atomic_remote_head",
    )
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_atomic_remote_block_next",
    )
    retry_limit = int(plan["retry_attempt_limit"])
    done_bb = builder.function.append_basic_block("fastmem_atomic_remote_retry_done")
    exhausted_bb = builder.function.append_basic_block(
        "fastmem_atomic_remote_retry_exhausted"
    )
    for attempt in range(retry_limit):
        old_head = builder.load_atomic(
            head_ptr,
            "acquire",
            int(plan["remote_head_alignment"]),
            name=f"fastmem_atomic_remote_old_head_{attempt}",
        )
        builder.store(old_head, block_next_ptr)
        cas_result = builder.cmpxchg(
            head_ptr,
            old_head,
            block_addr,
            "acq_rel",
            "acquire",
            name=f"fastmem_atomic_remote_head_cas_{attempt}",
        )
        if attempt + 1 == retry_limit:
            success = builder.extract_value(
                cas_result,
                1,
                name=f"fastmem_atomic_remote_cas_success_{attempt}",
            )
            builder.cbranch(success, done_bb, exhausted_bb)
            builder.position_at_end(exhausted_bb)
            builder.unreachable()
            break
        retry_bb = builder.function.append_basic_block(
            f"fastmem_atomic_remote_retry_{attempt + 1}"
        )
        success = builder.extract_value(
            cas_result,
            1,
            name=f"fastmem_atomic_remote_cas_success_{attempt}",
        )
        builder.cbranch(success, done_bb, retry_bb)
        builder.position_at_end(retry_bb)
    builder.position_at_end(done_bb)


def _lower_atomic_remote_head_drain(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
) -> ir.Value:
    _require_operands("atomic_remote_head_drain", operands, 1)
    plan = _require_complete_atomic_remote_head_drain_plan(
        _current_fastmem_access_plan(resolver, "atomic_remote_head_drain", dst, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["remote_head_layout_id"]),
    )
    i64 = ir.IntType(64)
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["remote_head_byte_offset"]),
        name_prefix="fastmem_atomic_remote_drain_head",
    )
    return builder.atomic_rmw(
        "xchg",
        head_ptr,
        ir.Constant(i64, 0),
        "acquire",
        name=f"fastmem_atomic_remote_drain_xchg_{dst}",
    )


def _lower_drain_remote_list_to_local(
    builder: ir.IRBuilder,
    resolver,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("drain_remote_list_to_local", operands, 2)
    plan = _require_complete_drain_remote_list_to_local_plan(
        _current_fastmem_access_plan(
            resolver, "drain_remote_list_to_local", None, operands
        )
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["local_free_head_layout_id"]),
    )
    i64 = ir.IntType(64)
    token_head = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_drain_remote_token_{operands[1]}",
    )
    local_head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["local_free_head_byte_offset"]),
        name_prefix="fastmem_drain_remote_local_head",
    )
    old_local_head = builder.load(
        local_head_ptr,
        name="fastmem_drain_remote_old_local_head",
    )

    entry_bb = builder.block
    done_bb = builder.function.append_basic_block("fastmem_drain_remote_done")
    scan_bb = builder.function.append_basic_block("fastmem_drain_remote_scan")
    tail_found_bb = builder.function.append_basic_block("fastmem_drain_remote_tail_found")
    token_is_null = builder.icmp_unsigned(
        "==",
        token_head,
        ir.Constant(i64, 0),
        name="fastmem_drain_remote_token_is_null",
    )
    builder.cbranch(token_is_null, done_bb, scan_bb)

    builder.position_at_end(scan_bb)
    tail_addr = builder.phi(i64, name="fastmem_drain_remote_tail_addr")
    tail_addr.add_incoming(token_head, entry_bb)
    tail_ptr = builder.inttoptr(
        tail_addr,
        ir.IntType(8).as_pointer(),
        name="fastmem_drain_remote_tail_ptr",
    )
    tail_next_ptr = _gep_i64_field_ptr(
        builder,
        tail_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_drain_remote_tail_next",
    )
    next_addr = builder.load(tail_next_ptr, name="fastmem_drain_remote_next_addr")
    next_is_null = builder.icmp_unsigned(
        "==",
        next_addr,
        ir.Constant(i64, 0),
        name="fastmem_drain_remote_next_is_null",
    )
    tail_addr.add_incoming(next_addr, scan_bb)
    builder.cbranch(next_is_null, tail_found_bb, scan_bb)

    builder.position_at_end(tail_found_bb)
    builder.store(old_local_head, tail_next_ptr)
    builder.store(token_head, local_head_ptr)
    builder.branch(done_bb)

    builder.position_at_end(done_bb)
