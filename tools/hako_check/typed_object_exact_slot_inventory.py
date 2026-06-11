"""Typed-object exact-slot inventory helpers.

These helpers derive report/check vocabulary from MIR JSON without changing
runtime behavior.
"""

from __future__ import annotations

from collections import Counter
from typing import Any


SIGNED_STORAGE_FAMILIES = {"i8", "i16", "i32", "i64", "isize"}
UNSIGNED_STORAGE_FAMILIES = {"u8", "u16", "u32", "u64", "usize"}
HANDLE_STORAGE_FAMILY = {"handle"}


def root_list(data: dict[str, Any], key: str) -> list[dict[str, Any]]:
    value = data.get(key)
    if not isinstance(value, list):
        return []
    return [row for row in value if isinstance(row, dict)]


def typed_object_exact_slot_route_decisions(mir: dict[str, Any]) -> list[dict[str, Any]]:
    decisions: list[dict[str, Any]] = []
    for function in root_list(mir, "functions"):
        metadata = function.get("metadata")
        if not isinstance(metadata, dict):
            continue
        route_decisions = metadata.get("route_decisions")
        if not isinstance(route_decisions, list):
            continue
        for decision in route_decisions:
            if not isinstance(decision, dict):
                continue
            if decision.get("source_plan_kind") != "TypedObjectExactSlotRoute":
                continue
            if decision.get("selected_lowering_form") != "exact_helper_bridge":
                continue
            decisions.append(decision)
    return decisions


def _sorted_join(values: set[str]) -> str:
    if not values:
        return "none"
    return ",".join(sorted(values))


def typed_object_exact_bridge_symbol(semantic_op: str, storage: str) -> str:
    if semantic_op == "FieldGet" and storage in SIGNED_STORAGE_FAMILIES:
        return "hako.object.exact_slot_get_i64_hii"
    if semantic_op == "FieldSet" and storage in SIGNED_STORAGE_FAMILIES:
        return "hako.object.exact_slot_set_i64_hii"
    if semantic_op == "FieldGet" and storage in UNSIGNED_STORAGE_FAMILIES:
        return "hako.object.exact_slot_get_u64_hii"
    if semantic_op == "FieldSet" and storage in UNSIGNED_STORAGE_FAMILIES:
        return "hako.object.exact_slot_set_u64_hiu"
    if semantic_op == "FieldGet" and storage in HANDLE_STORAGE_FAMILY:
        return "hako.object.exact_slot_get_handle_hii"
    if semantic_op == "FieldSet" and storage in HANDLE_STORAGE_FAMILY:
        return "hako.object.exact_slot_set_handle_hii"
    return "none"


def typed_object_exact_route_sample_rows(
    exact_slot_route_decisions: list[dict[str, Any]],
) -> list[tuple[str, str]]:
    if not exact_slot_route_decisions:
        return []
    first_route = exact_slot_route_decisions[0]
    return [
        (
            "typed_object_exact_route_0_function",
            str(first_route.get("function", "unknown")),
        ),
        (
            "typed_object_exact_route_0_site_id",
            str(first_route.get("site_id", "unknown")),
        ),
        (
            "typed_object_exact_route_0_selected_route",
            str(first_route.get("selected_route", "unknown")),
        ),
        (
            "typed_object_exact_route_0_selected_lowering_form",
            str(first_route.get("selected_lowering_form", "unknown")),
        ),
        (
            "typed_object_exact_route_0_selected_bridge_symbol",
            str(
                first_route.get("selected_bridge_symbol")
                or typed_object_exact_bridge_symbol(
                    str(first_route.get("semantic_op") or ""),
                    str(first_route.get("selected_storage") or ""),
                )
            ),
        ),
    ]


