"""Shared PageModel field bucket vocabulary for allocator evidence tools."""

from __future__ import annotations

import re


FIELD_HINT_RE = re.compile(r"0x[0-9a-fA-F]+:(?P<field>[A-Za-z_][A-Za-z0-9_]*)")

FIELD_BUCKETS: dict[str, str] = {
    "used": "primitive_hot_state",
    "free_top": "primitive_hot_state",
    "local_free_top": "primitive_hot_state",
    "retired": "primitive_hot_state",
    "decommitted": "primitive_hot_state",
    "peak_used": "primitive_hot_state",
    "page_id": "public_semantics",
    "block_size": "public_semantics",
    "capacity": "public_semantics",
    "reserved": "public_semantics",
    "requested_bytes": "public_semantics_proof_evidence",
    "alloc_count": "observer_counter",
    "local_free_count": "observer_counter",
    "release_count": "observer_counter",
    "reject_count": "observer_counter",
    "retire_count": "observer_counter",
    "reactivate_count": "observer_counter",
    "free": "direct_array_owner",
    "local_free": "direct_array_owner",
    "block_used": "direct_array_owner",
}


def bucket_for_field(field: str) -> str:
    return FIELD_BUCKETS.get(field, "unknown")


def fields_from_hint(value: str) -> list[str]:
    fields: list[str] = []
    for match in FIELD_HINT_RE.finditer(value):
        name = match.group("field")
        if name not in fields:
            fields.append(name)
    return fields


def fields_from_context(value: str) -> list[str]:
    if not value or value in {"none", "not_found"}:
        return []
    return fields_from_hint(value)


def format_field_buckets(fields: list[str]) -> str:
    if not fields:
        return "none"
    return ",".join(f"{field}:{bucket_for_field(field)}" for field in fields)
