"""Generic manifest evidence checks for active MIRBuilder R4 fences."""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
from typing import Any


FENCE_KINDS = frozenset({"operation", "dev-mutation", "runtime-bridge", "transport", "dependency"})
ACTIVE_FENCES = frozenset(
    {
        "RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001",
        "JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0",
        "VM-BRIDGE-COMPAT-SUNSET-001",
        "RAW-RECURSIVE-UNLOCATED-TRANSPORT-SUNSET-001",
        "RAW-LAMBDA-CHILD-OWNER-SOURCE-LINEAGE-SUNSET-001",
        "RAW-LOCATED-LOOP-ROUTE-SOURCE-HANDOFF-SUNSET-001",
        "JOINMODULE-SHARED-REFERENCE-SUBSTRATE-SUNSET-001",
    }
)


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
