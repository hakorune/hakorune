"""
String-result policy helpers for LLVM-Py lowering.

This module keeps result-tagging separate from by-name invocation lowering so
callers can share the annotation policy without owning invoke mechanics.
"""

from typing import Optional

from instructions.mir_call.intrinsic_registry import produces_string_result


def mark_string_result_if_needed(resolver, dst_vid: Optional[int], method_name: Optional[str]) -> None:
    if dst_vid is None or resolver is None or not hasattr(resolver, "mark_string"):
        return
    if produces_string_result(method_name):
        resolver.mark_string(dst_vid)
