"""
Shared collection-style method lowering for MIR call lowerers.

This module owns the common `get/push/set/has/clear` route order shared by
`method_call.py` and `mir_call_legacy.py`.
"""

from typing import Any, Callable, Dict, List, Optional

from llvmlite import ir

from .direct_array_birth import direct_array_i64_exact_lane_enabled
from .runtime_data_dispatch import (
    lower_runtime_data_method_call,
    select_array_collection_call_spec,
)
from utils.resolver_helpers import is_arrayrepr_direct_i64

DIRECT_ARRAY_HANDLE_TAG_MASK = -4
DIRECT_ARRAY_HEADER_LEN_OFFSET_BYTES = 16
DIRECT_ARRAY_HEADER_CAPACITY_OFFSET_BYTES = 24
DIRECT_ARRAY_DATA_OFFSET_BYTES = 32
DIRECT_ARRAY_ELEMENT_BYTES = 8

BOX_MAP = "MapBox"
BOX_RUNTIME_DATA = "RuntimeDataBox"
MAP_METHOD_CLEAR = "clear"
MAP_METHOD_DELETE = "delete"
MAP_METHOD_GET = "get"
MAP_METHOD_HAS = "has"
MAP_METHOD_PUSH = "push"
MAP_METHOD_SET = "set"
MAP_LOOKUP_CONST_FOLD_ROUTE = "map_lookup_const_fold"
MAP_LOOKUP_FUSION_SOURCE_PLAN = "MapLookupFusionRoute"
MAP_MISSING_EMPTY_CONST_ZERO_ROUTE = "map_get_missing_empty_const_zero"
MAP_MISSING_EMPTY_SOURCE_PLAN = "MapMissingEmptyRoute"
MAP_LOOKUP_GET_SEMANTIC_OP = "MapGet"
MAP_LOOKUP_HAS_SEMANTIC_OP = "MapHas"
LOCAL_I64_MAP_DIRECT_STORAGE_REPRESENTATION = "closed_world_i64_key_value_table"

_SAFE_COLLECTION_METHOD_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)


def _resolve_or_zero(
    resolve_arg: Callable[[int], Optional[ir.Value]], arg_ids: List[int], index: int, zero
):
    if index >= len(arg_ids):
        return zero
    return resolve_arg(arg_ids[index]) or zero


from .collection_method_call_direct_array import (
    _current_direct_array_access_plan,
    _direct_array_base,
    _direct_array_i64_element_ptr,
    _direct_array_i64_header_ptr,
    _direct_array_i64_ptr,
    _direct_array_op_for_method,
    _lower_direct_array_i64_get,
    _lower_direct_array_i64_set,
    _lower_direct_array_i64_set_proved_unchecked,
    _lower_direct_array_i64_set_unchecked_overwrite,
    _lower_direct_array_nativedirect_call,
    _resolve_or_zero,
    _route_decision_allows_direct_array_plan,
)
from .collection_method_call_local_fastpath import (
    LOCAL_FASTPATH_ROUTE_MAP_SCALAR_NO_PUBLICATION_GET,
    current_local_fastpath_known_receiver_direct_call_fact,
)

def _lower_call_spec(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    spec,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    symbol, call_name, arity = spec
    if arity == 1:
        if not arg_ids:
            return zero
        arg0 = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        callee = declare(symbol, i64, [i64, i64])
        return builder.call(callee, [recv_h, arg0], name=call_name)
    if arity == 2:
        if len(arg_ids) < 2:
            return recv_h
        arg0 = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        arg1 = _resolve_or_zero(resolve_arg, arg_ids, 1, zero)
        callee = declare(symbol, i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, arg0, arg1], name=call_name)
    return None


def _lower_store_map_value_current_lowering(
    *, builder: ir.IRBuilder, declare: Callable, recv_h, key, value
):
    """
    phase-151x visibility lock:
    current concrete lowering for canonical `store.map.value`.
    Keep semantic ownership above this helper.
    """
    i64 = ir.IntType(64)
    callee = declare("nyash.map.slot_store_hhh", i64, [i64, i64, i64])
    return builder.call(callee, [recv_h, key, value], name="unified_map_slot_store_hhh")


