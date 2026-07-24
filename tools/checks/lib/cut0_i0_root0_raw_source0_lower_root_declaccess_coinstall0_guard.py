#!/usr/bin/env python3
"""DECLACCESS COINSTALL0 Builder/shell aggregate guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-declaccess-coinstall0-execution-task-2026-07-24.md"
)
SOURCE = tuple(
    ROOT / path
    for path in (
        "src/mir/builder/raw_root_environment_install.rs",
        "src/mir/builder/raw_root_physical.rs",
        "src/mir/builder/module_invocation_brand0.rs",
        "src/mir/builder/module_invocation_session.rs",
        "src/mir/builder/module_lowering_shell.rs",
        "src/mir/builder/module_lowering_shell/declaration_fact_commit.rs",
        "src/mir/compiler/raw_root_environment_manifest.rs",
        "src/mir/compiler/raw_root_source_facts.rs",
    )
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    joined = "\n".join(path.read_text() for path in SOURCE)
    restricted = "\n".join(
        path.read_text()
        for path in SOURCE
        if path.name in {"raw_root_environment_install.rs", "raw_root_physical.rs"}
    )

    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-DECLACCESS0-S0"', "next row")
    require(task, "Decision: **COINSTALL-prime-r1**", "decision lock")
    for fragment in (
        "RawRootEnvironmentInstallOwnerV1",
        "into_install_parts(",
        "RawRootEnvironmentProjectionV1::from_manifest",
        "install_raw_root_environment_preflighted",
        "install_environment_preflighted",
        "infallible manifest-derived Builder/shell co-install",
        "production consumer",
        "all modified/new source/check files < 800 lines",
    ):
        require(task + joined, fragment, f"contract {fragment}")

    for forbidden in (
        "prepare_module_session",
        "current_module",
        "RawRootEnvironmentInstallOwnerV1::into_parts",
        "declare_environment(self)",
    ):
        if forbidden in restricted:
            raise AssertionError(f"forbidden COINSTALL0 authority: {forbidden}")

    for path in (STATE, TASK, *SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-declaccess-coinstall0-guard] ok "
        "aggregate=1 preflight=1 co_install=1 no_production_consumer=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
