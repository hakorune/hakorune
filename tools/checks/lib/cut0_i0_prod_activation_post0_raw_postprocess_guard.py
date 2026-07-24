#!/usr/bin/env python3
"""CUT0-I0-POST0-RAW-POSTPROCESS0 disconnected Raw postprocess guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "post": ROOT / "src/mir/compiler/module_postprocess.rs",
    "stages": ROOT / "src/mir/compiler/module_postprocess_stages.rs",
    "raw_finalizer": ROOT / "src/mir/compiler/raw_finalization.rs",
    "raw_fixture": ROOT / "src/mir/builder/raw_physical_finalization.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    texts["post"] += texts["stages"]
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"POST0-RAW-POSTPROCESS0 file must remain below 800 lines: {path}")

    require(texts["task"], "## POST0-RAW-POSTPROCESS0", "Raw postprocess task row")
    for fragment in (
        "ModulePostprocessInputV1",
        "run_raw(",
        "ModuleVerificationEvidenceV1::Raw",
        "ReportPreTransformOnly",
        "let family = input.token.family();",
        "ModulePostprocessScheduleV1::for_family(family)",
    ):
        require(texts["post"], fragment, f"Raw postprocess owner: {fragment}")
    require(texts["raw_finalizer"], "RawFinalizedModuleInvocationV1", "Raw finalizer product")
    require(
        texts["raw_fixture"],
        "raw_finalizer_consumes_physical_input_without_legacy_finalize",
        "Raw postprocess fixture",
    )
    require(texts["raw_fixture"], "run_raw(finalized)", "Raw postprocess consumer fixture")

    for forbidden in (
        "DrainedModuleCandidateV1",
        "MirBuilder::finalize_module",
        "current_module.functions",
        "retry(",
        "fallback(",
    ):
        if forbidden in texts["post"]:
            raise AssertionError(f"Raw postprocess leaks forbidden authority: {forbidden}")

    production = [
        path.relative_to(ROOT)
        for path in ROOT.glob("src/**/*.rs")
        if path not in (FILES["post"], FILES["raw_finalizer"], FILES["raw_fixture"])
        and not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
        and "run_raw(" in path.read_text()
    ]
    if production:
        raise AssertionError(f"POST0-RAW-POSTPROCESS0 has production consumers: {production}")

    print(
        "[cut0-i0-prod-activation-post0-raw-postprocess-guard] ok "
        "raw_schedule=1 reportable_pre_verify=1 fixture=1 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
