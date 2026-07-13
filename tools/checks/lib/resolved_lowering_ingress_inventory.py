#!/usr/bin/env python3
"""Validate the B0-L ingress history and atomic SA3-B first family."""

from __future__ import annotations

import json
import sys
from pathlib import Path


MODULE_IDS = {
    "rust-source-default-bare-ast",
    "rust-source-imports-bare-ast",
    "direct-bare-ast-api",
    "program-v0-import-bundle",
    "repl-wrapper-source",
}
FUNCTION_IDS = {
    "free-static-function",
    "static-box-method-app",
    "static-box-method-script",
    "instance-constructor",
    "instance-method",
    "script-runtime-root",
    "main-callable-optional",
    "main-inline-wrapper",
    "lambda-closure-body",
    "repl-submission-main",
}
BODY_SEAM_IDS = {
    "raw-body-suffix-router",
    "function-body-program-wrapper",
}
BASELINE_ZERO_FIELDS = {
    "production_semantic_activation",
    "canonical_lower_route_count",
    "resolved_scope_consumer_count",
    "exact_source_site_transport_count",
    "route_selection_before_prepare_module",
    "legacy_retry_after_canonical_selection",
}


def fail(message: str) -> None:
    raise SystemExit(f"[resolved-lowering-ingress-inventory] ERROR: {message}")


def exact_ids(rows: list[dict], expected: set[str], label: str) -> None:
    actual = [row.get("id") for row in rows]
    if len(actual) != len(set(actual)):
        fail(f"{label} contains duplicate ids: {actual}")
    if set(actual) != expected:
        fail(
            f"{label} drifted: missing={sorted(expected - set(actual))} "
            f"extra={sorted(set(actual) - expected)}"
        )


