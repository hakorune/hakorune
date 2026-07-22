#!/usr/bin/env python3
"""CUT0-I0-COLLECT0-S0 disconnected raw/canonical co-seal guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
SRC = ROOT / "src/mir/builder"
BUILDER = SRC / ".." / "builder.rs"
COLLECTION = SRC / "module_invocation_collection.rs"
FIXTURE = SRC / "module_invocation_collect0_s0_p0.rs"
OWNER_CHAIN = SRC / "module_invocation_owner_chain.rs"
COLLECTOR = SRC / "module_draft_collector.rs"
RAW_LEDGER = SRC / "raw_expansion_receipt_ledger.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    collection = COLLECTION.read_text()
    fixture = FIXTURE.read_text()
    builder = BUILDER.resolve().read_text()

    for path in (COLLECTION, FIXTURE, pathlib.Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"COLLECT0-S0 file must remain below 800 lines: {path}")

    require(state, "CUT0-I0-COLLECT0-S0 is closed as a disconnected raw/canonical co-seal proof", "row closeout")
    require(state, "CUT0-I0-COLLECT0-BATCH0 is closed as a disconnected atomic callable-batch proof", "successor closeout")
    require(state, "CUT0-I0-SESSION0 is closed as a disconnected Builder transaction", "session closeout")
    require(state, "CUT0-I0-ROOT0-D0 is a design stop before ROOT0 implementation", "ROOT0 design stop")
    require(task, "CUT0-I0-COLLECT0-S0 — closed", "task row")
    require(task, "Raw: final ledger rows", "raw acceptance")
    require(task, "Canonical-single: exact one row", "canonical acceptance")

    for fragment, label in (
        ("RawInvocationSourceProofV1", "raw source wrapper"),
        ("CanonicalSingleInvocationSourceProofV1", "canonical source wrapper"),
        ("InvocationPhysicalReceiptV1", "physical receipt"),
        ("RawCollectedInvocationDraftSetV1", "raw collected set"),
        ("CanonicalSingleCollectedInvocationDraftSetV1", "canonical collected set"),
        ("pub(in crate::mir::builder) fn seal_raw", "raw seal terminal"),
        ("pub(in crate::mir::builder) fn seal_canonical_single", "canonical seal terminal"),
        ("SourceFamilyMismatch", "family rejection"),
        ("ForeignOwner", "foreign brand rejection"),
        ("CardinalityMismatch", "cardinality rejection"),
        ("ReplacementHistoryMismatch", "raw replacement check"),
        ("CanonicalReplacementForbidden", "canonical replacement check"),
    ):
        require(collection, fragment, label)
    for fragment, label in (
        ("raw_seal_checks_final_ledger_and_replacement_history", "raw success"),
        ("canonical_a_plus_and_trivial_each_seal_one_exact_row", "A+/trivial success"),
        ("foreign_brand_missing_row_and_wrong_policy_fail_before_co_seal", "failure matrix"),
    ):
        require(fixture, fragment, label)
    require(builder, "mod module_invocation_collection;", "collection registration")
    require(builder, "mod module_invocation_collect0_s0_p0;", "fixture registration")
    require(COLLECTOR.read_text(), "fn key_for_symbol", "collector key projection")
    require(RAW_LEDGER.read_text(), "fn policy", "raw policy projection")

    allowed = {
        COLLECTION.relative_to(ROOT),
        FIXTURE.relative_to(ROOT),
        OWNER_CHAIN.relative_to(ROOT),
        COLLECTOR.relative_to(ROOT),
        RAW_LEDGER.relative_to(ROOT),
        BUILDER.resolve().relative_to(ROOT),
    }
    forbidden = (
        "RawInvocationSourceProofV1",
        "CanonicalSingleInvocationSourceProofV1",
        "RawCollectedInvocationDraftSetV1",
        "CanonicalSingleCollectedInvocationDraftSetV1",
        "fn seal_raw",
        "fn seal_canonical_single",
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
        raise AssertionError("COLLECT0-S0 production consumers: " + ", ".join(consumers))

    print("[cut0-i0-collect0-s0-guard] ok raw=1 canonical_single=1 production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
