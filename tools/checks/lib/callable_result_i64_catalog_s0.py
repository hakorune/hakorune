#!/usr/bin/env python3
"""Guard disconnected exact-i64 callable result composition through S0b."""

from __future__ import annotations

import argparse
from pathlib import Path
import re


MODULE = Path("src/mir/callable_result_representation")
PRODUCT = "VerifiedSameModuleCallableResultCatalogV1"
CALL_SITE_PRODUCT = "VerifiedCallableResultCallSiteV1"
CALL_EVIDENCE_PRODUCT = "VerifiedCallableResultEvidenceV1"
CALL_PROOF_CONTEXT = "CallProofContextV1"
CALL_SUBSTITUTION = "substitute_required_arguments"
CORE_RESULT_LOOKUP = "lookup_core_method_result_row_v1"
TARGET_MODULE = Path("src/mir/source_call_target")
TARGET_PRODUCT = "VerifiedSourceStaticCallTargetCatalogV1"
CURRENT_OWNER_PRODUCT = "VerifiedCurrentOwnerStaticCallTargetV1"
SOURCE_METHOD_CALL_SITE_PRODUCT = "VerifiedSourceMethodCallSiteV1"
QUALIFIED_RECEIVER_LEXICAL_PRODUCT = "VerifiedQualifiedReceiverLexicalDispositionsV1"
QUALIFIED_ROUTE_FACT_PRODUCT = "VerifiedQualifiedCallRouteFactsV1"
IMPORT_VIEW = "VerifiedStaticImportAliasViewV1"
RESERVED_ROUTE_POLICY = Path("src/mir/policies/source_method_reserved_route.rs")
RESERVED_ROUTE_CLASSIFIER = "classify_source_method_reserved_route_v1"
BUILDER_RESERVED_ADAPTER = Path("src/mir/builder/calls/reserved_method_route.rs")
BUILDER_METHOD_ORCHESTRATOR = Path("src/mir/builder/calls/build.rs")
BUILDER_RESERVED_EMITTER = Path("src/mir/builder/calls/debug_method_routing.rs")
BUILDER_RESERVED_TESTS = Path("src/mir/builder/calls/reserved_method_route_tests.rs")
RECEIVER_MODULE = Path("src/mir/source_core_receiver")
RECEIVER_PRODUCT = "VerifiedSourceCoreReceiverV1"
SOURCE_PROJECTOR = Path("src/mir/resolved_semantics/source_projection.rs")
COMPILER_PROJECTION = Path("src/mir/compiler/source_projection.rs")


