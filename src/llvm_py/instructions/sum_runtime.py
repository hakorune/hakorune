from typing import Any, Dict, Iterable, List

SUM_TAG_FIELD = "__sum_tag"
SUM_PAYLOAD_FIELD = "__sum_payload"
_RUNTIME_BOX_PREFIX = "__NySum_"


def runtime_box_name(enum_name: str) -> str:
    return f"{_RUNTIME_BOX_PREFIX}{enum_name}"


def synthetic_user_box_decls(enum_decls: Iterable[Dict[str, Any]]) -> List[Dict[str, Any]]:
    decls: List[Dict[str, Any]] = []
    for enum_decl in enum_decls or []:
        if not isinstance(enum_decl, dict):
            continue
        enum_name = enum_decl.get("name")
        if not isinstance(enum_name, str) or not enum_name:
            continue
        decls.append(
            {
                "name": runtime_box_name(enum_name),
                "fields": [SUM_TAG_FIELD, SUM_PAYLOAD_FIELD],
            }
        )
    return decls


def merge_user_box_decls(
    user_box_decls: Iterable[Dict[str, Any]],
    enum_decls: Iterable[Dict[str, Any]],
) -> List[Dict[str, Any]]:
    merged: List[Dict[str, Any]] = []
    seen_names = set()
    for box_decl in list(user_box_decls or []) + synthetic_user_box_decls(enum_decls):
        if not isinstance(box_decl, dict):
            continue
        name = box_decl.get("name")
        if not isinstance(name, str) or not name or name in seen_names:
            continue
        merged.append(box_decl)
        seen_names.add(name)
    return merged
