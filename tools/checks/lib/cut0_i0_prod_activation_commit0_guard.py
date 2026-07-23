#!/usr/bin/env python3
"""CUT0-I0-COMMIT0 disconnected paired external-commit guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "commit": ROOT / "src/mir/compiler/external_commit.rs",
    "fixture": ROOT / "src/mir/compiler/external_commit_p0.rs",
    "session": ROOT / "src/mir/builder/module_invocation_session.rs",
    "mod": ROOT / "src/mir/compiler/mod.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"COMMIT0 file must remain below 800 lines: {path}")

    require(texts["task"], "## COMMIT0 — paired external commit", "COMMIT0 task row")
    for fragment in (
        "PreparedModuleExternalCommitV1",
        "ExternalCommitPreparationErrorV1",
        "ModuleVerificationEvidenceV1",
        "commit_prepared_module(",
        "MirCompileResult",
        "builder.commit(current)",
        "token.family() != builder.family()",
    ):
        require(texts["commit"], fragment, f"paired commit product: {fragment}")
    for fragment in (
        "into_external_commit(self)",
        "PreparedBuilderExternalCommitV1",
        "family: ModuleInvocationFamilyV1",
    ):
        require(texts["session"], fragment, f"Builder readiness capability: {fragment}")
    require(texts["mod"], "mod external_commit;", "COMMIT0 module registration")
    require(
        texts["fixture"],
        "paired_external_commit_consumes_builder_and_module_once",
        "COMMIT0 success fixture",
    )

    for forbidden in (
        "CanonicalModuleLoweringSessionV1",
        "DrainedModuleCandidateV1",
        "retry(",
        "prepare_again(",
        "execute_preflighted_module_invocation",
    ):
        if forbidden in texts["commit"]:
            raise AssertionError(f"COMMIT0 product leaks activation authority: {forbidden}")

    production = []
    raw_physical = ROOT / "src/mir/builder/raw_physical_finalization.rs"
    for path in ROOT.glob("src/**/*.rs"):
        if path in (FILES["commit"], FILES["fixture"]):
            continue
        if path.name.endswith("_p0.rs") or path.name.endswith("_tests.rs"):
            continue
        if "tests" in path.parts:
            continue
        text = path.read_text()
        if path == raw_physical:
            text = text.split("#[cfg(test)]", 1)[0]
        if "commit_prepared_module(" in text:
            production.append(path.relative_to(ROOT))
    if production:
        raise AssertionError(f"COMMIT0 has production consumers: {production}")

    print(
        "[cut0-i0-prod-activation-commit0-guard] ok "
        "paired_product=1 readiness_family=1 one_shot=1 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
