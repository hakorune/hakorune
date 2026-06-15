"""Direct-array collection method lowering helpers."""

from typing import Any, Callable, Dict, List, Optional

from llvmlite import ir

from .direct_array_birth import direct_array_i64_exact_lane_enabled
from .runtime_data_dispatch import select_array_collection_call_spec
from utils.resolver_helpers import is_arrayrepr_direct_i64

DIRECT_ARRAY_HANDLE_TAG_MASK = -4
DIRECT_ARRAY_HEADER_LEN_OFFSET_BYTES = 16
DIRECT_ARRAY_HEADER_CAPACITY_OFFSET_BYTES = 24
DIRECT_ARRAY_DATA_OFFSET_BYTES = 32
DIRECT_ARRAY_ELEMENT_BYTES = 8

_SAFE_COLLECTION_METHOD_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)


def _resolve_or_zero(
    resolve_arg: Callable[[int], Optional[ir.Value]], arg_ids: List[int], index: int, zero
):
    if index >= len(arg_ids):
        return zero
    return resolve_arg(arg_ids[index]) or zero


def _direct_array_op_for_method(method_name: str) -> Optional[str]:
    if method_name == "get":
        return "load"
    if method_name == "set":
        return "store"
    return None


def _route_decision_allows_direct_array_plan(
    *,
    resolver,
    block_id: int,
    instruction_index: int,
    expected_route,
) -> bool:
    decisions_by_site = getattr(resolver, "route_decisions_by_site", None)
    if not isinstance(decisions_by_site, dict):
        return True
    decisions = decisions_by_site.get((block_id, instruction_index), [])
    if not decisions:
        return not bool(getattr(resolver, "route_decisions_metadata_present", False))
    for decision in decisions:
        if not isinstance(decision, dict):
            continue
        if decision.get("selected_route") == expected_route:
            return True
    return False


def _current_direct_array_access_plan(
    *,
    resolver,
    method_name: str,
    receiver_vid,
    arg_ids: List[int],
    dst_vid=None,
) -> Optional[Dict[str, Any]]:
    if not direct_array_i64_exact_lane_enabled():
        return None
    if resolver is None or receiver_vid is None:
        return None
    try:
        block_id = int(getattr(resolver, "current_block_id"))
        instruction_index = int(getattr(resolver, "current_instruction_index"))
    except (TypeError, ValueError):
        return None
    plans_by_site = getattr(resolver, "direct_array_access_plans_by_site", None)
    if not isinstance(plans_by_site, dict):
        return None
    plans = plans_by_site.get((block_id, instruction_index), [])
    expected_op = _direct_array_op_for_method(method_name)
    if expected_op is None:
        return None
    try:
        receiver_vid = int(receiver_vid)
    except _SAFE_COLLECTION_METHOD_EXC:
        return None

    for plan in plans:
        if not isinstance(plan, dict):
            continue
        if plan.get("op") != expected_op:
            continue
        if plan.get("array_kind") != "DirectArrayI64":
            continue
        if plan.get("element_type") != "i64":
            continue
        proof_ids = plan.get("proof_ids")
        proof_kind = plan.get("proof_kind")
        if isinstance(proof_ids, list) and proof_kind not in proof_ids:
            continue
        if plan.get("receiver_value") != receiver_vid:
            continue
        if not arg_ids or plan.get("index_value") != int(arg_ids[0]):
            continue
        if method_name == "set":
            if len(arg_ids) < 2 or plan.get("value_value") != int(arg_ids[1]):
                continue
        result_value = plan.get("result_value")
        if result_value is not None and dst_vid is not None and result_value != int(dst_vid):
            continue
        if not is_arrayrepr_direct_i64(resolver, receiver_vid):
            continue
        if not _route_decision_allows_direct_array_plan(
            resolver=resolver,
            block_id=block_id,
            instruction_index=instruction_index,
            expected_route=plan.get("route"),
        ):
            continue
        return plan
    return None


def _direct_array_base(builder: ir.IRBuilder, recv_h):
    i64 = ir.IntType(64)
    return builder.and_(
        recv_h,
        ir.Constant(i64, DIRECT_ARRAY_HANDLE_TAG_MASK),
        name="direct_array_i64_base",
    )


