#!/usr/bin/env python3
"""CUT0-I0-POST0-RAW-S0 physical-owner census guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "builder": ROOT / "src/mir/builder/raw_physical_finalization.rs",
    "raw_root": ROOT / "src/mir/builder/raw_root_completion.rs",
    "builder_mod": ROOT / "src/mir/builder.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"POST0-RAW-S0 file must remain below 800 lines: {path}")

    if not any(
        heading in texts["task"]
        for heading in (
            "## POST0-RAW-S0 — Raw finalization input (next)",
            "## POST0-RAW-S0 — Raw finalization input (closed)",
        )
    ):
        raise AssertionError("missing Raw boundary heading")
    for fragment in (
        "RawPhysicalCompleteInvocationV1",
        "RawFinalizationInputV1",
        "bind_physical(",
        "prepare_finalization(",
        "PublishedShell",
        "InventoryMismatch",
    ):
        require(texts["builder"], fragment, f"Raw physical owner: {fragment}")
    require(texts["raw_root"], "pub(in crate::mir::builder) fn into_parts(", "retained Raw proof extraction")
    require(texts["builder_mod"], "raw_physical_finalization", "Raw physical module registration")
    for fixture in (
        "raw_physical_owner_retains_module_session_and_legacy_evidence",
        "raw_physical_prepare_rejects_published_shell_before_move",
    ):
        require(texts["builder"], fixture, f"Raw fixture: {fixture}")

    for forbidden in (
        "DrainedModuleCandidateV1",
        "InvocationDrainExpectationV1",
        "retry(",
        "prepare_again(",
        "current_module.functions",
    ):
        if any(forbidden in texts[name] for name in ("builder", "raw_root")):
            raise AssertionError(f"Raw physical seam leaks forbidden authority: {forbidden}")

    production = [
        path.relative_to(ROOT)
        for path in ROOT.glob("src/**/*.rs")
        if path not in (FILES["builder"], FILES["builder_mod"])
        and not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
        and "RawPhysicalCompleteInvocationV1" in path.read_text()
    ]
    if production:
        raise AssertionError(f"POST0-RAW-S0 has production consumers: {production}")

    print(
        "[cut0-i0-prod-activation-post0-raw-guard] ok "
        "physical_owner=1 retained_root=1 rejection_matrix=1 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