def check_evidence(root: Path, rows: list[dict], label: str) -> None:
    for row in rows:
        evidence = row.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            fail(f"{label}/{row.get('id')} has no evidence")
        for item in evidence:
            if not isinstance(item, str) or "#" not in item:
                fail(f"{label}/{row.get('id')} has malformed evidence: {item!r}")
            relative, anchor = item.split("#", 1)
            path = root / relative
            if not path.is_file():
                fail(f"{label}/{row.get('id')} evidence path is missing: {relative}")
            if anchor not in path.read_text(encoding="utf-8"):
                fail(f"{label}/{row.get('id')} evidence is stale: {item}")


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: resolved_lowering_ingress_inventory.py ROOT INVENTORY")
    root = Path(sys.argv[1]).resolve()
    inventory_path = Path(sys.argv[2]).resolve()
    data = json.loads(inventory_path.read_text(encoding="utf-8"))

    if data.get("schema") != "ResolvedLoweringIngressInventoryV1":
        fail("schema drifted")
    if data.get("decision") != "A-prime" or data.get("slice") != "B0-L1":
        fail("Decision A-prime / B0-L1 marker drifted")
    for field in BASELINE_ZERO_FIELDS:
        if data.get(field) != 0:
            fail(f"B0-L1 baseline field drifted: {field}={data.get(field)!r}")

    modules = data.get("module_ingresses")
    functions = data.get("function_families")
    seams = data.get("body_route_seams")
    if not all(isinstance(rows, list) for rows in (modules, functions, seams)):
        fail("inventory row arrays are missing")
    exact_ids(modules, MODULE_IDS, "module_ingresses")
    exact_ids(functions, FUNCTION_IDS, "function_families")
    exact_ids(seams, BODY_SEAM_IDS, "body_route_seams")

    if any(row.get("source_unit_provenance_preserved") is not False for row in modules):
        fail("a current bare-AST module ingress claims preserved source-unit provenance")
    if any(row.get("preflight_before_prepare_module") is not False for row in modules):
        fail("a current module ingress claims canonical preflight")
    if any(row.get("exact_source_root_transport") is not False for row in functions):
        fail("a current function family claims exact source-root transport")
    if any(row.get("located_transport") is not False for row in seams):
        fail("a current body seam claims located transport")
    if any(row.get("planner_connection_allowed_in_b0_l2") is not False for row in seams):
        fail("B0-L2 must not activate Planner transport")

    atomic = data.get("atomic_activation_contract", {})
    required = atomic.get("first_production_canonical_owner_requires", [])
    forbidden = atomic.get("forbidden_partial_states", [])
    if len(required) != 8 or len(set(required)) != 8:
        fail("atomic activation requirement inventory drifted")
    if len(forbidden) != 5 or len(set(forbidden)) != 5:
        fail("forbidden partial-state inventory drifted")
    if atomic.get("carrier_infrastructure_may_land_disconnected") is not True:
        fail("behavior-neutral carrier landing decision drifted")
    if data.get("b0_l1_selected_next_slice") != "B0-L2a-typed-source-unit-ingress":
        fail("B0-L1 must mechanically select B0-L2a")

    typed = data.get("typed_ingress_contract", {})
    expected_typed = {
        "slice": "B0-L2a",
        "status": "closed",
        "verified_source_unit_type": "VerifiedResolvedSourceUnitV1",
        "resolved_input_type": "ResolvedModuleLoweringInputV1",
        "legacy_input_type": "LegacyModuleLoweringInputV1",
        "request_type": "MirLoweringRequestV1",
        "canonical_error_type": "CanonicalLoweringErrorV1",
        "request_match_sites": 1,
        "production_verified_unit_constructors": 0,
        "production_resolved_request_callers": 0,
        "production_semantic_activation": 0,
        "exact_source_site_transport_count": 0,
        "resolved_scope_consumer_count": 0,
        "planner_connection_count": 0,
        "selected_next_slice": "B0-L2b-immutable-source-navigator",
    }
    if typed != expected_typed:
        fail(f"B0-L2a typed ingress contract drifted: {typed!r}")

    lowering_input = root / "src/mir/compiler/lowering_input.rs"
    compiler = root / "src/mir/compiler/mod.rs"
    for path in (lowering_input, compiler):
        if not path.is_file():
            fail(f"B0-L2a source is missing: {path.relative_to(root)}")
    lowering_text = lowering_input.read_text(encoding="utf-8")
    compiler_text = compiler.read_text(encoding="utf-8")
    for anchor in (
        "pub struct VerifiedResolvedSourceUnitV1",
        "pub struct ResolvedModuleLoweringInputV1<'a>",
        "pub struct LegacyModuleLoweringInputV1",
        "pub enum CanonicalLoweringErrorV1",
        "pub(super) enum MirLoweringRequestV1<'a>",
        "#[cfg(test)]\npub(super) fn verified_source_unit_for_test(",
    ):
        if anchor not in lowering_text:
            fail(f"B0-L2a lowering-input anchor is missing: {anchor}")
    if compiler_text.count("match request {") != 1:
        fail("MirLoweringRequestV1 must have exactly one match site")
    for anchor in (
        "pub fn compile_resolved(",
        "pub fn compile_legacy(",
        ".compile_resolved_first_family(input, source_file)",
        "CanonicalLoweringPreflightV1::verify(input.source_unit())?",
        ".build_resolved_function_module(plan)",
        "let (ast, _legacy_origin) = input.into_parts();",
        ".compile_with_source_internal(ast, source_file)",
    ):
        if anchor not in compiler_text:
            fail(f"resolved compiler boundary anchor is missing: {anchor}")
    if lowering_text.count("pub fn resolve_function(") != 1:
        fail("verified source unit must have one atomic production constructor")

    navigator = data.get("source_navigator_contract", {})
    expected_navigator = {
        "slice": "B0-L2b",
        "status": "closed",
        "shared_path_builder_type": "SourcePathV1",
        "verified_projection_type": "VerifiedSourceProjectionV1",
        "function_view_type": "FunctionSourceViewV1",
        "located_body_type": "LocatedBodyV1",
        "located_statement_type": "LocatedStmtV1",
        "located_expression_type": "LocatedExprV1",
        "located_suffix_type": "LocatedBodySuffixV1",
        "request_test_count": 4,
        "disconnected_exact_source_transport_count": 1,
        "production_verified_unit_constructors": 0,
        "production_resolved_request_callers": 0,
        "builder_consumer_count": 0,
        "planner_connection_count": 0,
        "mutable_source_cursor_count": 0,
        "pointer_identity_lookup_count": 0,
        "span_identity_lookup_count": 0,
        "name_identity_lookup_count": 0,
        "selected_next_slice": "B0-L2c-function-transaction",
    }
    if navigator != expected_navigator:
        fail(f"B0-L2b source navigator contract drifted: {navigator!r}")
    if navigator.get("selected_next_slice") != "B0-L2c-function-transaction":
        fail("closed B0-L2b must mechanically select B0-L2c")

    compiler_dir = root / "src/mir/compiler"
    navigator_files = {
        "located.rs": compiler_dir / "located.rs",
        "source_projection.rs": compiler_dir / "source_projection.rs",
        "source_view.rs": compiler_dir / "source_view.rs",
        "source_view_tests.rs": compiler_dir / "source_view_tests.rs",
    }
    for label, path in navigator_files.items():
        if not path.is_file():
            fail(f"B0-L2b source is missing: {label}")
    located_text = navigator_files["located.rs"].read_text(encoding="utf-8")
    projection_text = navigator_files["source_projection.rs"].read_text(encoding="utf-8")
    view_text = navigator_files["source_view.rs"].read_text(encoding="utf-8")
    view_tests_text = navigator_files["source_view_tests.rs"].read_text(encoding="utf-8")
    source_site_text = (root / "src/mir/resolved_semantics/source_site.rs").read_text(
        encoding="utf-8"
    )
    shadow_path_text = (root / "src/mir/resolved_semantics/shadow/path.rs").read_text(
        encoding="utf-8"
    )
    for anchor, text in (
        ("pub(crate) struct SourcePathV1", source_site_text),
        ("SourcePathV1 as ShadowSourcePathV0", shadow_path_text),
        ("pub(crate) struct VerifiedSourceProjectionV1", projection_text),
        ("pub(super) struct SourceViewSealV1(())", view_text),
        ("pub(crate) struct FunctionSourceViewV1<'a>", view_text),
        ("pub(crate) struct LocatedBodyV1<'a>", located_text),
        ("pub(crate) struct LocatedStmtV1<'a>", located_text),
        ("pub(crate) struct LocatedExprV1<'a>", located_text),
        ("pub(crate) struct LocatedBodySuffixV1<'a>", located_text),
    ):
        if anchor not in text:
            fail(f"B0-L2b source navigator anchor is missing: {anchor}")
    if "struct ShadowSourcePathV0" in shadow_path_text:
        fail("shadow resolver regained a second source-path builder")
    if view_tests_text.count("#[test]") != 4:
        fail("B0-L2b must retain four focused source-navigation tests")

    lowering_production_text = lowering_text.split("#[cfg(test)]", 1)[0]
    production_navigator_text = "\n".join(
        (located_text, projection_text, view_text, lowering_production_text)
    )
    forbidden_identity_tokens = {
        "mutable source cursor": "current_source_site",
        "raw pointer": "NonNull<",
        "pointer lookup": ".as_ptr()",
        "Span identity": "Span",
    }
    for label, token in forbidden_identity_tokens.items():
        if token in production_navigator_text:
            fail(f"B0-L2b introduced forbidden {label}: {token}")
    if "name: String" in projection_text or "name: Box<str>" in projection_text:
        fail("source projection must not carry name identity")
    for forbidden in ("MirBuilder", "control_flow::plan", "Recipe", "RegionFlow"):
        if forbidden in production_navigator_text:
            fail(f"B0-L2b connected a forbidden consumer: {forbidden}")

    located_constructors = (
        "LocatedBodyV1::new(",
        "LocatedStmtV1::new(",
        "LocatedExprV1::new(",
        "LocatedBodySuffixV1::new(",
        "SourceBodySiteV1::new_root(",
        "SourceBodySiteV1::new_child(",
    )
    for source in compiler_dir.rglob("*.rs"):
        if source == navigator_files["source_view.rs"]:
            continue
        text = source.read_text(encoding="utf-8")
        if any(token in text for token in located_constructors):
            fail(f"located carrier constructor escaped FunctionSourceViewV1: {source}")

    resolved_lowering_dir = root / "src/mir/builder/resolved_lowering"
    allowed_view_files = set(navigator_files.values()) | {
        lowering_input,
        compiler_dir / "capability.rs",
        compiler_dir / "function_input.rs",
        root / "src/mir/resolved_region_flow/analyzer.rs",
    } | set(resolved_lowering_dir.glob("*.rs"))
    external_view_consumers = []
    view_tokens = (
        "VerifiedSourceProjectionV1",
        "FunctionSourceViewV1",
        "LocatedBodyV1",
        "LocatedStmtV1",
        "LocatedExprV1",
        "LocatedBodySuffixV1",
    )
    for source in (root / "src").rglob("*.rs"):
        if source in allowed_view_files:
            continue
        text = source.read_text(encoding="utf-8")
        if any(token in text for token in view_tokens):
            external_view_consumers.append(source.relative_to(root).as_posix())
    if external_view_consumers:
        fail(f"source navigator escaped its compiler/lower allowlist: {external_view_consumers}")

    transaction = data.get("function_transaction_contract", {})
    expected_transaction = {
        "slice": "B0-L2c",
        "status": "closed",
        "session_type": "CanonicalFunctionLoweringSessionV1",
        "session_entry": "with_function_lowering_session",
        "static_session_callers": 1,
        "instance_session_callers": 1,
        "manual_prepare_restore_pairs_in_lowering": 0,
        "manual_function_region_pops_in_lowering": 0,
        "manual_fn_body_mutations_in_lowering": 0,
        "unpublished_draft_finalize_count": 1,
        "focused_test_count": 4,
        "injected_checkpoint_count": 5,
        "combined_primary_cleanup_error_contracts": 1,
        "production_semantic_activation": 0,
        "source_view_builder_consumers": 0,
        "planner_connection_count": 0,
    }
    if transaction != expected_transaction:
        fail(f"B0-L2c function transaction contract drifted: {transaction!r}")
    calls_dir = root / "src/mir/builder/calls"
    session_file = calls_dir / "function_session.rs"
    session_tests_file = calls_dir / "function_session_tests.rs"
    context_file = calls_dir / "context_lifecycle.rs"
    lowering_file = calls_dir / "lowering.rs"
    for path in (session_file, session_tests_file, context_file, lowering_file):
        if not path.is_file():
            fail(f"B0-L2c source is missing: {path.relative_to(root)}")
    session_text = session_file.read_text(encoding="utf-8")
    session_tests_text = session_tests_file.read_text(encoding="utf-8")
    context_text = context_file.read_text(encoding="utf-8")
    function_lowering_text = lowering_file.read_text(encoding="utf-8")
    for anchor, text in (
        ("struct CanonicalFunctionLoweringSessionV1<'builder>", session_text),
        ("fn with_function_lowering_session(", session_text),
        ("fn finalize_function_draft(", function_lowering_text),
        ("saved_fn_body_ast", context_text),
        ("saved_frag_emit_session", context_text),
        ("saved_region_stack", context_text),
        ("fastmem_region_stack", context_text),
        ("canonical_function_session/during_cleanup", session_text),
    ):
        if anchor not in text:
            fail(f"B0-L2c function transaction anchor is missing: {anchor}")
    static_section = function_lowering_text.split(
        "fn lower_static_method_as_function(", 1
    )[1].split("fn lower_method_as_function(", 1)[0]
    instance_section = function_lowering_text.split(
        "fn lower_method_as_function(", 1
    )[1]
    if static_section.count(".with_function_lowering_session(") != 1:
        fail("static function lowering must enter exactly one function session")
    if instance_section.count(".with_function_lowering_session(") != 1:
        fail("instance function lowering must enter exactly one function session")
    if function_lowering_text.count("fn finalize_function_draft(") != 1:
        fail("function draft finalization must have one unpublished owner")
    if session_text.count("canonical_function_session/during_cleanup") != 1:
        fail("primary+cleanup error composition must have one stable owner")
    for forbidden in (
        "prepare_lowering_context(",
        "restore_lowering_context(",
        "pop_function_region(",
        "fn_body_ast =",
    ):
        if forbidden in function_lowering_text:
            fail(f"lowering.rs regained manual function cleanup: {forbidden}")
    if session_tests_text.count("#[test]") != 4:
        fail("B0-L2c must retain four focused transaction tests")
    for checkpoint in (
        "BeforeSkeleton",
        "AfterSkeleton",
        "AfterParameters",
        "AfterBody",
        "AfterFinalize",
    ):
        if checkpoint not in session_tests_text:
            fail(f"B0-L2c injected checkpoint is missing: {checkpoint}")
    transaction_text = "\n".join(
        (session_text, context_text, function_lowering_text)
    )
    for forbidden in ("std::env", "config::env", "eprintln!"):
        if forbidden in "\n".join((session_text, context_text)):
            fail(f"B0-L2c added a lifecycle toggle/log side channel: {forbidden}")
    for forbidden in (
        "FunctionSourceViewV1",
        "LocatedStmtV1",
        "VerifiedResolvedFunctionV1",
        "control_flow::plan",
        "RegionFlow",
    ):
        if forbidden in transaction_text:
            fail(f"B0-L2c activated a forbidden dependency: {forbidden}")

    sa3 = data.get("sa3_b_first_family_contract", {})
    expected_sa3 = {
        "slice": "SA3-B",
        "status": "closed",
        "owner_family": "single-non-main-static-free-function",
        "verified_unit_constructor": "VerifiedResolvedSourceUnitV1::resolve_function",
        "function_input_type": "ResolvedFunctionLoweringInputV1",
        "module_session_type": "CanonicalModuleLoweringSessionV1",
        "function_lowerer_type": "CanonicalFunctionLowererV1",
        "value_environment_key": "BindingRefV1",
        "production_verified_unit_constructors": 1,
        "production_resolved_routes": 1,
        "default_source_route_cutover": 0,
        "whole_unit_preflight_before_builder": 1,
        "exact_declaration_site_authority": 1,
        "exact_variable_use_authority": 1,
        "exact_assignment_target_authority": 1,
        "legacy_allocator_veto_count": 1,
        "canonical_failure_legacy_retry_count": 0,
        "partial_function_publication_on_error": 0,
        "focused_test_count": 6,
        "blockexpr_runtime_claims": 0,
        "if_loop_coreplan_runtime_claims": 0,
        "lambda_runtime_claims": 0,
        "program_v0_repl_main_claims": 0,
        "planner_regionflow_connections": 0,
        "selected_next_slice": "B0-L3a-straight-line-blockexpr",
    }
    if sa3 != expected_sa3:
        fail(f"SA3-B first-family contract drifted: {sa3!r}")
    if sa3.get("selected_next_slice") != "B0-L3a-straight-line-blockexpr":
        fail("closed SA3-B must mechanically select B0-L3a")

    sa3_files = {
        "capability": compiler_dir / "capability.rs",
        "function_input": compiler_dir / "function_input.rs",
        "module_session": compiler_dir / "module_session.rs",
        "resolved_mod": resolved_lowering_dir / "mod.rs",
        "identity": resolved_lowering_dir / "identity.rs",
        "lowerer": resolved_lowering_dir / "lowerer.rs",
        "tests": resolved_lowering_dir / "tests.rs",
        "gate": root / "src/mir/builder/vars/resolved_binding_state.rs",
        "allocator": root / "src/mir/builder/builder_init.rs",
        "session": session_file,
    }
    for label, path in sa3_files.items():
        if not path.is_file():
            fail(f"SA3-B source is missing: {label}")
    sa3_text = {label: path.read_text(encoding="utf-8") for label, path in sa3_files.items()}
    for anchor, label in (
        ("struct ResolvedFunctionLoweringInputV1<'a>", "function_input"),
        ("struct CanonicalModuleLoweringSessionV1", "module_session"),
        ("struct CanonicalFunctionLowererV1<'builder, 'source>", "lowerer"),
        ("values: BTreeMap<BindingRefV1, ValueId>", "identity"),
        ("fn variable_value(", "identity"),
        ("fn assignment_binding(", "identity"),
        ("fn with_resolved_function_lowering_session(", "session"),
        ("fn veto_legacy_allocation(", "gate"),
        ("self.resolved_binding_state.veto_legacy_allocation()?", "allocator"),
        (".install(input.function())?", "resolved_mod"),
        ("self.identity.finish()?", "lowerer"),
        (".finish(self.input.owner())", "lowerer"),
    ):
        if anchor not in sa3_text[label]:
            fail(f"SA3-B {label} anchor is missing: {anchor}")
    lowerer_text = sa3_text["lowerer"]
    for forbidden in (
        "build_expression(",
        "build_variable_access(",
        "build_assignment(",
        "declare_local_in_current_scope(",
        "allocate_binding_id(",
        "variable_map",
        "binding_ctx",
        "LexicalScopeGuard",
        "control_flow::plan",
        "RegionFlow",
    ):
        if forbidden in lowerer_text:
            fail(f"SA3-B lowerer crossed a forbidden legacy/later seam: {forbidden}")
    compile_section = compiler_text.split("fn compile_resolved_first_family(", 1)[1].split(
        "fn compile_with_source_internal(", 1
    )[0]
    preflight_pos = compile_section.find("CanonicalLoweringPreflightV1::verify")
    session_pos = compile_section.find("CanonicalModuleLoweringSessionV1::open")
    build_pos = compile_section.find("build_resolved_function_module")
    if not (0 <= preflight_pos < session_pos < build_pos):
        fail("SA3-B preflight must precede module/session Builder effects")
    if "compile_legacy" in compile_section or "compile_with_source_internal" in compile_section:
        fail("canonical first-family failure gained a legacy retry")
    if sa3_text["tests"].count("#[test]") != 6:
        fail("SA3-B must retain six focused first-family tests")
    if "self.compile_legacy(LegacyModuleLoweringInputV1::bare_ast(ast), source_file)" not in compiler_text:
        fail("default bare-AST source route changed during SA3-B")

    b0_l3a = data.get("b0_l3a_blockexpr_contract", {})
    expected_b0_l3a = {
        "slice": "B0-L3a",
        "status": "closed",
        "exact_pair_query": "VerifiedResolvedFunctionV1::block_expr_scope_region_pair",
        "scope_session_type": "ResolvedScopeSessionV1",
        "value_disposition": "active-or-retired-BindingRef",
        "located_prelude_transport": 1,
        "located_tail_transport": 1,
        "canonical_blockexpr_runtime_claims": 1,
        "if_loop_coreplan_runtime_claims": 0,
        "lambda_call_runtime_claims": 0,
        "planner_regionflow_connections": 0,
        "legacy_scope_guard_calls": 0,
        "focused_test_count": 5,
        "vm_runtime_fixture_count": 2,
        "selected_next_slice": "B0-L3b-located-control-flow",
    }
    if b0_l3a != expected_b0_l3a:
        fail(f"B0-L3a BlockExpr contract drifted: {b0_l3a!r}")
    if data.get("selected_next_slice") != "B0-L3b-located-control-flow":
        fail("closed B0-L3a must mechanically select B0-L3b")

    b0_l3a_files = {
        "product": root / "src/mir/resolved_semantics/product.rs",
        "capability": compiler_dir / "capability.rs",
        "scope": resolved_lowering_dir / "scope.rs",
        "identity": resolved_lowering_dir / "identity.rs",
        "lowerer": resolved_lowering_dir / "lowerer.rs",
        "tests": resolved_lowering_dir / "block_expr_tests.rs",
    }
    for label, path in b0_l3a_files.items():
        if not path.is_file():
            fail(f"B0-L3a source is missing: {label}")
    b0_text = {label: path.read_text(encoding="utf-8") for label, path in b0_l3a_files.items()}
    for anchor, label in (
        ("fn block_expr_scope_region_pair(", "product"),
        ("struct ResolvedScopeSessionV1", "scope"),
        ("retire_scope_success", "identity"),
        ("BodyChildRoleV1::BlockExprPrelude", "lowerer"),
        ("ExprChildRoleV1::BlockExprTail", "lowerer"),
        ("ASTNode::BlockExpr", "capability"),
    ):
        if anchor not in b0_text[label]:
            fail(f"B0-L3a {label} anchor is missing: {anchor}")
    if b0_text["tests"].count("#[test]") != 5:
        fail("B0-L3a must retain five focused BlockExpr Lower tests")
    if b0_text["tests"].count('#[cfg(feature = "vm-reference")]\n#[test]') != 2:
        fail("B0-L3a must retain two VM-reference runtime fixtures")

    check_evidence(root, modules, "module_ingresses")
    check_evidence(root, functions, "function_families")
    check_evidence(root, seams, "body_route_seams")
    print("resolved_lowering_ingress_inventory=closed")
    print("resolved_lowering_module_ingresses=5")
    print("resolved_lowering_function_families=10")
    print("resolved_lowering_body_route_seams=2")
    print("resolved_lowering_typed_ingress=closed")
    print("resolved_lowering_request_match_sites=1")
    print("resolved_lowering_production_verified_unit_constructors=1")
    print("resolved_lowering_production_resolved_request_callers=0")
    print("resolved_lowering_production_activation=1-closed-family")
    print("resolved_lowering_source_navigator=closed")
    print("resolved_lowering_disconnected_exact_source_transport=1")
    print("resolved_lowering_mutable_source_cursor=0")
    print("resolved_lowering_pointer_span_name_identity=0")
    print("resolved_lowering_builder_consumers=1-closed-family")
    print("resolved_lowering_planner_consumers=0")
    print("resolved_lowering_function_transaction=closed")
    print("resolved_lowering_manual_function_cleanup_sites=0")
    print("resolved_lowering_unpublished_draft_before_cleanup=1")
    print("resolved_lowering_transaction_injected_checkpoints=5")
    print("resolved_lowering_sa3_b=closed")
    print("resolved_lowering_legacy_allocator_during_canonical=forbidden")
    print("resolved_lowering_selected_next_slice=B0-L3b")


if __name__ == "__main__":
    main()
