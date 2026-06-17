import llvmlite.ir as ir


class _DummyResolver:
    def __init__(self, value_types=None, integerish_ids=None):
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])
        self.direct_array_access_plans_by_site = {}
        self.map_lookup_fusion_routes_by_site = {}
        self.map_repr_plans_by_site = {}
        self.local_fastpath_facts_by_site = {}
        self.local_map_storage_realization_plans_by_receiver = {}
        self.local_i64_map_direct_storage_plans_by_receiver = {}
        self.local_i64_map_entry_value_tracking_plans_by_receiver = {}
        self.route_decisions_by_site = {}
        self.route_decisions_metadata_present = False
        self.current_block_id = 0
        self.current_instruction_index = 0


def _new_builder():
    return _new_builder_named("main")


def _new_builder_named(name):
    i64 = ir.IntType(64)
    module = ir.Module(name="test_collection_method_call")
    fn = ir.Function(module, ir.FunctionType(i64, []), name=name)
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder


def _seed_direct_array_plan(
    resolver,
    *,
    op,
    receiver_value=1,
    index_value=2,
    value_value=None,
    result_value=9,
    bounds_policy="checked",
    cfg_shape="checked_branching",
    fallback_policy="allow_checked",
):
    plan = {
        "op": op,
        "receiver_value": receiver_value,
        "index_value": index_value,
        "value_value": value_value,
        "result_value": result_value,
        "array_kind": "DirectArrayI64",
        "element_type": "i64",
        "route": f"direct_array_i64_{'load' if op == 'load' else 'store'}",
        "bounds_policy": bounds_policy,
        "proof_kind": "range_index" if bounds_policy == "proved_unchecked" else "exact_front_contract",
        "proof_ids": ["range_index"] if bounds_policy == "proved_unchecked" else ["exact_front_contract"],
        "fallback_policy": fallback_policy,
        "cfg_shape": cfg_shape,
        "store_semantics": "append_or_overwrite" if op == "store" else "not_store",
    }
    resolver.direct_array_access_plans_by_site[(resolver.current_block_id, resolver.current_instruction_index)] = [plan]


def _seed_route_decision(resolver, *, selected_route):
    resolver.route_decisions_by_site[
        (resolver.current_block_id, resolver.current_instruction_index)
    ] = [
        {
            "selected_route": selected_route,
            "fallback_policy": "opportunistic",
            "source_plan_kind": "DirectArrayAccessPlan",
        }
    ]


def _seed_map_lookup_fusion_route(
    resolver,
    *,
    receiver_value=1,
    key_value=2,
    key_const=-1,
    stored_value_const=None,
    stored_value_proof="unknown_scalar",
    get_instruction_index=3,
    has_instruction_index=4,
):
    route = {
        "route_id": "map_lookup.same_key",
        "block": int(resolver.current_block_id),
        "get_instruction_index": int(get_instruction_index),
        "has_instruction_index": int(has_instruction_index),
        "fusion_op": "MapLookupSameKey",
        "receiver_origin_box": "MapBox",
        "receiver_value": int(receiver_value),
        "key_value": int(key_value),
        "key_const": int(key_const),
        "get_result_value": 30,
        "has_result_value": 31,
        "get_return_shape": "scalar_i64_or_missing_zero",
        "get_value_demand": "scalar_i64",
        "get_publication_policy": "no_publication",
        "has_result_shape": "presence_bool",
        "stored_value_proof": stored_value_proof,
        "stored_value_const": stored_value_const,
        "stored_value_known_nonzero": None
        if stored_value_const is None
        else bool(int(stored_value_const) != 0),
        "proof": "same_receiver_same_i64_key_scalar_get_has",
        "lowering_tier": "cold_fallback",
    }
    resolver.map_lookup_fusion_routes_by_site.setdefault(
        (int(resolver.current_block_id), int(get_instruction_index)),
        [],
    ).append(route)
    resolver.map_lookup_fusion_routes_by_site.setdefault(
        (int(resolver.current_block_id), int(has_instruction_index)),
        [],
    ).append(route)


def _seed_local_i64_map_shadow_get_plan(
    resolver,
    *,
    receiver_value=1,
    key_value=2,
    instruction_index=0,
):
    resolver.map_repr_plans_by_site.setdefault(
        (int(resolver.current_block_id), int(instruction_index)),
        [],
    ).append(
        {
            "route_id": "map_repr.local_i64_key_map_shadow",
            "repr_kind": "local_i64_key_map_shadow",
            "source_route_kind": "map_load_scalar_i64",
            "receiver_value": int(receiver_value),
            "key_value": int(key_value),
            "proof_tag": "local_i64_key_map_shadow",
        }
    )


