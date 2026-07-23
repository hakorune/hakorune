#!/usr/bin/env python3
"""CUT0-I0-FINAL0 disconnected finalization census guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "builder": ROOT / "src/mir/builder/module_invocation_session.rs",
    "builder_tests": ROOT / "src/mir/builder/module_invocation_session_p0.rs",
    "builder_mod": ROOT / "src/mir/builder.rs",
    "compiler_mod": ROOT / "src/mir/compiler/mod.rs",
    "finalizer": ROOT / "src/mir/compiler/canonical_finalization.rs",
    "finalizer_tests": ROOT / "src/mir/compiler/canonical_finalization_p0.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"FINAL0 file must remain below 800 lines: {path}")

    require(texts["task"], "FINAL0", "FINAL0 execution row")
    require(texts["task"], "POST0 -> COMMIT0 -> P0-R1", "next-row boundary")
    require(texts["compiler_mod"], "mod canonical_finalization;", "finalizer module registration")

    for fragment in (
        "prepare_module_session(",
        "PreparedBuilderModuleSessionV1",
        "RejectedPreparedBuilderModuleSessionV1",
        "readiness_error(&self)",
    ):
        require(texts["builder"], fragment, f"Builder readiness product: {fragment}")
    require(texts["builder_mod"], "PreparedBuilderModuleSessionV1", "Builder readiness re-export")
    for fixture in (
        "module_session_readiness_success_is_non_clone_and_consuming",
        "module_session_readiness_rejects_current_module",
        "module_session_readiness_rejects_function_state",
        "module_session_readiness_rejects_compilation_context",
    ):
        require(texts["builder_tests"], fixture, f"readiness fixture: {fixture}")

    for fragment in (
        "CanonicalFinalizationInputV1",
        "CanonicalModuleFinalizerV1",
        "FinalizedModuleInvocationV1",
        "prepare_finalization(",
        "prepare_module_session()",
        "BuilderReadiness",
        "CanonicalFinalizationInputV1::Single",
        "CanonicalFinalizationInputV1::Callable",
    ):
        require(texts["finalizer"], fragment, f"route-specific finalization: {fragment}")
    for forbidden in (
        "DrainedModuleCandidateV1",
        "finalize_module(",
        "current_module",
        "source AST",
        "retry(",
        "prepare_again(",
    ):
        if forbidden in texts["finalizer"]:
            raise AssertionError(f"FINAL0 finalizer leaks forbidden authority: {forbidden}")
    for fixture in (
        "final0_prepares_and_finalizes_single_route",
        "final0_prepares_and_finalizes_callable_route",
    ):
        require(texts["finalizer_tests"], fixture, f"FINAL0 fixture: {fixture}")

    # Finalization remains a disconnected proof product until the later
    # activation row; only the definition and focused tests may mention it.
    production = [
        path.relative_to(ROOT)
        for path in ROOT.glob("src/**/*.rs")
        if path != FILES["finalizer"]
        if not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
        and "prepare_finalization(" in path.read_text()
    ]
    if production:
        raise AssertionError(f"FINAL0 has production finalizer consumers: {production}")

    print(
        "[cut0-i0-prod-activation-final0-guard] ok "
        "readiness=1 route_finalizer=1 single_callable=1 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
