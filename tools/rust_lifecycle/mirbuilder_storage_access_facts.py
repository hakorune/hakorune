#!/usr/bin/env python3
"""Language-neutral storage access facts for MirBuilder lowering."""

from __future__ import annotations

from typing import Any


ELIDE_TO_LEAF_PROJECTION = "ElideToLeafProjection"
ELIDE_TO_READ_FOLD = "ElideToReadFold"
FREEZE_OWNED = "FreezeOwned"
KEEP_SHARED_HANDLE = "KeepSharedHandle"
MATERIALIZE_SHARED_CELL = "MaterializeSharedCell"
MATERIALIZE_SPAN = "MaterializeSpan"
REQUIRE_UNSAFE_CAPABILITY = "RequireUnsafeCapability"
DENY = "Deny"


def storage_access_from_borrow_use(use_fact: dict[str, Any]) -> dict[str, Any]:
    """Normalize Rust borrow-use facts into source-neutral access facts."""

    if use_fact.get("borrowed_kind") != "Aggregate":
        raise ValueError("Deny(UnsupportedDirectShape): expected aggregate borrow")

    return {
        "carrier": "SharedHandle",
        "access": "Read",
        "alias": "Shared",
        "lifetime": "Lexical",
        "escape": "None" if use_fact.get("escapes") is False else "Unknown",
        "order": use_fact.get("order", "Unobserved"),
        "cleanup": "Trivial",
        "consumer_kind": use_fact.get("consumer_kind"),
        "identity_observed": use_fact.get("identity_observed") is True,
        "owner_mutated_during_use": use_fact.get("owner_mutated_during_use") is True,
        "element_reference_escapes": use_fact.get("element_reference_escapes") is True,
        "owned_projection_available": use_fact.get("owned_projection_available") is True,
    }


def classify_storage_access(access_fact: dict[str, Any]) -> str:
    """Classify a normalized storage access without source-language names."""

    if access_fact.get("carrier") == "RawAddress":
        return REQUIRE_UNSAFE_CAPABILITY
    if access_fact.get("escape") != "None":
        return DENY
    if access_fact.get("identity_observed") is True:
        return DENY
    if access_fact.get("owner_mutated_during_use") is True:
        return DENY

    consumer = access_fact.get("consumer_kind")
    if consumer in {"GetCopy", "GetClone", "LastCopy"} and access_fact.get("owned_projection_available") is True:
        return ELIDE_TO_LEAF_PROJECTION
    if consumer == "ReadOnlyFold" and access_fact.get("element_reference_escapes") is not True:
        return ELIDE_TO_READ_FOLD
    if access_fact.get("owned_projection_available") is True:
        return FREEZE_OWNED
    if access_fact.get("access") == "ReadWrite" and access_fact.get("alias") == "Shared":
        return MATERIALIZE_SHARED_CELL
    if access_fact.get("carrier") == "Span":
        return MATERIALIZE_SPAN
    if access_fact.get("carrier") == "SharedHandle":
        return KEEP_SHARED_HANDLE
    return DENY


def require_storage_decision(access_fact: dict[str, Any], expected: str) -> None:
    decision = classify_storage_access(access_fact)
    if decision != expected:
        raise ValueError(f"Deny(ReturnedReadBorrow): detail={decision}")
