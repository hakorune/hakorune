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


def typed_object_exact_slot_inventory(mir: dict[str, Any]) -> dict[str, int]:
    plans = root_list(mir, "typed_object_plans")
    user_box_decls = root_list(mir, "user_box_decls")

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

    signed_count = sum(field_counts[storage] for storage in SIGNED_STORAGE_FAMILIES)
    unsigned_count = sum(field_counts[storage] for storage in UNSIGNED_STORAGE_FAMILIES)
    handle_count = sum(field_counts[storage] for storage in HANDLE_STORAGE_FAMILY)

    return {
        "typed_object_exact_slot_get_i64_count": signed_count,
        "typed_object_exact_slot_set_i64_count": signed_count,
        "typed_object_exact_slot_get_u64_count": unsigned_count,
        "typed_object_exact_slot_set_u64_count": unsigned_count,
        "typed_object_exact_slot_get_handle_count": handle_count,
        "typed_object_exact_slot_set_handle_count": handle_count,
        "typed_object_exact_helper_call_count": exact_eligible_count,
        "typed_object_inline_slot_load_count": 0,
        "typed_object_inline_slot_store_count": 0,
        "typed_object_compat_field_get_count": compat_field_get_count,
        "typed_object_get_compat_i64_count": 0,
        "typed_object_exact_name_lookup_count": 0,
        "typed_object_exact_internal_dispatch_count": 0,
        "typed_object_exact_silent_fallback_count": 0,
        "typed_object_required_route_failfast_count": compat_field_get_count,
        "typed_object_exact_slot_eligible_count": exact_eligible_count,
        "typed_object_exact_slot_compat_legacy_count": compat_field_get_count,
    }