def typed_object_exact_slot_inventory(mir: dict[str, Any]) -> dict[str, int | str]:
    plans = root_list(mir, "typed_object_plans")
    user_box_decls = root_list(mir, "user_box_decls")
    exact_slot_route_decisions = typed_object_exact_slot_route_decisions(mir)

    field_counts: Counter[str] = Counter()
    exact_eligible_count = 0
    for plan in plans:
        fields = plan.get("fields", [])
        if not isinstance(fields, list):
            continue
        for field in fields:
            if not isinstance(field, dict):
                continue
            storage = str(field.get("storage") or "")
            field_counts[storage] += 1
            exact_eligible_count += 1

    total_decl_count = 0
    for box in user_box_decls:
        fields = box.get("field_decls", [])
        if not isinstance(fields, list):
            continue
        total_decl_count += sum(1 for field in fields if isinstance(field, dict))

    compat_field_get_count = max(0, total_decl_count - exact_eligible_count)

    if exact_slot_route_decisions:
        route_counts: Counter[str] = Counter()
        bridge_symbols: set[str] = set()
        lowering_forms: set[str] = set()
        for decision in exact_slot_route_decisions:
            storage = str(decision.get("selected_storage") or "")
            route_counts[storage] += 1
            bridge_symbol = str(decision.get("selected_bridge_symbol") or "")
            if not bridge_symbol or bridge_symbol == "None":
                bridge_symbol = typed_object_exact_bridge_symbol(
                    str(decision.get("semantic_op") or ""),
                    storage,
                )
            if bridge_symbol:
                bridge_symbols.add(bridge_symbol)
            lowering_form = str(decision.get("selected_lowering_form") or "")
            if lowering_form:
                lowering_forms.add(lowering_form)
        signed_count = sum(route_counts[storage] for storage in SIGNED_STORAGE_FAMILIES)
        unsigned_count = sum(route_counts[storage] for storage in UNSIGNED_STORAGE_FAMILIES)
        handle_count = sum(route_counts[storage] for storage in HANDLE_STORAGE_FAMILY)
        exact_helper_call_count = len(exact_slot_route_decisions)
        exact_slot_eligible_count = exact_helper_call_count
        exact_bridge_symbols = _sorted_join(bridge_symbols)
        exact_lowering_forms = _sorted_join(lowering_forms)
    else:
        signed_count = sum(field_counts[storage] for storage in SIGNED_STORAGE_FAMILIES)
        unsigned_count = sum(field_counts[storage] for storage in UNSIGNED_STORAGE_FAMILIES)
        handle_count = sum(field_counts[storage] for storage in HANDLE_STORAGE_FAMILY)
        exact_helper_call_count = exact_eligible_count
        exact_slot_eligible_count = exact_eligible_count
        exact_bridge_symbols = _sorted_join(
            {
                "hako.object.exact_slot_get_i64_hii"
                if signed_count
                else "",
                "hako.object.exact_slot_set_i64_hii"
                if signed_count
                else "",
                "hako.object.exact_slot_get_u64_hii"
                if unsigned_count
                else "",
                "hako.object.exact_slot_set_u64_hiu"
                if unsigned_count
                else "",
                "hako.object.exact_slot_get_handle_hii"
                if handle_count
                else "",
                "hako.object.exact_slot_set_handle_hii"
                if handle_count
                else "",
            }
            - {""}
        )
        exact_lowering_forms = "exact_helper_bridge" if exact_eligible_count else "none"

    return {
        "typed_object_exact_slot_get_i64_count": signed_count,
        "typed_object_exact_slot_set_i64_count": signed_count,
        "typed_object_exact_slot_get_u64_count": unsigned_count,
        "typed_object_exact_slot_set_u64_count": unsigned_count,
        "typed_object_exact_slot_get_handle_count": handle_count,
        "typed_object_exact_slot_set_handle_count": handle_count,
        "typed_object_exact_helper_call_count": exact_helper_call_count,
        "typed_object_inline_slot_load_count": 0,
        "typed_object_inline_slot_store_count": 0,
        "typed_object_compat_field_get_count": compat_field_get_count,
        "typed_object_get_compat_i64_count": 0,
        "typed_object_exact_name_lookup_count": 0,
        "typed_object_exact_internal_dispatch_count": 0,
        "typed_object_exact_silent_fallback_count": 0,
        "typed_object_required_route_failfast_count": compat_field_get_count,
        "typed_object_exact_slot_eligible_count": exact_slot_eligible_count,
        "typed_object_exact_slot_compat_legacy_count": compat_field_get_count,
        "typed_object_exact_route_decision_count": len(exact_slot_route_decisions),
        "typed_object_exact_lowering_forms": exact_lowering_forms,
        "typed_object_exact_bridge_symbols": exact_bridge_symbols,
    }


