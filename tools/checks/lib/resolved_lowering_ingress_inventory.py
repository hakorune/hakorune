#!/usr/bin/env python3
"""Validate B0-L1 plus behavior-neutral B0-L2a/B0-L2b transport slices."""

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
    if lowering_text.count("\n    VerifiedResolvedSourceUnitV1 {\n") != 1:
        fail("verified source unit gained a constructor outside its type/test factory")
    if compiler_text.count("match request {") != 1:
        fail("MirLoweringRequestV1 must have exactly one match site")
    for anchor in (
        "pub fn compile_resolved(",
        "pub fn compile_legacy(",
        "Self::compile_resolved_inactive(input, source_file)",
        "let (ast, _legacy_origin) = input.into_parts();",
        ".compile_with_source_internal(ast, source_file)",
        'boundary: "B0-L2a"',
    ):
        if anchor not in compiler_text:
            fail(f"B0-L2a compiler boundary anchor is missing: {anchor}")
    production_resolved_callers = []
    for source in (root / "src").rglob("*.rs"):
        if source == lowering_input:
            continue
        if ".compile_resolved(" in source.read_text(encoding="utf-8"):
            production_resolved_callers.append(source.relative_to(root).as_posix())
    if production_resolved_callers:
        fail(f"resolved ingress gained production callers: {production_resolved_callers}")

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
    }
    if navigator != expected_navigator:
        fail(f"B0-L2b source navigator contract drifted: {navigator!r}")
    if data.get("selected_next_slice") != "B0-L2c-function-transaction":
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

    allowed_view_files = set(navigator_files.values()) | {lowering_input}
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
        fail(f"source navigator escaped compiler transport: {external_view_consumers}")

    check_evidence(root, modules, "module_ingresses")
    check_evidence(root, functions, "function_families")
    check_evidence(root, seams, "body_route_seams")
    print("resolved_lowering_ingress_inventory=closed")
    print("resolved_lowering_module_ingresses=5")
    print("resolved_lowering_function_families=10")
    print("resolved_lowering_body_route_seams=2")
    print("resolved_lowering_typed_ingress=closed")
    print("resolved_lowering_request_match_sites=1")
    print("resolved_lowering_production_verified_unit_constructors=0")
    print("resolved_lowering_production_resolved_request_callers=0")
    print("resolved_lowering_production_activation=0")
    print("resolved_lowering_source_navigator=closed")
    print("resolved_lowering_disconnected_exact_source_transport=1")
    print("resolved_lowering_mutable_source_cursor=0")
    print("resolved_lowering_pointer_span_name_identity=0")
    print("resolved_lowering_builder_planner_consumers=0")
    print("resolved_lowering_selected_next_slice=B0-L2c")


if __name__ == "__main__":
    main()