def _seed_local_fastpath_known_receiver_direct_call_fact(
    resolver,
    *,
    receiver_value=1,
    key_value=2,
    instruction_index=0,
    route_plan="map_repr.generic_hash_runtime",
    method_name="get",
    fallback_reason=None,
):
    resolver.local_fastpath_facts_by_site.setdefault(
        (int(resolver.current_block_id), int(instruction_index)),
        [],
    ).append(
        {
            "route_id": "local_fastpath.known_receiver_direct_call",
            "fact_kind": "local_fastpath_fact",
            "backend_kind": "known_receiver_direct_call",
            "route_plan": route_plan,
            "box_name": "MapBox",
            "method_name": method_name,
            "receiver_value": int(receiver_value),
            "key_value": int(key_value),
            "fallback_reason": fallback_reason,
        }
    )


def _seed_local_i64_map_storage_realization_plan(
    resolver,
    *,
    receiver_value=1,
    candidate_set_count=3,
    candidate_scalar_get_count=2,
):
    resolver.local_map_storage_realization_plans_by_receiver.setdefault(
        int(receiver_value),
        [],
    ).append(
        {
            "receiver_value": int(receiver_value),
            "representation": "local_i64_key_map",
            "candidate_set_count": int(candidate_set_count),
            "candidate_scalar_get_count": int(candidate_scalar_get_count),
            "publication_materialization_required": True,
            "backend_lowering_enabled": False,
            "runtime_helper_enabled": False,
        }
    )


def _seed_local_i64_map_direct_storage_plan(
    resolver,
    *,
    receiver_value=1,
    known_i64_key_set_count=3,
    scalar_get_count=2,
):
    resolver.local_i64_map_direct_storage_plans_by_receiver.setdefault(
        int(receiver_value),
        [],
    ).append(
        {
            "receiver_value": int(receiver_value),
            "representation": "closed_world_i64_key_value_table",
            "known_i64_key_set_count": int(known_i64_key_set_count),
            "scalar_get_count": int(scalar_get_count),
            "entry_value_tracking_enabled": False,
            "publication_materialization_required": True,
            "backend_lowering_enabled": False,
            "runtime_helper_enabled": False,
        }
    )


def _seed_local_i64_map_entry_value_tracking_plan(
    resolver,
    *,
    receiver_value=1,
    set_block=0,
    set_instruction_index=4,
    key_value=2,
    value_value=3,
    key_const_if_known=0,
    value_const_if_known=1,
):
    resolver.local_i64_map_entry_value_tracking_plans_by_receiver.setdefault(
        int(receiver_value),
        [],
    ).append(
        {
            "receiver_value": int(receiver_value),
            "set_block": int(set_block),
            "set_instruction_index": int(set_instruction_index),
            "key_value": int(key_value),
            "value_value": int(value_value),
            "key_const_if_known": None
            if key_const_if_known is None
            else int(key_const_if_known),
            "value_const_if_known": None
            if value_const_if_known is None
            else int(value_const_if_known),
            "backend_lowering_enabled": False,
            "runtime_helper_enabled": False,
        }
    )


def _seed_map_lookup_route_decision(
    resolver,
    *,
    selected_route="map_lookup_const_fold",
    selected_i64_const=None,
    selected_bool_const=None,
):
    resolver.route_decisions_by_site[
        (resolver.current_block_id, resolver.current_instruction_index)
    ] = [
        {
            "selected_route": selected_route,
            "fallback_policy": "opportunistic",
            "source_plan_kind": "MapLookupFusionRoute",
            "semantic_op": "MapHas"
            if int(resolver.current_instruction_index) == 1
            else "MapGet",
            "selected_i64_const": selected_i64_const,
            "selected_bool_const": selected_bool_const,
        }
    ]


def _seed_map_missing_empty_route_decision(resolver, *, selected_i64_const=0):
    resolver.route_decisions_by_site[
        (resolver.current_block_id, resolver.current_instruction_index)
    ] = [
        {
            "selected_route": "map_get_missing_empty_const_zero",
            "fallback_policy": "opportunistic",
            "source_plan_kind": "MapMissingEmptyRoute",
            "semantic_op": "MapGet",
            "selected_i64_const": selected_i64_const,
            "selected_bool_const": None,
        }
    ]


def _seed_non_map_route_decision(resolver, *, selected_route="map_lookup_const_fold"):
    resolver.route_decisions_by_site[
        (resolver.current_block_id, resolver.current_instruction_index)
    ] = [
        {
            "selected_route": selected_route,
            "fallback_policy": "opportunistic",
            "source_plan_kind": "DirectArrayAccessPlan",
            "semantic_op": "MapGet",
            "selected_i64_const": 7,
            "selected_bool_const": None,
        }
    ]