def _current_map_lookup_fusion_decision(
    *,
    resolver,
    box_name,
    method_name: str,
    receiver_vid,
    arg_ids: List[int],
):
    if str(box_name or "") not in (BOX_MAP, BOX_RUNTIME_DATA):
        return None
    if resolver is None or receiver_vid is None or not arg_ids:
        return None
    try:
        block_id = int(getattr(resolver, "current_block_id"))
        instruction_index = int(getattr(resolver, "current_instruction_index"))
        receiver_vid = int(receiver_vid)
        key_vid = int(arg_ids[0])
    except _SAFE_COLLECTION_METHOD_EXC:
        return None
    expected_semantic_op = _map_lookup_semantic_op(method_name)
    if expected_semantic_op is None:
        return None

    decision = _selected_map_lookup_decision(
        resolver=resolver,
        block_id=block_id,
        instruction_index=instruction_index,
        expected_routes=_map_lookup_expected_routes(expected_semantic_op),
        expected_semantic_op=expected_semantic_op,
    )
    if not isinstance(decision, dict):
        return None
    if _as_optional_int(decision.get("receiver_value")) not in (None, receiver_vid):
        return None
    if _as_optional_int(decision.get("key_value")) not in (None, key_vid):
        return None
    return decision


def _current_local_i64_map_storage_realization_plan(
    *,
    resolver,
    receiver_vid,
):
    if resolver is None or receiver_vid is None:
        return None
    try:
        receiver_vid = int(receiver_vid)
    except _SAFE_COLLECTION_METHOD_EXC:
        return None

    plans_by_receiver = getattr(
        resolver, "local_map_storage_realization_plans_by_receiver", None
    )
    if not isinstance(plans_by_receiver, dict):
        return None

    for plan in plans_by_receiver.get(receiver_vid, []):
        if not isinstance(plan, dict):
            continue
        if plan.get("representation") != "local_i64_key_map":
            continue
        if plan.get("publication_materialization_required") is not True:
            continue
        if plan.get("backend_lowering_enabled") is not False:
            continue
        if plan.get("runtime_helper_enabled") is not False:
            continue
        return plan
    return None


def _current_local_i64_map_direct_storage_shadow_candidate(
    *,
    resolver,
    box_name,
    method_name: str,
    receiver_vid,
    arg_ids: List[int],
):
    fact = current_local_fastpath_known_receiver_direct_call_fact(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
        required_route_plan=LOCAL_FASTPATH_ROUTE_MAP_SCALAR_NO_PUBLICATION_GET,
    )
    if fact is None or resolver is None or receiver_vid is None:
        return None
    try:
        receiver_vid = int(receiver_vid)
    except _SAFE_COLLECTION_METHOD_EXC:
        return None
    plans_by_receiver = getattr(
        resolver, "local_i64_map_direct_storage_plans_by_receiver", None
    )
    if not isinstance(plans_by_receiver, dict):
        return None

    for plan in plans_by_receiver.get(receiver_vid, []):
        if not isinstance(plan, dict):
            continue
        if plan.get("representation") != LOCAL_I64_MAP_DIRECT_STORAGE_REPRESENTATION:
            continue
        if plan.get("entry_value_tracking_enabled") is not False:
            continue
        if plan.get("publication_materialization_required") is not True:
            continue
        if plan.get("backend_lowering_enabled") is not False:
            continue
        if plan.get("runtime_helper_enabled") is not False:
            continue
        return {"fact": fact, "plan": plan}
    return None


def _current_local_i64_map_entry_value_tracking_shadow_candidate(
    *,
    resolver,
    box_name,
    method_name: str,
    receiver_vid,
    arg_ids: List[int],
):
    direct_candidate = _current_local_i64_map_direct_storage_shadow_candidate(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
    )
    if direct_candidate is None or resolver is None or receiver_vid is None:
        return None
    try:
        receiver_vid = int(receiver_vid)
    except _SAFE_COLLECTION_METHOD_EXC:
        return None

    tracking_by_receiver = getattr(
        resolver, "local_i64_map_entry_value_tracking_plans_by_receiver", None
    )
    if not isinstance(tracking_by_receiver, dict):
        return None

    tracking_rows = []
    for row in tracking_by_receiver.get(receiver_vid, []):
        if not isinstance(row, dict):
            continue
        if row.get("backend_lowering_enabled") is not False:
            continue
        if row.get("runtime_helper_enabled") is not False:
            continue
        if _as_optional_int(row.get("key_value")) is None:
            continue
        if _as_optional_int(row.get("value_value")) is None:
            continue
        tracking_rows.append(row)

    if not tracking_rows:
        return None

    return {
        "fact": direct_candidate["fact"],
        "plan": direct_candidate["plan"],
        "entry_value_tracking": tracking_rows,
    }


