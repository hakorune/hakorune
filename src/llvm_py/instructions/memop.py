from typing import Any, Dict, List, Optional

import llvmlite.ir as ir

from instructions.llvm_decl import declare_function
from instructions.fastmem_plan_validation import (
    _ATOMIC_REMOTE_HEAD_ALLOWED_CLASS,
    _FIELD_LOAD_I64_TYPES,
    _FIELD_STORE_I64_TYPES,
    _FREE_HEAD_ALLOWED_CLASS,
    _LOCAL_FREE_BLOCK_NEXT_ALLOWED_CLASS,
    _LOCAL_FREE_HEAD_ALLOWED_CLASS,
    _current_fastmem_access_plan,
    _require_complete_atomic_remote_head_drain_plan,
    _require_complete_atomic_remote_head_push_plan,
    _require_complete_drain_remote_list_to_local_plan,
    _require_complete_field_load_plan,
    _require_complete_field_store_plan,
    _require_complete_free_head_pop_plan,
    _require_complete_free_head_push_plan,
    _require_complete_local_free_pop_plan,
    _require_complete_local_free_push_plan,
    _require_complete_table_index_plan,
    _require_operands,
)
from utils.values import resolve_i64_strict, safe_vmap_write

_SAFE_MEMOP_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)
_CURRENT_ALLOC_OWNER_HELPER = "hako_fastmem_current_alloc_owner_id"


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


def _layout_refs(resolver) -> Dict[int, Dict[str, Any]]:
    refs = getattr(resolver, "fastmem_layout_refs", None)
    if not isinstance(refs, dict):
        refs = {}
        setattr(resolver, "fastmem_layout_refs", refs)
    return refs


def _get_layout_ref(resolver, value_id: int, expected_layout_id: str) -> Dict[str, Any]:
    refs = _layout_refs(resolver)
    ref = refs.get(int(value_id))
    if not isinstance(ref, dict):
        raise RuntimeError(f"[llvm/fastmem:expected-layout-ref] v{int(value_id)}")
    actual_layout_id = ref.get("layout_id")
    if actual_layout_id != expected_layout_id:
        raise RuntimeError(
            "[llvm/fastmem:layout-ref-mismatch] "
            f"v{int(value_id)} expected={expected_layout_id} actual={actual_layout_id}"
        )
    ptr = ref.get("ptr")
    if ptr is None:
        raise RuntimeError(f"[llvm/fastmem:layout-ref-missing-ptr] v{int(value_id)}")
    return ref


def _lower_current_alloc_owner_id(builder: ir.IRBuilder, dst: int) -> ir.Value:
    i64 = ir.IntType(64)
    helper = declare_function(builder.module, _CURRENT_ALLOC_OWNER_HELPER, i64, [])
    return builder.call(helper, [], name=f"fastmem_current_alloc_owner_id_{dst}")


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


def _lower_field_load_from_layout_ref(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
) -> None:
    _require_operands("field_load", operands, 1)
    plan = _require_complete_field_load_plan(
        _current_fastmem_access_plan(resolver, "field_load", dst, operands)
    )
    layout_ref = _get_layout_ref(resolver, int(operands[0]), str(plan["layout_id"]))
    i64 = ir.IntType(64)
    i8_ptr = ir.IntType(8).as_pointer()
    base_ptr = layout_ref["ptr"]
    try:
        base_type = base_ptr.type
    except _SAFE_MEMOP_EXC:
        base_type = None
    if base_type != i8_ptr:
        base_ptr = builder.bitcast(base_ptr, i8_ptr, name=f"fastmem_field_base_{dst}")
    byte_offset = int(plan["byte_offset"])
    field_addr = builder.gep(
        base_ptr,
        [ir.Constant(i64, byte_offset)],
        name=f"fastmem_field_addr_{dst}",
    )
    field_ptr = builder.bitcast(
        field_addr,
        i64.as_pointer(),
        name=f"fastmem_field_ptr_{dst}",
    )
    loaded = builder.load(field_ptr, name=f"fastmem_field_load_{dst}")
    safe_vmap_write(vmap, int(dst), loaded, "fastmem_field_load", resolver=resolver)


def _lower_field_store_from_layout_ref(
    builder: ir.IRBuilder,
    resolver,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("field_store", operands, 2)
    plan = _require_complete_field_store_plan(
        _current_fastmem_access_plan(resolver, "field_store", None, operands)
    )
    layout_ref = _get_layout_ref(resolver, int(operands[0]), str(plan["layout_id"]))
    i64 = ir.IntType(64)
    i8_ptr = ir.IntType(8).as_pointer()
    value = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_field_store_value_{operands[1]}",
    )
    base_ptr = layout_ref["ptr"]
    try:
        base_type = base_ptr.type
    except _SAFE_MEMOP_EXC:
        base_type = None
    if base_type != i8_ptr:
        base_ptr = builder.bitcast(base_ptr, i8_ptr, name="fastmem_field_store_base")
    byte_offset = int(plan["byte_offset"])
    field_addr = builder.gep(
        base_ptr,
        [ir.Constant(i64, byte_offset)],
        name="fastmem_field_store_addr",
    )
    field_ptr = builder.bitcast(
        field_addr,
        i64.as_pointer(),
        name="fastmem_field_store_ptr",
    )
    builder.store(value, field_ptr)


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
            builder.branch(done_bb)
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
    if kind == "field_load":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:field-load-missing-dst]")
        _lower_field_load_from_layout_ref(builder, resolver, int(dst), operands, vmap)
        return
    if kind == "field_store":
        if dst is not None:
            raise RuntimeError("[llvm/fastmem:field-store-has-dst]")
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
        return
    if kind == "local_free_push":
        if dst is not None:
            raise RuntimeError("[llvm/fastmem:local-free-push-has-dst]")
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
        return
    if kind == "local_free_pop":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:local-free-pop-missing-dst]")
        result = _lower_local_free_pop(builder, resolver, int(dst), operands)
        safe_vmap_write(vmap, int(dst), result, "fastmem_local_free_pop", resolver=resolver)
        return
    if kind == "free_head_pop":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:free-head-pop-missing-dst]")
        result = _lower_free_head_pop(builder, resolver, int(dst), operands)
        safe_vmap_write(vmap, int(dst), result, "fastmem_free_head_pop", resolver=resolver)
        return
    if kind == "free_head_push":
        if dst is not None:
            raise RuntimeError("[llvm/fastmem:free-head-push-has-dst]")
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
        return
    if kind == "atomic_remote_head_push":
        if dst is not None:
            raise RuntimeError("[llvm/fastmem:atomic-remote-head-push-has-dst]")
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
        return
    if kind == "atomic_remote_head_drain":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:atomic-remote-head-drain-missing-dst]")
        result = _lower_atomic_remote_head_drain(builder, resolver, int(dst), operands)
        safe_vmap_write(
            vmap,
            int(dst),
            result,
            "fastmem_atomic_remote_head_drain",
            resolver=resolver,
        )
        return
    if kind == "drain_remote_list_to_local":
        if dst is not None:
            raise RuntimeError("[llvm/fastmem:drain-remote-list-to-local-has-dst]")
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
        return
    if kind == "current_alloc_owner_id":
        _require_operands(kind, operands, 0)
        if dst is None:
            raise RuntimeError("[llvm/fastmem:current-alloc-owner-id-missing-dst]")
        result = _lower_current_alloc_owner_id(builder, int(dst))
    elif kind == "addr_of":
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
