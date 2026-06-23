#!/usr/bin/env python3
"""Generic ordered read-fold compiler checks for MirBuilder converter slices."""

from __future__ import annotations

from typing import Any


def _rows_by_id(facts: dict[str, Any], key: str) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get(key, [])}


def compile_ordered_read_fold(facts: dict[str, Any], plan: dict[str, Any]) -> list[dict[str, Any]]:
    """Compile a read-only fold only when its ordering capability is proven."""

    borrow = _rows_by_id(facts, "borrow_use_facts").get(plan.get("borrow_use_id"))
    if borrow is None:
        raise ValueError("Deny(UnsupportedDirectShape): missing borrow-use fact")
    if borrow.get("consumer_kind") != "ReadOnlyFold":
        raise ValueError("Deny(UnsupportedDirectShape): expected read-only fold")
    if borrow.get("escapes") is not False or borrow.get("element_reference_escapes") is not False:
        raise ValueError("Deny(ReturnedReadBorrow): detail=BorrowEscapes")
    if borrow.get("owner_mutated_during_use") is not False:
        raise ValueError("Deny(ReturnedReadBorrow): detail=OwnerMutationDuringBorrow")

    order = borrow.get("order")
    if order == "SourceOrdered" and plan.get("source_order_proof") != "ExeAotAccepted":
        raise ValueError("Deny(UnsupportedKeyTransport): detail=SourceOrderedStringKeyCompareUnavailable")
    if order not in {"Unobserved", "Unspecified", "SourceOrdered"}:
        raise ValueError("Deny(UnsupportedDirectShape): unsupported order fact")

    return [
        {
            "kind": "ReadFoldOwnedOutput",
            "source": plan.get("source"),
            "destination": plan.get("destination"),
            "order": order,
        }
    ]
