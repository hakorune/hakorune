"""Close the V0 preserved/split provenance ownership law."""

from __future__ import annotations

from collections import Counter, defaultdict
from typing import Any


REASON_CONTRACTS = {
    "selected_pinned_text_lowerer": {
        "base_block": ("preserved", {"block"}),
        "jump": ("preserved", {"edge"}),
        "branch": ("preserved", {"edge"}),
        "residence_enter": ("preserved", {"edge"}),
        "utf8_width_at": ("split", {"block", "edge"}),
        "scalar_eq": ("split", {"block", "edge"}),
        "direct_continuation": ("split", {"edge"}),
    },
    "selected_dynamic_c1_lowerer": {
        "base_block": ("preserved", {"block"}),
        "jump": ("preserved", {"edge"}),
        "branch": ("preserved", {"edge"}),
        "checked_callout": ("split", {"block", "edge"}),
        "checked_callout_end": ("split", {"block", "edge"}),
    },
}


def validate_disposition_closure(
    rows: list[dict[str, Any]], *, issuer: str,
    mir_blocks: set[int], mir_sites: set[tuple[int, int]],
    mir_edges: set[tuple[int, int, str, int]],
    llvm_blocks: set[str], llvm_edges: set[tuple[str, str]],
) -> None:
    contracts = REASON_CONTRACTS.get(issuer)
    if contracts is None:
        raise SystemExit("provenance disposition issuer is unsupported")

    mir_block_owners: Counter[int] = Counter()
    mir_edge_owners: Counter[tuple[int, int, str, int]] = Counter()
    llvm_block_owners: Counter[str] = Counter()
    llvm_edge_owners: Counter[tuple[str, str]] = Counter()
    split_cohorts: dict[tuple[str, int, int, str], list[dict[str, Any]]] = (
        defaultdict(list)
    )

    for row in rows:
        if row.get("issuer") != issuer:
            raise SystemExit("provenance disposition issuer mismatch")
        entity = row["entity"]
        disposition = row["disposition"]
        reason = row["reason_kind"]
        mir = row["mir"]
        llvm = row["llvm"]
        contract = contracts.get(reason)
        if contract is None:
            raise SystemExit("provenance disposition reason is unsupported")
        expected_disposition, entities = contract
        if disposition != expected_disposition or entity not in entities:
            raise SystemExit("provenance disposition reason contract mismatch")

        source = (mir["block"], mir["instruction"], mir["arm"], mir["target"])
        if disposition == "preserved":
            if entity == "block":
                if source != (mir["block"], -1, "none", -1):
                    raise SystemExit("provenance preserved block source mismatch")
                mir_block_owners[mir["block"]] += 1
            else:
                if source not in mir_edges:
                    raise SystemExit("provenance preserved edge source mismatch")
                mir_edge_owners[source] += 1
        else:
            site = (mir["block"], mir["instruction"])
            if site not in mir_sites:
                raise SystemExit("provenance split site mismatch")
            if entity == "block":
                if (mir["arm"], mir["target"]) != ("none", -1):
                    raise SystemExit("provenance split block source mismatch")
            elif mir["arm"] == "none":
                if mir["target"] != -1:
                    raise SystemExit("provenance split internal edge mismatch")
            elif source in mir_edges:
                mir_edge_owners[source] += 1
            else:
                raise SystemExit("provenance split edge source mismatch")
            split_cohorts[(entity, mir["block"], mir["instruction"], reason)].append(row)

        if entity == "block":
            llvm_block_owners[llvm["from"]] += 1
        else:
            llvm_edge_owners[(llvm["from"], llvm["to"])] += 1

    if any(len(cohort) < 2 for cohort in split_cohorts.values()):
        raise SystemExit("provenance split cohort is singleton")
    if set(mir_block_owners) != mir_blocks or any(
        count != 1 for count in mir_block_owners.values()
    ):
        raise SystemExit("provenance MIR block ownership mismatch")
    if set(mir_edge_owners) != mir_edges or any(
        count != 1 for count in mir_edge_owners.values()
    ):
        raise SystemExit("provenance MIR edge ownership mismatch")
    if set(llvm_block_owners) != llvm_blocks or any(
        count != 1 for count in llvm_block_owners.values()
    ):
        raise SystemExit("provenance LLVM block ownership mismatch")
    if set(llvm_edge_owners) != llvm_edges or any(
        count != 1 for count in llvm_edge_owners.values()
    ):
        raise SystemExit("provenance LLVM edge ownership mismatch")
