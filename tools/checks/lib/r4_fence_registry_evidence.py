"""Generic manifest evidence checks for active MIRBuilder R4 fences."""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
from typing import Any


FENCE_KINDS = frozenset({"operation", "dev-mutation", "transport"})
ACTIVE_FENCES = frozenset(
    {
        "RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001",
        "JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0",
        "RAW-RECURSIVE-UNLOCATED-TRANSPORT-SUNSET-001",
        "RAW-LAMBDA-CHILD-OWNER-SOURCE-LINEAGE-SUNSET-001",
        "RAW-LOCATED-LOOP-ROUTE-SOURCE-HANDOFF-SUNSET-001",
    }
)
NORMALIZED_SHADOW_FAMILIES = {
    "tail_break": "gap",
    "if_continue_none": "overlap",
    "if_break_break": "overlap",
    "if_break_continue": "overlap",
    "if_continue_break": "overlap",
    "if_continue_continue": "overlap",
}


def validate_r4_fence_registry(
    root: Path, caller_manifest: dict[str, Any], require: Callable[[str, str, str], None]
) -> None:
    """Require one manifest-owned row per active fence, without route claims."""
    fences = caller_manifest["r4_fences"]
    if set(fences) != ACTIVE_FENCES:
        raise AssertionError("active R4 fence registry drift")
    for fence_id, fence in fences.items():
        if fence["state"] not in {"retain-fenced", "active-compatibility"}:
            raise AssertionError(f"{fence_id} R4 state drift")
        if fence["kind"] not in FENCE_KINDS:
            raise AssertionError(f"{fence_id} R4 kind drift")
        if not fence["exact_surface"] or not fence["release_condition"]:
            raise AssertionError(f"{fence_id} R4 contract incomplete")
        owners = fence["owners"]
        if not owners:
            raise AssertionError(f"{fence_id} R4 owners missing")
        for owner in owners:
            require((root / owner["path"]).read_text(), owner["anchor"], f"{fence_id} owner")
        evidence = fence["evidence"]
        if not evidence:
            raise AssertionError(f"{fence_id} R4 evidence missing")
        for item in evidence:
            if item["kind"] not in {"source-anchor", "fixture-anchor", "guard-anchor"}:
                raise AssertionError(f"{fence_id} R4 evidence kind drift")
            require((root / item["path"]).read_text(), item["anchor"], f"{fence_id} evidence")
        dependencies = fence.get("depends_on", [])
        if fence["kind"] == "dependency" and not dependencies:
            raise AssertionError(f"{fence_id} dependency targets missing")
        if fence["kind"] != "dependency" and dependencies:
            raise AssertionError(f"{fence_id} unexpected dependency targets")
        if not set(dependencies) <= set(fences):
            raise AssertionError(f"{fence_id} dependency target drift")

    normalized = fences["JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0"][
        "normalization_family_coverage"
    ]
    if set(normalized["entries"]) != {"direct_loop", "block_suffix"}:
        raise AssertionError("normalized-shadow mutation entry inventory drift")
    for entry_id, entry in normalized["entries"].items():
        require(
            (root / entry["path"]).read_text(),
            entry["anchor"],
            f"normalized-shadow {entry_id} entry",
        )
    retry = normalized["retry_edge"]
    require(
        (root / retry["path"]).read_text(),
        retry["anchor"],
        "normalized-shadow suffix-to-direct retry edge",
    )
    fixture = normalized["fixture"]
    require(
        (root / fixture["path"]).read_text(),
        fixture["anchor"],
        "normalized-shadow grammar-family fixture",
    )
    families = normalized["families"]
    if {family_id: row["status"] for family_id, row in families.items()} != (
        NORMALIZED_SHADOW_FAMILIES
    ):
        raise AssertionError("normalized-shadow grammar family/status drift")
    route_registry = (
        root / "src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs"
    ).read_text()
    for family_id, row in families.items():
        if not row["grammar"] or not row["ordinary_candidates"]:
            raise AssertionError(f"{family_id} normalized-shadow domain incomplete")
        executor = row["executor"]
        require(
            (root / executor["path"]).read_text(),
            executor["anchor"],
            f"{family_id} normalized-shadow executor",
        )
        for candidate in row["ordinary_candidates"]:
            require(
                route_registry,
                candidate,
                f"{family_id} ordinary route candidate",
            )
