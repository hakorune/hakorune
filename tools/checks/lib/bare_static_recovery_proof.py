#!/usr/bin/env python3
"""Validate the canonical bare-static disposition CUT0 matrix."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any


APP_DIR = Path("apps/bare-static-recovery-proof")
ARTIFACT_DIR = Path("target/checks/bare-static-recovery-proof")
CATALOG_DIR = Path("src/mir/builder/callable_declaration_catalog")
REQUIRED = {
    "provider_first_script.hako",
    "caller_first_script.hako",
    "provider_first_app.hako",
    "caller_first_app.hako",
    "cross_provider_first.hako",
    "cross_caller_first.hako",
    "ambiguous.hako",
    "instance_control.hako",
    "arity_overload.hako",
    "zero_arg.hako",
    "wrong_arity.hako",
    "no_candidate.hako",
    "qualified_control.hako",
    "text_merged_helper.hako",
    "text_merged_main.hako",
}


class ProofFailure(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ProofFailure(message)


def read_sources(root: Path) -> dict[str, str]:
    app = root / APP_DIR
    actual = {path.name for path in app.glob("*.hako")}
    require(actual == REQUIRED, f"fixture set drift: expected={sorted(REQUIRED)} actual={sorted(actual)}")
    sources = {
        name: (app / name).read_text(encoding="utf-8") for name in sorted(REQUIRED)
    }
    for name, source in sources.items():
        require(len(source.splitlines()) < 800, f"{name} reached 800 lines")
        require(not re.search(r"\bHMI\b|\bHmi", source), f"{name} must remain HMI-independent")
    return sources


def verify_source_matrix(sources: dict[str, str]) -> dict[str, Any]:
    for mode in ("script", "app"):
        provider = sources[f"provider_first_{mode}.hako"]
        caller = sources[f"caller_first_{mode}.hako"]
        require(provider.index("m_seed(x)") < provider.index("z_use(x)"), f"{mode} provider-first drift")
        require(caller.index("a_use(x)") < caller.index("m_seed(x)"), f"{mode} caller-first drift")
        require("return m_seed(x)" in provider, f"{mode} provider-first lost bare call")
        require("return m_seed(x)" in caller, f"{mode} caller-first lost bare call")
        if mode == "app":
            require("static box Main" in provider and "static box Main" in caller, "app mode requires Main.main")
        else:
            require("static box Main" not in provider and "static box Main" not in caller, "script mode must stay top-level")

    cross_provider = sources["cross_provider_first.hako"]
    cross_caller = sources["cross_caller_first.hako"]
    require(cross_provider.index("static box Provider") < cross_provider.index("static box Consumer"), "cross provider-first drift")
    require(cross_caller.index("static box Consumer") < cross_caller.index("static box Provider"), "cross caller-first drift")

    ambiguous = sources["ambiguous.hako"]
    require(len(re.findall(r"(?m)^\s{4}m_seed\(x\)", ambiguous)) == 2, "ambiguous fixture must own two static candidates")
    require(
        ambiguous.index("static box Alpha")
        < ambiguous.index("static box Consumer")
        < ambiguous.index("static box Zeta"),
        "ambiguous fixture must expose one lowered suffix before the second candidate",
    )
    instance = sources["instance_control.hako"]
    require("box InstanceProvider" in instance, "instance control lost ordinary box")
    require("static box StaticProvider" in instance, "instance control lost static candidate")
    require("return m_seed(x)" in instance, "instance control lost bare call")

    overload = sources["arity_overload.hako"]
    require("m_seed(x)" in overload and "m_seed(x, y)" in overload, "arity overload matrix drift")
    require("m_seed()" in sources["zero_arg.hako"], "zero-arity fixture drift")
    require("m_seed(x, y)" in sources["wrong_arity.hako"], "wrong-arity provider drift")
    require("return m_seed(x)" in sources["wrong_arity.hako"], "wrong-arity call drift")
    require("return missing_seed(x)" in sources["no_candidate.hako"], "no-candidate call drift")
    require(
        "return Provider.m_seed(x)" in sources["qualified_control.hako"],
        "qualified static control drift",
    )

    using = sources["text_merged_main.hako"].splitlines()[0]
    require(using == 'using "apps/bare-static-recovery-proof/text_merged_helper.hako" as TextMergedHelpers', "text-merge using row drift")
    require("return m_seed(x)" in sources["text_merged_helper.hako"], "text-merged helper lost bare call")
    return {
        "fixture_count": len(sources),
        "same_box_order_modes": 4,
        "cross_box_orders": 2,
        "ambiguous_static_candidates": 2,
        "instance_contamination": 0,
        "text_merged_surfaces": 1,
        "hmi_mentions": 0,
    }


def verify_cutover_owner(root: Path) -> dict[str, int]:
    catalog_root = root / CATALOG_DIR
    owner = (catalog_root / "recovery.rs").read_text(encoding="utf-8")
    require(owner.count("pub(crate) enum BareStaticRecoveryDecisionV1") == 1, "decision owner count drift")
    consumer_paths = [
        root / "src/mir/builder/calls/function_call_preflight_route.rs",
        root / "src/mir/builder/calls/materializer.rs",
    ]
    consumers = sum(
        path.read_text(encoding="utf-8").count("BareStaticRecoveryDecisionV1::decide(")
        for path in consumer_paths
    )
    require(consumers == 1, f"CUT0 production consumer count must be one, got {consumers}")

    builder_sources = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
    )
    for retired in (
        "static_method_index",
        "LoweredMethodAst",
        "lowered_method_asts",
        "register_lowered_method_ast",
        "lowered_method_ast",
    ):
        require(retired not in builder_sources, f"retired authority remains in Rust: {retired}")

    lifecycle = (root / "src/mir/builder/module_lifecycle.rs").read_text(encoding="utf-8")
    normal_lifecycle = (root / "src/mir/builder/normal_default_root_catalog_lifecycle.rs").read_text(
        encoding="utf-8"
    )
    require(
        lifecycle.count(".clear_callable_declaration_catalog()") == 1,
        "catalog clear production caller count drift",
    )
    require(
        normal_lifecycle.count(".install_callable_declaration_catalog(catalog)") == 1,
        "catalog install production caller count drift",
    )
    require(
        normal_lifecycle.index("VerifiedSameModuleCallableDeclarationCatalogV1::seal_root")
        < normal_lifecycle.index("install_callable_declaration_catalog(catalog)"),
        "catalog seal must precede compatibility install",
    )
    return {"decision_owners": 1, "production_consumers": consumers}


def verify_closeout_contract(root: Path) -> dict[str, int]:
    builder_root = root / "src/mir/builder"
    production_paths = [
        path
        for path in builder_root.rglob("*.rs")
        if not path.name.endswith("_tests.rs") and path.name != "tests.rs"
    ]
    production = "\n".join(
        path.read_text(encoding="utf-8") for path in production_paths
    )
    lifecycle = (builder_root / "module_lifecycle.rs").read_text(encoding="utf-8")
    normal_lifecycle = (builder_root / "normal_default_root_catalog_lifecycle.rs").read_text(
        encoding="utf-8"
    )
    context = (builder_root / "compilation_context.rs").read_text(encoding="utf-8")
    catalog = (CATALOG_DIR / "catalog.rs")
    catalog_source = (root / catalog).read_text(encoding="utf-8")
    recovery_source = (root / CATALOG_DIR / "recovery.rs").read_text(
        encoding="utf-8"
    )
    session_errors = (root / CATALOG_DIR / "error.rs").read_text(encoding="utf-8")
    session_tests = (root / CATALOG_DIR / "tests.rs").read_text(encoding="utf-8")
    call_consumers = "\n".join(
        (builder_root / relative).read_text(encoding="utf-8")
        for relative in (
            "calls/function_call_preflight_route.rs",
            "calls/materializer.rs",
        )
    )
    build_call = (builder_root / "calls/build.rs").read_text(encoding="utf-8")
    helper = (builder_root / "record_helper_args.rs").read_text(encoding="utf-8")

    require(
        production.count("pub(crate) struct VerifiedSameModuleCallableDeclarationCatalogV1")
        == 1,
        "declaration catalog definition count drift",
    )
    require(
        normal_lifecycle.count("VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(")
        == 1,
        "production catalog producer count drift",
    )
    require(
        normal_lifecycle.count(".install_callable_declaration_catalog(catalog)") == 1,
        "catalog install-per-root count drift",
    )
    require(
        context.count("callable_declaration_catalog: Option<") == 1,
        "catalog session slot count drift",
    )
    require(
        session_errors.count("QueryBeforeInstall") == 2
        and session_errors.count("DuplicateInstall") == 2,
        "catalog session error vocabulary drift",
    )
    require(
        "SameModuleCallableDeclarationCatalogSessionErrorV1::QueryBeforeInstall"
        in session_tests
        and "SameModuleCallableDeclarationCatalogSessionErrorV1::DuplicateInstall"
        in session_tests,
        "catalog session rejection fixtures missing",
    )
    require(
        call_consumers.count(".callable_declaration_catalog()") == 1,
        "catalog recovery query count drift",
    )
    require(
        "callable_declaration_catalog().unwrap_or" not in call_consumers
        and "callable_declaration_catalog().ok()" not in call_consumers,
        "missing catalog must not become an ordinary no-candidate result",
    )

    require(
        len(
            re.findall(
                r"(?m)^\s*(?:pub\(super\)\s+)?static_keys_by_method_and_arity:\s*$",
                catalog_source,
            )
        )
        == 1,
        "static candidate index owner count drift",
    )
    require(
        "if namespace == SameModuleCallableNamespaceV1::StaticBoxMethod"
        in catalog_source,
        "static candidate insertion guard missing",
    )
    require(
        "InstanceBoxMethod" not in recovery_source,
        "instance declarations must not enter static recovery policy",
    )
    catalog_consumers = "\n".join((call_consumers, helper, lifecycle, context))
    require(
        catalog_consumers.count(".return_type_name()") == 0,
        "call-result representation gained a production catalog consumer",
    )

    for retired in (
        "static_method_index",
        "register_static_method",
        "get_static_method_candidates",
        "lowered_method_asts",
        "LoweredMethodAst",
        "register_lowered_method_ast",
        "lowered_method_ast",
    ):
        require(retired not in production, f"retired callable authority remains: {retired}")
    require(
        "generate_method_function_name" not in helper,
        "record-helper lookup must not reconstruct a physical symbol",
    )
    for retired in (
        "build_resolved_function_call",
        "try_unique_static_method_recovery",
        "try_tail_based_resolver",
        "BareStaticRecoveryEmissionV1",
    ):
        require(retired not in build_call, f"caller-zero recovery edge remains: {retired}")
    require(
        "callable_declaration_catalog" not in "\n".join(
            path.read_text(encoding="utf-8")
            for path in (builder_root / "control_flow/plan/generic_loop").rglob("*.rs")
        ),
        "GenericLoop must not consume callable catalog authority",
    )

    guarded_paths = [
        root / CATALOG_DIR / "catalog.rs",
        root / CATALOG_DIR / "error.rs",
        root / CATALOG_DIR / "recovery.rs",
        root / CATALOG_DIR / "tests.rs",
        builder_root / "calls/build.rs",
        builder_root / "calls/materializer.rs",
        builder_root / "calls/static_resolution.rs",
        builder_root / "compilation_context.rs",
        builder_root / "module_lifecycle.rs",
        builder_root / "normal_default_root_catalog_lifecycle.rs",
        builder_root / "record_helper_args.rs",
        root / "tools/checks/lib/bare_static_recovery_proof.py",
    ]
    for path in guarded_paths:
        line_count = len(path.read_text(encoding="utf-8").splitlines())
        require(line_count < 800, f"closeout source/check file reached 800 lines: {path}")

    return {
        "catalog_definitions": 1,
        "catalog_producers": 1,
        "catalog_installs_per_root": 1,
        "static_candidate_index_owners": 1,
        "result_representation_consumers": 0,
        "retired_authority_occurrences": 0,
        "generic_loop_consumers": 0,
    }


def run_focused_tests(root: Path) -> None:
    completed = subprocess.run(
        [
            "cargo",
            "test",
            "--profile",
            "quick",
            "--lib",
            "mir::builder::callable_declaration_catalog::recovery_tests",
            "--",
            "--test-threads=1",
        ],
        cwd=root,
        text=True,
        capture_output=True,
        check=False,
    )
    if completed.returncode != 0:
        raise ProofFailure(
            f"focused recovery tests failed rc={completed.returncode}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    require(
        re.search(r"(?m)^running [1-9][0-9]* tests?$", completed.stdout) is not None,
        "focused recovery filter matched zero tests",
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    args = parser.parse_args()
    root = Path(args.root).resolve()

    sources = read_sources(root)
    source_report = verify_source_matrix(sources)
    owner_report = verify_cutover_owner(root)
    closeout_report = verify_closeout_contract(root)
    run_focused_tests(root)
    report = {
        "schema_version": 1,
        "row": "R0-CALLABLE-CATALOG-L0B-G0",
        "selection": "CANONICAL-CALLABLE-CATALOG-DISPOSITION-GREEN",
        "production_behavior_delta": 0,
        "source": source_report,
        "owner": owner_report,
        "closeout": closeout_report,
        "production": {
            "runtime_cases": 0,
            "fixture_role": "catalog-decision-only",
            "legacy_vm_reference_authority": 0,
        },
    }
    artifact = root / ARTIFACT_DIR
    artifact.mkdir(parents=True, exist_ok=True)
    (artifact / "p0_observation.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print("selection=CANONICAL-CALLABLE-CATALOG-DISPOSITION-GREEN")
    print("decision_owner_count=1")
    print(f"production_consumer_count={owner_report['production_consumers']}")
    print(f"fixture_count={source_report['fixture_count']}")
    print("production_runtime_cases=0")
    print("legacy_vm_reference_authority=0")
    print("production_behavior_delta=0")
    print("g0_behavior_delta=0")
    print("catalog_definitions=1")
    print("catalog_producers=1")
    print("catalog_installs_per_root=1")
    print("static_candidate_index_owners=1")
    print("retired_authority_occurrences=0")
    print("call_result_representation_consumers=0")
    print("generic_loop_consumers=0")
    print("summary=observed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProofFailure as exc:
        print(f"[bare-static-recovery-proof] ERROR: {exc}", file=sys.stderr)
        raise SystemExit(1)