def _local_i64_map_const_entries_from_tracking_candidate(candidate):
    if not isinstance(candidate, dict):
        return None
    rows = candidate.get("entry_value_tracking")
    if not isinstance(rows, list) or not rows:
        return None

    entries_by_key: Dict[int, int] = {}
    for row in sorted(
        rows,
        key=lambda item: (
            _as_optional_int(item.get("set_block")) or 0,
            _as_optional_int(item.get("set_instruction_index")) or 0,
        )
        if isinstance(item, dict)
        else (0, 0),
    ):
        if not isinstance(row, dict):
            return None
        key_const = _as_optional_int(row.get("key_const_if_known"))
        value_const = _as_optional_int(row.get("value_const_if_known"))
        if key_const is None or value_const is None:
            return None
        entries_by_key[key_const] = value_const

    if not entries_by_key:
        return None
    return sorted(entries_by_key.items(), key=lambda item: item[0])


def _lower_local_i64_map_entry_table_dispatch(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    recv_h,
    key,
    entries,
):
    if not entries:
        return None

    i64 = ir.IntType(64)
    function = builder.block.function
    done_bb = function.append_basic_block("local_i64_map_entry_dispatch.done")
    fallback_bb = function.append_basic_block("local_i64_map_entry_dispatch.fallback")
    incoming = []

    for index, (key_const, value_const) in enumerate(entries):
        match_bb = function.append_basic_block(f"local_i64_map_entry_dispatch.match{index}")
        next_bb = (
            fallback_bb
            if index == len(entries) - 1
            else function.append_basic_block(f"local_i64_map_entry_dispatch.next{index}")
        )
        cond = builder.icmp_signed(
            "==",
            key,
            ir.Constant(i64, int(key_const)),
            name=f"local_i64_map_entry_dispatch_hit{index}",
        )
        builder.cbranch(cond, match_bb, next_bb)

        builder.position_at_end(match_bb)
        builder.branch(done_bb)
        incoming.append((ir.Constant(i64, int(value_const)), builder.block))

        builder.position_at_end(next_bb)

    callee = declare("nyash.map.slot_load_hh", i64, [i64, i64])
    fallback_value = builder.call(callee, [recv_h, key], name="local_i64_map_entry_dispatch_fallback")
    builder.branch(done_bb)
    incoming.append((fallback_value, builder.block))

    builder.position_at_end(done_bb)
    result = builder.phi(i64, name="local_i64_map_entry_dispatch_result")
    for value, block in incoming:
        result.add_incoming(value, block)
    return result


def _map_lookup_semantic_op(method_name: str) -> Optional[str]:
    if method_name == MAP_METHOD_GET:
        return MAP_LOOKUP_GET_SEMANTIC_OP
    if method_name == MAP_METHOD_HAS:
        return MAP_LOOKUP_HAS_SEMANTIC_OP
    return None


def _map_lookup_expected_routes(semantic_op: str):
    if semantic_op == MAP_LOOKUP_GET_SEMANTIC_OP:
        return (MAP_LOOKUP_CONST_FOLD_ROUTE, MAP_MISSING_EMPTY_CONST_ZERO_ROUTE)
    if semantic_op == MAP_LOOKUP_HAS_SEMANTIC_OP:
        return (MAP_LOOKUP_CONST_FOLD_ROUTE,)
    return ()


def _selected_map_lookup_decision(
    *,
    resolver,
    block_id: int,
    instruction_index: int,
    expected_routes,
    expected_semantic_op: str,
):
    decisions_by_site = getattr(resolver, "route_decisions_by_site", None)
    if not isinstance(decisions_by_site, dict):
        return None
    expected_routes = set(expected_routes or ())
    if not expected_routes:
        return None
    for decision in decisions_by_site.get((block_id, instruction_index), []):
        if not isinstance(decision, dict):
            continue
        selected_route = decision.get("selected_route")
        if selected_route not in expected_routes:
            continue
        if decision.get("semantic_op") != expected_semantic_op:
            continue
        source_plan_kind = decision.get("source_plan_kind")
        if selected_route == MAP_LOOKUP_CONST_FOLD_ROUTE:
            if source_plan_kind != MAP_LOOKUP_FUSION_SOURCE_PLAN:
                continue
        elif selected_route == MAP_MISSING_EMPTY_CONST_ZERO_ROUTE:
            if (
                source_plan_kind != MAP_MISSING_EMPTY_SOURCE_PLAN
                or expected_semantic_op != MAP_LOOKUP_GET_SEMANTIC_OP
            ):
                continue
        else:
            continue
        return decision
    return None


