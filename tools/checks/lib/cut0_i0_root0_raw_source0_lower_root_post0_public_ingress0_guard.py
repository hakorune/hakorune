#!/usr/bin/env python3
"""PUBLIC-INGRESS0-S0 guard for the explicit NarrowV1 Raw entry."""
from __future__ import annotations
import json
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / ("docs/development/current/main/investigations/"
               "cut0-i0-raw-source0-lower-root-post0-public-ingress0-s0-execution-task-2026-07-24.md")
REPAIR_TASK = ROOT / ("docs/development/current/main/investigations/"
                      "cut0-i0-raw-source0-lower-root-post0-public-ingress0-closeout-repair0-s0-execution-task-2026-07-24.md")
CALLER_MANIFEST = ROOT / "tools/checks/manifests/raw_public_cutover_caller_manifest_v1.json"
CURRENT_WORKSTREAM = ROOT / ("docs/development/current/main/workstreams/"
                             "mirbuilder-inplace-replacement-current.md")
NORMAL_PIPELINE = ROOT / "src/mir/compiler/normal_default_pipeline.rs"
NORMAL_ROOT_LIFECYCLE = ROOT / "src/mir/builder/normal_default_root_catalog_lifecycle.rs"
PROGRAM_ROOT_LOWERING = ROOT / "src/mir/builder/program_root_lowering.rs"
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
RAW_NONPROGRAM_ROOT_DESCENT = ROOT / "src/mir/builder/raw_nonprogram_root_descent.rs"
RAW_NONPROGRAM_ROOT_DESCENT_TESTS = ROOT / "src/mir/builder/raw_nonprogram_root_descent_tests.rs"
RAW_NONPROGRAM_ROOT_DESCENT_PARITY_TESTS = ROOT / "src/mir/builder/raw_nonprogram_root_descent_tests/parity.rs"
NORMAL_TESTS = ROOT / "src/mir/compiler/legacy_candidate_session_tests.rs"
SOURCES = (
    ROOT / "src/mir/compiler/raw_public_ingress.rs",
    ROOT / "src/mir/compiler/raw_public_ingress_p0.rs",
    ROOT / "src/mir/compiler/raw_root_publication_adapter.rs",
    ROOT / "src/mir/compiler/raw_published_compile.rs",
)

_RUST_IGNORED = re.compile(r"(?P<raw>r(?P<hash>#*)\".*?\"(?P=hash))|(?P<string>(?:b|c)?\"(?:\\.|[^\"\\])*\")"
                           r"|(?P<block>/\*.*?\*/)|(?P<line>//[^\n]*)", re.S)
_CFG_TEST_MODULE = re.compile(r"#\[cfg\(test\)\]\s*(?:#\[path\s*=\s*\"[^\"]+\"\]\s*)?mod\s+\w+")


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
                r"#\[cfg\(test\)\]\s*(?:#\[path\s*=\s*\"[^\"]+\"\]\s*)?"
                r"mod\s+([A-Za-z0-9_]+)\s*;",
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


def ast_kinds(text: str) -> set[str]:
    return set(re.findall(r"ASTNode::([A-Za-z0-9_]+)", text))


