#!/usr/bin/env python3
"""FINAL0-GUARD-SCOPE0: production caller census with test-module exclusion."""

from __future__ import annotations

import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
COMPILER_MOD = ROOT / "src/mir/compiler/mod.rs"
FINALIZER = ROOT / "src/mir/compiler/raw_root_finalization.rs"
PHYSICAL = ROOT / "src/mir/builder/raw_root_physical/finalization_terminal.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-final0-source-drain-handoff-consultation-question-2026-07-24.md"
)


def test_only_modules() -> set[str]:
    text = COMPILER_MOD.read_text()
    names = set(re.findall(r"#\[cfg\(test\)\]\s*mod\s+([A-Za-z0-9_]+)\s*;", text))
    return names


def production_rust_files() -> list[Path]:
    test_modules = test_only_modules()
    files = []
    for path in ROOT.glob("src/**/*.rs"):
        if path.stem in test_modules:
            continue
        if path.name.endswith("_tests.rs") or path.name.endswith("_p0.rs"):
            continue
        files.append(path)
    return files


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    finalizer = FINALIZER.read_text()
    physical = PHYSICAL.read_text()
    require(task, "FINAL0-GUARD-SCOPE0", "guard prerequisite")
    require(task, "RawDrainedInvocationV1::prepare_finalization(self)", "direct entry")
    require(finalizer, "impl RawDrainedInvocationV1", "new finalizer owner")
    require(finalizer, "RawFinalizedInvocationV1", "new finalizer product")
    require(physical, "prepare_raw_finalization", "Builder physical terminal")
    require(physical, "PreparedRawDrainedPhysicalFinalizationV1", "prepared physical product")

    for path in (TASK, FINALIZER, PHYSICAL):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"FINAL0 file must remain below 800 lines: {path}")

    # The old guard counted calls in cfg(test)-registered modules.  This census
    # excludes those modules from the production scope before checking old
    # bridge reachability.
    production = production_rust_files()
    old_bridge_callers = []
    for path in production:
        if path in (FINALIZER, PHYSICAL, ROOT / "src/mir/builder/raw_physical_finalization.rs",
                    ROOT / "src/mir/compiler/raw_finalization.rs"):
            continue
        text = path.read_text()
        if (
            "RawModuleFinalizerV1::prepare(" in text
            or ("RawPhysicalCompleteInvocationV1" in text and ".prepare_finalization(" in text)
        ):
            old_bridge_callers.append(path.relative_to(ROOT))
    if old_bridge_callers:
        raise AssertionError(f"old Raw finalization production callers: {old_bridge_callers}")

    for forbidden in ("into_draft_functions", '["condition_fn", "main"]'):
        if forbidden in finalizer or forbidden in physical:
            raise AssertionError(f"new FINAL0 source retains old bridge authority: {forbidden}")

    if "prepare_finalization(" not in FINALIZER.read_text():
        raise AssertionError("new FINAL0 entry missing")

    print(
        "[cut0-i0-root0-raw-source0-lower-final0-guard-scope] ok "
        "test_scope_excluded=1 old_bridge_callers=0 direct_owner=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
