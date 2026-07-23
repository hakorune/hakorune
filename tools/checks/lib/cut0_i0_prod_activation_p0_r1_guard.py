#!/usr/bin/env python3
"""CUT0-I0-P0-R1 real-authority disconnected chain guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "canonical": ROOT / "src/mir/compiler/canonical_bridge_fixture0_p0.rs",
    "raw": ROOT / "src/mir/builder/raw_physical_finalization.rs",
    "finalizer": ROOT / "src/mir/compiler/canonical_finalization.rs",
    "raw_finalizer": ROOT / "src/mir/compiler/raw_finalization.rs",
    "postprocess": ROOT / "src/mir/compiler/module_postprocess.rs",
    "commit": ROOT / "src/mir/compiler/external_commit.rs",
    "failure": ROOT / "src/mir/compiler/prod_activation_p0_r1.rs",
    "mod": ROOT / "src/mir/compiler/mod.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"P0-R1 file must remain below 800 lines: {path}")

    require(texts["task"], "## P0-R1 — real-authority all-route proof", "P0-R1 row")
    require(texts["task"], "Production consumers stay zero", "disconnected stop line")
    require(
        texts["canonical"],
        "p0_r1_canonical_four_route_real_authority_chain",
        "canonical aggregate fixture",
    )
    require(
        texts["canonical"],
        "p0_r1_real_authority_readiness_failure_keeps_commit_zero",
        "readiness failure fixture",
    )
    require(
        texts["canonical"],
        "p0_r1_real_authority_drain_failure_keeps_commit_zero",
        "drain failure fixture",
    )
    require(
        texts["canonical"],
        "p0_r1_callable_capability_mismatch_stops_before_commit",
        "capability failure fixture",
    )
    require(texts["canonical"], "CapabilityMismatch", "capability fail-fast")
    require(
        texts["failure"],
        "p0_r1_final_verifier_failure_keeps_commit_zero",
        "final verifier failure fixture",
    )
    require(texts["failure"], "BasicBlockId::new(9999)", "invalid CFG edge fixture")
    require(texts["failure"], "ModulePostprocessErrorV1::FinalVerification", "final verifier error")
    require(texts["raw"], "p0_r1_raw_real_authority_chain", "Raw aggregate fixture")
    for family in (
        "CanonicalAPlus",
        "BindingSsaTrivial",
        "BindingSsaAcyclic",
        "BindingSsaRecursive",
    ):
        require(texts["canonical"], family, f"canonical family row: {family}")
    for fragment in (
        ".prepare_drain()",
        ".drain()",
        ".prepare_finalization()",
        "CanonicalModuleFinalizerV1::finalize",
        "ModulePostprocessOwnerV1::new",
        "prepare_module_external_commit",
        "commit_prepared_module",
    ):
        require(texts["canonical"], fragment, f"canonical chain: {fragment}")
    for fragment in (
        "RawModuleFinalizerV1::prepare",
        "RawModuleFinalizerV1::finalize",
        "run_raw(finalized)",
        "prepare_module_external_commit",
        "commit_prepared_module",
    ):
        require(texts["raw"], fragment, f"Raw chain: {fragment}")
    require(texts["finalizer"], "CanonicalModuleFinalizerV1", "canonical finalizer")
    require(texts["raw_finalizer"], "RawModuleFinalizerV1", "Raw finalizer")
    require(texts["postprocess"], "ModulePostprocessOwnerV1", "postprocess owner")
    require(texts["commit"], "PreparedModuleExternalCommitV1", "paired commit product")
    require(texts["mod"], "mod external_commit;", "external commit registration")

    # P0-R1 is intentionally disconnected.  Only test fixtures may call the
    # new terminals until atomic CUT0; this census prevents accidental wiring.
    forbidden_production_terms = (
        "CanonicalModuleFinalizerV1::finalize(",
        "RawModuleFinalizerV1::prepare(",
        "prepare_module_external_commit(",
        "commit_prepared_module(",
    )
    exempt = {
        FILES["canonical"],
        FILES["finalizer"],
        FILES["raw_finalizer"],
        FILES["postprocess"],
        FILES["commit"],
        FILES["failure"],
    }
    production = []
    for path in ROOT.glob("src/**/*.rs"):
        if path in exempt or path.name.endswith("_p0.rs") or path.name.endswith("_tests.rs"):
            continue
        if "tests" in path.parts:
            continue
        text = path.read_text()
        if path == FILES["raw"]:
            text = text.split("#[cfg(test)]", 1)[0]
        if any(term in text for term in forbidden_production_terms):
            production.append(path.relative_to(ROOT))
    if production:
        raise AssertionError(f"P0-R1 production consumers are non-zero: {production}")

    print(
        "[cut0-i0-prod-activation-p0-r1-guard] ok "
        "canonical_routes=4 raw_route=1 full_chain=5 readiness_failure=1 "
        "drain_failure=1 capability_failure=1 "
        "post_failure=1 "
        "production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