def text_between(text: str, start: str, end: str) -> str:
    start_index = text.index(start)
    end_index = text.index(end, start_index)
    return text[start_index:end_index]


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
            PROGRAM_ROOT_LOWERING,
            MODULE_LIFECYCLE,
            RAW_NONPROGRAM_ROOT_DESCENT,
            RAW_NONPROGRAM_ROOT_DESCENT_TESTS,
            RAW_NONPROGRAM_ROOT_DESCENT_PARITY_TESTS,
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
    program_root_lowering = production_code(PROGRAM_ROOT_LOWERING)
    module_lifecycle = production_code(MODULE_LIFECYCLE)
    raw_nonprogram_root_descent = production_code(RAW_NONPROGRAM_ROOT_DESCENT)
    raw_nonprogram_root_descent_tests = texts[RAW_NONPROGRAM_ROOT_DESCENT_TESTS]
    raw_nonprogram_root_descent_parity_tests = texts[
        RAW_NONPROGRAM_ROOT_DESCENT_PARITY_TESTS
    ]
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
                ".compile_legacy(",
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
    require(current_workstream, lifecycle.get("sunset_id", ""), "normal compatibility sunset")
    if lifecycle.get("sunset_state") != "closed":
        raise AssertionError("normal compatibility sunset must close with lifecycle cutover")
    for fragment in (
        "pub struct NormalCompileRequestV1",
        "enum NormalSourceIdentityV1",
        "enum NormalCompileAdmissionV1",
        "pub fn for_mir_mode",
        "pub fn for_minimal_mir_json",
        "pub fn for_llvm_source",
        "pub fn for_wasm_source", "pub(crate) fn for_repl_program",
        "pub struct RejectedNormalProgramCompileRequestV1",
        "PreparedNormalDefaultProgramRootV1::seal(ast)",
        "program: PreparedNormalDefaultProgramRootV1",
        "struct NormalDefaultPublishedPipelineV1",
        "complete_normal_default_program_root_catalog_lifecycle",
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
    for forbidden in ("ExistingGeneralModuleCompatibilityV1", ".build_module(",
                      ".builder_mut()"):
        if forbidden in code_only(normal_pipeline):
            raise AssertionError(f"retired normal compatibility edge returned: {forbidden}")
    compiler = code_only((ROOT / "src/mir/compiler/mod.rs").read_text())
    for forbidden in ("compile_legacy_candidate", "compile_legacy_request", "MirLoweringRequestV1",
                      ".build_module("):
        if forbidden in compiler:
            raise AssertionError(f"retired public compiler edge returned: {forbidden}")
    for fragment in ("fn compile_public_program", "NormalCompileRequestV1::for_mir_mode", "self.compile_normal(request)"):
        require(compiler, fragment, f"public Program admission {fragment}")
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
        "lower_normal_default_program_root_catalog_v1(&source)",
        "finalize_module",
    ):
        require(normal_root_lifecycle, fragment, f"normal lifecycle contract {fragment}")
    lifecycle_anchors = (
        "VerifiedRawRootExpansionV1::from_program",
        "prepare_module()",
        "lower_normal_default_program_root_catalog_v1(&source)",
        "finalize_module",
    )
    kernel_anchors = (
        "source.clone_lowering_statements()",
        "VerifiedSameModuleCallableDeclarationCatalogV1::seal_root",
        "install_callable_declaration_catalog",
        "lower_program_root_after_catalog_install_v1",
    )
    ordered_contracts = ((normal_root_lifecycle, lifecycle_anchors), (program_root_lowering, kernel_anchors))
    for text, anchors in ordered_contracts:
        positions = [text.index(anchor) for anchor in anchors]
        if positions != sorted(positions):
            raise AssertionError("normal root/catalog lifecycle ordering drift")
    if program_root_lowering.count("source.clone_lowering_statements()") != 1:
        raise AssertionError("normal Program kernel root clone drift")
    if normal_root_lifecycle.count("self.ast.clone()") != 1:
        raise AssertionError("normal Program source root clone implementation drift")
    for forbidden in (
        "build_module(",
        "compile_legacy",
        "NormalDefaultRootPartitionV1",
        "NonProgramCompatibility",
        "PreparedRawRootPartitionV1",
        "complete_normal_default_root_catalog_lifecycle",
        "OwnedRawSourceV1",
        "InstalledPreloopStageBContextV1",
        "retry(",
        "fallback(",
        "recover(",
    ):
        if forbidden in code_only(normal_root_lifecycle):
            raise AssertionError(f"normal lifecycle gained forbidden authority: {forbidden}")
    admission = caller_manifest.get("normal_default_program_root_admission", {})
    if ROOT / admission.get("program_kernel_file", "") != PROGRAM_ROOT_LOWERING:
        raise AssertionError("normal Program root kernel path drift")
    issuer = admission.get("request_issuer_anchor", "")
    if normal_pipeline.count(issuer) != admission.get("request_issuer_calls"):
        raise AssertionError("normal Program admission issuer drift")
    selected_call = "lower_normal_default_program_root_catalog_v1(&source)"
    if normal_root_lifecycle.count(selected_call) != admission.get("selected_lifecycle_calls"):
        raise AssertionError("selected Program lifecycle caller drift")
    if normal_root_lifecycle.count("PreparedRawRootPartitionV1") != admission.get(
        "selected_generic_partition_calls"
    ):
        raise AssertionError("selected normal generic partition drift")
    for sunset in (
        admission.get("mircompiler_compat_sunset", ""),
        admission.get("runtime_ast_json_compat_sunset", ""),
    ):
        require(current_workstream, sunset, "arbitrary-AST compatibility sunset")
    post_macro = caller_manifest.get("post_macro_program_admission", {})
    stage1_path = ROOT / post_macro.get("first_caller_file", "")
    if ROOT / post_macro.get("partition_file", "") != NORMAL_PIPELINE:
        raise AssertionError("post-macro partition path drift")
    require(normal_pipeline, post_macro.get("partition_definition_anchor", ""), "post-macro partition")
    stage1 = production_code(stage1_path)
    if stage1.count(post_macro.get("caller_anchor", "")) != post_macro.get("partition_callers"):
        raise AssertionError("post-macro partition caller drift")
    if stage1.index("maybe_expand_and_dump") > stage1.index(post_macro.get("caller_anchor", "")):
        raise AssertionError("post-macro partition must follow complete macro expansion")
    if stage1.count("compile_normal(") != post_macro.get("typed_program_calls"):
        raise AssertionError("Stage1 typed Program caller drift")
    if stage1.count("ExistingStage1DirectPostMacroCompatibilityV1::compile(") != post_macro.get("nonprogram_compatibility_calls"):
        raise AssertionError("Stage1 post-macro compatibility caller drift")
    require(stage1, post_macro.get("typed_rejection_anchor", ""), "Stage1 typed rejection")
    for key in ("sunset_id", "retire_row"):
        require(current_workstream, post_macro.get(key, ""), f"Stage1 compatibility {key}")
    require(
        program_root_lowering,
        "lower_normal_default_program_root_catalog_v1",
        "selected Program-only root kernel",
    )
    require(
        program_root_lowering,
        "lower_program_root_with_callable_port_v1",
        "shared generic Program kernel",
    )
    if module_lifecycle.count("lower_program_root_with_callable_port_v1") != admission.get(
        "generic_program_branch_calls"
    ):
        raise AssertionError("generic Program branch reuse drift")
    if "ast: ASTNode" in text_between(
        normal_pipeline,
        "pub struct NormalCompileRequestV1",
        "pub enum NormalProgramCompileRequestErrorV1",
    ):
        raise AssertionError("normal request regained bare AST authority")
    root_descent = caller_manifest.get("raw_nonprogram_root_descent", {})
    if (
        ROOT / root_descent.get("definition_file", "")
        != RAW_NONPROGRAM_ROOT_DESCENT
        or ROOT / root_descent.get("caller_file", "") != MODULE_LIFECYCLE
    ):
        raise AssertionError("raw non-Program root descent path drift")
    require(
        raw_nonprogram_root_descent,
        root_descent.get("definition_anchor", ""),
        "raw root partition owner",
    )
    caller_anchor = root_descent.get("caller_anchor", "")
    if module_lifecycle.count(caller_anchor) != root_descent.get("callers"):
        raise AssertionError("raw non-Program root descent caller drift")
    retired_anchor = root_descent.get("retired_anchor", "")
    if module_lifecycle.count(retired_anchor) != root_descent.get("retired_calls"):
        raise AssertionError("broad non-Program build_expression edge returned")
    for anchor_key, count_key in (
        ("selected_driver_anchor", "selected_driver_calls"),
        ("compatibility_driver_anchor", "compatibility_driver_calls"),
    ):
        anchor = root_descent.get(anchor_key, "")
        if raw_nonprogram_root_descent.count(anchor) != root_descent.get(count_key):
            raise AssertionError(f"raw root driver drift: {anchor_key}")
    require(
        current_workstream,
        root_descent.get("sunset_id", ""),
        "raw root compatibility sunset",
    )
    if root_descent.get("sunset_state") != "active":
        raise AssertionError("raw root compatibility sunset must remain active")
    if root_descent.get("residual_kind_count") != 36:
        raise AssertionError("raw root compatibility residual count drift")
    ast_node_kinds = ast_kinds(
        (
            ROOT
            / "crates/hakorune_frontend_ast/src/utils/node_type.rs"
        ).read_text()
    )
    classified_kinds = ast_kinds(raw_nonprogram_root_descent)
    if len(ast_node_kinds) != 57 or classified_kinds != ast_node_kinds:
        raise AssertionError(
            "raw root partition must exhaust all 57 AST kinds: "
            f"missing={sorted(ast_node_kinds - classified_kinds)} "
            f"extra={sorted(classified_kinds - ast_node_kinds)}"
        )
    selected_arms = re.findall(
        r"node\s*@\s*(.*?)=>\s*\{\s*Self::selected_(?:expr_tree|print_root|nowait_root|local_root|variable_assignment_root|variable_compound_assignment_root|return_root|plain_scope_box_root|task_scope_root)\(node\)\s*\}"
        r"|node\s*@\s*(.*?)=>\s*Self::selected_(?:expr_tree|print_root|nowait_root|local_root|variable_assignment_root|variable_compound_assignment_root|return_root|plain_scope_box_root|task_scope_root)\(node\)",
        raw_nonprogram_root_descent,
        re.S,
    )
    selected_kinds = set()
    for braced, direct in selected_arms:
        selected_kinds.update(ast_kinds(braced or direct))
    expected_selected = {
        "Literal", "Variable", "Me", "UnaryOp", "BinaryOp", "AwaitExpression",
        "CheckExpr", "ArrayLiteral", "MapLiteral", "GroupedAssignmentExpr", "Index",
        "BlockExpr", "Print", "Nowait", "Local", "Assignment", "CompoundAssignment", "Return", "ScopeBox", "TaskScope",
    }
    if selected_kinds != expected_selected:
        raise AssertionError(
            "selected raw-root kind ratchet drift: "
            f"expected={sorted(expected_selected)} actual={sorted(selected_kinds)}"
        )
    explicit_kinds = ast_kinds(
        text_between(
            raw_nonprogram_root_descent,
            "node @ (ASTNode::BoxDeclaration",
            "RawNonProgramRootCompatibilityClassV1::ExplicitRoot",
        )
    )
    if explicit_kinds != {"BoxDeclaration", "Loop"}:
        raise AssertionError(f"explicit root compatibility drift: {sorted(explicit_kinds)}")
    separate_kinds = ast_kinds(
        text_between(
            raw_nonprogram_root_descent,
            "node @ (ASTNode::If",
            "RawNonProgramRootCompatibilityClassV1::SeparateDesignStop",
        )
    )
    expected_separate = {
        "If", "QMarkPropagate", "MatchExpr",
        "EnumMatchExpr", "RecordLiteral",
        "RecordUpdate", "Lambda", "TryCatch", "Throw",
        "MethodCall", "FieldAccess", "New",
        "FromCall", "FunctionCall", "Call",
    }
    if separate_kinds != expected_separate:
        raise AssertionError(
            "separate-design root compatibility drift: "
            f"expected={sorted(expected_separate)} actual={sorted(separate_kinds)}"
        )
    outside_kinds = ast_kinds(
        text_between(
            raw_nonprogram_root_descent,
            "node @ (ASTNode::LoopRange",
            "RawNonProgramRootCompatibilityClassV1::OutsideNormalFileIngress",
        )
    )
    expected_outside = ast_node_kinds - expected_selected - explicit_kinds - expected_separate - {
        "Program"
    }
    if len(expected_outside) != 19 or outside_kinds != expected_outside:
        raise AssertionError(
            "outside-normal root compatibility drift: "
            f"expected={sorted(expected_outside)} actual={sorted(outside_kinds)}"
        )
    actual_residual_kind_count = len(explicit_kinds | separate_kinds | outside_kinds)
    if actual_residual_kind_count != root_descent.get("residual_kind_count"):
        raise AssertionError(
            "raw root residual ratchet drift: "
            f"manifest={root_descent.get('residual_kind_count')} "
            f"actual={actual_residual_kind_count}"
        )
    for fragment in (
        "node @ ASTNode::AwaitExpression { .. } if is_port_neutral_expr_tree(&node)",
        "ASTNode::AwaitExpression { expression, .. } => is_port_neutral_expr_tree(expression)",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive Await partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::AwaitExpression { .. }") != 2:
        raise AssertionError("Await root must have one safe and one compatibility arm")
    for fragment in (
        "node @ ASTNode::CheckExpr { .. } if is_port_neutral_expr_tree(&node)",
        ".all(|item| is_port_neutral_expr_tree(&item.expression))",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive Check partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::CheckExpr { .. }") != 2:
        raise AssertionError("Check root must have one safe and one compatibility arm")
    for fragment in (
        "node @ ASTNode::ArrayLiteral { .. } if is_port_neutral_expr_tree(&node)",
        "ASTNode::ArrayLiteral { elements, .. }",
        "elements.iter().all(is_port_neutral_expr_tree)",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive Array partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::ArrayLiteral { .. }") != 2:
        raise AssertionError("Array root must have one safe and one compatibility arm")
    for fragment in (
        "node @ ASTNode::MapLiteral { .. } if is_port_neutral_expr_tree(&node)",
        "ASTNode::MapLiteral { entries, .. }",
        ".all(|(_, value)| is_port_neutral_expr_tree(value))",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive Map partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::MapLiteral { .. }") != 2:
        raise AssertionError("Map root must have one safe and one compatibility arm")
    for fragment in (
        "node @ ASTNode::GroupedAssignmentExpr { .. }",
        "if is_port_neutral_expr_tree(&node)",
        "ASTNode::GroupedAssignmentExpr { rhs, .. }",
        "is_port_neutral_expr_tree(rhs)",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive Grouped Assignment partition")
    if (
        raw_nonprogram_root_descent.count(
            "node @ ASTNode::GroupedAssignmentExpr { .. }"
        )
        != 2
    ):
        raise AssertionError(
            "Grouped Assignment root must have one safe and one compatibility arm"
        )
    for fragment in (
        "node @ ASTNode::Index { .. } if is_port_neutral_expr_tree(&node)",
        "ASTNode::Index { target, index, .. }",
        "is_port_neutral_expr_tree(target) && is_port_neutral_expr_tree(index)",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive Index partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::Index { .. }") != 2:
        raise AssertionError("Index root must have one safe and one compatibility arm")
    for fragment in (
        "node @ ASTNode::BlockExpr { .. } if is_port_neutral_expr_tree(&node)",
        "ASTNode::BlockExpr {",
        ".all(is_port_neutral_block_prelude_stmt)",
        "is_port_neutral_expr_tree(tail_expr)",
        "fn is_port_neutral_block_prelude_stmt(node: &ASTNode) -> bool",
        "is_port_neutral_print_root(node)",
        "is_port_neutral_nowait_root(node)",
        "is_port_neutral_local_root(node)",
        "is_port_neutral_plain_scope_box_root(node)", "is_port_neutral_task_scope_root(node)",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive BlockExpr prelude partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::BlockExpr { .. }") != 2:
        raise AssertionError("BlockExpr root must have one safe and one compatibility arm")
    expected_block_prelude = ["expr_tree", "print", "nowait", "annotation_free_local", "variable_assignment", "variable_compound_assignment", "plain_scope_box", "task_scope"]
    if root_descent.get("selected_block_prelude_responsibilities") != expected_block_prelude:
        raise AssertionError("selected BlockExpr prelude responsibility ratchet drift")
    if root_descent.get("safe_nonempty_block_compatibility_edge") != 0:
        raise AssertionError("safe non-empty BlockExpr compatibility edge must remain zero")
    for fragment in (
        "node @ ASTNode::Print { .. } if is_port_neutral_print_root(&node)",
        "SelectedRawNonProgramRootV1", "fn is_port_neutral_return_root",
        "PortNeutralPrintRootV1",
        "SelectedRawNonProgramRootV1::PrintRoot",
        "root.into_node()",
    ):
        require(raw_nonprogram_root_descent, fragment, "selected Print root partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::Print { .. }") != 2:
        raise AssertionError("Print root must have one safe and one compatibility arm")
    for fragment in (
        "node @ ASTNode::Nowait { .. } if is_port_neutral_nowait_root(&node)",
        "PortNeutralNowaitRootV1",
        "SelectedRawNonProgramRootV1::NowaitRoot",
    ):
        require(raw_nonprogram_root_descent, fragment, "selected Nowait root partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::Nowait { .. }") != 2:
        raise AssertionError("Nowait root must have one safe and one compatibility arm")
    for fragment in (
        "node @ ASTNode::Local { .. } if is_port_neutral_local_root(&node)",
        "declared_type_names.iter().all(Option::is_none)",
        ".is_none_or(is_port_neutral_expr_tree)",
        "PortNeutralLocalRootV1",
        "SelectedRawNonProgramRootV1::LocalRoot",
    ):
        require(raw_nonprogram_root_descent, fragment, "annotation-free Local partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::Local { .. }") != 2:
        raise AssertionError("Local root must have one safe and one compatibility arm")
    for kind, predicate in (("Assignment", "is_port_neutral_variable_assignment_root"), ("CompoundAssignment", "is_port_neutral_variable_compound_assignment_root")):
        for fragment in (f"node @ ASTNode::{kind} {{ .. }}", f"{predicate}(&node)",
                         f"fn {predicate}(node: &ASTNode) -> bool"):
            require(raw_nonprogram_root_descent, fragment, f"variable {kind} partition")
        if raw_nonprogram_root_descent.count(f"node @ ASTNode::{kind} {{ .. }}") != 2:
            raise AssertionError(f"{kind} root must have safe and compatibility arms")
    for fragment in (
        "node @ ASTNode::TaskScope { .. } if is_port_neutral_task_scope_root(&node)",
        "fn is_port_neutral_task_scope_root(node: &ASTNode) -> bool",
        "body.iter().all(is_port_neutral_block_prelude_stmt)",
        "PortNeutralTaskScopeRootV1",
        "SelectedRawNonProgramRootV1::TaskScopeRoot",
    ):
        require(raw_nonprogram_root_descent, fragment, "recursive TaskScope partition")
    if raw_nonprogram_root_descent.count("node @ ASTNode::TaskScope { .. }") != 2:
        raise AssertionError("TaskScope root must have one safe and one compatibility arm")
    if root_descent.get("safe_task_scope_compatibility_edge") != 0:
        raise AssertionError("safe TaskScope compatibility edge must remain zero")
    for fragment in (
        "PreparedRawNonProgramRootV1",
        "SelectedRawNonProgramRootV1",
        "PortNeutralExprTreeV1",
        "ExistingRawNonProgramRootCompatibilityV1",
        "RawNonProgramRootCompatibilityClassV1::ExplicitRoot",
        "RawNonProgramRootCompatibilityClassV1::SeparateDesignStop",
        "RawNonProgramRootCompatibilityClassV1::OutsideNormalFileIngress",
    ):
        require(raw_nonprogram_root_descent, fragment, f"raw root partition {fragment}")
    for forbidden in (
        "build_expression(",
        ".clone()",
        "NyashParser",
        "parse_",
        "retry(",
        "fallback(",
        "or_else(",
        "unreachable!(",
        "panic!(",
    ):
        if forbidden in raw_nonprogram_root_descent:
            raise AssertionError(f"raw root partition gained forbidden surface: {forbidden}")
    if re.search(r"\b_\s*=>", raw_nonprogram_root_descent):
        raise AssertionError("raw root partition must not use a wildcard AST arm")
    require(
        texts[RAW_NONPROGRAM_ROOT_DESCENT],
        '#[path = "raw_nonprogram_root_descent_tests.rs"]\nmod tests;',
        "path-bound raw root test seam",
    )
    if "fn port_neutral_partition_is_recursive_and_disjoint" in texts[
        RAW_NONPROGRAM_ROOT_DESCENT
    ]:
        raise AssertionError("raw root partition tests must remain outside production source")
    require(
        raw_nonprogram_root_descent_tests,
        '#[path = "raw_nonprogram_root_descent_tests/parity.rs"]\nmod parity;',
        "private raw root parity test child",
    )
    for fixture in (
        "port_neutral_partition_is_recursive_and_disjoint",
        "program_box_and_loop_keep_their_existing_root_owners",
    ):
        require(
            raw_nonprogram_root_descent_tests,
            fixture,
            f"raw root partition fixture {fixture}",
        )
        if fixture in raw_nonprogram_root_descent_parity_tests:
            raise AssertionError(f"raw root partition fixture moved into parity child: {fixture}")
    for fixture in (
        "selected_print_root_matches_the_raw_legacy_port_exactly",
        "selected_nowait_root_matches_raw_legacy_effects_exactly",
        "selected_grouped_assignment_matches_raw_legacy_effects_exactly",
        "selected_grouped_assignment_preflights_and_reuses_without_retry",
        "selected_index_matches_raw_legacy_effects_exactly", "selected_safe_return_root_matches_raw_legacy_without_retry", "selected_plain_scope_box_composes_without_retry",
        "selected_safe_block_prelude_matches_raw_legacy_effects_exactly",
        "selected_block_prelude_local_keeps_existing_scope_failure",
        "selected_task_scope_matches_raw_legacy_effects_exactly",
        "selected_task_scope_child_failure_keeps_pop_order_without_retry",
        "selected_empty_block_expr_matches_raw_legacy_effects_exactly",
    ):
        if fixture in raw_nonprogram_root_descent_tests:
            raise AssertionError(f"raw root parity fixture remains in parent hub: {fixture}")
        require(
            raw_nonprogram_root_descent_parity_tests,
            fixture,
            f"raw root parity fixture {fixture}",
        )
    for fixture in (
        "late_normal_lowering_failure_leaves_live_builder_unchanged_and_reusable",
        "explicit_imports_commit_only_with_the_finished_normal_candidate",
        "normal_pipeline_matches_legacy_compatibility_for_general_module",
        "program_v0_typed_failure_keeps_live_builder_reusable_without_retry", "repl_program_matches_legacy_config_and_failure_reuse",
        "program_v0_typed_errors_match_legacy_program_stages_exactly",
        "public_program_admission_rejects_non_program_roots",
        "rejected_nonprogram_admission_leaves_live_builder_unchanged_and_reusable",
        "responsibility_local_nonprogram_roots_share_public_admission_and_reuse",
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
    runtime_emit = (ROOT / "src/runtime/mirbuilder_emit.rs").read_text()
    if "json_to_ast" in runtime_emit or "lower_ast_json_to_module" in runtime_emit: raise AssertionError("runtime AST-JSON compatibility must remain retired")
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
