from __future__ import annotations
from typing import Any
import os
import json

from .debug_helper import is_phi_trace_enabled

def trace(msg: Any):
    if not is_phi_trace_enabled():
        return
    out = os.environ.get("NYASH_LLVM_TRACE_OUT")
    if not isinstance(msg, (str, bytes)):
        try:
            msg = json.dumps(msg, ensure_ascii=False, separators=(",", ":"))
        except Exception:
            msg = str(msg)
    if out:
        try:
            with open(out, "a", encoding="utf-8") as f:
                f.write(msg.rstrip() + "\n")
        except Exception:
            pass
    else:
        try:
            print(msg)
        except Exception:
            pass

