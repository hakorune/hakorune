"""
DirectArrayI64 birth policy for LLVM Python lowering.

This module keeps the temporary exact-lane ArrayBox env gate and explicit
DirectArrayI64 constructor policy in one place until constructor route metadata
becomes the SSOT.
"""

import os

from utils.resolver_helpers import mark_arrayrepr_direct_i64

_SAFE_DIRECT_ARRAY_BIRTH_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)

DIRECT_ARRAY_I64_BIRTH_SYMBOL = "nyash.array.direct_i64.birth_h"
PUBLIC_ARRAY_BIRTH_SYMBOL = "nyash.array.birth_h"


def direct_array_i64_exact_lane_enabled() -> bool:
    return os.environ.get("HAKO_ARRAY_SLOT_STORE") == "direct_array_i64_exact"


def should_use_direct_array_i64_birth(box_type: str) -> bool:
    return box_type == "DirectArrayI64" or (
        box_type == "ArrayBox" and direct_array_i64_exact_lane_enabled()
    )


def mark_direct_array_i64_origin(resolver, dst_vid) -> None:
    if resolver is None or dst_vid is None:
        return
    try:
        if not hasattr(resolver, "direct_array_i64_ids"):
            resolver.direct_array_i64_ids = set()
        resolver.direct_array_i64_ids.add(int(dst_vid))
        mark_arrayrepr_direct_i64(resolver, int(dst_vid))
    except _SAFE_DIRECT_ARRAY_BIRTH_EXC:
        pass
