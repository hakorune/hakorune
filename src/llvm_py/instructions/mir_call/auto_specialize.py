"""
AutoSpecializeBox helpers (v0).

v0 keeps specialization conservative and contract-safe:
- no new env toggles
- no behavior changes on miss (fallback stays as-is)
- decision source = resolver type facts + intrinsic registry
"""

from typing import Any, Optional, Sequence

from .intrinsic_registry import is_length_like_method


def _is_stringbox_value_type(value_type: Any) -> bool:
    if not isinstance(value_type, dict):
        return False
    kind = value_type.get("kind")
    if kind == "string":
        return True
    return kind == "handle" and value_type.get("box_type") == "StringBox"


def _is_arraybox_value_type(value_type: Any) -> bool:
    if not isinstance(value_type, dict):
        return False
    return value_type.get("kind") == "handle" and value_type.get("box_type") == "ArrayBox"


def _is_mapbox_value_type(value_type: Any) -> bool:
    if not isinstance(value_type, dict):
        return False
    return value_type.get("kind") == "handle" and value_type.get("box_type") == "MapBox"


def receiver_is_stringish(resolver: Any, receiver_vid: Optional[int]) -> bool:
    if resolver is None or receiver_vid is None:
        return False
    try:
        vid = int(receiver_vid)
    except (TypeError, ValueError):
        return False

    # Primary hint: propagated stringish tags.
    try:
        if hasattr(resolver, "is_stringish") and resolver.is_stringish(vid):
            return True
    except Exception:
        pass

    # Secondary hint: value_types facts.
    try:
        value_types = getattr(resolver, "value_types", None)
        if isinstance(value_types, dict) and _is_stringbox_value_type(value_types.get(vid)):
            return True
    except Exception:
        pass

    return False


def receiver_is_arrayish(resolver: Any, receiver_vid: Optional[int]) -> bool:
    if resolver is None or receiver_vid is None:
        return False
    try:
        vid = int(receiver_vid)
    except (TypeError, ValueError):
        return False

    try:
        if hasattr(resolver, "is_arrayish") and resolver.is_arrayish(vid):
            return True
    except Exception:
        pass

    try:
        value_types = getattr(resolver, "value_types", None)
        if isinstance(value_types, dict) and _is_arraybox_value_type(value_types.get(vid)):
            return True
    except Exception:
        pass

    return False


def receiver_is_mapish(resolver: Any, receiver_vid: Optional[int]) -> bool:
    if resolver is None or receiver_vid is None:
        return False
    try:
        vid = int(receiver_vid)
    except (TypeError, ValueError):
        return False

    try:
        if hasattr(resolver, "is_mapish") and resolver.is_mapish(vid):
            return True
    except Exception:
        pass

    try:
        value_types = getattr(resolver, "value_types", None)
        if isinstance(value_types, dict) and _is_mapbox_value_type(value_types.get(vid)):
            return True
    except Exception:
        pass

    return False


def prefer_string_len_h_route(
    method: Optional[str], args_count: int, resolver: Any, receiver_vid: Optional[int]
) -> bool:
    if not is_length_like_method(method):
        return False
    if args_count != 0:
        return False
    return receiver_is_stringish(resolver, receiver_vid)


def prefer_array_len_h_route(
    method: Optional[str], args_count: int, resolver: Any, receiver_vid: Optional[int]
) -> bool:
    if not is_length_like_method(method):
        return False
    if args_count != 0:
        return False
    return receiver_is_arrayish(resolver, receiver_vid)


def prefer_map_len_h_route(
    method: Optional[str], args_count: int, resolver: Any, receiver_vid: Optional[int]
) -> bool:
    if not is_length_like_method(method):
        return False
    if args_count != 0:
        return False
    return receiver_is_mapish(resolver, receiver_vid)


def prefer_runtime_data_array_route(
    method: Optional[str],
    box_name: Optional[str],
    resolver: Any,
    receiver_vid: Optional[int],
    arg_vids: Optional[Sequence[Optional[int]]],
) -> bool:
    if str(box_name or "") != "RuntimeDataBox":
        return False
    if not receiver_is_arrayish(resolver, receiver_vid):
        return False

    method_name = str(method or "")
    vids = list(arg_vids or [])

    if method_name == "push" and len(vids) == 1:
        return True

    if method_name in ("get", "has") and len(vids) == 1:
        return True

    if method_name == "set" and len(vids) == 2:
        return True

    return False


def _value_is_i64_hint(resolver: Any, value_vid: Optional[int]) -> bool:
    if resolver is None or value_vid is None:
        return False
    try:
        vid = int(value_vid)
    except (TypeError, ValueError):
        return False

    # Primary hint: pre-lowering integer-like facts.
    try:
        integerish_ids = getattr(resolver, "integerish_ids", None)
        if isinstance(integerish_ids, set) and vid in integerish_ids:
            return True
    except Exception:
        pass

    # Secondary hint: explicit value type metadata.
    try:
        value_types = getattr(resolver, "value_types", None)
        if isinstance(value_types, dict):
            value_type = value_types.get(vid)
            if isinstance(value_type, str):
                if value_type.lower() in ("i64", "int", "integer"):
                    return True
            if isinstance(value_type, dict):
                kind = str(value_type.get("kind") or "").lower()
                if kind in ("i64", "int", "integer"):
                    return True
    except Exception:
        pass

    return False


def prefer_array_i64_key_route(
    method: Optional[str],
    resolver: Any,
    arg_vids: Optional[Sequence[Optional[int]]],
) -> bool:
    method_name = str(method or "")
    vids = list(arg_vids or [])

    key_vid: Optional[int] = None
    if method_name in ("get", "has") and len(vids) == 1:
        key_vid = vids[0]
    elif method_name == "set" and len(vids) == 2:
        key_vid = vids[0]
    else:
        return False

    return _value_is_i64_hint(resolver, key_vid)


def prefer_array_i64_key_i64_value_route(
    method: Optional[str],
    resolver: Any,
    arg_vids: Optional[Sequence[Optional[int]]],
) -> bool:
    if str(method or "") != "set":
        return False
    vids = list(arg_vids or [])
    if len(vids) != 2:
        return False
    key_vid, value_vid = vids[0], vids[1]
    return _value_is_i64_hint(resolver, key_vid) and _value_is_i64_hint(
        resolver, value_vid
    )


def prefer_runtime_data_array_i64_key_route(
    method: Optional[str],
    resolver: Any,
    arg_vids: Optional[Sequence[Optional[int]]],
) -> bool:
    return prefer_array_i64_key_route(method, resolver, arg_vids)


def prefer_runtime_data_array_i64_key_i64_value_route(
    method: Optional[str],
    resolver: Any,
    arg_vids: Optional[Sequence[Optional[int]]],
) -> bool:
    return prefer_array_i64_key_i64_value_route(method, resolver, arg_vids)
