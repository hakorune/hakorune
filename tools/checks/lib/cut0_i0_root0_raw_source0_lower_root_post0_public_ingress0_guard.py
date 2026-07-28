#!/usr/bin/env python3
"""PUBLIC-INGRESS0-S0 guard for the explicit NarrowV1 Raw entry."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-ingress0-s0-"
    "execution-task-2026-07-24.md"
)
REPAIR_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-ingress0-closeout-repair0-s0-"
    "execution-task-2026-07-24.md"
)
CALLER_MANIFEST = ROOT / "tools/checks/manifests/raw_public_cutover_caller_manifest_v1.json"
CURRENT_WORKSTREAM = ROOT / (
    "docs/development/current/main/workstreams/"
    "mirbuilder-inplace-replacement-current.md"
)
NORMAL_PIPELINE = ROOT / "src/mir/compiler/normal_default_pipeline.rs"
NORMAL_ROOT_LIFECYCLE = (
    ROOT / "src/mir/builder/normal_default_root_catalog_lifecycle.rs"
)
NORMAL_TESTS = ROOT / "src/mir/compiler/legacy_candidate_session_tests.rs"
SOURCES = (
    ROOT / "src/mir/compiler/raw_public_ingress.rs",
    ROOT / "src/mir/compiler/raw_public_ingress_p0.rs",
    ROOT / "src/mir/compiler/raw_root_publication_adapter.rs",
    ROOT / "src/mir/compiler/raw_published_compile.rs",
)

_RUST_IGNORED = re.compile(
    r"(?P<raw>r(?P<hash>#*)\".*?\"(?P=hash))"
    r"|(?P<string>(?:b|c)?\"(?:\\.|[^\"\\])*\")"
    r"|(?P<block>/\*.*?\*/)"
    r"|(?P<line>//[^\n]*)",
    re.S,
)
_CFG_TEST_MODULE = re.compile(r"#\[cfg\(test\)\]\s*mod\s+\w+")


def code_only(text: str) -> str:
    return _RUST_IGNORED.sub(
        lambda match: "".join("\n" if char == "\n" else " " for char in match.group()),
        text,
    )


def strip_cfg_test_modules(text: str) -> str:
    cursor = 0
    output: list[str] = []
    while True:
        match = _CFG_TEST_MODULE.search(text, cursor)
        if match is None:
            output.append(text[cursor:])
            return "".join(output)
        output.append(text[cursor : match.start()])
        brace = text.find("{", match.start())
        semicolon = text.find(";", match.start())
        if semicolon >= 0 and (brace < 0 or semicolon < brace):
            cursor = semicolon + 1
            continue
        if brace < 0:
            raise AssertionError("cfg(test) module without body or declaration terminator")
        depth = 0
        for end in range(brace, len(text)):
            if text[end] == "{":
                depth += 1
            elif text[end] == "}":
                depth -= 1
                if depth == 0:
                    cursor = end + 1
                    break
        else:
            raise AssertionError("unterminated cfg(test) module")


def production_paths() -> list[Path]:
    declared_test_modules: set[str] = set()
    for path in ROOT.glob("src/**/*.rs"):
        declared_test_modules.update(
            re.findall(
                r"#\[cfg\(test\)\]\s*mod\s+([A-Za-z0-9_]+)\s*;",
                path.read_text(),
            )
        )
    return sorted(
        path
        for path in ROOT.glob("src/**/*.rs")
        if path.stem not in declared_test_modules
        and not path.name.endswith("_tests.rs")
        and not path.name.endswith("_p0.rs")
        and "tests" not in path.parts
    )


def production_code(path: Path) -> str:
    return strip_cfg_test_modules(code_only(path.read_text()))


def count_by_manifest(rows: dict[str, int], token: str) -> None:
    if not rows:
        raise AssertionError(f"caller manifest has no rows for {token!r}")
    for relative, expected in rows.items():
        path = ROOT / relative
        if not path.is_file():
            raise AssertionError(f"caller manifest path missing: {relative}")
        count = production_code(path).count(token)
        if count != expected:
            raise AssertionError(
                f"caller drift in {relative}: token={token!r} expected={expected} actual={count}"
            )


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    repair_task = REPAIR_TASK.read_text()
    caller_manifest = json.loads(CALLER_MANIFEST.read_text())
    require(task, "Status: closed", "landed ingress row")
    require(repair_task, "Status: closed", "closed closeout-repair task")
    for fragment in (
        "RAW-PUBLIC-ADAPTER-prime-r1",
        "compile_raw_with_source",
        "NarrowV1",
        "compile_with_source cutover",
        "Program(JSON v0)",
        "RAW-PUBLICATION-SUNSET-001",
    ):
        require(task, fragment, f"task contract {fragment}")

    texts = {
        path: path.read_text()
        for path in (
            TASK,
            REPAIR_TASK,
            CALLER_MANIFEST,
            CURRENT_WORKSTREAM,
            NORMAL_PIPELINE,
            NORMAL_ROOT_LIFECYCLE,
            NORMAL_TESTS,
            *SOURCES,
        )
    }
    for path, text in texts.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"ingress file must remain below 800 lines: {path}")

    ingress = texts[SOURCES[0]]
    tests = texts[SOURCES[1]]
    adapter = texts[SOURCES[2]]
    compile_kernel = texts[SOURCES[3]]
    normal_pipeline = texts[NORMAL_PIPELINE]
    normal_root_lifecycle = texts[NORMAL_ROOT_LIFECYCLE]
    normal_tests = texts[NORMAL_TESTS]
    current_workstream = texts[CURRENT_WORKSTREAM]
    for fragment in (
        "compile_raw_with_source",
        "RawPublicIngressPolicyV1",
        "RawPublicImportDispositionV1",
    ):
        require(ingress, fragment, f"ingress contract {fragment}")
    for fragment in (
        "compile_raw_published_v1",
        "bind_raw_source_for_public",
        "into_root_package",
        "prepare_public_eligibility",
        "prepare_root_batch",
        "prepare_drain",
        "prepare_finalization",
        "prepare_external_commit",
        "publish_raw_direct",
    ):
        require(compile_kernel, fragment, f"compiled Raw chain {fragment}")
    for fragment in ("into_compatibility_envelope", "map_err(|rejected|"):
        require(ingress, fragment, f"ingress compatibility/error boundary {fragment}")
    for fragment in ("fn into_public_string", "self.discard()"):
        require(compile_kernel, fragment, f"typed rejection boundary {fragment}")
    require(tests, "raw_public_ingress_compiles_empty_script_without_legacy_fallback", "success fixture")
    require(tests, "raw_public_ingress_rejects_repl_before_source_binding", "REPL fixture")
    require(tests, "raw_public_ingress_reuses_one_compiler_for_two_successes", "reuse fixture")
    require(tests, "raw_public_ingress_failure_is_discarded_before_reuse", "failure/reuse fixture")
    require(adapter, "into_compatibility", "adapter handoff")

    if ingress.count("pub fn compile_raw_with_source") != 1:
        raise AssertionError("explicit Raw ingress producer must be exactly one")
    for forbidden in (
        "compile_legacy(",
        "compile_legacy_request(",
        "build_module(",
        "compile_with_source(",
        "ProgramV0Compatibility",
        "catch_unwind",
        "retry(",
        "fallback(",
    ):
        if forbidden in ingress:
            raise AssertionError(f"Raw ingress leaks forbidden route: {forbidden}")
    if "runtime/mirbuilder_emit" in ingress or "Program(JSON" in ingress:
        raise AssertionError("Raw ingress must not alter JSON/runtime bridges")

    if caller_manifest.get("schema_version") != 1:
        raise AssertionError("caller manifest schema drift")
    raw_public = caller_manifest.get("raw_public", {})
    definition_path = ROOT / raw_public.get("definition_file", "")
    definition_anchor = raw_public.get("definition_anchor", "")
    if definition_path != SOURCES[0] or definition_anchor not in ingress:
        raise AssertionError("Raw public definition manifest drift")
    production = {path: production_code(path) for path in production_paths()}
    raw_calls = [
        path.relative_to(ROOT)
        for path, text in production.items()
        if path != SOURCES[0] and "compile_raw_with_source(" in text
    ]
    if len(raw_calls) != raw_public.get("non_test_callers"):
        raise AssertionError(f"Raw public non-test caller drift: {raw_calls}")

    normal = caller_manifest.get("normal_source_hint", {})
    count_by_manifest(normal.get("no_import_callers", {}), "compile_with_source_hint(")
    count_by_manifest(
        normal.get("import_callers", {}),
        "compile_with_source_hint_and_imports(",
    )
    count_by_manifest(
        caller_manifest.get("normal_default_callers", {}),
        "compile_normal(",
    )
    count_by_manifest(
        caller_manifest.get("normal_default_construction_sites", {}),
        "NormalCompileRequestV1::for_",
    )
    for relative, expected in caller_manifest.get(
        "normal_default_legacy_reachability", {}
    ).items():
        path = ROOT / relative
        code = production_code(path)
        actual = sum(
            code.count(token)
            for token in (
                "compile_with_source_hint(",
                "compile_with_source_hint_and_imports(",
                ".compile_with_source(",
                ".compile_with_source_and_imports(",
                "compile_legacy_request(",
                "compile_legacy_candidate(",
            )
        )
        if actual != expected:
            raise AssertionError(
                f"selected normal Legacy reachability in {relative}: "
                f"expected={expected} actual={actual}"
            )
    lifecycle = caller_manifest.get("normal_default_root_catalog_lifecycle", {})
    lifecycle_path = ROOT / lifecycle.get("definition_file", "")
    caller_path = ROOT / lifecycle.get("caller_file", "")
    if lifecycle_path != NORMAL_ROOT_LIFECYCLE or caller_path != NORMAL_PIPELINE:
        raise AssertionError("normal root/catalog lifecycle path drift")
    require(
        normal_root_lifecycle,
        lifecycle.get("definition_anchor", ""),
        "normal root/catalog lifecycle owner",
    )
    caller_anchor = lifecycle.get("caller_anchor", "")
    if normal_pipeline.count(caller_anchor) != lifecycle.get("callers"):
        raise AssertionError("normal root/catalog lifecycle caller drift")
    require(
        current_workstream,
        lifecycle.get("sunset_id", ""),
        "normal compatibility sunset",
    )
    if lifecycle.get("sunset_state") != "closed":
        raise AssertionError("normal compatibility sunset must close with lifecycle cutover")
    for fragment in (
        "pub struct NormalCompileRequestV1",
        "enum NormalSourceIdentityV1",
        "enum NormalCompileAdmissionV1",
        "pub fn for_mir_mode",
        "pub fn for_minimal_mir_json",
        "pub fn for_llvm_source",
        "pub fn for_wasm_source",
        "struct NormalDefaultPublishedPipelineV1",
        "complete_normal_default_root_catalog_lifecycle",
        "prepare_external_commit",
        "finish_built_module",
    ):
        require(normal_pipeline, fragment, f"normal pipeline contract {fragment}")
    for forbidden in (
        "compile_with_source(",
        "compile_with_source_and_imports(",
        "compile_legacy(",
        "compile_legacy_request(",
        "compile_legacy_candidate(",
        "compile_raw_published_v1(",
        "retry(",
        "fallback(",
    ):
        if forbidden in code_only(normal_pipeline):
            raise AssertionError(f"normal pipeline leaks forbidden route: {forbidden}")
    for forbidden in (
        "ExistingGeneralModuleCompatibilityV1",
        ".build_module(",
        ".builder_mut()",
    ):
        if forbidden in code_only(normal_pipeline):
            raise AssertionError(f"retired normal compatibility edge returned: {forbidden}")
    for fragment in (
        "CompletedNormalDefaultRootCatalogLifecycleV1",
        "RejectedNormalDefaultRootCatalogLifecycleV1",
        "NormalDefaultRootCatalogLifecycleErrorV1",
        "RootExpansion",
        "PrepareModule",
        "CatalogSeal",
        "CatalogInstall",
        "RootLower",
        "FinalizeModule",
        "VerifiedRawRootExpansionV1::from_program",
        "prepare_module()",
        "VerifiedSameModuleCallableDeclarationCatalogV1::seal_root",
        "install_callable_declaration_catalog",
        "lower_root_after_callable_catalog_install_v1",
        "finalize_module",
    ):
        require(normal_root_lifecycle, fragment, f"normal lifecycle contract {fragment}")
    anchors = (
        "VerifiedRawRootExpansionV1::from_program",
        "prepare_module()",
        "source.ast.clone()",
        "VerifiedSameModuleCallableDeclarationCatalogV1::seal_root",
        "install_callable_declaration_catalog",
        "lower_root_after_callable_catalog_install_v1",
        "finalize_module",
    )
    positions = [normal_root_lifecycle.index(anchor) for anchor in anchors]
    if positions != sorted(positions):
        raise AssertionError("normal root/catalog lifecycle ordering drift")
    if normal_root_lifecycle.count("source.ast.clone()") != 1:
        raise AssertionError("normal root/catalog lifecycle root clone drift")
    for forbidden in (
        "build_module(",
        "compile_legacy",
        "OwnedRawSourceV1",
        "InstalledPreloopStageBContextV1",
        "retry(",
        "fallback(",
        "recover(",
    ):
        if forbidden in code_only(normal_root_lifecycle):
            raise AssertionError(f"normal lifecycle gained forbidden authority: {forbidden}")
    for fixture in (
        "late_normal_lowering_failure_leaves_live_builder_unchanged_and_reusable",
        "explicit_imports_commit_only_with_the_finished_normal_candidate",
        "normal_pipeline_matches_legacy_compatibility_for_general_module",
        "normal_pipeline_matches_legacy_compatibility_for_non_program_root",
    ):
        require(normal_tests, fixture, f"normal pipeline fixture {fixture}")
    count_by_manifest(caller_manifest.get("normal_compile_adapters", {}), ".compile(")
    expected_build_module = caller_manifest.get("direct_build_module_production", {})
    actual_build_module = {
        str(path.relative_to(ROOT)): text.count(".build_module(")
        for path, text in production.items()
        if text.count(".build_module(")
    }
    if actual_build_module != expected_build_module:
        raise AssertionError(
            "direct production build_module caller drift: "
            f"expected={expected_build_module} actual={actual_build_module}"
        )
    test_bridge = ROOT / "src/host_providers/mir_builder/lowering.rs"
    require(test_bridge.read_text(), "#[cfg(test)]\nmod ast_json;", "cfg(test) AST-JSON bridge")
    for relative in caller_manifest.get("direct_build_module_cfg_test", {}):
        if not (ROOT / relative).is_file():
            raise AssertionError(f"cfg(test) AST-JSON bridge path missing: {relative}")

    old_calls = []
    old_definitions = {
        ROOT / "src/mir/compiler/module_postprocess.rs",
        ROOT / "src/mir/compiler/external_commit.rs",
    }
    for path, text in production.items():
        if path in old_definitions:
            continue
        if "RawModuleFinalizerV1::prepare(" in text or (
            "RawPhysicalCompleteInvocationV1" in text and ".prepare_finalization(" in text
        ):
            old_calls.append(path.relative_to(ROOT))
    if old_calls:
        raise AssertionError(f"old Raw non-test callers: {old_calls}")
    if caller_manifest.get("old_raw_non_test_callers") != 0:
        raise AssertionError("old Raw caller manifest drift")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-public-ingress0-guard] ok "
        "landed=1 closeout=1 raw_non_test=0 old_raw_non_test=0 json=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
