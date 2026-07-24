#!/usr/bin/env python3
"""CONFIG0 guard for request-owned Raw public import disposition."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-ingress-config0-s0-"
    "execution-task-2026-07-24.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
SOURCES = (
    ROOT / "src/mir/builder/module_invocation_session.rs",
    ROOT / "src/mir/builder/module_invocation_session_p0.rs",
    ROOT / "src/mir/compiler/mod.rs",
    ROOT / "src/mir/compiler/raw_public_ingress.rs",
    ROOT / "src/mir/compiler/raw_public_ingress_p0.rs",
    Path(__file__),
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def require_successor_row(state: str) -> None:
    rows = (
        'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-COVERAGE0-REPAIR-S0"',
        'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-S0"',
    )
    if not any(row in state for row in rows):
        raise AssertionError("current pointer must remain on COVERAGE0 repair or its PARITY0 successor")


def count(text: str, fragment: str, expected: int, label: str) -> None:
    actual = text.count(fragment)
    if actual != expected:
        raise AssertionError(f"{label}: expected {expected}, got {actual}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    texts = {path: path.read_text() for path in SOURCES}

    require_successor_row(state)
    require(state, "CONFIG0 are closed", "closed CONFIG0 state")
    require(task, "Status: closed", "closed CONFIG0 task")
    require(task, "RawPublicImportDispositionV1::None", "disposition contract")
    require(task, "ambient Builder using_import_boxes as Raw authority = 0", "ambient import law")

    session = texts[SOURCES[0]]
    session_tests = texts[SOURCES[1]]
    compiler = texts[SOURCES[2]]
    ingress = texts[SOURCES[3]]
    ingress_tests = texts[SOURCES[4]]

    count(session, "snapshot_for_raw_without_imports", 1, "no-import projection producer")
    count(compiler, "bind_raw_source_for_public", 1, "public bind producer")
    count(ingress, "RawPublicImportDispositionV1::None", 1, "NarrowV1 disposition")
    require(session, "config.using_import_boxes.clear();", "candidate-only import clear")
    require(compiler, "BuilderInvocationConfigV1::snapshot_for_raw_without_imports", "public config projection")
    require(ingress, "bind_raw_source_for_public", "public ingress bind handoff")
    require(session_tests, "raw_public_snapshot_forces_empty_imports_without_mutating_live", "config fixture")
    require(ingress_tests, "raw_public_ingress_failure_preserves_live_imports", "failure isolation fixture")

    start = ingress.index("pub fn compile_raw_with_source")
    body = ingress[start:]
    if "snapshot_for_raw(" in body or "set_using_import_boxes" in body:
        raise AssertionError("public Raw ingress must not use ambient import snapshot/mutation")
    if "compile_with_source_and_imports" in body or "imports:" in body:
        raise AssertionError("explicit-import Raw capability leaked into NarrowV1")
    count(ingress, "RawPublicImportDispositionV1::None", 1, "single selection authority")
    if any(len(text.splitlines()) >= 800 for text in texts.values()):
        raise AssertionError("CONFIG0 source/check file must remain below 800 lines")

    print("[cut0-i0-root0-raw-source0-lower-root-post0-public-ingress-config0-guard] ok disposition=none candidate_imports=empty live_unchanged=1 below_800=1")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
