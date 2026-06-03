from __future__ import annotations
from typing import Any
import os
import json

from .debug_helper import is_phi_trace_enabled

_SAFE_PHI_TRACE_EXC = (AttributeError, OSError, RuntimeError, TypeError, ValueError)

def _stringify_trace_msg(msg: Any) -> str:
    if isinstance(msg, (str, bytes)):
        return msg if isinstance(msg, str) else msg.decode(errors="replace")
    try:
        return json.dumps(msg, ensure_ascii=False, separators=(",", ":"))
    except _SAFE_PHI_TRACE_EXC:
        return str(msg)

def _append_trace_line(path: str, msg: str) -> None:
    try:
        with open(path, "a", encoding="utf-8") as f:
            f.write(msg.rstrip() + "\n")
    except _SAFE_PHI_TRACE_EXC:
        pass

def trace(msg: Any):
    if not is_phi_trace_enabled():
        return
    out = os.environ.get("NYASH_LLVM_TRACE_OUT")
    msg = _stringify_trace_msg(msg)
    if out:
        _append_trace_line(out, msg)
    else:
        try:
            print(msg)
        except _SAFE_PHI_TRACE_EXC:
            pass
