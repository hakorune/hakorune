#!/usr/bin/env python3
"""Ordering capability facts for MirBuilder converter routes."""

from __future__ import annotations

from typing import Any


RUST_STRING_ORD_V1 = "RustStringOrdV1"
REQUIRED_ORDER_TIERS = ("VM", "EXE", "AOT")


def key_ascending(comparator: str) -> dict[str, str]:
    return {"kind": "KeyAscending", "comparator": comparator}


def order_kind(order: Any) -> str:
    if isinstance(order, dict):
        return str(order.get("kind", ""))
    if isinstance(order, str):
        return order
    return ""


def order_comparator(order: Any) -> str | None:
    if isinstance(order, dict) and order.get("kind") in {"KeyAscending", "KeyDescending"}:
        comparator = order.get("comparator")
        return str(comparator) if comparator else None
    return None


def require_order_capability(order: Any, plan: dict[str, Any]) -> None:
    """Fail unless the plan proves the comparator needed by an ordered fold."""

    kind = order_kind(order)
    if kind in {"Unobserved", "Unspecified", "Insertion"}:
        return
    if kind not in {"KeyAscending", "KeyDescending"}:
        raise ValueError("Deny(UnsupportedDirectShape): unsupported order fact")

    comparator = order_comparator(order)
    if comparator is None:
        raise ValueError("Deny(UnsupportedOrderCapability): detail=ComparatorUnavailable")

    capability = plan.get("comparator_capabilities", {}).get(comparator, {})
    if capability.get("proof") == "VmExeAotAccepted":
        return

    tiers = ",".join(REQUIRED_ORDER_TIERS)
    raise ValueError(
        "Deny(UnsupportedOrderCapability): "
        f"detail=ComparatorUnavailable comparator={comparator} required_tiers={tiers}"
    )
