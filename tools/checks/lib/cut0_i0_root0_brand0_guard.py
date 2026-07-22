#!/usr/bin/env python3
"""CUT0-I0-ROOT0-BRAND0 disconnected real-owner guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
SRC = ROOT / "src/mir/builder"
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
BRIEF = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-design-stop-2026-07-22.md"
)
OWNER = SRC / "module_invocation_owner_chain.rs"
SESSION = SRC / "module_invocation_session.rs"
LEDGER = SRC / "raw_expansion_receipt_ledger.rs"
ACTIVE = SRC / "module_invocation_brand0.rs"
FIXTURE = SRC / "module_invocation_brand_p0.rs"
BUILDER = SRC.parent / "builder.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    brief = BRIEF.read_text()
    owner = OWNER.read_text()
    session = SESSION.read_text()
    ledger = LEDGER.read_text()
    active = ACTIVE.read_text()
    fixture = FIXTURE.read_text()
    builder = BUILDER.read_text()

    for path in (OWNER, SESSION, LEDGER, ACTIVE, FIXTURE, pathlib.Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"ROOT0-BRAND0 file must remain below 800 lines: {path}")

    require(state, "CUT0-I0-ROOT0-BRAND0 is closed", "state closeout")
    if "ROOT0-RAW0 is next" not in state and "CUT0-I0-ROOT0-RAW0 is closed" not in state:
        raise AssertionError("missing ROOT0-RAW0 successor/closeout")
    require(task, "CUT0-I0-ROOT0-BRAND0 — closed", "task row")
    require(brief, "ROOT0 R-prime selected", "decision lock")

    for fragment, label in (
        ("brand: ModuleInvocationBrandV1", "session brand"),
        ("family: ModuleInvocationFamilyV1", "session family"),
        ("open_for_token", "token session constructor"),
        ("PreparedBuilderExternalCommitV1", "prepared commit"),
    ):
        require(session, fragment, label)
    for fragment, label in (
        ("brand: ModuleInvocationBrandV1", "ledger brand"),
        ("new_for_token", "token ledger constructor"),
        ("fn new_with_brand", "brand-only ledger constructor"),
        ("pub(in crate::mir::builder) const fn brand", "ledger brand witness"),
    ):
        require(ledger, fragment, label)
    for fragment, label in (
        ("InvocationPhysicalStateV1", "physical state"),
        ("ActiveModuleInvocationV1", "active owner"),
        ("ModuleBuilderInvocationSessionV1::open_for_token", "real session wiring"),
        ("InvocationBranded::from_source", "physical brand terminal"),
        ("issue_collected_receipt", "collector receipt terminal"),
    ):
        require(active, fragment, label)
    for fragment, label in (
        ("one_token_brands_actual_session_shell_collector_and_ledger", "same-brand fixture"),
        ("foreign_tokens_cannot_be_confused_with_the_active_owner", "foreign fixture"),
        ("prepared_commit_keeps_the_invocation_brand", "commit fixture"),
        ("dropping_an_active_owner_does_not_mutate_the_live_builder", "drop fixture"),
    ):
        require(fixture, fragment, label)
    require(builder, "mod module_invocation_brand0;", "active owner registration")

    forbidden = (
        "NEXT_RAW_EXPANSION_LEDGER_OWNER",
        "AtomicU64",
        "owner: u64",
        "BrandedShellV1<()>",
        "BrandedCollectorV1<()>",
        "advance_to_prepared_commit",
        "CollectedInvocationDraftSetV1",
    )
    combined = "\n".join((owner, session, ledger, active, fixture))
    for fragment in forbidden:
        if fragment in combined:
            raise AssertionError(f"retired ROOT0-BRAND0 placeholder remains: {fragment}")

    allowed = {
        OWNER.relative_to(ROOT),
        SESSION.relative_to(ROOT),
        LEDGER.relative_to(ROOT),
        ACTIVE.relative_to(ROOT),
        FIXTURE.relative_to(ROOT),
        BUILDER.resolve().relative_to(ROOT),
    }
    production_hits = []
    for path in ROOT.glob("src/**/*.rs"):
        if path.relative_to(ROOT) in allowed:
            continue
        text = path.read_text()
        if "ActiveModuleInvocationV1::open" in text or "InvocationPhysicalStateV1" in text:
            production_hits.append(str(path.relative_to(ROOT)))
    if production_hits:
        raise AssertionError("ROOT0-BRAND0 production consumers: " + ", ".join(production_hits))

    print("[cut0-i0-root0-brand0-guard] ok real_owner=1 production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
