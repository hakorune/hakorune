#!/usr/bin/env python3
"""CUT0-I0 T-prime-r1 decision-lock guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
CONSULT = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-production-transaction-consultation-2026-07-22.md"
)
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
OLD_CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-hdr0-p0-execution-task-2026-07-22.md"
)
SRC = ROOT / "src"
IDENTITY_ALLOWED = {
    pathlib.Path("src/mir/builder.rs"),
    pathlib.Path("src/mir/builder/module_invocation_identity.rs"),
    pathlib.Path("src/mir/builder/module_invocation_identity_p0.rs"),
    pathlib.Path("src/mir/builder/module_invocation_owner_chain.rs"),
    pathlib.Path("src/mir/builder/module_invocation_brand_p0.rs"),
    pathlib.Path("src/mir/builder/module_invocation_collection.rs"),
    pathlib.Path("src/mir/builder/module_invocation_collect0_s0_p0.rs"),
    pathlib.Path("src/mir/builder/module_invocation_callable_batch.rs"),
    pathlib.Path("src/mir/builder/module_draft_collector/callable_batch.rs"),
    pathlib.Path("src/mir/builder/resolved_lowering/callable_module_transaction.rs"),
    pathlib.Path("src/mir/builder/resolved_lowering/callable_batch_collection_p0.rs"),
    pathlib.Path("src/mir/builder/module_lowering_shell.rs"),
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    consultation = CONSULT.read_text()
    task = TASK.read_text()
    old_card = OLD_CARD.read_text()

    for path in (CONSULT, TASK, pathlib.Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"T-prime-r1 file must stay below 800 lines: {path}")

    require(state, "CUT0-I0-CONSULT0 is closed with Candidate T-prime-r1", "state decision")
    require(state, "CUT0-I0-COLLECT0-BATCH0 is closed as a disconnected atomic callable-batch proof", "batch closeout")
    require(state, "CUT0-I0-SESSION0 is closed as a disconnected Builder transaction", "session closeout")
    require(state, "CUT0-I0-ROOT0-D0 is a design stop before ROOT0 implementation", "ROOT0 design stop")
    require(
        state,
        'latest_card = "cut0-i0-t-prime-r1-execution-task-2026-07-22"',
        "latest card pointer",
    )
    require(consultation, "Decision: accepted", "accepted decision")
    require(consultation, "T-prime-r1 closeout", "consultation closeout")
    require(old_card, "Closed — superseded", "single active card")

    for fragment, label in (
        ("five private variants", "five-family token"),
        ("InvocationDraftSourceProofV1", "source-proof identity"),
        ("whole batch preflight", "atomic batch preflight"),
        ("BuilderCoreIdSeedV1", "CoreContext seed policy"),
        ("CompletedRootBodyV1", "raw root witness"),
        ("legacy non-fatal verifier", "legacy verifier policy"),
        ("PreparedModuleExternalCommitV1", "one-shot external commit"),
        ("DuringCleanup { primary, cleanup }", "primary plus cleanup"),
        ("later sibling descent = 0", "outer child-failure abort"),
        ("CUT0-I0-ID0-S0 — closed", "identity row closeout"),
        ("CUT0-I0-ID0-P0 — closed", "brand row closeout"),
        ("CUT0-I0-COLLECT0-S0 — closed", "collection row closeout"),
        ("CUT0-I0-COLLECT0-BATCH0", "next collection batch row"),
        ("CUT0-I0-SESSION0 — closed", "session row closeout"),
        ("CUT0-I0-ROOT0-D0", "ROOT0 design stop"),
        ("CUT0-I0-P0-R1", "real-authority proof row"),
        ("Production consumer count remains zero", "pre-cutover production zero"),
    ):
        require(task, fragment, label)

    forbidden = (
        "ModuleInvocationTokenV1",
        "CollectedInvocationDraftSetV1",
        "PreparedModuleExternalCommitV1",
    )
    consumers = []
    for path in SRC.rglob("*.rs"):
        text = path.read_text()
        for fragment in forbidden:
            if fragment in text and path.relative_to(ROOT) not in IDENTITY_ALLOWED:
                consumers.append(f"{path.relative_to(ROOT)}:{fragment}")
    if consumers:
        raise AssertionError(
            "T-prime-r1 source consumers before COLLECT0-BATCH0: " + ", ".join(consumers)
        )

    print("[cut0-i0-t-prime-r1-guard] ok decision=locked ROOT0=design_stop production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
