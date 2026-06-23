#!/usr/bin/env python3
"""Borrow-use classification for aggregate returned read borrows."""

from __future__ import annotations

from typing import Any


ELIDE_TO_LEAF_PROJECTION = "ElideToLeafProjection"
ELIDE_TO_READ_FOLD = "ElideToReadFold"
FREEZE_OWNED = "FreezeOwned"
REQUIRE_READ_LEASE = "RequireReadLease"


def classify_borrow_use(use_fact: dict[str, Any]) -> str:
    """Classify one aggregate borrow use without naming a Rust family.

    The decision depends on how the borrow is consumed, not on the borrowed
    method name. Unsupported live aliases remain ReturnedReadBorrow details.
    """

    if use_fact.get("borrowed_kind") != "Aggregate":
        raise ValueError("Deny(UnsupportedDirectShape): expected aggregate borrow")
    if use_fact.get("escapes") is not False:
        return REQUIRE_READ_LEASE
    if use_fact.get("identity_observed") is True:
        return REQUIRE_READ_LEASE
    if use_fact.get("owner_mutated_during_use") is True:
        return REQUIRE_READ_LEASE

    consumer = use_fact.get("consumer_kind")
    if consumer in {"GetCopy", "GetClone", "LastCopy"} and use_fact.get("owned_projection_available") is True:
        return ELIDE_TO_LEAF_PROJECTION
    if consumer == "ReadOnlyFold" and use_fact.get("element_reference_escapes") is False:
        return ELIDE_TO_READ_FOLD
    if use_fact.get("owned_projection_available") is True and use_fact.get("identity_observed") is not True:
        return FREEZE_OWNED
    return REQUIRE_READ_LEASE


def require_decision(use_fact: dict[str, Any], expected: str) -> None:
    decision = classify_borrow_use(use_fact)
    if decision != expected:
        raise ValueError(f"Deny(ReturnedReadBorrow): detail={decision}")
