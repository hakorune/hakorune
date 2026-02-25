"""
String FAST-path policy helpers (SSOT for LLVM Python lowering).
"""

import os
from typing import Any, Optional
import llvmlite.ir as ir


def llvm_fast_enabled() -> bool:
    return os.environ.get("NYASH_LLVM_FAST") == "1"


def string_const_boxer_symbol() -> str:
    return (
        "nyash.box.from_i8_string_const"
        if llvm_fast_enabled()
        else "nyash.box.from_i8_string"
    )


def can_reuse_literal_string_handle(
    resolver: Optional[Any],
    source_vid: Optional[int],
    source_value: Optional[ir.Value],
) -> bool:
    if not llvm_fast_enabled():
        return False
    if resolver is None or source_vid is None or source_value is None:
        return False
    literals = getattr(resolver, "string_literals", None)
    if not isinstance(literals, dict) or source_vid not in literals:
        return False
    return isinstance(source_value.type, ir.IntType) and source_value.type.width == 64


def literal_string_for_receiver(
    resolver: Optional[Any], receiver_vid: Optional[int]
) -> Optional[str]:
    if resolver is None or receiver_vid is None:
        return None
    literals = getattr(resolver, "string_literals", None)
    if not isinstance(literals, dict):
        return None
    try:
        recv_key = int(receiver_vid)
    except (TypeError, ValueError):
        return None

    direct = literals.get(recv_key)
    if isinstance(direct, str):
        return direct

    src_map = getattr(resolver, "newbox_string_args", None)
    if not isinstance(src_map, dict):
        return None
    arg_vid = src_map.get(recv_key)
    if arg_vid is None:
        return None
    try:
        mapped = literals.get(int(arg_vid))
    except (TypeError, ValueError):
        mapped = None
    return mapped if isinstance(mapped, str) else None
