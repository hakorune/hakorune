#!/usr/bin/env python3
"""CUT0-I0-POST0 disconnected postprocess-owner census guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "post": ROOT / "src/mir/compiler/module_postprocess.rs",
    "tests": ROOT / "src/mir/compiler/module_postprocess_p0.rs",
    "mod": ROOT / "src/mir/compiler/mod.rs",
    "final": ROOT / "src/mir/compiler/canonical_finalization.rs",
    "raw": ROOT / "src/mir/compiler/raw_finalization.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"POST0 file must remain below 800 lines: {path}")

    require(
        texts["task"],
        "## POST0 — one postprocess owner (closed; production disconnected)",
        "POST0 boundary",
    )
    require(texts["mod"], "mod module_postprocess;", "POST0 module registration")
    for fragment in (
        "ModulePostprocessScheduleV1",
        "ModulePostprocessOwnerV1",
        "ModuleVerificationEvidenceV1",
        "ModulePostprocessInputV1",
        "PostprocessedModuleInvocationV1",
        "run_raw(",
        "ModuleVerificationEvidenceV1::Raw",
        "for_family(",
        "refresh_module_rune_plans",
        "optimize_module",
        "refresh_and_validate_for_boundary",
        "verify_module",
        "insert_rc_instructions",
        "canonicalize_for_site",
    ):
        require(texts["post"], fragment, f"postprocess owner: {fragment}")

    order = [
        "refresh_module_rune_plans(module)",
        "optimize_module(module)",
        "refresh_and_validate_for_boundary(module",
        "verify_module(module)",
        "insert_rc_instructions(module)",
        "refresh_module_semantic_metadata(module)",
        "canonicalize_for_site",
    ]
    positions = [texts["post"].find(fragment) for fragment in order]
    if any(position < 0 for position in positions) or positions != sorted(positions):
        raise AssertionError(f"POST0 stage order drift: {positions}")

    for forbidden in (
        "current_module",
        "source AST",
        "ModuleInvocationPolicyV1",
        "ConditionFnPolicyV1::Optional",
        "DrainedModuleCandidateV1",
        "retry(",
        "fallback(",
    ):
        if forbidden in texts["post"]:
            raise AssertionError(f"POST0 leaks forbidden authority: {forbidden}")

    for fixture in (
        "postprocess_schedule_is_family_owned",
        "postprocess_consumes_finalized_single_without_publication",
    ):
        require(texts["tests"], fixture, f"POST0 fixture: {fixture}")

    production = [
        path.relative_to(ROOT)
        for path in ROOT.glob("src/**/*.rs")
        if path not in (FILES["post"], FILES["final"], FILES["raw"], ROOT / "src/mir/builder/raw_physical_finalization.rs")
        and not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
        and ("ModulePostprocessOwnerV1::new" in path.read_text()
             or "run(finalized)" in path.read_text())
    ]
    if production:
        raise AssertionError(f"POST0 has production consumers: {production}")

    print(
        "[cut0-i0-prod-activation-post0-guard] ok "
        "schedule=1 stage_order=1 family_policy=1 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
