from __future__ import annotations

from typing import Any, Iterable, Optional

try:
    from utils.resolver_helpers import get_box_type, mark_as_handle
except ImportError:
    from ..utils.resolver_helpers import get_box_type, mark_as_handle


def _dst_type_is_arraybox(dst_type: Any) -> bool:
    if not isinstance(dst_type, dict):
        return False
    return dst_type.get("kind") == "handle" and dst_type.get("box_type") == "ArrayBox"


def resolver_is_arrayish_vid(resolver: Any, value_id: Any) -> bool:
    try:
        vid = int(value_id)
    except Exception:
        return False

    try:
        if resolver is not None and hasattr(resolver, "is_arrayish") and resolver.is_arrayish(vid):
            return True
    except Exception:
        pass

    return get_box_type(resolver, vid) == "ArrayBox"


def _incoming_pair_has_arrayish_source(resolver: Any, pair: Any) -> bool:
    try:
        left, right = pair
    except Exception:
        return False

    # setup_phi_placeholders sees raw JSON order (value, block), while
    # finalize_phis sees normalized order (block, value). Accept either shape
    # and stay conservative by requiring that one side already carries ArrayBox fact.
    return resolver_is_arrayish_vid(resolver, left) or resolver_is_arrayish_vid(resolver, right)


def should_mark_phi_arrayish(
    resolver: Any,
    dst_type: Any,
    incoming: Optional[Iterable[Any]],
) -> bool:
    if _dst_type_is_arraybox(dst_type):
        return True

    for pair in incoming or []:
        if _incoming_pair_has_arrayish_source(resolver, pair):
            return True

    return False


def mark_arrayish_handle(resolver: Any, value_id: Any) -> None:
    try:
        vid = int(value_id)
    except Exception:
        return

    try:
        array_ids = getattr(resolver, "array_ids", None)
        if isinstance(array_ids, set):
            array_ids.add(vid)
    except Exception:
        pass

    mark_as_handle(resolver, vid, "ArrayBox")
