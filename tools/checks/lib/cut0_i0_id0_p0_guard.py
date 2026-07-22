#!/usr/bin/env python3
"""CUT0-I0-ID0-P0 disconnected invocation-brand guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
SRC = ROOT / "src/mir/builder"
BUILDER = SRC / ".." / "builder.rs"
CHAIN = SRC / "module_invocation_owner_chain.rs"
FIXTURE = SRC / "module_invocation_brand_p0.rs"
COLLECTION = SRC / "module_invocation_collection.rs"
COLLECT_FIXTURE = SRC / "module_invocation_collect0_s0_p0.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state_path = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
    state = state_path.read_text()
    task = TASK.read_text()
    chain = CHAIN.read_text()
    fixture = FIXTURE.read_text()
    builder = BUILDER.resolve().read_text()

    for path in (CHAIN, FIXTURE, COLLECTION, COLLECT_FIXTURE, pathlib.Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"ID0-P0 file must remain below 800 lines: {path}")

    require(state, "CUT0-I0-ID0-P0 is closed as a disconnected branded owner-chain proof", "row closeout")
    require(state, "CUT0-I0-COLLECT0-S0 is closed as a disconnected raw/canonical co-seal proof", "successor closeout")
    require(state, "CUT0-I0-COLLECT0-BATCH0 is next", "next pointer")
    require(task, "CUT0-I0-ID0-P0 — closed", "task row")
    require(task, "foreign shell + collector", "foreign owner fixture")
    require(task, "source proof + collector co-seal", "co-seal acceptance")

    for fragment, label in (
        ("ModuleBuilderInvocationSessionV1", "Builder session brand"),
        ("InvocationDraftSourceProofV1", "source proof"),
        ("InvocationBrandedReceiptV1", "receipt brand"),
        ("CollectedInvocationDraftSetV1", "collected draft set"),
        ("PreparedModuleExternalCommitV1", "prepared commit"),
        ("ForeignOwner", "foreign owner error"),
        ("ReceiptKindMismatch", "receipt family check"),
        ("advance_to_prepared_commit", "same-brand terminal"),
    ):
        require(chain, fragment, label)
    for fragment, label in (
        ("one_source_brand_survives_session_collection_and_prepared_commit", "happy path"),
        ("source_and_collector_foreign_pair_fails_before_co_seal", "foreign pair"),
        ("foreign_receipt_and_wrong_kind_fail_before_co_seal", "foreign receipt"),
    ):
        require(fixture, fragment, label)
    require(builder, "mod module_invocation_owner_chain;", "chain registration")
    require(builder, "mod module_invocation_brand_p0;", "fixture registration")

    allowed = {
        CHAIN.relative_to(ROOT),
        FIXTURE.relative_to(ROOT),
        COLLECTION.relative_to(ROOT),
        COLLECT_FIXTURE.relative_to(ROOT),
        BUILDER.resolve().relative_to(ROOT),
    }
    forbidden = (
        "ModuleBuilderInvocationSessionV1",
        "InvocationDraftSourceProofV1",
        "PreparedModuleExternalCommitV1",
    )
    consumers = []
    for path in ROOT.glob("src/**/*.rs"):
        if path.relative_to(ROOT) in allowed:
            continue
        text = path.read_text()
        for fragment in forbidden:
            if fragment in text:
                consumers.append(f"{path.relative_to(ROOT)}:{fragment}")
    if consumers:
        raise AssertionError("ID0-P0 production consumers: " + ", ".join(consumers))

    print("[cut0-i0-id0-p0-guard] ok brand_chain=8 stages production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
