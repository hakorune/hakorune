"""
extern_normalize.py — Single point of truth for extern name normalization.

Policy (MVP):
- Map bare "print"/"println" → "nyash.console.log"
- Map "env.console.*" (println/log/print/warn/error) → "nyash.console.<method>"
  * println is normalized to log (pointer API).
- Keep already-qualified "nyash.console.*" as-is, but normalize ...println → ...log

This module is imported by both instructions.externcall and instructions.mir_call
to avoid duplication and drift.
"""

from typing import Optional


def normalize_extern_name(name: Optional[str]) -> str:
    if not name:
        return ""
    try:
        n = str(name)
    except Exception:
        return ""

    try:
        if n.startswith("env.console."):
            method = n.split(".")[-1]
            if method == "println":
                method = "log"
            return f"nyash.console.{method}"
        if n in ("println", "print"):
            return "nyash.console.log"
        if n.startswith("nyash.console.") and n.endswith("println"):
            return "nyash.console.log"
        # Future helpers (LLVM harness)
        # Keep env.* as language-facing name, but map to nyash.* runtime exports.
        if n == "env.future.delay":
            return "nyash.future.delay_i64"
        if n == "env.get":
            return "nyash.env.get"
        if n == "env.set":
            return "nyash.env.set"
    except Exception:
        # Fallthrough to original if anything odd happens
        pass
    return n
