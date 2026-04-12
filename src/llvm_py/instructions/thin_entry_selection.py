from typing import Any, Dict, Optional

from instructions.primitive_handles import resolver_value_type


def lookup_thin_entry_selection_row(
    *,
    resolver,
    surface: str,
    subject: str,
    selection_value_id: Optional[int] = None,
) -> Optional[Dict[str, Any]]:
    if not isinstance(surface, str) or not isinstance(subject, str):
        return None

    by_value = getattr(resolver, "thin_entry_selection_by_value", None)
    if isinstance(selection_value_id, int) and isinstance(by_value, dict):
        for row in by_value.get(int(selection_value_id), []) or []:
            if (
                isinstance(row, dict)
                and row.get("surface") == surface
                and row.get("subject") == subject
            ):
                return row

    by_subject = getattr(resolver, "thin_entry_selection_by_subject", None)
    if isinstance(by_subject, dict):
        for row in by_subject.get((surface, subject), []) or []:
            if isinstance(row, dict):
                return row

    rows = getattr(resolver, "thin_entry_selections", None)
    if isinstance(rows, list):
        for row in rows:
            if (
                isinstance(row, dict)
                and row.get("surface") == surface
                and row.get("subject") == subject
            ):
                return row
    return None


def thin_entry_field_subject(
    resolver,
    box_vid: Optional[int],
    field_name: str,
) -> Optional[str]:
    receiver_box_type = _receiver_box_type(resolver, box_vid)
    if not isinstance(receiver_box_type, str):
        return None
    return f"{receiver_box_type}.{field_name}"


def thin_entry_prefers_inline_scalar_subject(
    *,
    resolver,
    surface: str,
    subject: str,
    selection_value_id: Optional[int] = None,
) -> Optional[bool]:
    row = lookup_thin_entry_selection_row(
        resolver=resolver,
        surface=surface,
        subject=subject,
        selection_value_id=selection_value_id,
    )
    if not isinstance(row, dict):
        return None
    if row.get("selected_entry") != "thin_internal_entry":
        return False
    return row.get("manifest_row") == f"{surface}.inline_scalar"


def thin_entry_prefers_inline_scalar_field(
    *,
    resolver,
    surface: str,
    box_vid: Optional[int],
    field_name: str,
    selection_value_id: Optional[int] = None,
) -> Optional[bool]:
    subject = thin_entry_field_subject(resolver, box_vid, field_name)
    if not isinstance(subject, str):
        return None
    return thin_entry_prefers_inline_scalar_subject(
        resolver=resolver,
        surface=surface,
        subject=subject,
        selection_value_id=selection_value_id,
    )


def thin_entry_prefers_known_receiver_method(
    *,
    resolver,
    box_name: Optional[str],
    method_name: Optional[str],
    selection_value_id: Optional[int] = None,
) -> Optional[bool]:
    if not isinstance(box_name, str) or not isinstance(method_name, str):
        return None

    row = lookup_thin_entry_selection_row(
        resolver=resolver,
        surface="user_box_method",
        subject=f"{box_name}.{method_name}",
        selection_value_id=selection_value_id,
    )
    if not isinstance(row, dict):
        return None
    if row.get("selected_entry") != "thin_internal_entry":
        return False
    return row.get("manifest_row") == "user_box_method.known_receiver"


def _receiver_box_type(resolver, box_vid: Optional[int]) -> Optional[str]:
    if not isinstance(box_vid, int):
        return None
    return _handle_box_type(resolver_value_type(resolver, int(box_vid)))


def _handle_box_type(meta: Any) -> Optional[str]:
    if isinstance(meta, dict) and meta.get("kind") == "handle":
        box_type = meta.get("box_type")
        if isinstance(box_type, str):
            return box_type
    return None

