#!/usr/bin/env python3
"""Guard the disconnected Raw OWNER0-PHYSICAL0 owner boundary."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-owner0-physical0-execution-task-2026-07-24.md"
)
BUILDER = ROOT / "src/mir/builder/raw_root_physical.rs"
ELIGIBILITY = ROOT / "src/mir/compiler/raw_root_eligibility.rs"
PACKAGE = ROOT / "src/mir/compiler/raw_root_package.rs"
SOURCES = (BUILDER, ELIGIBILITY, PACKAGE, ROOT / "src/mir/builder.rs")


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    builder = BUILDER.read_text()
    eligibility = ELIGIBILITY.read_text()
    package = PACKAGE.read_text()

    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PHYSICAL0"', "active row")
    require(state, 'latest_card = "cut0-i0-raw-source0-lower-root-owner0-physical0-execution-task-2026-07-24"', "active card")
    for fragment in (
        "EligibleSourceBoundRawRootPackageV1",
        "RawRootInvocationV1::{Script, App}",
        "child/root lowering",
        "production consumer = 0",
        "RawDraftInvocationV1",
        "Main-only",
    ):
        require(task, fragment, f"task {fragment}")
    for fragment in (
        "RawRootPhysicalStateV1",
        "InvocationPhysicalStateV1::from_token",
        "RawExpansionReceiptLedgerV1::new_for_token",
        "RootBodyCompletionTrackerV1::new_for_brand",
        "shell_is_empty",
    ):
        require(builder, fragment, f"physical carrier {fragment}")
    for fragment in (
        "open_physical(",
        "RejectedRawRootPhysicalOpenV1",
        "RawRootInvocationV1::Script",
        "RawRootInvocationV1::App",
        "RawRootPhysicalStateV1::open",
    ):
        require(eligibility, fragment, f"compiler terminal {fragment}")
    require(package, "RawRootPhysicalOpenPartsV1", "named consuming handoff")

    forbidden = (
        "RawDraftInvocationV1",
        "ModuleLoweringInvocationStateV1",
        "execute_preflighted_module_invocation",
        "finalize_module",
        "commit_prepared_module",
        "retry(",
        "fallback(",
    )
    for path in (BUILDER, ELIGIBILITY):
        text = path.read_text()
        for fragment in forbidden:
            if fragment in text:
                raise AssertionError(f"physical row widens into forbidden path: {path}: {fragment}")

    for path in (STATE, TASK, *SOURCES):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print("[cut0-i0-root0-raw-source0-lower-root-owner0-physical0-guard] ok consumer=1 production=0 below_800=1")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
