from typing import Any, Dict, Iterable, List

ENUM_TAG_FIELD = "__variant_tag"
ENUM_PAYLOAD_FIELD = "__variant_payload"
_RUNTIME_BOX_PREFIX = "__NyVariant_"


def runtime_box_name(enum_name: str) -> str:
    return f"{_RUNTIME_BOX_PREFIX}{enum_name}"


def merge_user_box_decls(user_box_decls: Iterable[Dict[str, Any]]) -> List[Dict[str, Any]]:
    merged: List[Dict[str, Any]] = []
    seen_names = set()
    for box_decl in list(user_box_decls or []):
        if not isinstance(box_decl, dict):
            continue
        name = box_decl.get("name")
        if not isinstance(name, str) or not name or name in seen_names:
            continue
        merged.append(box_decl)
        seen_names.add(name)
    return merged
