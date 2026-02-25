"""
Lightweight tracing helpers for the LLVM Python backend.

Environment flags (string '1' to enable):
- NYASH_CLI_VERBOSE: general lowering/debug logs
- NYASH_LLVM_TRACE_PHI: PHI resolution/snapshot wiring logs
- NYASH_LLVM_TRACE_VALUES: value resolution logs
- NYASH_LLVM_HOT_TRACE: perf-oriented hotspot summary logs

Import and use:
  from trace import debug, phi, values, hot
  debug("message")
  phi("phi message")
  values("values message")
  hot("[llvm/hot] ...")
"""

import os
import json
from typing import Mapping

from phi_wiring.debug_helper import is_phi_trace_enabled

_TRACE_OUT = os.environ.get('NYASH_LLVM_TRACE_OUT')

HOT_SUMMARY_FIELDS = (
    "binop_total",
    "binop_mod",
    "binop_expr_cache_hit",
    "binop_expr_cache_miss",
    "compare_total",
    "compare_keep_i1",
    "compare_to_i64",
    "compare_expr_cache_hit",
    "compare_expr_cache_miss",
    "call_total",
    "resolve_local_hit_binop",
    "resolve_global_hit_binop",
    "resolve_fallback_binop",
    "resolve_local_hit_compare",
    "resolve_global_hit_compare",
    "resolve_fallback_compare",
    "resolve_local_hit_call",
    "resolve_global_hit_call",
    "resolve_fallback_call",
)

def _write(msg: str) -> None:
    if _TRACE_OUT:
        try:
            with open(_TRACE_OUT, 'a', encoding='utf-8') as f:
                f.write(msg.rstrip() + "\n")
            return
        except Exception:
            pass
    try:
        print(msg, flush=True)
    except Exception:
        pass

def _enabled(env_key: str) -> bool:
    return os.environ.get(env_key) == '1'

def debug(msg: str) -> None:
    if _enabled('NYASH_CLI_VERBOSE'):
        _write(msg)

def phi(msg) -> None:
    if is_phi_trace_enabled():
        # Accept raw strings or arbitrary objects; non-strings are JSON-encoded
        if not isinstance(msg, (str, bytes)):
            try:
                msg = json.dumps(msg, ensure_ascii=False, separators=(",", ":"))
            except Exception:
                msg = str(msg)
        _write(msg)

def values(msg: str) -> None:
    if _enabled('NYASH_LLVM_TRACE_VALUES'):
        _write(msg)


def hot_enabled() -> bool:
    return _enabled('NYASH_LLVM_HOT_TRACE')


def hot(msg: str) -> None:
    if hot_enabled():
        _write(msg)


def hot_count(resolver, key: str, inc: int = 1) -> None:
    """Increment function-local LLVM hot-trace counter if available."""
    if resolver is None:
        return
    try:
        ctx = getattr(resolver, "context", None)
        counts = getattr(ctx, "hot_trace_counts", None) if ctx is not None else None
        if isinstance(counts, dict):
            counts[key] = int(counts.get(key, 0)) + int(inc)
    except Exception:
        pass


def format_hot_summary(func_name: str, counts: Mapping[str, int]) -> str:
    parts = [f"[llvm/hot] fn={func_name}"]
    for key in HOT_SUMMARY_FIELDS:
        parts.append(f"{key}={int(counts.get(key, 0))}")
    return " ".join(parts)


def phi_json(msg):
    """Safe JSON-style PHI trace delegator.

    - Gated by NYASH_LLVM_DEBUG_PHI_TRACE=1 (same gate as phi())
    - Delegates to phi_wiring.common.trace if available; otherwise no-op
    - Accepts arbitrary Python objects and forwards as-is
    """
    if not is_phi_trace_enabled():
        return
    try:
        from phi_wiring.common import trace as _trace_phi_json  # type: ignore
        _trace_phi_json(msg)
    except Exception:
        # Fallback: stringify and route via plain phi
        try:
            if not isinstance(msg, (str, bytes)):
                try:
                    msg = json.dumps(msg, ensure_ascii=False, separators=(",", ":"))
                except Exception:
                    msg = str(msg)
            phi(msg)
        except Exception:
            pass