def _direct_array_i64_ptr(builder: ir.IRBuilder, address, name: str):
    return builder.inttoptr(address, ir.IntType(64).as_pointer(), name=name)


def _direct_array_i64_header_ptr(builder: ir.IRBuilder, base, offset: int, name: str):
    i64 = ir.IntType(64)
    address = builder.add(base, ir.Constant(i64, offset), name=f"{name}_addr")
    return _direct_array_i64_ptr(builder, address, name)


def _direct_array_i64_element_ptr(builder: ir.IRBuilder, base, index, name: str):
    i64 = ir.IntType(64)
    byte_index = builder.mul(
        index,
        ir.Constant(i64, DIRECT_ARRAY_ELEMENT_BYTES),
        name=f"{name}_byte_index",
    )
    data_addr = builder.add(
        base,
        ir.Constant(i64, DIRECT_ARRAY_DATA_OFFSET_BYTES),
        name=f"{name}_data_base",
    )
    element_addr = builder.add(data_addr, byte_index, name=f"{name}_addr")
    return _direct_array_i64_ptr(builder, element_addr, name)


def _lower_direct_array_i64_get(builder: ir.IRBuilder, recv_h, index):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    base = _direct_array_base(builder, recv_h)
    len_ptr = _direct_array_i64_header_ptr(
        builder,
        base,
        DIRECT_ARRAY_HEADER_LEN_OFFSET_BYTES,
        "direct_array_i64_len_ptr",
    )
    len_value = builder.load(len_ptr, name="direct_array_i64_len")
    non_negative = builder.icmp_signed(
        ">=",
        index,
        zero,
        name="direct_array_i64_get_non_negative",
    )
    in_len = builder.icmp_unsigned(
        "<",
        index,
        len_value,
        name="direct_array_i64_get_in_len",
    )
    can_load = builder.and_(
        non_negative,
        in_len,
        name="direct_array_i64_get_can_load",
    )

    function = builder.block.function
    load_bb = function.append_basic_block("direct_array_i64_get.load")
    oob_bb = function.append_basic_block("direct_array_i64_get.oob")
    done_bb = function.append_basic_block("direct_array_i64_get.done")
    builder.cbranch(can_load, load_bb, oob_bb)

    builder.position_at_end(load_bb)
    element_ptr = _direct_array_i64_element_ptr(
        builder,
        base,
        index,
        "direct_array_i64_get_ptr",
    )
    loaded = builder.load(element_ptr, name="direct_array_i64_get_value")
    builder.branch(done_bb)
    load_bb = builder.block

    builder.position_at_end(oob_bb)
    builder.branch(done_bb)
    oob_bb = builder.block

    builder.position_at_end(done_bb)
    result = builder.phi(i64, name="direct_array_i64_get_result")
    result.add_incoming(loaded, load_bb)
    result.add_incoming(zero, oob_bb)
    return result


def _lower_direct_array_i64_set(builder: ir.IRBuilder, recv_h, index, value):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    one = ir.Constant(i64, 1)
    base = _direct_array_base(builder, recv_h)
    len_ptr = _direct_array_i64_header_ptr(
        builder,
        base,
        DIRECT_ARRAY_HEADER_LEN_OFFSET_BYTES,
        "direct_array_i64_len_ptr",
    )
    cap_ptr = _direct_array_i64_header_ptr(
        builder,
        base,
        DIRECT_ARRAY_HEADER_CAPACITY_OFFSET_BYTES,
        "direct_array_i64_capacity_ptr",
    )
    len_value = builder.load(len_ptr, name="direct_array_i64_len")
    capacity = builder.load(cap_ptr, name="direct_array_i64_capacity")
    non_negative = builder.icmp_signed(
        ">=",
        index,
        zero,
        name="direct_array_i64_set_non_negative",
    )
    not_past_append = builder.icmp_unsigned(
        "<=",
        index,
        len_value,
        name="direct_array_i64_set_not_past_append",
    )
    within_capacity = builder.icmp_unsigned(
        "<",
        index,
        capacity,
        name="direct_array_i64_set_within_capacity",
    )
    can_store = builder.and_(
        builder.and_(non_negative, not_past_append, name="direct_array_i64_set_index_ok"),
        within_capacity,
        name="direct_array_i64_set_can_store",
    )

    function = builder.block.function
    store_bb = function.append_basic_block("direct_array_i64_set.store")
    fail_bb = function.append_basic_block("direct_array_i64_set.fail")
    done_bb = function.append_basic_block("direct_array_i64_set.done")
    builder.cbranch(can_store, store_bb, fail_bb)

    builder.position_at_end(store_bb)
    element_ptr = _direct_array_i64_element_ptr(
        builder,
        base,
        index,
        "direct_array_i64_set_ptr",
    )
    builder.store(value, element_ptr)
    is_append = builder.icmp_unsigned(
        "==",
        index,
        len_value,
        name="direct_array_i64_set_is_append",
    )
    incremented_len = builder.add(len_value, one, name="direct_array_i64_set_len_plus_one")
    next_len = builder.select(is_append, incremented_len, len_value, name="direct_array_i64_next_len")
    builder.store(next_len, len_ptr)
    builder.branch(done_bb)
    store_bb = builder.block

    builder.position_at_end(fail_bb)
    builder.branch(done_bb)
    fail_bb = builder.block

    builder.position_at_end(done_bb)
    result = builder.phi(i64, name="direct_array_i64_set_result")
    result.add_incoming(one, store_bb)
    result.add_incoming(zero, fail_bb)
    return result