def _as_optional_int(value) -> Optional[int]:
    if value is None:
        return None
    try:
        return int(value)
    except _SAFE_COLLECTION_METHOD_EXC:
        return None


def _lower_map_route_decision_constant(
    *,
    resolver,
    box_name,
    method_name: str,
    receiver_vid,
    arg_ids: List[int],
):
    i64 = ir.IntType(64)
    decision = _current_map_lookup_fusion_decision(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
    )
    if decision is None:
        return None
    if method_name == MAP_METHOD_GET:
        selected_i64_const = decision.get("selected_i64_const")
        if selected_i64_const is not None:
            return ir.Constant(i64, int(selected_i64_const))
    if method_name == MAP_METHOD_HAS:
        selected_bool_const = decision.get("selected_bool_const")
        if selected_bool_const is not None:
            return ir.Constant(i64, 1 if bool(selected_bool_const) else 0)
    return None


def _lower_map_clear_collection_method_call(
    *, builder: ir.IRBuilder, declare: Callable, box_name, recv_h, arg_ids: List[int]
):
    if str(box_name or "") != BOX_MAP:
        return None
    if arg_ids:
        return ir.Constant(ir.IntType(64), 0)
    callee = declare("nyash.map.clear_h", ir.IntType(64), [ir.IntType(64)])
    return builder.call(callee, [recv_h], name="unified_map_clear_h")


def _lower_map_delete_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    box_name,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    if str(box_name or "") != BOX_MAP:
        return None
    if len(arg_ids) < 1:
        return zero
    key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
    callee = declare("nyash.map.delete_hh", i64, [i64, i64])
    return builder.call(callee, [recv_h, key], name="unified_map_delete_hh")


def _lower_map_get_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    box_name,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
    receiver_vid=None,
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    route_decision_constant = _lower_map_route_decision_constant(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
    )
    if route_decision_constant is not None:
        return route_decision_constant
    key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
    if not arg_ids:
        return zero
    entry_tracking_candidate = _current_local_i64_map_entry_value_tracking_shadow_candidate(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
    )
    if entry_tracking_candidate is not None:
        entries = _local_i64_map_const_entries_from_tracking_candidate(
            entry_tracking_candidate
        )
        entry_table_result = _lower_local_i64_map_entry_table_dispatch(
            builder=builder,
            declare=declare,
            recv_h=recv_h,
            key=key,
            entries=entries,
        )
        if entry_table_result is not None:
            return entry_table_result
    local_fastpath_fact = current_local_fastpath_known_receiver_direct_call_fact(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
        required_route_plan=LOCAL_FASTPATH_ROUTE_MAP_SCALAR_NO_PUBLICATION_GET,
    )
    if local_fastpath_fact is not None:
        local_storage_plan = _current_local_i64_map_storage_realization_plan(
            resolver=resolver,
            receiver_vid=receiver_vid,
        )
        if local_storage_plan is not None:
            callee = declare("nyash.map.local_i64_get_hi", i64, [i64, i64])
            return builder.call(callee, [recv_h, key], name="local_fastpath_map_get_hi")
    callee = declare("nyash.map.slot_load_hh", i64, [i64, i64])
    return builder.call(callee, [recv_h, key], name="unified_map_slot_load_hh")


def _lower_map_has_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    box_name,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
    receiver_vid=None,
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    route_decision_constant = _lower_map_route_decision_constant(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
    )
    if route_decision_constant is not None:
        return route_decision_constant
    key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
    if not arg_ids:
        return zero
    callee = declare("nyash.map.probe_hh", i64, [i64, i64])
    return builder.call(callee, [recv_h, key], name="unified_map_probe_hh")


