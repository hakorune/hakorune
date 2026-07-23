#!/usr/bin/env python3
"""CUT0-I0-POST-FAILURE0 natural-failure proof guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = {
    "task": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md",
    "consult": ROOT / "docs/development/current/main/investigations/cut0-i0-prod-activation-post-failure-consultation-2026-07-23.md",
    "post": ROOT / "src/mir/compiler/module_postprocess.rs",
    "tests": ROOT / "src/mir/compiler/module_postprocess_failure_p0.rs",
    "mod": ROOT / "src/mir/compiler/mod.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    texts = {name: path.read_text() for name, path in FILES.items()}
    for name, path in FILES.items():
        if len(texts[name].splitlines()) >= 800:
            raise AssertionError(f"POST-FAILURE0 file must remain below 800 lines: {path}")

    require(texts["consult"], "NF-prime-r1", "NF-prime decision closeout")
    require(texts["task"], "POST-FAILURE0-NATURAL-P0", "natural failure task row")
    require(
        texts["mod"],
        "#[cfg(test)]\nmod module_postprocess_failure_p0;",
        "test-only failure fixture registration",
    )
    for fixture in (
        "optimizer_natural_failure_retains_discard_only_owner",
        "orphan_static_plan_natural_failure_retains_discard_only_owner",
    ):
        require(texts["tests"], fixture, f"natural failure fixture: {fixture}")
    for fragment in (
        "ExistingOptimizerPolicyScopeV1",
        "NYASH_OPT_DIAG_FAIL",
        "NYASH_OPT_DIAG_FORBID_LEGACY",
        "NYASH_MIR_DISABLE_OPT",
        "HAKO_MIR_DISABLE_OPT",
        "MutexGuard",
        "PostprocessFailureStageV1::Optimizer",
        "PostprocessFailureStageV1::ContractRefresh",
        "OptimizerDiagnostics",
        "static_table_contract_spec_missing",
        "discard()",
    ):
        require(texts["tests"], fragment, f"failure proof: {fragment}")

    for forbidden in (
        "catch_unwind",
        "PostprocessFailureStageV1::RcInsertion",
        "fault_disposition",
        "POST_FAILURE_FAULT",
        "retry(",
        "fallback(",
    ):
        if forbidden in texts["tests"] or forbidden in texts["post"]:
            raise AssertionError(f"POST-FAILURE0 adds forbidden authority: {forbidden}")

    production = []
    for path in ROOT.glob("src/**/*.rs"):
        if path in (FILES["post"], FILES["tests"]):
            continue
        if (
            path.name.endswith("_p0.rs")
            or path.name.endswith("_tests.rs")
            or path.name == "prod_activation_p0_r1.rs"
        ):
            continue
        if "tests" in path.parts:
            continue
        text = path.read_text()
        if path.name == "raw_physical_finalization.rs":
            text = text.split("#[cfg(test)]", 1)[0]
        if "ModulePostprocessOwnerV1::new" in text:
            production.append(path.relative_to(ROOT))
    if production:
        raise AssertionError(f"POST-FAILURE0 has production consumers: {production}")

    print(
        "[cut0-i0-prod-activation-post-failure0-guard] ok "
        "optimizer=1 contract=1 rc_nonclaim=1 production_consumers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
