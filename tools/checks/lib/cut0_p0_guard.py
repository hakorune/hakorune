#!/usr/bin/env python3
"""CUT0-P0 disconnected all-route adapter guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
SRC = ROOT / "src/mir/builder"
ADAPTER = SRC / "module_invocation_cut0_p0.rs"
BUILDER = ROOT / "src/mir/builder.rs"
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-hdr0-p0-execution-task-2026-07-22.md"
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    adapter = ADAPTER.read_text()
    builder = BUILDER.read_text()
    state = STATE.read_text()
    card = CARD.read_text()
    if len(adapter.splitlines()) >= 800:
        raise AssertionError("CUT0-P0 adapter must remain below 800 lines")
    for fragment, label in (
        ("struct Cut0P0OuterAdapterV1", "single outer adapter"),
        ("fn execute(", "shared adapter entry"),
        ("InvocationRouteMatrixV1::rows()", "all route rows"),
        ("SCENARIOS", "fault scenario matrix"),
        ("RouteOwnedInvocationInventoryV2::derive", "opaque authority lane"),
        ("ModuleLoweringInvocationCandidateV1::open", "candidate owner"),
        ("complete_success", "success handoff"),
        ("prepare_complete", "same-state drain"),
        ("drain_candidate", "typed drain"),
        ("finalize_drained_module_once", "Builder-free finalizer"),
        ("ExternalCommitProbeV1", "external commit probe"),
        ("catch_unwind(AssertUnwindSafe", "panic boundary"),
        ("9 * 8", "nine-by-eight execution evidence"),
        ("external_commit_count, expected", "failure publication law"),
    ):
        require(adapter, fragment, label)
    require(builder, "#[cfg(test)]\nmod module_invocation_cut0_p0;", "test-only registration")
    require(card, "CUT0-P0 — disconnected all-route proof", "active CUT0-P0 task")
    require(card, "CUT0-P0 closeout", "CUT0-P0 closeout")
    require(
        state,
        "CUT0-S0-OWNER0, CUT0-S0-COMPAT0, and CUT0-P0 are closed as disconnected proofs; CUT0-I0-CONSULT0 is closed with Candidate T-prime-r1; CUT0-I0-ID0-S0 is closed as a disconnected identity/token proof; CUT0-I0-ID0-P0 is closed as a disconnected branded owner-chain proof; CUT0-I0-COLLECT0-S0 is closed as a disconnected raw/canonical co-seal proof; CUT0-I0-COLLECT0-BATCH0 is next",
        "state pointer",
    )

    consumers = []
    for path in SRC.rglob("*.rs"):
        if path in (ADAPTER, BUILDER):
            continue
        if "Cut0P0OuterAdapterV1" in path.read_text():
            consumers.append(str(path.relative_to(ROOT)))
    if consumers:
        raise AssertionError("CUT0-P0 production adapter consumers: " + ", ".join(consumers))
    print("[cut0-p0-guard] ok routes=9 scenarios=8 production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
