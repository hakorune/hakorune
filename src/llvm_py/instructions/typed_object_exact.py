from typing import Any, Dict, Iterable, Optional


_STORAGE_TAGS = {
    "i64": 1,
    "handle": 2,
    "isize": 3,
    "usize": 4,
    "i8": 5,
    "i16": 6,
    "i32": 7,
    "u8": 8,
    "u16": 9,
    "u32": 10,
    "u64": 11,
}

_UNSIGNED_STORAGES = {"usize", "u8", "u16", "u32", "u64"}
_SIGNED_STORAGES = {"isize", "i8", "i16", "i32", "i64"}
_LEGACY_STORAGES = {"i64", "handle"}


def normalize_storage(raw: Any) -> Optional[str]:
    if not isinstance(raw, str):
        return None
    storage = raw.strip().lower()
    if storage in _STORAGE_TAGS:
        return storage
    return None


def storage_tag(raw: Any) -> Optional[int]:
    storage = normalize_storage(raw)
    if storage is None:
        return None
    return _STORAGE_TAGS.get(storage)


def is_unsigned_storage(raw: Any) -> bool:
    storage = normalize_storage(raw)
    return storage in _UNSIGNED_STORAGES


def is_signed_storage(raw: Any) -> bool:
    storage = normalize_storage(raw)
    return storage in _SIGNED_STORAGES


def is_handle_storage(raw: Any) -> bool:
    return normalize_storage(raw) == "handle"


def plan_requires_typed_object_helpers(plan: Any) -> bool:
    if not isinstance(plan, dict):
        return False
    for field in plan.get("fields", []) or []:
        if not isinstance(field, dict):
            return False
        storage = normalize_storage(field.get("storage"))
        if storage is None:
            return False
        if storage not in _LEGACY_STORAGES:
            return True
    return False


def exact_object_plans(plans: Iterable[Any]) -> list[Dict[str, Any]]:
    return [
        plan
        for plan in plans or []
        if isinstance(plan, dict) and plan_requires_typed_object_helpers(plan)
    ]


def exact_object_plan_for_box(plans: Iterable[Any], box_name: Any) -> Optional[Dict[str, Any]]:
    if not isinstance(box_name, str):
        return None
    for plan in exact_object_plans(plans):
        if plan.get("box_name") == box_name:
            return plan
    return None


def exact_field_plan_for_box(
    plans: Iterable[Any],
    box_name: Any,
    field_name: Any,
) -> Optional[Dict[str, Any]]:
    plan = exact_object_plan_for_box(plans, box_name)
    if plan is None or not isinstance(field_name, str):
        return None
    for field in plan.get("fields", []) or []:
        if isinstance(field, dict) and field.get("name") == field_name:
            storage = normalize_storage(field.get("storage"))
            if storage is None:
                return None
            row = dict(field)
            row["storage"] = storage
            return row
    return None

