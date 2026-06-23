#!/usr/bin/env python3
"""Borrow-use classification for aggregate returned read borrows."""

from __future__ import annotations

from typing import Any

from mirbuilder_storage_access_facts import (
    ELIDE_TO_LEAF_PROJECTION,
    ELIDE_TO_READ_FOLD,
    FREEZE_OWNED,
    classify_storage_access,
    storage_access_from_borrow_use,
)

REQUIRE_READ_LEASE = "RequireReadLease"


def classify_borrow_use(use_fact: dict[str, Any]) -> str:
    """Classify one aggregate borrow use without naming a Rust family.

    The decision depends on how the borrow is consumed, not on the borrowed
    method name. Unsupported live aliases remain ReturnedReadBorrow details.
    """

    decision = classify_storage_access(storage_access_from_borrow_use(use_fact))
    if decision in {ELIDE_TO_LEAF_PROJECTION, ELIDE_TO_READ_FOLD, FREEZE_OWNED}:
        return decision
    return REQUIRE_READ_LEASE


def require_decision(use_fact: dict[str, Any], expected: str) -> None:
    decision = classify_borrow_use(use_fact)
    if decision != expected:
        raise ValueError(f"Deny(ReturnedReadBorrow): detail={decision}")