def typed_object_exact_slot_nativedirect_readiness_inventory(
    mir: dict[str, Any],
) -> dict[str, int | str]:
    exact_inventory = typed_object_exact_slot_inventory(mir)
    direct_state_plans = root_list(mir, "direct_state_plans")
    selected_direct_state_plans = [
        plan
        for plan in direct_state_plans
        if bool(plan.get("field_decl_authority"))
        and bool(plan.get("materialization_boundary_known"))
        and bool(plan.get("positive_net_expected"))
    ]

    selected_direct_state_field_count = 0
    for plan in selected_direct_state_plans:
        fields = plan.get("fields", [])
        if isinstance(fields, list):
            selected_direct_state_field_count += sum(
                1 for field in fields if isinstance(field, dict)
            )

    direct_state_field_count = 0
    for plan in direct_state_plans:
        fields = plan.get("fields", [])
        if isinstance(fields, list):
            direct_state_field_count += sum(1 for field in fields if isinstance(field, dict))

    return {
        **exact_inventory,
        "typed_object_direct_state_plan_count": len(direct_state_plans),
        "typed_object_direct_state_field_count": direct_state_field_count,
        "typed_object_direct_state_selected_count": len(selected_direct_state_plans),
        "typed_object_direct_state_selected_field_count": selected_direct_state_field_count,
        "typed_object_native_direct_candidate_count": len(selected_direct_state_plans),
        "typed_object_native_direct_ready": 0,
        "typed_object_native_direct_open": 0,
        "typed_object_direct_load_store_open": 0,
        "typed_object_native_direct_storage_substrate": "PinnedTypedObjectArena",
        "typed_object_native_direct_fallback_boundary": "explicit_materialized_view_handle",
        "typed_object_native_direct_selected_next": "typed_object_exact_slot_nativedirect_guard_surface",
    }


def typed_object_exact_slot_nativedirect_guard_surface_inventory(
    readiness: dict[str, int | str]
) -> dict[str, int | str]:
    return {
        "output_contract": "typed-object-exact-slot-nativedirect-guard-surface-v0",
        "input_contract": "typed-object-exact-slot-nativedirect-readiness-inventory-v0",
        "workload_id": str(
            readiness.get("workload_id", "representative-object-lifecycle-small-block-v0")
        ),
        "candidate_representation": "NativeDirect",
        "selected_route": "hako.typed_object.slot_load_i64",
        "selected_lowering_form": "exact_helper_bridge",
        "storage_substrate": "PinnedTypedObjectArena",
        "fallback_boundary": "explicit_materialized_view_handle",
        "typed_object_native_direct_ready": int(
            readiness.get("typed_object_native_direct_ready", 0)
        ),
        "typed_object_native_direct_open": int(
            readiness.get("typed_object_native_direct_open", 0)
        ),
        "typed_object_direct_load_store_open": int(
            readiness.get("typed_object_direct_load_store_open", 0)
        ),
        "object_storage_pinned_required": 1,
        "field_address_stable_required": 1,
        "object_generation_required": 1,
        "slot_layout_stable_required": 1,
        "handle_generation_validation_required": 1,
        "lease_region_required": 1,
        "lease_barrier_policy_required": 1,
        "silent_fallback_allowed": 0,
        "helper_load_writeback_substitution_allowed": 0,
        "raw_runtime_vec_pointer_exposure_allowed": 0,
        "by_name_hako_alloc_special_case_allowed": 0,
        "selected_next": "typed_object_exact_slot_nativedirect_pilot_selection",
        "winner_claim": 0,
        "replacement_active": 0,
        "hook_installed": 0,
        "global_allocator": 0,
        "summary": "ok",
    }