def _lower_array_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
    receiver_vid=None,
    dst_vid=None,
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)

    # Preserve the existing fail-safe return shape for missing arguments.
    if method_name in (MAP_METHOD_GET, MAP_METHOD_HAS) and not arg_ids:
        return zero
    if method_name in (MAP_METHOD_PUSH, MAP_METHOD_SET) and (
        (method_name == MAP_METHOD_PUSH and not arg_ids)
        or (method_name == MAP_METHOD_SET and len(arg_ids) < 2)
    ):
        return recv_h

    direct_result = _lower_direct_array_nativedirect_call(
        builder=builder,
        method_name=method_name,
        recv_h=recv_h,
        arg_ids=arg_ids,
        resolve_arg=resolve_arg,
        resolver=resolver,
        receiver_vid=receiver_vid,
        dst_vid=dst_vid,
    )
    if direct_result is not None:
        return direct_result

    # Keep ArrayBox and RuntimeDataBox(array-specialized) on the same canonical
    # RawArray symbol table so lowering truth cannot drift across entrypoints.
    spec = select_array_collection_call_spec(
        method=method_name,
        resolver=resolver,
        arg_vids=arg_ids,
    )
    if spec is None:
        return None
    return _lower_call_spec(
        builder=builder,
        declare=declare,
        spec=spec,
        recv_h=recv_h,
        arg_ids=arg_ids,
        resolve_arg=resolve_arg,
    )


def _lower_non_array_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    box_name,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
    receiver_vid=None,
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)

    if method_name == MAP_METHOD_CLEAR:
        return _lower_map_clear_collection_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            recv_h=recv_h,
            arg_ids=arg_ids,
        )

    if method_name == MAP_METHOD_DELETE:
        return _lower_map_delete_collection_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            recv_h=recv_h,
            arg_ids=arg_ids,
            resolve_arg=resolve_arg,
        )

    if method_name == MAP_METHOD_GET:
        return _lower_map_get_collection_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            method_name=method_name,
            recv_h=recv_h,
            arg_ids=arg_ids,
            resolve_arg=resolve_arg,
            resolver=resolver,
            receiver_vid=receiver_vid,
        )

    if method_name == MAP_METHOD_PUSH:
        # Legacy non-ArrayBox collection fallback. This is intentionally kept
        # separate from MapBox-only clear/delete until route metadata can
        # distinguish array-like unknown receivers from true map receivers.
        value = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        if not arg_ids:
            return recv_h
        callee = declare("nyash.array.slot_append_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, value], name="unified_array_slot_append_hh")

    if method_name == MAP_METHOD_SET:
        if len(arg_ids) < 2:
            return recv_h
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        value = _resolve_or_zero(resolve_arg, arg_ids, 1, zero)
        return _lower_store_map_value_current_lowering(
            builder=builder,
            declare=declare,
            recv_h=recv_h,
            key=key,
            value=value,
        )

    if method_name == MAP_METHOD_HAS:
        return _lower_map_has_collection_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            method_name=method_name,
            recv_h=recv_h,
            arg_ids=arg_ids,
            resolve_arg=resolve_arg,
            resolver=resolver,
            receiver_vid=receiver_vid,
        )

    return None


def lower_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    box_name,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
    receiver_vid=None,
    dst_vid=None,
    prefer_array_mono_route=None,
):
    route_decision_constant = _lower_map_route_decision_constant(
        resolver=resolver,
        box_name=box_name,
        method_name=method_name,
        receiver_vid=receiver_vid,
        arg_ids=arg_ids,
    )
    if route_decision_constant is not None:
        return route_decision_constant

    runtime_result = lower_runtime_data_method_call(
        builder=builder,
        declare=declare,
        box_name=box_name,
        method=method_name,
        recv_h=recv_h,
        args=[resolve_arg(arg_id) for arg_id in arg_ids],
        resolver=resolver,
        receiver_vid=receiver_vid,
        arg_vids=arg_ids,
        prefer_array_mono_route=prefer_array_mono_route,
    )
    if runtime_result is not None:
        return runtime_result

    if str(box_name or "") == "ArrayBox":
        return _lower_array_collection_method_call(
            builder=builder,
            declare=declare,
            method_name=method_name,
            recv_h=recv_h,
            arg_ids=arg_ids,
            resolve_arg=resolve_arg,
            resolver=resolver,
            receiver_vid=receiver_vid,
            dst_vid=dst_vid,
        )

    return _lower_non_array_collection_method_call(
        builder=builder,
        declare=declare,
        box_name=box_name,
        method_name=method_name,
        recv_h=recv_h,
        arg_ids=arg_ids,
        resolve_arg=resolve_arg,
        resolver=resolver,
        receiver_vid=receiver_vid,
    )
