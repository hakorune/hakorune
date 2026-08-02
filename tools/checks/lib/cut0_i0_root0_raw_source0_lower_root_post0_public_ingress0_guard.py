#!/usr/bin/env python3
"""PUBLIC-INGRESS0-S0 guard for the explicit NarrowV1 Raw entry."""
from __future__ import annotations
import json, re
from pathlib import Path
from r4_fence_registry_evidence import validate_r4_fence_registry
from script_r4_ratchet_evidence import validate_script_r4_ratchet_evidence
ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / "docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-post0-public-ingress0-s0-execution-task-2026-07-24.md"
REPAIR_TASK = ROOT / "docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-post0-public-ingress0-closeout-repair0-s0-execution-task-2026-07-24.md"
CALLER_MANIFEST = ROOT / "tools/checks/manifests/raw_public_cutover_caller_manifest_v1.json"
CURRENT_WORKSTREAM = ROOT / "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md"
NORMAL_PIPELINE = ROOT / "src/mir/compiler/normal_default_pipeline.rs"
NORMAL_ROOT_LIFECYCLE = ROOT / "src/mir/builder/normal_default_root_catalog_lifecycle.rs"; NORMAL_COLLECTOR_DRAIN = ROOT / "src/mir/builder/module_draft_collector/normal_collector_drain_lifecycle.rs"
PROGRAM_ROOT_LOWERING, PROGRAM_STATIC_TABLE_METADATA, MODULE_FINALIZATION_DECLARATION_METADATA, MODULE_FINALIZATION_FUNCTION_METADATA = ROOT / "src/mir/builder/program_root_lowering.rs", ROOT / "src/mir/builder/program_static_table_metadata.rs", ROOT / "src/mir/builder/module_finalization_declaration_metadata.rs", ROOT / "src/mir/builder/module_finalization_function_metadata.rs"
DECLS = ROOT / "src/mir/builder/decls.rs"; RAW_STATIC_MAIN_COMPAT = ROOT / "src/mir/builder/raw_static_main_compat_batch.rs"
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"; TOP_LEVEL_ADMISSION = ROOT / "src/mir/builder/normal_top_level_function_admission.rs"; CONSTRUCTOR_ADMISSION = ROOT / "src/mir/builder/normal_instance_constructor_admission.rs"
BUILDER_ROOT = ROOT / "src/mir/builder.rs"
NORMAL_TESTS = ROOT / "src/mir/compiler/legacy_candidate_session_tests.rs"
RAW_EXPRESSION_DISPATCH, QMARK, CHECK, INDIRECT_CALL = (ROOT / "src/mir/builder/raw_expression_dispatch/mod.rs", ROOT / "src/mir/builder/exprs_qmark.rs", ROOT / "src/mir/builder/exprs_check.rs", ROOT / "src/mir/builder/exprs_call.rs")
MATCH_EXPRESSION_OWNER = ROOT / "src/mir/builder/exprs_peek.rs"
BUILDER_BUILD = ROOT / "src/mir/builder/builder_build.rs"
FUNCTION_CALL_PREFLIGHT = ROOT / "src/mir/builder/calls/function_call_preflight_route.rs"
SPECIAL_METHOD_HANDLERS = ROOT / "src/mir/builder/calls/special_method_handlers.rs"
RESERVED_METHOD_ROUTE = ROOT / "src/mir/builder/calls/reserved_method_route.rs"
RESERVED_METHOD_TESTS = ROOT / "src/mir/builder/calls/reserved_method_route_tests.rs"
FASTMEM_CALLS = ROOT / "src/mir/builder/fastmem/calls.rs"
CALL_BUILD = ROOT / "src/mir/builder/calls/build.rs"
RAW_UNARY_OWNER = ROOT / "src/mir/builder/ops/unary.rs"
OPS_MOD = ROOT / "src/mir/builder/ops/mod.rs"
CALLABLE_SEMANTIC_SOURCE, CALLABLE_SEMANTIC_LOAN, CATALOGED_METHOD_LOWERING = (ROOT / "src/mir/builder/normal_callable_semantic_source.rs", ROOT / "src/mir/builder/normal_callable_semantic_loan_port.rs", ROOT / "src/mir/builder/normal_cataloged_box_method_lowering.rs")
CALLABLE_CATALOG, CALLABLE_RESOLVER, RAW_SOURCE_TRANSPORT = ROOT / "src/mir/builder/callable_declaration_catalog/catalog.rs", ROOT / "src/mir/resolved_semantics/owner_resolver.rs", ROOT / "src/mir/builder/raw_invocation_source_transport.rs"
CALLABLE_LEDGER, CALLABLE_MATERIALIZATION, CALLABLE_ENTRY_PORT = map(lambda name: ROOT / f"src/mir/builder/{name}.rs", ("normal_callable_semantic_lowering_state", "normal_callable_binding_materialization", "normal_callable_binding_materialization_port"))
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
            NORMAL_ROOT_LIFECYCLE, NORMAL_COLLECTOR_DRAIN,
            PROGRAM_ROOT_LOWERING, PROGRAM_STATIC_TABLE_METADATA, MODULE_FINALIZATION_DECLARATION_METADATA, MODULE_FINALIZATION_FUNCTION_METADATA,
            RAW_STATIC_MAIN_COMPAT, TOP_LEVEL_ADMISSION,
            MODULE_LIFECYCLE,
            BUILDER_ROOT,
            NORMAL_TESTS,
            RAW_EXPRESSION_DISPATCH, QMARK, CHECK,
            BUILDER_BUILD,
            FUNCTION_CALL_PREFLIGHT,
            SPECIAL_METHOD_HANDLERS,
            RESERVED_METHOD_ROUTE,
            RESERVED_METHOD_TESTS,
            FASTMEM_CALLS,
            CALL_BUILD,
            RAW_UNARY_OWNER,
            OPS_MOD,
            CALLABLE_SEMANTIC_SOURCE, CALLABLE_SEMANTIC_LOAN, CATALOGED_METHOD_LOWERING,
            CALLABLE_CATALOG, CALLABLE_RESOLVER, RAW_SOURCE_TRANSPORT,
            CALLABLE_LEDGER, CALLABLE_MATERIALIZATION, CALLABLE_ENTRY_PORT,
            *SOURCES,
        )
    }
    for path, text in texts.items():
        if path.suffix in {".rs", ".py", ".sh"} and len(text.splitlines()) >= 800:
            raise AssertionError(f"ingress file must remain below 800 lines: {path}")
    ingress = texts[SOURCES[0]]
    tests = texts[SOURCES[1]]
    adapter = texts[SOURCES[2]]
    compile_kernel = texts[SOURCES[3]]
    normal_pipeline = texts[NORMAL_PIPELINE]
    normal_root_lifecycle = texts[NORMAL_ROOT_LIFECYCLE]; normal_collector_drain = texts[NORMAL_COLLECTOR_DRAIN]
    callable_source, callable_loan, callable_catalog, callable_resolver, raw_source_transport = map(production_code, (CALLABLE_SEMANTIC_SOURCE, CALLABLE_SEMANTIC_LOAN, CALLABLE_CATALOG, CALLABLE_RESOLVER, RAW_SOURCE_TRANSPORT))
    callable_ledger, callable_materialization, callable_entry_port = map(production_code, (CALLABLE_LEDGER, CALLABLE_MATERIALIZATION, CALLABLE_ENTRY_PORT))
    program_root_lowering = production_code(PROGRAM_ROOT_LOWERING)
    decls = production_code(DECLS)
    raw_static_main_compat = production_code(RAW_STATIC_MAIN_COMPAT)
    module_lifecycle = production_code(MODULE_LIFECYCLE)
    builder_root = production_code(BUILDER_ROOT)
    normal_tests = texts[NORMAL_TESTS]
    raw_expression_dispatch = code_only(texts[RAW_EXPRESSION_DISPATCH])
    qmark, check, indirect_call = production_code(QMARK), production_code(CHECK), production_code(INDIRECT_CALL)
    builder_build = production_code(BUILDER_BUILD)
    function_call_preflight = production_code(FUNCTION_CALL_PREFLIGHT)
    function_call_preflight_tests = texts[FUNCTION_CALL_PREFLIGHT]
    special_method_handlers = production_code(SPECIAL_METHOD_HANDLERS)
    special_method_handler_tests = texts[SPECIAL_METHOD_HANDLERS]
    reserved_method_route = production_code(RESERVED_METHOD_ROUTE)
    reserved_method_tests = texts[RESERVED_METHOD_TESTS]
    fastmem_calls = production_code(FASTMEM_CALLS)
    call_build = production_code(CALL_BUILD)
    raw_unary_owner = code_only(texts[RAW_UNARY_OWNER])
    ops_mod = code_only(texts[OPS_MOD])
    current_workstream = texts[CURRENT_WORKSTREAM]
    for fragment in (
        "enum PreparedRawOrdinaryFunctionCompletionV1",
        "StrNormalization",
        "Resolved",
        "fn prepare_ordinary_function_completion_v1",
    ):
        require(function_call_preflight, fragment, f"direct str route {fragment}")
    if function_call_preflight.count(
        "lower_prepared_raw_ordinary_function_completion_with_port_v1("
    ) != 1:
        raise AssertionError("direct str prepared consumer count drift")
    for retired in (
        "lower_ordinary_function_call_with_port_v1",
        'if name == "str" && arg_values.len() == 1',
    ):
        if retired in call_build:
            raise AssertionError(f"post-child direct str authority returned: {retired}")
    if call_build.count(
        "fn lower_prepared_raw_ordinary_function_completion_with_port_v1"
    ) != 1:
        raise AssertionError("direct str completion owner count drift")
    str_prepare = text_between(
        function_call_preflight,
        "fn prepare_ordinary_function_completion_v1",
        "fn prepare_typeop_route",
    )
    for forbidden in (
        "MirBuilder",
        "drive_legacy_expression_v1",
        "next_value_id",
        "emit_",
        "fallback",
        "retry",
        ".clone()",
    ):
        if forbidden in str_prepare:
            raise AssertionError(f"direct str prepare gained effect/retry edge: {forbidden}")
    for fragment in (
        "enum PreparedRawExplicitExternCallV1",
        "PreparedRawExplicitExternCallV1::MissingTarget",
        "PreparedRawExplicitExternCallV1::TargetMustBeString",
        "PreparedRawExplicitExternCallV1::Ready",
        "fn lower_prepared_raw_explicit_extern_call_with_port_v1",
    ):
        require(function_call_preflight, fragment, f"explicit extern preflight {fragment}")
    for fragment in (
        "super::special_handlers::extract_string_literal(target)",
        "super::extern_calls::explicit_extern_return_type(&extern_name)",
        "super::extern_calls::split_explicit_extern_name(&extern_name)",
        "arguments.into_iter().skip(1).collect()",
    ):
        if function_call_preflight.count(fragment) != 1:
            raise AssertionError(f"explicit extern prepare count drift: {fragment}")
    for retired in (
        "build_explicit_extern_call_with_port_v1",
        "args.is_empty()",
        "extract_string_literal(&args[0])",
        "&args[1..]",
    ):
        if retired in call_build:
            raise AssertionError(f"explicit extern lower-side authority returned: {retired}")
    require(
        function_call_preflight_tests,
        "explicit_extern_preflight_defers_rejection_and_preserves_stringbox_target",
        "explicit extern target/child evidence",
    )
    for fragment in (
        "enum PreparedRawBrandConstructorV1",
        "PreparedRawBrandConstructorV1::ArityMismatch",
        "PreparedRawBrandConstructorV1::Ready",
        "fn lower_prepared_raw_brand_constructor_with_port_v1",
    ):
        require(function_call_preflight, fragment, f"Brand preflight {fragment}")
    if function_call_preflight.count("PreparedRawBrandConstructorV1::prepare(") != 1:
        raise AssertionError("Brand arity/source receipt issuer count drift")
    for retired in (
        "build_brand_constructor_call_with_port_v1",
        "PreparedRawFunctionPreflightRouteV1::Brand { arguments }",
    ):
        if retired in call_build or retired in function_call_preflight:
            raise AssertionError(f"Brand lower-side authority returned: {retired}")
    require(
        function_call_preflight_tests,
        "rejecting_routes_precede_children_and_typeop_uses_one_child",
        "Brand arity rejection precedes child evidence",
    )
    for fragment in (
        "enum PreparedRawMathArgumentV1",
        "PreparedRawMathArgumentV1::Direct",
        "PreparedRawMathArgumentV1::IntegerBoxToFloat",
        "fn prepare_raw_math_arguments_v1",
        "fn lower_math_function_with_port_v1",
    ):
        require(special_method_handlers, fragment, f"Math argument Recipe {fragment}")
    if function_call_preflight.count(
        "special_method_handlers::prepare_raw_math_arguments_v1("
    ) != 1:
        raise AssertionError("direct Math argument Recipe issuer count drift")
    math_prepare = text_between(
        special_method_handlers, "impl PreparedRawMathArgumentV1", "impl MirBuilder"
    )
    math_lower = text_between(
        special_method_handlers,
        "fn lower_math_function_with_port_v1",
        "fn build_str_normalization",
    )
    for forbidden in ("MirBuilder", "drive_legacy_expression_v1", "emit_", "next_value_id"):
        if forbidden in math_prepare:
            raise AssertionError(f"Math argument prepare gained effect edge: {forbidden}")
    for retired in ("ASTNode::", "class ==", "arguments.len()", "raw_args"):
        if retired in math_lower:
            raise AssertionError(f"Math lower-side source classifier returned: {retired}")
    for evidence in (
        "math_argument_recipes_preserve_exact_wrapper_boundary",
        "math_argument_failure_stops_suffix_and_mathbox_effects",
    ):
        require(special_method_handler_tests, evidence, f"Math Recipe evidence {evidence}")
    for fragment in (
        "struct PreparedFastMemIntrinsicV1",
        "PreparedFastMemIntrinsicRouteV1::Selected",
        "PreparedFastMemIntrinsicRouteV1::Forbidden",
        "PreparedFastMemIntrinsicRouteV1::ArityMismatch",
    ):
        require(fastmem_calls, fragment, f"FastMem prepared receipt {fragment}")
    if fastmem_calls.count("let route = match lookup_fastmem_intrinsic(name)") != 1:
        raise AssertionError("FastMem vocabulary lookup issuer count drift")
    for retired in (
        "if arguments.len() != expected",
        "ensure_no_fastmem_args",
        "lower_fastmem_function_call_with_port_v1",
        "lower_fastmem_method_call_with_port",
    ):
        if retired in fastmem_calls:
            raise AssertionError(f"FastMem lower-side authority returned: {retired}")
    if function_call_preflight.count("PreparedFastMemIntrinsicV1::prepare(") != 1:
        raise AssertionError("direct FastMem receipt issuer count drift")
    if reserved_method_route.count("PreparedFastMemIntrinsicV1::prepare(") != 1:
        raise AssertionError("method FastMem receipt issuer count drift")
    for fragment in (
        "fn lower_prepared_fastmem_function_call_with_port_v1",
        "fn lower_prepared_fastmem_method_call_with_port_v1",
    ):
        if fastmem_calls.count(fragment) != 1:
            raise AssertionError(f"FastMem prepared terminal definition drift: {fragment}")
    require(
        reserved_method_tests,
        "selected_fastmem_forbidden_failure_precedes_argument_effects",
        "FastMem forbidden pre-child evidence",
    )
    for fragment in (
        "struct PreparedRawNewExpressionV1",
        "enum PreparedRawNewExpressionRouteV1",
        "PreparedRawNewExpressionRouteV1::Core13Pure",
        "PreparedRawNewExpressionRouteV1::IntegerLiteral",
        "PreparedRawNewExpressionRouteV1::Ordinary",
    ):
        require(builder_build, fragment, f"prepared raw New route {fragment}")
    for fragment in (
        "PreparedRawNewExpressionV1::prepare(",
        "self.lower_prepared_raw_new_expression_with_port_v1(port, prepared)",
    ):
        if raw_expression_dispatch.count(fragment) != 1:
            raise AssertionError(f"raw New production handoff drift: {fragment}")
    new_prepare = text_between(builder_build, "fn prepare(", "impl MirBuilder")
    new_lower = builder_build.split(
        "fn lower_prepared_raw_new_expression_with_port_v1", 1
    )[1]
    if new_prepare.count("crate::config::env::mir_core13_pure()") != 1:
        raise AssertionError("raw New mode route must be selected exactly once")
    for retired in (
        "crate::config::env::mir_core13_pure()",
        '("IntegerBox",',
        "fn into_parts",
    ):
        if retired in new_lower:
            raise AssertionError(f"lower-side raw New redecision returned: {retired}")
    for forbidden in (
        "next_value_id(",
        "emit_instruction(",
        "drive_legacy_expression_v1(",
        "retry(",
        "fallback(",
        ".clone()",
        "NyashParser",
    ):
        if forbidden in new_prepare:
            raise AssertionError(f"raw New prepare gained effect/retry edge: {forbidden}")
    for fragment in (
        "struct PreparedRawUnaryV1",
        "enum PreparedRawUnaryRouteV1",
        "fn prepare(operator: UnaryOperator, operand: ASTNode) -> Self",
        "fn lower_prepared_raw_unary_with_port_v1<Port>",
        "fn lower_prepared_raw_ordinary_unary_with_port_v1<Port>",
    ):
        require(raw_unary_owner, fragment, f"raw Unary owner {fragment}")
    for fragment in (
        "PreparedRawUnaryV1::prepare(operator, *operand)",
        "lower_prepared_raw_unary_with_port_v1(self, port, prepared)",
    ):
        if raw_expression_dispatch.count(fragment) != 1:
            raise AssertionError(f"raw Unary production handoff drift: {fragment}")
    for retired in (
        "build_unary_op_with_port_v1",
        "fn build_unary_op(",
        "emit_weak_new",
        "UnaryOperator::Weak",
        '"-".to_string()',
        '"not".to_string()',
        '"~".to_string()',
    ):
        if retired in raw_expression_dispatch:
            raise AssertionError(f"retired raw Unary dispatcher policy returned: {retired}")
    if "fn build_unary_op(" in ops_mod:
        raise AssertionError("caller-zero MirBuilder Unary facade returned")
    if "fn build_qmark_propagate_expression(" in qmark or "RawLegacyChildLoweringPortV1" in qmark or raw_expression_dispatch.count("port.has_qmark_propagation_receipt_v1(&node)") != 1 or raw_expression_dispatch.count("self.build_qmark_propagate_expression_with_port_v1(port, *expression)") != 2 or "fn build_check_expression(" in check or "RawLegacyChildLoweringPortV1" in check or raw_expression_dispatch.count("self.build_check_expression_with_port_v1(port, items, sources)") != 1 or "fn build_indirect_call_expression(" in indirect_call or "RawLegacyChildLoweringPortV1" in indirect_call or raw_expression_dispatch.count("self.build_indirect_call_expression_with_port_v1(port, *callee, arguments)") != 1:
        raise AssertionError("caller-zero QMark/Check/indirect-Call facade or selected port handoff drift")
    if raw_unary_owner.count("drive_legacy_expression_v1(builder, port, operand)") != 2:
        raise AssertionError("raw Unary Weak/Ordinary operand descent count drift")
    if raw_unary_owner.count("builder.emit_weak_new(box_value)") != 1:
        raise AssertionError("raw Unary Weak completion count drift")
    for forbidden in (
        "RawLegacyChildLoweringPortV1",
        "retry(",
        "fallback(",
        "or_else(",
    ):
        if forbidden in raw_unary_owner:
            raise AssertionError(f"raw Unary owner gained forbidden edge: {forbidden}")
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
    if ROOT / lifecycle.get("definition_file", "") != NORMAL_ROOT_LIFECYCLE or ROOT / lifecycle.get("caller_file", "") != NORMAL_PIPELINE:
        raise AssertionError("normal root/catalog lifecycle path drift")
    require(normal_root_lifecycle, lifecycle.get("definition_anchor", ""), "normal root/catalog lifecycle owner")
    caller_anchor = lifecycle.get("caller_anchor", "")
    if normal_pipeline.count(caller_anchor) != lifecycle.get("callers"):
        raise AssertionError("normal root/catalog lifecycle caller drift")
    require(current_workstream, lifecycle.get("sunset_id", ""), "normal compatibility sunset")
    if lifecycle.get("sunset_state") != "closed": raise AssertionError("normal compatibility sunset must close with lifecycle cutover")
    if normal_root_lifecycle.count("VerifiedNormalCallableSemanticSourceV1::seal(") != 1 or program_root_lowering.count("loan.complete()?") != 1: raise AssertionError("callable semantic issuer/consumer drift")
    if not all(x in callable_source for x in ("VerifiedNormalCallableSemanticSourceV1", "if !is_app_mode && !inventory.blockers().is_empty()", "resolve_selected_callable_forests", "seal_with_root_profile", "fn loan")) or "RawInvocationSourceTransportV1::root" in callable_source: raise AssertionError("callable semantic source authority drift")
    if not all(x in callable_loan for x in ("NormalCallableSemanticLoanPortV1", "CallableLoanConsumptionV1", "with_callable_source_scope", "semantic_ledger.take()", "fn consume", "fn complete")) or "RawInvocationSourceTransportV1::root" in callable_loan: raise AssertionError("callable semantic loan drift")
    if not all(x in callable_ledger + callable_materialization + callable_entry_port for x in ("install_entry_values", "record_completed_local", "read_variable", "lower_callable_binding_rebind_v1", "CallableEntryShapeV1")): raise AssertionError("callable BindingRef materialization drift")
    if not all(x in callable_catalog for x in ("SelectedCallableSemanticBlockerV1::NonPlainInstanceBox", "selected_semantic_blockers.push")) or not all(x in callable_resolver for x in ("for root in roots", "deferred |= selected_callable_source_deferral(error)?", "if deferred", "for tree in trees")): raise AssertionError("callable batch admission drift")
    if raw_source_transport.count("fn callable_semantic_root(") != 1 or "loan: &VerifiedNormalCallableSemanticLoanV1" not in raw_source_transport: raise AssertionError("callable typed transport drift")
    for terminal in ("lower_normal_top_level_function_with_source_v1", "lower_normal_cataloged_static_box_method_with_source_v1", "lower_normal_cataloged_instance_box_method_with_source_v1"):
        if callable_loan.count(terminal) != 1: raise AssertionError(f"callable semantic terminal drift: {terminal}")
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
        "prepare_normal_default_module(runtime_inputs.entry_safepoint_enabled())",
        "lower_normal_default_program_root_after_catalog_install_v1(",
        "finalize_module",
    ):
        require(normal_root_lifecycle, fragment, f"normal lifecycle contract {fragment}")
    lifecycle_anchors = (
        "VerifiedRawRootExpansionV1::from_program",
        "prepare_normal_default_module(runtime_inputs.entry_safepoint_enabled())",
        "source.clone_lowering_statements()",
        "VerifiedSameModuleCallableDeclarationCatalogV1::seal_root",
        "install_callable_declaration_catalog",
        "lower_normal_default_program_root_after_catalog_install_v1(",
        "finalize_module",
    )
    kernel_anchors = ("lower_normal_default_program_root_after_catalog_install_v1", "lower_program_root_after_catalog_install_v1")
    ordered_contracts = ((normal_root_lifecycle, lifecycle_anchors), (program_root_lowering, kernel_anchors))
    for text, anchors in ordered_contracts:
        positions = [text.index(anchor) for anchor in anchors]
        if positions != sorted(positions):
            raise AssertionError("normal root/catalog lifecycle ordering drift")
    if normal_root_lifecycle.count("source.clone_lowering_statements()") != 1:
        raise AssertionError("normal Program kernel root clone drift")
    if normal_root_lifecycle.count("self.ast.clone()") != 1:
        raise AssertionError("normal Program source root clone implementation drift")
    if normal_root_lifecycle.count("let expansion = VerifiedRawRootExpansionV1::from_program") != 1:
        raise AssertionError("verified root expansion handoff issuer drift")
    require(program_root_lowering, "expansion.is_app_mode()", "verified root route consumer")
    require(program_root_lowering, "VerifiedRawRootExpansionV1::App(main)", "verified Main route")
    require(decls, "build_verified_static_main_box_with_port_v1", "verified Main terminal")
    verified_main = text_between(decls, "fn build_verified_static_main_box_with_port_v1", "fn lower_verified_static_main_root_with_port_v1")
    if not all(fragment in verified_main for fragment in ("child.to_owned_lowering().into_parts()", "main.to_owned_root_lowering()")): raise AssertionError("verified Main typed lowering handoff drift")
    if "ASTNode::FunctionDeclaration" in verified_main or "main.root().source()" in verified_main or "main-expansion/static-child-source" in verified_main: raise AssertionError("verified Main lower-side AST reclassification returned")
    if any(f"fn {name}" in decls for name in ("build_static_main_box", "build_static_main_box_typed", "build_static_main_box_with_port_v1", "build_box_declaration")) or any(fragment not in production_code(ROOT / "src/mir/builder/instance_box_declaration_metadata.rs") for fragment in ("PreparedInstanceBoxDeclarationMetadataV1", "sorted_method_entries", "lower_with_builder_v1")): raise AssertionError("retired Box facade or metadata projection drift")
    raw_static_main = production_code(ROOT / "src/mir/builder/recursive_child_lowering.rs")
    if "PreparedRawStaticMainBoxCompatibilityV1::prepare(box_name, methods)" not in raw_static_main or ".lower_with_port_v1(builder, self)" not in raw_static_main: raise AssertionError("raw static-Main direct prepared handoff drift")
    if caller_manifest["compatibility_sunsets"]["RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001"]["production_facade_edges"] != 0 or caller_manifest["compatibility_sunsets"]["SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001"]["state"] != "active": raise AssertionError("active compatibility sunset drift")
    if not all(fragment in raw_static_main_compat for fragment in ("struct PreparedRawStaticMainBoxCompatibilityV1", "enum RawStaticMainRootDispositionV1", "sorted_method_entries(&methods)", "lower_static_main_function_parts_with_port_v1")): raise AssertionError("raw static-Main compatibility batch contract drift")
    for retired in ("main_static:", "build_static_main_box_with_port_v1(callables"):
        if retired in program_root_lowering:
            raise AssertionError(f"retired selected Main projection returned: {retired}")
    declaration_facts, static_table_metadata, finalization_declaration_metadata, finalization_function_metadata = production_code(ROOT / "src/mir/builder/program_declaration_facts.rs"), production_code(PROGRAM_STATIC_TABLE_METADATA), production_code(MODULE_FINALIZATION_DECLARATION_METADATA), production_code(MODULE_FINALIZATION_FUNCTION_METADATA)
    work_plan, script_runtime_work, script_direct_owner, script_program_item_admission, raw_expression_dispatch, top_level_admission, constructor_admission, callable_catalog, selected_source_inventory = production_code(ROOT / "src/mir/builder/program_root_work_plan.rs"), production_code(ROOT / "src/mir/builder/normal_script_runtime_work.rs"), production_code(ROOT / "src/mir/builder/normal_script_direct_statement_owner.rs"), production_code(ROOT / "src/mir/builder/normal_script_program_item_admission.rs"), production_code(RAW_EXPRESSION_DISPATCH), production_code(TOP_LEVEL_ADMISSION), production_code(CONSTRUCTOR_ADMISSION), production_code(ROOT / "src/mir/builder/callable_declaration_catalog/catalog.rs"), production_code(ROOT / "src/mir/builder/callable_declaration_catalog/selected_source_inventory.rs")
    if (ROOT / "src/mir/builder/raw_expression_dispatch/legacy_facade.rs").exists(): raise AssertionError("retired raw expression facade returned")
    if any(fragment in production_code(ROOT / "src/mir/builder/raw_expression_dispatch/input_view.rs") for fragment in ("RawLegacyExpressionInputV1", "RawExpressionInputViewV1")): raise AssertionError("retired raw expression input view returned")
    if "declaration_indexer" in production_code(BUILDER_ROOT) or "declaration_indexer" in program_root_lowering:
        raise AssertionError("raw declaration indexer returned")
    if not all(fragment in declaration_facts for fragment in ("struct PreparedNormalProgramDeclarationFactsV1", "fn collect", "fn install_into", "collect_static_scalar_updates")):
        raise AssertionError("normal Program declaration facts owner drift")
    if program_root_lowering.count("PreparedNormalProgramDeclarationFactsV1::collect(snapshot)") != 1:
        raise AssertionError("normal Program declaration facts caller drift")
    if any((ROOT / path).exists() for path in ("src/mir/builder/phi_type_inference.rs", "src/mir/join_ir/lowering/type_hint_policy.rs", "src/mir/join_ir/lowering/generic_type_resolver.rs")): raise AssertionError("retired JoinIR return-type helper returned")
    if not all(fragment in static_table_metadata for fragment in ("struct PreparedNormalProgramStaticTableMetadataV1", "fn prepare", "fn commit", "collect_static_table_specs_from_ast", "static_data_plans_from_specs")) or any(fragment in static_table_metadata for fragment in ("MirBuilder", "retry", "fallback", "NyashParser")) or not all(fragment in finalization_declaration_metadata for fragment in ("struct PreparedModuleFinalizationDeclarationMetadataV1", "fn prepare", "fn commit_into", "enum_decls_for_module_metadata")) or any(fragment in finalization_declaration_metadata for fragment in ("MirBuilder", "refresh_module_", "retry", "fallback", "NyashParser")) or module_lifecycle.count("PreparedModuleFinalizationDeclarationMetadataV1::prepare(&self.comp_ctx)") != 1 or any(fragment in module_lifecycle for fragment in ("module.metadata.user_box_decls = self.comp_ctx", "module.metadata.user_box_field_decls = self", "module.metadata.record_decls = self.comp_ctx", "module.metadata.enum_decls = self.comp_ctx")) or not module_lifecycle.index("module.add_function(function)") < module_lifecycle.index("PreparedModuleFinalizationDeclarationMetadataV1::prepare(&self.comp_ctx)") < module_lifecycle.index("refresh_module_record_and_packed_layout_plans") or not all(fragment in finalization_function_metadata for fragment in ("struct PreparedModuleFinalizationFunctionMetadataV1", "fn prepare", "fn commit_into", "value_origin_callers.insert")) or any(fragment in finalization_function_metadata for fragment in ("MirBuilder", "TypePropagationPipeline", "phi_type_inference", "refresh_module_", "retry", "fallback", "NyashParser")) or module_lifecycle.count("PreparedModuleFinalizationFunctionMetadataV1::prepare(") != 1 or any(fragment in module_lifecycle for fragment in ("function.metadata.value_types = self.function_state.type_ctx.value_types.clone();", "let mut origin_callers = function.metadata.value_origin_callers.clone();", "function.metadata.value_origin_callers = origin_callers;")) or not module_lifecycle.index("type_hint_providers::annotate_missing_result_types_from_calls_and_await(") < module_lifecycle.index("PreparedModuleFinalizationFunctionMetadataV1::prepare(") < module_lifecycle.index("return_type_strategy::infer_return_type_from_phi"): raise AssertionError("normal static/finalization metadata owner drift")
    if any(fragment in program_root_lowering for fragment in ("collect_static_table_specs_from_ast", "static_data_plans_from_specs", "module.metadata.static_table_contract_specs", "module.metadata.static_data_plans")): raise AssertionError("selected Program direct static-table metadata authority returned")
    static_prepare = "PreparedNormalProgramStaticTableMetadataV1::prepare(snapshot, module)?.commit()"
    if program_root_lowering.count(static_prepare) != 1 or program_root_lowering.index("PreparedNormalProgramDeclarationFactsV1::collect(snapshot)") > program_root_lowering.index(static_prepare) or not all(fragment in text_between(script_program_item_admission, "ASTNode::Assignment", "=> DirectPortAwareExpression") for fragment in ("Assignment", "CompoundAssignment", "Loop", "Nowait", "TaskScope", "ContextScope", "TryCatch", "Throw", "Local", "ScopeBox", "Outbox", "Program", "UsingStatement", "Return")) or not all(fragment in script_direct_owner for fragment in ("lower_direct_static_const_runtime_completion_v1", "set_current_span(statement.span())", "emit_void(builder)", "lower_direct_fastmem_region_v1", "build_fastmem_region_with_port_v1")) or not all(fragment in script_program_item_admission for fragment in ("DirectFastMemRegion", "ASTNode::FastMemRegion { .. } => DirectFastMemRegion")) or "ASTNode::FastMemRegion { .. } => StatementControlCompatibility" in script_program_item_admission or any(fragment in script_direct_owner for fragment in ("PreparedNormalProgramStaticTableMetadataV1", "static_table_contract_specs", "static_data_plans")): raise AssertionError("Program static-table metadata or selected Script direct completion drift")
    if not all(fragment in work_plan for fragment in ("struct PreparedProgramRootWorkPlanV1", "fn prepare", "fn classify_statement", "ProgramRootTerminalScheduleV1", "PreparedNormalScriptRuntimeWorkV1", "PreparedNormalScriptRuntimeInputV1", "PreparedProgramRootRuntimeWorkV1", "ProgramRootWorkPlanAdmissionV1", "NormalTopLevelFunctionDraftAdmissionV1", "NormalInstanceConstructorSourceBatchV1", "PreparedInstanceBoxConstructorBatchV1", "source_statement_index")) or any(fragment in work_plan for fragment in ("NormalTopLevelFunctionSourceKeyV1::new", "SelectedTopLevelFunctionKeyV1::new", "NormalTopLevelFunctionDraftAdmissionV1::seal")) or callable_catalog.count("SelectedTopLevelFunctionKeyV1::new(statement_index, name, params.len())") != 1 or not all(fragment in selected_source_inventory for fragment in ("struct SelectedTopLevelFunctionKeyV1", "struct VerifiedSelectedNormalCallableSourceInventoryV1", "fn top_level_function")) or normal_root_lifecycle.count("catalog.selected_source_inventory()") != 2 or not all(fragment in top_level_admission for fragment in ("struct NormalTopLevelFunctionDraftAdmissionV1", "from_catalog_key", "FunctionDraftKeyV1::LegacySymbol", "commit_legacy_symbol_pending", "lower_normal_top_level_function_v1")) or any(fragment in top_level_admission for fragment in ("NormalTopLevelFunctionSourceKeyV1", "SelectedTopLevelFunctionKeyV1::new", "LegacyChildDraftAdmissionV1", "ResolvedChildDraftAdmissionV1", "retry", "fallback", "NyashParser")) or not all(fragment in constructor_admission for fragment in ("struct NormalInstanceConstructorSourceKeyV1", "struct NormalInstanceConstructorDraftAdmissionV1", "FunctionDraftKeyV1::LegacySymbol", "commit_legacy_symbol_pending", "lower_normal_instance_constructor_v1")) or any(fragment in constructor_admission for fragment in ("LegacyChildDraftAdmissionV1", "ResolvedChildDraftAdmissionV1", "retry", "fallback", "NyashParser")) or not all(fragment in script_runtime_work for fragment in ("struct PreparedNormalScriptRuntimeWorkV1", "PreparedNormalScriptRuntimeInputV1", "LocatedNormalScriptRuntimeAdmissionV1", "NormalInstanceConstructorSourceBatchV1", "NormalScriptRuntimeStatementAdmissionV1", "DirectPrint", "DirectIfStatement", "DirectPortAwareExpression", "DirectSelectedUnsupportedStatement", "CatalogedNonMainStaticBox", "InstancePrefixCompatibility", "drive_legacy_block_v1", "lower_normal_runtime_prefix_with_port_v1")) or not all(fragment in script_direct_owner for fragment in ("lower_direct_print_v1", "lower_raw_print_statement_with_port_v1", "lower_direct_if_statement_v1", "drive_raw_if_statement_with_port_v1", "complete_if_statement_v1", "lower_direct_port_aware_expression_v1", "drive_legacy_expression_v1", "lower_direct_selected_unsupported_statement_v1", "with_legacy_expression_recursion_guard_v1", "unsupported_raw_ast_node_error_v1")) or any(fragment in script_direct_owner for fragment in ("drive_legacy_statement_v1", "RawLegacyChildLoweringPortV1", "build_expression(", "retry", "fallback")) or not all(fragment in script_program_item_admission for fragment in ("enum NormalScriptProgramItemAdmissionV1", "fn classify_normal_script_program_item_v1", "DirectPrint", "DirectIfStatement", "DirectPortAwareExpression", "DirectSelectedUnsupportedStatement", "ASTNode::If { .. } => DirectIfStatement", "ASTNode::LoopRange", "ASTNode::Break", "ASTNode::Continue", "ASTNode::ImportStatement", "ASTNode::BuildGate", "ASTNode::EnumDeclaration", "ASTNode::BrandDeclaration", "ASTNode::TypeAliasDeclaration", "ASTNode::GlobalVar")) or any(fragment in script_program_item_admission for fragment in ("StatementControlCompatibility", "DeclarationIngressCompatibility")) or raw_expression_dispatch.count("unsupported_raw_ast_node_error_v1(&ast)") != 1 or not all(fragment in text_between(script_program_item_admission, "ASTNode::Literal", "=> DirectPortAwareExpression") for fragment in ("QMarkPropagate", "MatchExpr", "EnumMatchExpr", "ArrayLiteral", "MapLiteral", "RecordLiteral", "RecordUpdate", "Lambda", "BlockExpr", "Arrow", "GroupedAssignmentExpr", "MethodCall", "FieldAccess", "Index", "New", "This", "FromCall", "ThisField", "MeField", "FunctionCall", "Call")) or "CallObjectHeaderCompatibility" in script_program_item_admission or "_ =>" in script_program_item_admission or any(fragment in script_runtime_work for fragment in ("LegacyChildDraftAdmissionV1", "lower_raw_with_port_v1", "retry", "fallback", "NyashParser")) or "lower_program_statements_with_callable_port_v1" in program_root_lowering: raise AssertionError("Program-root work partition drift")
    if normal_root_lifecycle.count("PreparedProgramRootWorkPlanV1::prepare(") != 1 or program_root_lowering.count("PreparedProgramRootWorkPlanV1::prepare(") != 1 or not normal_root_lifecycle.index("VerifiedSameModuleCallableDeclarationCatalogV1::seal_root") < normal_root_lifecycle.index("PreparedProgramRootWorkPlanV1::prepare(") < normal_root_lifecycle.index("install_callable_declaration_catalog") or not program_root_lowering.index("prepare_program_root_lowering_state_v1(snapshot") < program_root_lowering.index("PreparedProgramRootWorkPlanV1::prepare(") or "callables.lower_static_box_method(" in text_between(work_plan, "fn lower_normal_with_port_v1", "fn classify_statement") or not all(fragment in program_root_lowering for fragment in ("ProgramRootWorkPlanAdmissionV1::SelectedNormal", "ProgramRootWorkPlanAdmissionV1::RawCompatibility", "PreparedProgramRootRuntimeWorkV1::RawCompatibility(statements)", "PreparedProgramRootRuntimeWorkV1::SelectedNormal(work)", "work.lower_with_port_v1", "callables.lower_body(self, statements.into_vec())")): raise AssertionError("Program-root work preparation order drift")
    if any(fragment in work_plan for fragment in ("ModuleDraftCollectorV1", "collect_static_table_specs_from_ast", "PreparedNormalProgramDeclarationFactsV1", "retry", "fallback", "NyashParser")): raise AssertionError("Program-root work plan gained outer authority")
    for retired in ("has_main_static", "root_is_app_mode.unwrap_or_else"):
        if retired in program_root_lowering:
            raise AssertionError(f"retired root route classifier returned: {retired}")
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
    selected_call = "lower_normal_default_program_root_after_catalog_install_v1("
    if normal_root_lifecycle.count(selected_call) != admission.get("selected_lifecycle_calls"):
        raise AssertionError("selected Program lifecycle caller drift")
    if normal_root_lifecycle.count("PreparedRawRootPartitionV1") != admission.get(
        "selected_generic_partition_calls"
    ):
        raise AssertionError("selected normal generic partition drift")
    for sunset in (admission.get("mircompiler_compat_sunset", ""), admission.get("runtime_ast_json_compat_sunset", ""), admission.get("script_existing_root_lower_sunset", "")):
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
    require(program_root_lowering, "lower_normal_default_program_root_after_catalog_install_v1", "selected Program-only root kernel")
    require(program_root_lowering, "lower_program_root_with_callable_port_v1", "shared generic Program kernel")
    if program_root_lowering.count("self.lower_program_root_with_callable_port_v1(") != admission.get(
        "selected_program_kernel_calls"
    ):
        raise AssertionError("generic Program branch reuse drift")
    deferred_owner = "ProgramDeferredStaticBoxLifecycleV1"
    require(program_root_lowering, f"struct {deferred_owner}", "deferred static Box owner")
    if program_root_lowering.count(f"{deferred_owner}::new(name, methods)") != 1:
        raise AssertionError("deferred static Box production handoff drift")
    deferred_scope = text_between(program_root_lowering, "struct ProgramDeferredStaticCompilationContextScopeV1", f"pub(super) struct {deferred_owner}")
    deferred_lifecycle = text_between(program_root_lowering, f"impl {deferred_owner}", "impl MirBuilder")
    if deferred_scope.count("fn open") != 1 or deferred_scope.count("fn run") != 1 or deferred_scope.count("fn restore") != 1 or deferred_scope.count("impl Drop") != 1 or any(retired in deferred_lifecycle for retired in ("compilation_context = Some(", "compilation_context = None", "retry", "fallback")): raise AssertionError("deferred static Box context scope drift")
    if any(token in program_root_lowering for token in ("collector.into_draft_functions()", ".try_add_functions_atomic(")):
        raise AssertionError("selected Program collector direct drain returned")
    if program_root_lowering.count(".prepare_normal_legacy_drain(target)") or program_root_lowering.count(".prepare_normal_collector_drain(target, brand)") != 1 or (ROOT / "src/mir/builder/module_draft_collector/normal_legacy_drain.rs").exists():
        raise AssertionError("selected Program normal collector drain caller drift")
    if not all(fragment in normal_collector_drain for fragment in ("PreparedNormalCollectorDrainLifecycleV1", "RejectedNormalCollectorDrainLifecycleV1", "SealedNormalCollectorDrainReceiptV1", "BrandMismatch", "LegacyReplaceWholePair")) or any(fragment in normal_collector_drain for fragment in ("prepare_raw_drain", "prepare_canonical_drain", "retry", "fallback")):
        raise AssertionError("normal collector lifecycle authority drift")
    if "ast: ASTNode" in text_between(
        normal_pipeline,
        "pub struct NormalCompileRequestV1",
        "pub enum NormalProgramCompileRequestErrorV1",
    ):
        raise AssertionError("normal request regained bare AST authority")
    if "raw_nonprogram_root_descent" in caller_manifest:
        raise AssertionError("retired raw root inventory returned")
    raw_sunset = caller_manifest.get("compatibility_sunsets", {}).get(
        "RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001", {}
    )
    if raw_sunset.get("state") != "closed":
        raise AssertionError("raw non-Program root sunset must remain closed")
    for key in ("owner_definitions", "execution_callers", "residual_surface", "root_raw_edges"):
        if raw_sunset.get(key) != 0:
            raise AssertionError(f"raw root retirement count drift: {key}")
    for retired_path in (
        ROOT / "src/mir/builder/raw_nonprogram_root_descent.rs",
        ROOT / "src/mir/builder/raw_nonprogram_root_descent_tests.rs",
        ROOT / "src/mir/builder/raw_nonprogram_root_descent_tests/parity.rs",
    ):
        if retired_path.exists():
            raise AssertionError(f"retired raw root file returned: {retired_path}")
    for retired_symbol in (
        "PreparedRawRootPartitionV1",
        "PreparedRawNonProgramRootV1",
        "PortNeutralExprTreeV1",
        "ExistingRawNonProgramRootCompatibilityV1",
        "lower_raw_nonprogram_root_with_port_v1",
    ):
        if any(retired_symbol in text for text in production.values()):
            raise AssertionError(f"retired raw root symbol returned: {retired_symbol}")
    if "mod raw_nonprogram_root_descent;" in builder_root:
        raise AssertionError("retired raw root module wiring returned")
    for fixture in "late_normal_lowering_failure_leaves_live_builder_unchanged_and_reusable explicit_imports_commit_only_with_the_finished_normal_candidate normal_pipeline_matches_legacy_compatibility_for_general_module program_v0_typed_failure_keeps_live_builder_reusable_without_retry repl_program_matches_legacy_config_and_failure_reuse program_v0_typed_errors_match_legacy_program_stages_exactly public_program_admission_rejects_non_program_roots rejected_nonprogram_admission_leaves_live_builder_unchanged_and_reusable responsibility_local_nonprogram_roots_share_public_admission_and_reuse".split():
        require(normal_tests, fixture, f"normal pipeline fixture {fixture}")
    validate_script_r4_ratchet_evidence(ROOT, caller_manifest, require)
    validate_r4_fence_registry(ROOT, caller_manifest, require)
    count_by_manifest(caller_manifest.get("normal_compile_adapters", {}), ".compile(")
    expected_build_module = caller_manifest.get("direct_build_module_production", {})
    actual_build_module = {
        str(path.relative_to(ROOT)): text.count(".build_module(")
        for path, text in production.items()
        if text.count(".build_module(")
    }
    if actual_build_module != expected_build_module:
        raise AssertionError("direct production build_module caller drift: "
                             f"expected={expected_build_module} actual={actual_build_module}")
    if "json_to_ast" in (runtime_emit := (ROOT / "src/runtime/mirbuilder_emit.rs").read_text()) or "lower_ast_json_to_module" in runtime_emit: raise AssertionError("runtime AST-JSON compatibility must remain retired")
    test_bridge = ROOT / "src/host_providers/mir_builder/lowering/ast_json.rs"
    if test_bridge.exists(): raise AssertionError("cfg(test) AST-JSON compatibility returned")
    expected_test_build = caller_manifest.get("direct_build_module_repository_tests", {}); actual_test_build = {}
    for root in (ROOT / "src", ROOT / "tests"):
        for path in root.rglob("*.rs"):
            count = path.read_text().count(".build_module(")
            if count: actual_test_build[str(path.relative_to(ROOT))] = count
    if actual_test_build != expected_test_build: raise AssertionError(
        f"direct test build_module caller drift: expected={expected_test_build} actual={actual_test_build}")
    module_lifecycle = (ROOT / "src/mir/builder/module_lifecycle.rs").read_text()
    if "pub fn build_module" in (ROOT / "src/mir/builder/builder_build.rs").read_text() or any(retired in module_lifecycle for retired in ("fn lower_root(", "fn lower_root_after_callable_catalog_install_v1(", "fn lower_root_after_callable_catalog_install_with_callable_port_v1")): raise AssertionError("generic public/root wrapper returned")
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
    match_owner = production_code(MATCH_EXPRESSION_OWNER)
    if match_owner.count("self.build_literal(label)?") != 1:
        raise AssertionError("Match label must use canonical literal owner exactly once")
    for retired in ("match label", "LiteralValue::", "emission::constant::", "emit_typed_integer_literal"):
        if retired in match_owner:
            raise AssertionError(f"duplicate Match label literal authority remains: {retired}")
    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-public-ingress0-guard] ok "
        "landed=1 closeout=1 raw_non_test=0 old_raw_non_test=0 json=0 below_800=1"
    )
    return 0
if __name__ == "__main__":
    raise SystemExit(main())
