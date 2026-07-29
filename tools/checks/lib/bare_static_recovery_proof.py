#!/usr/bin/env python3
"""Validate the canonical bare-static recovery CUT0 matrix."""

from __future__ import annotations

import argparse
import json
import os
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
        root / "src/mir/builder/calls/static_resolution.rs",
        root / "src/mir/builder/calls/materializer.rs",
    ]
    consumers = sum(
        path.read_text(encoding="utf-8").count("BareStaticRecoveryDecisionV1::decide(")
        for path in consumer_paths
    )
    require(consumers == 2, f"CUT0 production consumer count must be two, got {consumers}")

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
    program_root = (root / "src/mir/builder/program_root_lowering.rs").read_text(
        encoding="utf-8"
    )
    require(
        lifecycle.count(".clear_callable_declaration_catalog()") == 1,
        "catalog clear production caller count drift",
    )
    require(
        program_root.count(".install_callable_declaration_catalog(catalog)") == 1,
        "catalog install production caller count drift",
    )
    require(
        program_root.index("VerifiedSameModuleCallableDeclarationCatalogV1::seal_root")
        < program_root.index("install_callable_declaration_catalog(catalog)")
        < program_root.index("PreparedNormalProgramDeclarationFactsV1::collect(snapshot)"),
        "catalog seal/install must precede Program declaration facts",
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
    program_root = (builder_root / "program_root_lowering.rs").read_text(encoding="utf-8")
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
        for relative in ("calls/static_resolution.rs", "calls/materializer.rs")
    )
    build_call = (builder_root / "calls/build.rs").read_text(encoding="utf-8")
    helper = (builder_root / "record_helper_args.rs").read_text(encoding="utf-8")

    require(
        production.count("pub(crate) struct VerifiedSameModuleCallableDeclarationCatalogV1")
        == 1,
        "declaration catalog definition count drift",
    )
    require(
        program_root.count("VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(source.source_ast())")
        == 1,
        "production catalog producer count drift",
    )
    require(
        program_root.count(".install_callable_declaration_catalog(catalog)") == 1,
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
        call_consumers.count(".callable_declaration_catalog()") == 2,
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
                r"(?m)^\s*static_keys_by_method_and_arity:\s*$", catalog_source
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
    require(
        "BareStaticRecoveryNoRecoveryReasonV1::NoCandidate" in build_call
        and "BareStaticRecoveryNoRecoveryReasonV1::Ambiguous" in build_call,
        "tail-recovery boundary lost explicit zero/ambiguous split",
    )
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


def run(
    argv: list[str], root: Path, env: dict[str, str] | None = None
) -> subprocess.CompletedProcess[str]:
    merged = os.environ.copy()
    if env:
        merged.update(env)
    return subprocess.run(
        argv, cwd=root, env=merged, text=True, capture_output=True, check=False
    )


def build_bins(root: Path) -> dict[str, Path]:
    common = ["cargo", "build", "-q", "--features", "vm-reference", "--bin", "hakorune"]
    for label, command in (
        ("debug", common),
        ("release", common[:2] + ["--release"] + common[2:]),
    ):
        completed = run(command, root)
        require(
            completed.returncode == 0,
            f"{label} build failed\nstdout:\n{completed.stdout}\nstderr:\n{completed.stderr}",
        )
    return {
        "debug": root / "target/debug/hakorune",
        "release": root / "target/release/hakorune",
    }


def iter_mir_calls(document: dict[str, Any]):
    for function in document.get("functions", []):
        for block in function.get("blocks", []):
            for instruction in block.get("instructions", []):
                call = instruction.get("mir_call")
                if call is not None:
                    yield function.get("name"), call


def verify_production_matrix(root: Path, bins: dict[str, Path]) -> dict[str, int]:
    pass_cases = {
        "provider_first_script": (2, "2", "Helpers.m_seed/1"),
        "caller_first_script": (2, "2", "Helpers.m_seed/1"),
        "provider_first_app": (2, "", "Helpers.m_seed/1"),
        "caller_first_app": (2, "", "Helpers.m_seed/1"),
        "cross_provider_first": (2, "2", "Provider.m_seed/1"),
        "cross_caller_first": (2, "2", "Provider.m_seed/1"),
        "arity_overload": (2, "2", "UnaryProvider.m_seed/1"),
        "zero_arg": (1, "1", "Helpers.m_seed/0"),
        "instance_control": (2, "2", "StaticProvider.m_seed/1"),
        "qualified_control": (2, "2", "Provider.m_seed/1"),
        "text_merged_main": (2, "", "TextMergedHelpers.m_seed/1"),
    }
    reject_cases = ("ambiguous", "wrong_arity", "no_candidate")
    text_merge_env = {
        "NYASH_USING_PROFILE": "dev",
        "NYASH_ENABLE_USING": "1",
        "HAKO_ENABLE_USING": "1",
        "NYASH_ALLOW_USING_FILE": "1",
        "HAKO_ALLOW_USING_FILE": "1",
        "NYASH_PREINCLUDE": "1",
        "HAKO_PREINCLUDE": "1",
    }

    for profile, binary in bins.items():
        for case, (expected_rc, expected_stdout, expected_target) in pass_cases.items():
            source = root / APP_DIR / f"{case}.hako"
            env = text_merge_env if case == "text_merged_main" else None
            runtime = run([str(binary), "--backend", "vm", str(source)], root, env)
            require(
                runtime.returncode == expected_rc and runtime.stdout.strip() == expected_stdout,
                f"{profile}/{case} runtime mismatch rc={runtime.returncode} "
                f"stdout={runtime.stdout!r} stderr={runtime.stderr!r}",
            )

            mir_path = root / ARTIFACT_DIR / f"{profile}_{case}.json"
            mir_path.parent.mkdir(parents=True, exist_ok=True)
            if mir_path.exists():
                mir_path.unlink()
            emitted = run(
                [str(binary), "--emit-mir-json", str(mir_path), str(source)], root, env
            )
            require(
                emitted.returncode == 0 and mir_path.is_file(),
                f"{profile}/{case} MIR emission failed: {emitted.stderr}",
            )
            document = json.loads(mir_path.read_text(encoding="utf-8"))
            selected = [
                (owner, call)
                for owner, call in iter_mir_calls(document)
                if call.get("callee", {}).get("name") == expected_target
            ]
            require(
                len(selected) == 1,
                f"{profile}/{case} expected one canonical target {expected_target}, got {len(selected)}",
            )

        for case in reject_cases:
            source = root / APP_DIR / f"{case}.hako"
            env = {"NYASH_BUILDER_TAIL_RESOLVE": "1"} if case == "ambiguous" else None
            rejected = run([str(binary), "--backend", "vm", str(source)], root, env)
            require(
                rejected.returncode == 1 and "Unresolved function:" in rejected.stderr,
                f"{profile}/{case} must reject without fallback: {rejected.stderr!r}",
            )

    return {
        "profiles": len(bins),
        "pass_cases": len(pass_cases),
        "reject_cases": len(reject_cases),
    }


def run_focused_tests(root: Path) -> None:
    completed = subprocess.run(
        [
            "cargo",
            "test",
            "-q",
            "--lib",
            "mir::builder::callable_declaration_catalog::recovery_tests",
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
    production_report = verify_production_matrix(root, build_bins(root))

    report = {
        "schema_version": 1,
        "row": "R0-CALLABLE-CATALOG-L0B-G0",
        "selection": "CANONICAL-CALLABLE-CATALOG-CLOSEOUT-GREEN",
        "production_behavior_delta": 1,
        "source": source_report,
        "owner": owner_report,
        "closeout": closeout_report,
        "production": production_report,
    }
    artifact = root / ARTIFACT_DIR
    artifact.mkdir(parents=True, exist_ok=True)
    (artifact / "p0_observation.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print("selection=CANONICAL-CALLABLE-CATALOG-CLOSEOUT-GREEN")
    print("decision_owner_count=1")
    print("production_consumer_count=2")
    print(f"fixture_count={source_report['fixture_count']}")
    print(f"production_pass_cases={production_report['pass_cases']}")
    print(f"production_reject_cases={production_report['reject_cases']}")
    print("production_behavior_delta=1")
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
