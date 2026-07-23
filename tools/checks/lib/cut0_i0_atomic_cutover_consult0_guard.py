#!/usr/bin/env python3
"""CUT0-I0 atomic-cutover design-stop census guard.

This guard is intentionally negative: while the consultation is open it
proves that no production executor or public cutover has been wired.
"""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-atomic-cutover-consultation-2026-07-23.md"
)
PROD_SOURCES = (
    ROOT / "src/mir/compiler/mod.rs",
    ROOT / "src/runtime/mirbuilder_emit.rs",
)
COMPAT = ROOT / "src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs"
CONFIG = ROOT / "src/mir/builder/module_invocation_session.rs"
SIZE_FILES = (*PROD_SOURCES, COMPAT, CONFIG, CARD)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    card = CARD.read_text()
    sources = {path: path.read_text() for path in PROD_SOURCES}
    compat = COMPAT.read_text()
    config = CONFIG.read_text()

    require(
        state,
        'current_design_stop = "RAW-SOURCE0-CONSULT0"',
        "Raw source design-stop pointer",
    )
    require(
        state,
        'latest_card = "cut0-i0-raw-source0-consultation-2026-07-23"',
        "latest Raw source consultation card",
    )
    for question in ("Q1 — atomic executor scope", "Q2 — Raw source authority", "Q3 — AST JSON"):
        require(card, question, f"consultation question {question}")
    for fragment in (
        "RAW-SOURCE0-CONSULT0",
        "SOURCE-FIRST-prime-r1 closeout",
        "Program(JSON v0)",
        "production outer executor = 0",
        "BuilderInvocationConfigV1",
    ):
        require(card, fragment, f"consultation boundary {fragment}")

    joined_prod = "\n".join(sources.values())
    if "execute_preflighted_module_invocation" in joined_prod:
        raise AssertionError("atomic executor is wired before consultation close")

    direct_build_callers = [
        path.relative_to(ROOT)
        for path, text in sources.items()
        if ".build_module(" in text
    ]
    expected = {
        pathlib.Path("src/mir/compiler/mod.rs"),
        pathlib.Path("src/runtime/mirbuilder_emit.rs"),
    }
    if set(direct_build_callers) != expected:
        raise AssertionError(
            "consultation census changed direct production build_module callers: "
            f"{direct_build_callers}"
        )

    require(compat, "Program(JSON v0) remains an explicit compat-only keep", "compat lane")
    require(compat, "NYASH_VM_USE_FALLBACK", "compat fallback gate")
    require(config, "pub(in crate::mir) struct BuilderInvocationConfigV1", "sealed builder config")

    for path in SIZE_FILES:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"design-stop file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-atomic-cutover-consult0-guard] ok "
        "atomic_decision_locked=1 executor=0 raw_source_binding=0 direct_build_module_callers=2 "
        "program_v0_compat_lane=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