def _lower_direct_array_i64_set_proved_unchecked(builder: ir.IRBuilder, recv_h, index, value):
    i64 = ir.IntType(64)
    one = ir.Constant(i64, 1)
    base = _direct_array_base(builder, recv_h)
    len_ptr = _direct_array_i64_header_ptr(
        builder,
        base,
        DIRECT_ARRAY_HEADER_LEN_OFFSET_BYTES,
        "direct_array_i64_len_ptr",
    )
    len_value = builder.load(len_ptr, name="direct_array_i64_len")
    element_ptr = _direct_array_i64_element_ptr(
        builder,
        base,
        index,
        "direct_array_i64_set_unchecked_ptr",
    )
    builder.store(value, element_ptr)
    is_append = builder.icmp_unsigned(
        "==",
        index,
        len_value,
        name="direct_array_i64_set_unchecked_is_append",
    )
    incremented_len = builder.add(
        len_value,
        one,
        name="direct_array_i64_set_unchecked_len_plus_one",
    )
    next_len = builder.select(
        is_append,
        incremented_len,
        len_value,
        name="direct_array_i64_set_unchecked_next_len",
    )
    builder.store(next_len, len_ptr)
    return one


def _lower_direct_array_i64_set_unchecked_overwrite(builder: ir.IRBuilder, recv_h, index, value):
    i64 = ir.IntType(64)
    one = ir.Constant(i64, 1)
    base = _direct_array_base(builder, recv_h)
    element_ptr = _direct_array_i64_element_ptr(
        builder,
        base,
        index,
        "direct_array_i64_set_unchecked_overwrite_ptr",
    )
    builder.store(value, element_ptr)
    return one


def _lower_direct_array_nativedirect_call(
    *,
    builder: ir.IRBuilder,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
    receiver_vid=None,
    dst_vid=None,
):
    plan = _current_direct_array_access_plan(
        resolver=resolver,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
        dst_vid=dst_vid,
    )
    if plan is None:
        return None
    direct_op = _direct_array_op_for_method(method_name)
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    if direct_op == "load":
        if not arg_ids:
            return zero
        index = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        return _lower_direct_array_i64_get(builder, recv_h, index)
    if direct_op == "store":
        if len(arg_ids) < 2:
            return recv_h
        index = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        value = _resolve_or_zero(resolve_arg, arg_ids, 1, zero)
        if (
            plan.get("bounds_policy") == "proved_unchecked"
            and plan.get("cfg_shape") == "branchless"
            and plan.get("fallback_policy") == "fail_fast"
        ):
            if plan.get("store_semantics") == "overwrite_existing":
                return _lower_direct_array_i64_set_unchecked_overwrite(
                    builder, recv_h, index, value
                )
            return _lower_direct_array_i64_set_proved_unchecked(builder, recv_h, index, value)
        return _lower_direct_array_i64_set(builder, recv_h, index, value)
    return None
