#!/usr/bin/env python3
"""DECLACCESS0-S0 compiler terminal and owner-boundary guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-declaccess0-s0-execution-task-2026-07-24.md"
)
SOURCE = tuple(
    ROOT / path
    for path in (
        "src/mir/compiler/raw_root_decl_access.rs",
        "src/mir/compiler/raw_root_decl_access_p0.rs",
        "src/mir/compiler/raw_root_callable_main.rs",
        "src/mir/compiler/raw_root_environment_manifest.rs",
        "src/mir/builder/raw_root_environment_install.rs",
        "src/mir/builder/module_invocation_session.rs",
        "src/mir/builder/compilation_context.rs",
    )
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    joined = "\n".join(path.read_text() for path in SOURCE)
    decl_access = (ROOT / "src/mir/compiler/raw_root_decl_access.rs").read_text()

    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-BODY0-S0"', "next row")
    require(state, 'latest_card = "cut0-i0-raw-source0-lower-root-body0-s0-execution-task-2026-07-24"', "next card")
    require(task, "Decision: **DECLACCESS-prime-r1**", "decision lock")
    for fragment in (
        "RawCallableMainReadyEnvironmentPartsV1",
        "declare_environment(",
        "DeclaredRawRootInvocationV1",
        "RejectedRawRootEnvironmentInvocationV1",
        "infallible",
        "BODY0 and production consumers remain zero",
        "all touched source/check files < 800 lines",
    ):
        require(task + joined, fragment, f"contract {fragment}")

    if decl_access.count("fn declare_environment(") != 1:
        raise AssertionError("DECLACCESS must have one compiler terminal")
    for forbidden in (
        "prepare_module_session",
        "current_module",
        "OwnedRawSourceV1::ast",
        "sorted_method_entries",
        "MainPending",
        "MainCaptured",
        "finalize_module",
        "catch_unwind",
        "retry(",
        "resume(",
        "fallback(",
    ):
        if forbidden in decl_access:
            raise AssertionError(f"forbidden DECLACCESS authority: {forbidden}")

    for fixture in (
        "script_declaration_installs_environment_once",
        "app_omitted_declaration_keeps_main_unselected",
        "app_required_declaration_keeps_callable_main_evidence",
        "dirty_builder_rejects_before_environment_commit",
    ):
        require(joined, fixture, f"fixture {fixture}")

    for path in (STATE, TASK, *SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-declaccess0-s0-guard] ok "
        "one_terminal=1 consuming_split=1 co_install=1 no_body=1 production_consumer=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
