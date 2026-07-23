#!/usr/bin/env python3
"""CUT0-I0-POST0-RAW-FINALIZER0 disconnected finalization guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "finalizer": ROOT / "src/mir/compiler/raw_finalization.rs",
    "builder": ROOT / "src/mir/builder/raw_physical_finalization.rs",
    "compiler_mod": ROOT / "src/mir/compiler/mod.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"POST0-RAW-FINALIZER0 file must remain below 800 lines: {path}")

    require(texts["task"], "## POST0-RAW-FINALIZER0", "Raw finalizer task row")
    require(texts["compiler_mod"], "mod raw_finalization;", "Raw finalizer registration")
    for fragment in (
        "RawModuleFinalizerV1",
        "RawFinalizationInputV1",
        "RawFinalizedModuleInvocationV1",
        "RawFinalizationErrorV1",
        "prepare(",
        "finalize(",
        "prepare_module_session()",
        "BuilderReadiness",
        "SealedRawExpansionReceiptLedgerV1",
        "RawInvocationRootWitnessV1",
    ):
        require(texts["finalizer"], fragment, f"Raw finalizer boundary: {fragment}")
    for forbidden in (
        "DrainedModuleCandidateV1",
        "finalize_module(",
        "current_module",
        "retry(",
        "prepare_again(",
        "MirCompileResult",
    ):
        if forbidden in texts["finalizer"]:
            raise AssertionError(f"Raw finalizer leaks forbidden authority: {forbidden}")

    for fixture in (
        "raw_finalizer_consumes_physical_input_without_legacy_finalize",
        "raw_finalizer_retains_readiness_failure_owner",
    ):
        require(texts["builder"], fixture, f"Raw finalizer fixture: {fixture}")

    production = [
        path.relative_to(ROOT)
        for path in ROOT.glob("src/**/*.rs")
        if path not in (FILES["finalizer"], FILES["builder"])
        and not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
        and "RawModuleFinalizerV1" in path.read_text()
    ]
    if production:
        raise AssertionError(f"POST0-RAW-FINALIZER0 has production consumers: {production}")

    print(
        "[cut0-i0-prod-activation-post0-raw-finalizer-guard] ok "
        "readiness=1 retained_evidence=1 legacy_finalize=0 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
