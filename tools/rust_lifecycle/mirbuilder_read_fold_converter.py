#!/usr/bin/env python3
"""Generic direct lowering for owned read-fold map merges."""

from __future__ import annotations

from typing import Any

from mirbuilder_storage_access_facts import (
    ELIDE_TO_READ_FOLD,
    classify_storage_access,
    storage_access_from_borrow_use,
)
from verified_hako_family_ir import HakoMethodIR, op


def compile_owned_map_merge_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    fold_fact_id: str,
    plan_ref: str,
    signature: str,
    source_arg: str,
    source_storage: str,
    base_arg: str,
    target_storage: str,
    target_name: str,
    key_binding: str,
    value_binding: str,
) -> list[HakoMethodIR]:
    """Compile `HashMap` read fold into a cloned owned ordered map."""

    borrow_use = {row["id"]: row for row in facts.get("borrow_use_facts", [])}
    fold_fact = borrow_use.get(fold_fact_id)
    if fold_fact is None:
        raise ValueError("Deny(UnsupportedDirectShape): missing read-fold borrow-use fact")
    access_fact = storage_access_from_borrow_use(fold_fact)
    if classify_storage_access(access_fact) != ELIDE_TO_READ_FOLD:
        raise ValueError("Deny(ReturnedReadBorrow): detail=ReadFoldNotAccepted")
    if access_fact.get("order") != "Unobserved":
        raise ValueError("Deny(UnsupportedOrderCapability): source order must be unobserved")
    if access_fact.get("element_reference_escapes") is True:
        raise ValueError("Deny(ReturnedReadBorrow): detail=ElementReferenceEscapes")
    if access_fact.get("owner_mutated_during_use") is True:
        raise ValueError("Deny(ReturnedReadBorrow): detail=OwnerMutationDuringBorrow")

    fold_semantics = fold_fact.get("fold_semantics")
    if not isinstance(fold_semantics, dict):
        raise ValueError("Deny(DefaultSemanticMismatch): detail=MissingFoldSemantics")
    required = {
        "input": "MapEntries",
        "key_projection": "Copy(ValueIdAsI64)",
        "value_projection": "OwnedImmutableAtom",
        "base": "CloneOwned",
        "collision": "SourceWins",
        "output": "OwnedOrderedMap",
        "output_order": "KeyAscending(ValueIdOrdV1)",
    }
    for key, expected in required.items():
        if fold_semantics.get(key) != expected:
            raise ValueError(f"Deny(DefaultSemanticMismatch): detail={key}")
    for proof_key in [
        "source_destination_alias",
        "source_mutated_during_use",
        "element_reference_escapes",
        "destination_identity_observed",
    ]:
        if fold_semantics.get(proof_key) is not False:
            raise ValueError(f"Deny(CarrierSensitiveAlias): detail={proof_key}")

    plan_entries = {row["id"]: row for row in plan.get("plans", [])}
    read_fold_plan = plan_entries.get(plan_ref)
    if read_fold_plan is None or read_fold_plan.get("shape_rule") != "borrow.read_fold":
        raise ValueError("Deny(UnsupportedDirectShape): expected borrow.read_fold plan")
    if read_fold_plan.get("comparator_proof") != "ValueIdOrdV1":
        raise ValueError("Deny(UnsupportedOrderCapability): missing ValueIdOrdV1 proof")
    if source_storage not in {"MapBox", "OrderedMapBox", "ValueIdOrderedMapBox"}:
        raise ValueError("Deny(UnsupportedTypeTransport): unsupported read-fold source storage")
    if target_storage not in {"OrderedMapBox", "ValueIdOrderedMapBox"}:
        raise ValueError("Deny(UnsupportedTypeTransport): unsupported read-fold target storage")

    return [
        HakoMethodIR(
            signature=signature,
            operations=[
                op("CloneOwnedMap", source=base_arg, target=target_name, target_storage=target_storage),
                op(
                    "ForEachMapEntry",
                    source=source_arg,
                    source_storage=source_storage,
                    key_binding=key_binding,
                    value_binding=value_binding,
                    order="Unobserved",
                    body=[
                        op(
                            "MapSet",
                            source=target_name,
                            key=key_binding,
                            value=value_binding,
                            storage=target_storage,
                        ).to_json()
                    ],
                ),
                op("ReturnSource", source=target_name),
            ],
        )
    ]
