#!/usr/bin/env python3
"""Freeze COPY-UNKNOWN0's LocalSSA post-success authority boundary."""

from __future__ import annotations

import sys
from pathlib import Path


TAG = "mirbuilder-copy-unknown-authority"
LOCAL = "src/mir/builder/ssa/local.rs"
POST_SUCCESS = "src/mir/builder/ssa/local/post_success.rs"
COPY_TYPE = "src/mir/builder/ssa/local/copy_type.rs"
SELF = "tools/checks/lib/mirbuilder_copy_unknown_authority_guard.py"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] ERROR: {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def production_source(source: str) -> str:
    return source.split("#[cfg(test)]", maxsplit=1)[0]


def require_count(source: str, needle: str, expected: int, label: str) -> None:
    actual = source.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def require_absent(source: str, needle: str, label: str) -> None:
    if needle in source:
        fail(f"{label}: forbidden token remains: {needle}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    local = production_source(read(root, LOCAL))
    post_success = production_source(read(root, POST_SUCCESS))
    copy_type = production_source(read(root, COPY_TYPE))

    require_count(
        local,
        "LocalSsaSourceTypeEntryV1::classify(",
        1,
        "source type-entry classifier consumer",
    )
    require_count(
        local,
        "PreparedLocalSsaPostSuccessV1::prepare(",
        1,
        "post-success decision owner",
    )
    require_count(
        local,
        "PreparedLocalSsaPhysicalCopyTypeV1::prepare(",
        1,
        "shared physical-Copy decision consumer",
    )
    require_count(
        local,
        "prepared_physical_copy_type.commit(",
        1,
        "shared physical-Copy commit consumer",
    )
    require_count(
        local,
        "prepared_post_success.commit(",
        1,
        "post-success commit consumer",
    )
    require_count(
        local,
        "fn try_materialize_local_v1(",
        1,
        "checked materialization entry",
    )
    require_count(
        local,
        "LocalSsaFailurePolicyV1::Checked",
        1,
        "checked policy declaration",
    )

    require_count(
        post_success,
        "pub(super) fn classify_materialization(",
        1,
        "physical materialization classifier owner",
    )
    require_count(
        post_success,
        "pub(super) fn commit(",
        1,
        "post-success commit owner",
    )
    require_count(
        post_success,
        "LocalSsaMaterializationKindV1::PhysicalCopy(_)",
        1,
        "physical-Copy exact-lane exclusion",
    )
    require_count(
        post_success,
        "SuppressedByStoredUnknown { owner: String }",
        1,
        "stored-Unknown receiver compatibility decision",
    )
    require_count(
        post_success,
        "PublishBoxFromMissingType { owner: String }",
        1,
        "receiver-only Box fallback decision",
    )
    require_count(
        copy_type,
        "TypeFactDecisionV1::prepare(",
        1,
        "physical-Copy exact decision owner",
    )
    require_count(
        copy_type,
        "pub(super) fn commit(",
        1,
        "physical-Copy exact commit owner",
    )

    for forbidden, label in (
        ("value_types.insert(loc", "direct LocalSSA destination type write"),
        (
            "value_origin_newbox\n                .insert(loc",
            "direct LocalSSA destination origin write",
        ),
        ("TypeFactDecisionV1", "premature COPY0 consumer"),
        ("metadata::propagate", "metadata propagation authority"),
    ):
        require_absent(local + post_success, forbidden, label)

    oversized = [
        relative
        for relative in (LOCAL, POST_SUCCESS, COPY_TYPE, SELF)
        if len(read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"[{TAG}] ok "
        "decision=1 commit=1 classifier=1 "
        "stored_unknown_lane=1 receiver_fallback_lane=1 "
        "physical_copy_decision=1 physical_copy_commit=1"
    )


if __name__ == "__main__":
    main()
