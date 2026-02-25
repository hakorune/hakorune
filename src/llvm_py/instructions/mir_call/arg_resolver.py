"""
Shared argument resolver helpers for MIR Call lowering.

SSOT:
- Prefer local vmap value first (same-block SSA).
- Fallback to resolve_i64_strict with call hot scope.
- Never raise from this helper; unresolved values return None.
"""

from typing import Any, Callable, Dict, Optional

from utils.values import resolve_i64_strict


def _owner_map(owner: Any, key: str):
    if owner is None:
        return None
    return getattr(owner, key, None)


def resolve_call_arg(
    vid: Any,
    builder: Any,
    vmap: Dict[int, Any],
    resolver: Any,
    owner: Any = None,
    *,
    preds: Optional[Dict[int, list]] = None,
    block_end_values: Optional[Dict[int, Dict[int, Any]]] = None,
    bb_map: Optional[Dict[int, Any]] = None,
    hot_scope: str = "call",
) -> Any:
    """Resolve call argument with local-first + strict resolver fallback."""
    try:
        local = vmap.get(vid)
        if local is not None:
            return local
    except (AttributeError, TypeError):
        pass

    if resolver is not None and hasattr(resolver, "resolve_i64"):
        try:
            return resolve_i64_strict(
                resolver,
                vid,
                builder.block,
                preds if preds is not None else _owner_map(owner, "preds"),
                block_end_values if block_end_values is not None else _owner_map(owner, "block_end_values"),
                vmap,
                bb_map if bb_map is not None else _owner_map(owner, "bb_map"),
                hot_scope=hot_scope,
            )
        except (AttributeError, TypeError, ValueError, KeyError):
            pass

    try:
        return vmap.get(vid)
    except (AttributeError, TypeError):
        return None


def make_call_arg_resolver(
    builder: Any,
    vmap: Dict[int, Any],
    resolver: Any,
    owner: Any = None,
    *,
    preds: Optional[Dict[int, list]] = None,
    block_end_values: Optional[Dict[int, Dict[int, Any]]] = None,
    bb_map: Optional[Dict[int, Any]] = None,
    hot_scope: str = "call",
) -> Callable[[Any], Any]:
    """Create a route-local argument resolver bound to the shared contract."""

    def _resolve_arg(vid: Any) -> Any:
        return resolve_call_arg(
            vid,
            builder,
            vmap,
            resolver,
            owner,
            preds=preds,
            block_end_values=block_end_values,
            bb_map=bb_map,
            hot_scope=hot_scope,
        )

    return _resolve_arg