class GuardFailure(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GuardFailure(message)


def code_only(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    text = re.sub(r"//.*", "", text)
    return re.sub(r'"(?:\\.|[^"\\])*"', '""', text)


def production_rust(root: Path) -> str:
    rows: list[str] = []
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root)
        if (
            MODULE in relative.parents
            or TARGET_MODULE in relative.parents
            or RECEIVER_MODULE in relative.parents
        ):
            continue
        if "tests" in path.parts or path.name.endswith("_tests.rs"):
            continue
        rows.append(code_only(path.read_text(encoding="utf-8")))
    return "\n".join(rows)


def verify(root: Path) -> dict[str, int]:
    projector = (root / SOURCE_PROJECTOR).read_text(encoding="utf-8")
    compiler_projection = (root / COMPILER_PROJECTION).read_text(encoding="utf-8")
    mir_code = "\n".join(
        code_only(path.read_text(encoding="utf-8"))
        for path in (root / "src/mir").rglob("*.rs")
        if "tests" not in path.parts and not path.name.endswith("_tests.rs")
    )
    require(
        mir_code.count("fn project_segment") == 1,
        "structural SourcePath projector owner count drift",
    )
    require(
        code_only(compiler_projection).count("fn project_segment") == 0,
        "compiler regained a private SourcePath projector",
    )
    require(
        code_only(compiler_projection).count("project_source_node_v1(root, site)") == 1,
        "compiler must remain one thin neutral-projector consumer",
    )
    require(
        code_only(projector).count("enum ProjectedSourceNodeV1") == 1,
        "neutral projected-source view definition count drift",
    )
    for forbidden in (
        "MirBuilder",
        "current_static_box",
        "variable_map",
        "__mir__",
        "__repl__",
    ):
        require(
            forbidden not in code_only(projector),
            f"Builder/route authority entered neutral projector: {forbidden}",
        )
    require(
        mir_code.count("enum SourcePathSegmentV1") == 1,
        "SourcePath vocabulary definition count drift",
    )
    require(
        mir_code.count("fn traverse_shadow_view") == 1,
        "lexical scope traversal engine count drift",
    )
    require(
        mir_code.count("fn observe_qualified_receiver_shadow_view_v0") == 1,
        "qualified receiver shadow observation entry count drift",
    )

    module_root = root / MODULE
    require(module_root.is_dir(), f"missing module: {MODULE}")
    rust_files = sorted(module_root.rglob("*.rs"))
    require(bool(rust_files), "result catalog has no Rust sources")
    sources = {path: path.read_text(encoding="utf-8") for path in rust_files}
    module_code = "\n".join(code_only(text) for text in sources.values())
    production = production_rust(root)
    solver = sources[module_root / "solver.rs"]
    disposition = sources[module_root / "disposition.rs"]
    expression_proof = sources[module_root / "expression_proof.rs"]

    require(
        module_code.count(f"struct {PRODUCT}") == 1,
        "result catalog product definition count drift",
    )
    require(
        not re.search(r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct " + PRODUCT, solver),
        "sealed result catalog must remain non-Clone",
    )
    require(
        production.count(PRODUCT) == 0,
        "S0 result catalog gained a production producer or consumer",
    )
    require(
        solver.count(".static_declarations()") == 1,
        "solver must derive rows from the single static declaration view",
    )
    require(
        "InstanceBoxMethod" not in code_only("\n".join(sources[path] for path in rust_files if "tests" not in path.parts)),
        "instance namespace entered production result rows",
    )
    require(
        "rows_by_key:" in solver and "body:" not in solver and "body:" not in disposition,
        "result product must retain keys/dispositions, not duplicate bodies",
    )
    for forbidden in (
        "MirBuilder",
        "MirFunction",
        "MirType",
        "ValueId",
        "type_ctx",
        "value_origin_newbox",
        "current_module",
        "function.metadata",
        "GenericLoop",
    ):
        require(forbidden not in module_code, f"forbidden S0 authority entered module: {forbidden}")
    require(
        "expect(" not in code_only(solver) and "unwrap(" not in code_only(solver),
        "solver must close structural drift through typed errors",
    )
    require(
        module_code.count("ExactTrivialScalarAbiV1::classify") == 2,
        "exact-i64 spelling must reuse the existing scalar ABI classifier twice",
    )
    require(
        module_code.count("BareStaticRecoveryDecisionV1::decide") == 0,
        "S0a must not infer final call targets from declaration recovery",
    )
    require(
        "StaticCallTargetAuthorityUnavailable" in module_code,
        "explicit call-target authority boundary is missing",
    )
    require(
        "actual_string_helpers_keeps_skip_ws_exact_and_records_to_i64_design_boundary" in module_code,
        "actual StringHelpers boundary fixture is missing",
    )
    require(
        re.search(
            rf"struct\s+{PRODUCT}\s*<\s*'targets\s*,\s*'catalog\s*>",
            code_only(solver),
        )
        is not None,
        "result catalog must remain lifetime-bound to target and declaration catalogs",
    )
    require(
        code_only(solver).count("is_branded_by(declarations)") == 1,
        "result solver must co-seal the exact target and declaration catalogs once",
    )
    require(
        module_code.count(f"struct {CALL_SITE_PRODUCT}") == 1,
        "call-site result product definition count drift",
    )
    require(
        module_code.count(f"enum {CALL_EVIDENCE_PRODUCT}") == 1,
        "call-result evidence product definition count drift",
    )
    require(
        module_code.count(f"struct {CALL_PROOF_CONTEXT}") == 1,
        "call proof context definition count drift",
    )
    require(
        module_code.count(f"fn {CALL_SUBSTITUTION}") == 1,
        "required-argument substitution owner count drift",
    )
    require(
        module_code.count(f"{CORE_RESULT_LOOKUP}(") == 1,
        "Core result-kind lookup consumer count drift",
    )
    require(
        module_code.count("VerifiedSourceCoreReceiverV1::verify(") == 1,
        "bounded String receiver consumer count drift",
    )
    require(
        module_code.count(".target()") >= 1,
        "call-result proof must consume the verified source target product",
    )
    require(
        "bare_qualified_and_shadowed_calls_never_guess_a_target" in module_code,
        "bare FunctionCall unavailable-boundary fixture is missing",
    )
    require(
        re.search(
            r"ASTNode::FunctionCall\s*\{\s*arguments,\s*\.\.\s*\}\s*=>\s*\{.*?"
            r"StaticCallTargetAuthorityUnavailable",
            code_only(expression_proof),
            flags=re.S,
        )
        is not None,
        "bare FunctionCall must remain one explicitly unavailable proof branch",
    )
    for forbidden in (
        "function.metadata",
        "physical_symbol",
        "runtime tag",
        "legacy resolver",
        "fallback",
        "retry",
    ):
        require(
            forbidden not in module_code,
            f"forbidden S0b authority entered result composition: {forbidden}",
        )

    target_root = root / TARGET_MODULE
    require(target_root.is_dir(), f"missing module: {TARGET_MODULE}")
    target_files = sorted(target_root.rglob("*.rs"))
    target_sources = {
        path: path.read_text(encoding="utf-8") for path in target_files
    }
    target_code = "\n".join(code_only(text) for text in target_sources.values())
    target_model = target_sources[target_root / "model.rs"]
    target_qualified = target_sources[target_root / "qualified.rs"]
    target_current_owner = target_sources[target_root / "current_owner.rs"]
    target_source_site = target_sources[target_root / "source_method_call_site.rs"]
    target_lexical = target_sources[target_root / "qualified_receiver_lexical.rs"]
    target_route_facts = target_sources[target_root / "qualified_route_facts.rs"]
    target_internal_non_site = "\n".join(
        code_only(text)
        for path, text in target_sources.items()
        if path.name not in {"mod.rs", "source_method_call_site.rs"}
        and path.name not in {"tests.rs", "test_support.rs"}
        and "tests" not in path.parts
        and not path.name.endswith("_tests.rs")
    )
    target_internal_non_lexical = "\n".join(
        code_only(text)
        for path, text in target_sources.items()
        if path.name not in {"mod.rs", "qualified_receiver_lexical.rs"}
        and path.name not in {"tests.rs", "test_support.rs"}
        and "tests" not in path.parts
        and not path.name.endswith("_tests.rs")
    )
    target_internal_non_route_facts = "\n".join(
        code_only(text)
        for path, text in target_sources.items()
        if path.name not in {"mod.rs", "qualified_route_facts.rs"}
        and path.name not in {"tests.rs", "test_support.rs"}
        and "tests" not in path.parts
        and not path.name.endswith("_tests.rs")
    )
    require(
        target_code.count(f"struct {TARGET_PRODUCT}") == 1,
        "source target catalog product definition count drift",
    )
    require(
        target_code.count(f"struct {IMPORT_VIEW}") == 1,
        "verified import alias view definition count drift",
    )
    require(
        target_code.count(f"struct {CURRENT_OWNER_PRODUCT}") == 1,
        "current-owner source target product definition count drift",
    )
    require(
        target_code.count(f"struct {SOURCE_METHOD_CALL_SITE_PRODUCT}") == 1,
        "exact source MethodCall site product definition count drift",
    )
    require(
        target_code.count(f"struct {QUALIFIED_RECEIVER_LEXICAL_PRODUCT}") == 1,
        "qualified receiver lexical product definition count drift",
    )
    require(
        target_code.count(f"struct {QUALIFIED_ROUTE_FACT_PRODUCT}") == 1,
        "qualified route fact product definition count drift",
    )
    require(
        not re.search(
            r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
            + f"(?:{TARGET_PRODUCT}|{IMPORT_VIEW})",
            target_model,
        ),
        "sealed source target catalog/import view must remain non-Clone",
    )
    require(
        not re.search(
            r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
            + SOURCE_METHOD_CALL_SITE_PRODUCT,
            target_source_site,
        ),
        "exact source MethodCall site product must remain non-Clone",
    )
    require(
        not re.search(
            r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
            + QUALIFIED_RECEIVER_LEXICAL_PRODUCT,
            target_lexical,
        ),
        "qualified receiver lexical product must remain non-Clone",
    )
    require(
        not re.search(
            r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
            + QUALIFIED_ROUTE_FACT_PRODUCT,
            target_route_facts,
        ),
        "qualified route fact product must remain non-Clone",
    )
    require(
        production.count(TARGET_PRODUCT) == 0,
        "Q0 source target catalog gained a production producer or consumer",
    )
    require(
        production.count(SOURCE_METHOD_CALL_SITE_PRODUCT) == 0,
        "S0 exact source MethodCall site gained an external production consumer",
    )
    require(
        target_internal_non_site.count(SOURCE_METHOD_CALL_SITE_PRODUCT)
        == code_only(target_lexical).count(SOURCE_METHOD_CALL_SITE_PRODUCT)
        + code_only(target_route_facts).count(SOURCE_METHOD_CALL_SITE_PRODUCT)
        + code_only(target_current_owner).count(SOURCE_METHOD_CALL_SITE_PRODUCT),
        "exact source site must have only disconnected L0/R0/current-owner consumers",
    )
    require(
        production.count(QUALIFIED_RECEIVER_LEXICAL_PRODUCT)
        + target_internal_non_lexical.count(QUALIFIED_RECEIVER_LEXICAL_PRODUCT)
        == code_only(target_route_facts).count(QUALIFIED_RECEIVER_LEXICAL_PRODUCT),
        "L0 lexical product must have only the disconnected R0 consumer",
    )
    require(
        production.count(QUALIFIED_ROUTE_FACT_PRODUCT)
        + target_internal_non_route_facts.count(QUALIFIED_ROUTE_FACT_PRODUCT)
        == code_only(target_qualified).count(QUALIFIED_ROUTE_FACT_PRODUCT),
        "R0 route facts must have only one disconnected qualified-target consumer",
    )
    require(
        target_source_site.count("catalog.declaration(caller)") == 1,
        "exact source site must start from one catalog caller lookup",
    )
    require(
        target_source_site.count("project_source_body_node_v1(") == 1,
        "exact source site must use the one neutral body projector",
    )
    require(
        "expression: &'catalog ASTNode" in target_source_site
        and "receiver: &'catalog ASTNode" in target_source_site
        and "body: Box<[ASTNode]>" not in target_source_site,
        "exact source site must borrow AST identity without owning a body",
    )
    require(
        "lexical" not in code_only(target_source_site)
        and "reserved_route" not in code_only(target_source_site)
        and "target:" not in code_only(target_source_site),
        "lexical/route/target authority entered exact source site product",
    )
    require(
        target_lexical.count("observe_qualified_receiver_shadow_view_v0(") == 1,
        "L0 must consume the existing shadow observation entry exactly once",
    )
    require(
        "FunctionOriginV1" not in code_only(target_lexical)
        and "BindingRefV1" not in code_only(target_lexical)
        and "ShadowBindingOrdinalV0" not in code_only(target_lexical),
        "synthetic function/binding identity entered L0 lexical product",
    )
    require(
        "variable_ref(" not in code_only(target_lexical)
        and "reserved_route" not in code_only(target_lexical)
        and "import" not in code_only(target_lexical)
        and "target:" not in code_only(target_lexical),
        "absence/route/import/target authority entered L0 lexical product",
    )
    require(
        target_route_facts.count(f"{RESERVED_ROUTE_CLASSIFIER}(") == 1,
        "R0 route facts must consume the shared reserved classifier once",
    )
    require(
        "std::ptr::eq(imports.catalog, call.catalog())" in target_route_facts,
        "R0 route facts must co-seal the catalog-branded import view",
    )
    require(
        target_route_facts.index("match decision")
        < target_route_facts.index("imports.canonical_owner(receiver)"),
        "reserved decision must precede alias lookup",
    )
    require(
        target_route_facts.index("imports.canonical_owner(receiver)")
        < target_route_facts.index(
            "lexical_disposition == QualifiedReceiverLexicalDispositionV1::Bound"
        ),
        "import alias must precede direct-receiver Bound rejection",
    )
    for forbidden in (
        "MirBuilder",
        "variable_map",
        "current_static_box",
        "declaration_for(",
        "VerifiedSourceStaticCallTargetV1",
        "MirType",
        "ValueId",
        "EffectMask",
    ):
        require(
            forbidden not in code_only(target_route_facts),
            f"forbidden target/Builder authority entered R0 route facts: {forbidden}",
        )
    require(
        "catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1" in target_model,
        "verified import alias view lost its exact catalog brand",
    )
    require(
        target_model.count(
            "declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1"
        )
        == 1,
        "final source target catalog lost its exact declaration-catalog brand",
    )
    for retired in (
        "QualifiedStaticCallCandidateV1",
        "CurrentOwnerStaticCallCandidateV1",
        "QualifiedReceiverLexicalFactV1",
        "ReservedQualifiedReceiverRouteV1",
        "from_method_call",
        "checked_explicit_arity",
    ):
        require(retired not in target_code, f"retired raw candidate surface remains: {retired}")
    require(
        target_qualified.count(".declaration_for(") == 1,
        "qualified target must project through one exact catalog lookup",
    )
    require(
        target_current_owner.count(".declaration_for(") == 1,
        "current-owner target must project through one exact catalog lookup",
    )
    require(
        "caller.key().owner()" in target_current_owner,
        "current-owner target must derive its owner from the caller catalog key",
    )
    require(
        "VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic" in target_current_owner,
        "current-owner route must extend the shared target catalog",
    )
    require(
        "facts.admission()" in target_qualified
        and "facts.canonical_owner()" in target_qualified,
        "qualified target must consume only co-sealed admission/owner facts",
    )
    require(
        "matches_import_view(imports)" in target_qualified
        and "std::ptr::eq(call.catalog(), declarations)" in target_qualified,
        "qualified target lost exact import-view/catalog identity checks",
    )
    require(
        "calls: impl IntoIterator<Item = &'site VerifiedSourceMethodCallSiteV1"
        in target_current_owner
        and "declarations:" not in target_current_owner,
        "current-owner target must accept only exact-site products",
    )
    for forbidden in (
        RESERVED_ROUTE_CLASSIFIER,
        "QualifiedReceiverLexicalDispositionV1",
        "canonical_owner(candidate",
    ):
        require(
            forbidden not in code_only(target_qualified),
            f"qualified target replayed sealed route policy: {forbidden}",
        )
    for forbidden in (
        "MirBuilder",
        "MirFunction",
        "MirType",
        "ValueId",
        "type_ctx",
        "current_module",
        "current_static_box",
        "mir_symbol_projection",
        "variable_map",
    ):
        require(
            forbidden not in target_code,
            f"forbidden Q0 authority entered source target module: {forbidden}",
        )
    require(
        "actual_parser_wrapper_projects_import_alias_to_string_helpers" in target_code,
        "actual ParserStringUtilsBox wrapper target fixture is missing",
    )
    require(
        "imported_alias_precedes_same_spelled_lexical_binding" in target_code,
        "import-alias/local-binding precedence fixture is missing",
    )
    require(
        "actual_string_helpers_projects_exact_digit_value_site" in target_code,
        "actual StringHelpers current-owner target fixture is missing",
    )
    for fixture in (
        "exact_import_view_instance_is_part_of_qualified_seal",
        "route_facts_from_equal_foreign_catalog_reject_by_identity",
        "equal_foreign_catalog_call_rejects_before_target_lookup",
        "duplicate_exact_route_fact_site_rejects",
        "duplicate_exact_current_owner_site_rejects_atomically",
    ):
        require(fixture in target_code, f"missing CUT0 false-seal fixture: {fixture}")
    require(
        "actual_string_helpers_accepts_only_the_exact_digit_value_site" in target_code,
        "actual StringHelpers exact-site false-seal fixture is missing",
    )
    require(
        "actual_parser_string_utils_binds_skip_ws_to_its_catalog_body" in target_code,
        "actual ParserStringUtils exact-site fixture is missing",
    )
    require(
        "same_relative_site_is_bound_to_each_catalog_caller_body" in target_code,
        "exact source site caller/body binding fixture is missing",
    )
    require(
        "rejects_a_nested_lambda_call_as_the_outer_catalog_caller" in target_code,
        "nested callable false-seal rejection fixture is missing",
    )
    require(
        "classifies_parameter_bound_and_direct_owner_proven_unbound" in target_code,
        "L0 Bound/ProvenUnbound fixture is missing",
    )
    require(
        "ordinary_unresolved_variable_outside_the_request_still_rejects" in target_code,
        "L0 ordinary unresolved-name regression fixture is missing",
    )
    require(
        "actual_parser_string_utils_receiver_is_positive_proven_unbound" in target_code,
        "actual ParserStringUtils lexical disposition fixture is missing",
    )
    for fixture in (
        "direct_unbound_and_bound_import_alias_follow_exact_precedence",
        "reserved_route_rejects_before_matching_import_alias",
        "source_site_alone_derives_fastmem_context",
        "rejects_missing_lexical_row_and_foreign_catalog_alias_view",
        "declaration_reorder_preserves_normalized_route_facts",
    ):
        require(fixture in target_code, f"missing R0 route fact fixture: {fixture}")

    policy = (root / RESERVED_ROUTE_POLICY).read_text(encoding="utf-8")
    policy_code = code_only(policy)
    builder_adapter = (root / BUILDER_RESERVED_ADAPTER).read_text(encoding="utf-8")
    builder_orchestrator = (root / BUILDER_METHOD_ORCHESTRATOR).read_text(
        encoding="utf-8"
    )
    builder_emitter = (root / BUILDER_RESERVED_EMITTER).read_text(encoding="utf-8")
    builder_reserved_tests = (root / BUILDER_RESERVED_TESTS).read_text(encoding="utf-8")
    require(
        policy_code.count(f"fn {RESERVED_ROUTE_CLASSIFIER}") == 1,
        "reserved-route classifier owner count drift",
    )
    require(
        code_only(builder_adapter).count(f"{RESERVED_ROUTE_CLASSIFIER}(") == 1,
        "Builder must consume the shared reserved classifier once",
    )
    require(
        code_only(target_route_facts).count(f"{RESERVED_ROUTE_CLASSIFIER}(") == 1,
        "source route facts must consume the shared reserved classifier once",
    )
    require(
        mir_code.count(f"{RESERVED_ROUTE_CLASSIFIER}(") == 3,
        "reserved classifier must have one owner and exactly two consumers",
    )
    for forbidden in (
        "MirBuilder",
        "current_fastmem_region",
        "SourcePathSegmentV1",
        "variable_map",
        "VerifiedStaticImportAliasViewV1",
        "declaration_for(",
        "ValueId",
    ):
        require(
            forbidden not in policy_code,
            f"non-neutral authority entered reserved route policy: {forbidden}",
        )
    require(
        'if name == "mem"' not in builder_orchestrator
        and 'if obj_name != "__mir__"' not in builder_orchestrator
        and 'if obj_name != "__repl"' not in builder_orchestrator,
        "Builder orchestrator regained a by-name reserved decision",
    )
    for old_owner in (
        "try_build_mir_debug_method_call",
        "try_build_repl_method_call",
        "try_build_mir_debug_call",
    ):
        require(
            old_owner not in mir_code,
            f"retired reserved-route decision owner remains: {old_owner}",
        )
    for forbidden in (
        'obj_name != "__mir__"',
        'obj_name != "__repl"',
        'method != "log"',
        'method != "get"',
        "arguments.is_empty()",
        "LiteralValue::String",
    ):
        require(
            forbidden not in builder_emitter,
            f"execution emitter regained reserved-route admission: {forbidden}",
        )
    require(
        "selected_mir_debug_route_preserves_debug_payload" in builder_reserved_tests
        and "selected_repl_route_preserves_extern_call" in builder_reserved_tests
        and "selected_fastmem_method_route_preserves_memop_lowering"
        in builder_reserved_tests,
        "Builder reserved-route parity fixtures are missing",
    )

    receiver_root = root / RECEIVER_MODULE
    require(receiver_root.is_dir(), f"missing module: {RECEIVER_MODULE}")
    receiver_files = sorted(receiver_root.rglob("*.rs"))
    receiver_sources = {
        path: path.read_text(encoding="utf-8") for path in receiver_files
    }
    receiver_code = "\n".join(code_only(text) for text in receiver_sources.values())
    require(
        receiver_code.count(f"struct {RECEIVER_PRODUCT}") == 1,
        "source receiver proof product definition count drift",
    )
    require(
        production.count(RECEIVER_PRODUCT) == 0,
        "String receiver S0 gained a production producer or consumer",
    )
    require(
        "SourceCoreReceiverFactV1::ExactStringOnSuccess" in receiver_code,
        "exact String-on-success fact is missing",
    )
    require(
        "let mut cursor = expression" in receiver_code and "cursor = left" in receiver_code,
        "source receiver proof must remain an iterative left-spine walk",
    )
    require(
        "actual_string_helpers_to_i64_initializer_is_exact_string_on_success"
        in receiver_code,
        "actual StringHelpers.to_i64 receiver fixture is missing",
    )
    for forbidden in (
        "I64ExpressionFactV1",
        "MirBuilder",
        "MirFunction",
        "MirType",
        "ValueId",
        "type_ctx",
        "value_origin_newbox",
        "current_module",
        "current_static_box",
        "runtime tag",
    ):
        require(
            forbidden not in receiver_code,
            f"forbidden String receiver authority entered module: {forbidden}",
        )
    for path, text in sources.items():
        lines = len(text.splitlines())
        require(lines < 800, f"source reached 800 lines: {path.relative_to(root)} ({lines})")
    for path, text in target_sources.items():
        lines = len(text.splitlines())
        require(lines < 800, f"source reached 800 lines: {path.relative_to(root)} ({lines})")
    for path, text in receiver_sources.items():
        lines = len(text.splitlines())
        require(lines < 800, f"source reached 800 lines: {path.relative_to(root)} ({lines})")
    self_path = root / "tools/checks/lib/callable_result_i64_catalog_s0.py"
    require(
        len(self_path.read_text(encoding="utf-8").splitlines()) < 800,
        "S0 guard reached 800 lines",
    )

    return {
        "product_definitions": 1,
        "production_producers_consumers": 0,
        "static_declaration_views": 1,
        "bare_static_policy_consumers": 0,
        "forbidden_authority_occurrences": 0,
        "line_cap_violations": 0,
        "source_target_product_definitions": 1,
        "source_target_production_producers_consumers": 0,
        "verified_import_alias_views": 1,
        "current_owner_target_product_definitions": 1,
        "source_target_forbidden_authority_occurrences": 0,
        "source_method_call_site_product_definitions": 1,
        "source_method_call_site_production_consumers": 0,
        "qualified_receiver_lexical_product_definitions": 1,
        "qualified_receiver_lexical_production_consumers": 0,
        "lexical_scope_traversal_engines": 1,
        "qualified_route_fact_product_definitions": 1,
        "qualified_route_fact_production_consumers": 0,
        "qualified_route_fact_target_consumers": 1,
        "current_owner_exact_site_target_consumers": 1,
        "raw_source_target_candidate_surfaces": 0,
        "source_target_catalog_brands": 1,
        "reserved_route_policy_definitions": 1,
        "reserved_route_builder_consumers": 1,
        "reserved_route_source_consumers": 1,
        "old_builder_route_decision_owners": 0,
        "source_receiver_product_definitions": 1,
        "source_receiver_production_producers_consumers": 0,
        "source_receiver_forbidden_authority_occurrences": 0,
        "structural_source_path_projector_owners": 1,
        "source_path_vocabularies": 1,
        "compiler_projector_consumers": 1,
        "call_site_result_product_definitions": 1,
        "call_result_evidence_product_definitions": 1,
        "call_proof_context_definitions": 1,
        "call_substitution_owners": 1,
        "core_result_lookup_consumers": 1,
        "bounded_string_receiver_consumers": 1,
        "catalog_identity_co_seals": 1,
        "bare_call_target_consumers": 0,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    root = Path(parser.parse_args().root).resolve()
    report = verify(root)
    for key, value in report.items():
        print(f"{key}={value}")
    print("summary=green")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GuardFailure as error:
        print(f"[callable-result-i64-catalog-s0] ERROR: {error}")
        raise SystemExit(1)
