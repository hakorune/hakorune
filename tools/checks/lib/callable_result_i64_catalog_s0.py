#!/usr/bin/env python3
"""Guard disconnected local-body exact-i64 callable result catalog S0a."""

from __future__ import annotations

import argparse
from pathlib import Path
import re


MODULE = Path("src/mir/callable_result_representation")
PRODUCT = "VerifiedSameModuleCallableResultCatalogV1"
TARGET_MODULE = Path("src/mir/source_call_target")
TARGET_PRODUCT = "VerifiedSourceStaticCallTargetCatalogV1"
CURRENT_OWNER_PRODUCT = "VerifiedCurrentOwnerStaticCallTargetV1"
SOURCE_METHOD_CALL_SITE_PRODUCT = "VerifiedSourceMethodCallSiteV1"
IMPORT_VIEW = "VerifiedStaticImportAliasViewV1"
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

    module_root = root / MODULE
    require(module_root.is_dir(), f"missing module: {MODULE}")
    rust_files = sorted(module_root.rglob("*.rs"))
    require(bool(rust_files), "result catalog has no Rust sources")
    sources = {path: path.read_text(encoding="utf-8") for path in rust_files}
    module_code = "\n".join(code_only(text) for text in sources.values())
    production = production_rust(root)
    solver = sources[module_root / "solver.rs"]
    disposition = sources[module_root / "disposition.rs"]

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
    target_internal_production = "\n".join(
        code_only(text)
        for path, text in target_sources.items()
        if path.name not in {"mod.rs", "source_method_call_site.rs"}
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
        production.count(TARGET_PRODUCT) == 0,
        "Q0 source target catalog gained a production producer or consumer",
    )
    require(
        production.count(SOURCE_METHOD_CALL_SITE_PRODUCT)
        + target_internal_production.count(SOURCE_METHOD_CALL_SITE_PRODUCT)
        == 0,
        "S0 exact source MethodCall site gained a production consumer",
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
        "imports.canonical_owner(candidate.receiver())" in target_qualified,
        "qualified target lost verified import-alias precedence",
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
        "actual_string_helpers_projects_digit_value_to_caller_owner" in target_code,
        "actual StringHelpers current-owner target fixture is missing",
    )
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
        "source_receiver_product_definitions": 1,
        "source_receiver_production_producers_consumers": 0,
        "source_receiver_forbidden_authority_occurrences": 0,
        "structural_source_path_projector_owners": 1,
        "source_path_vocabularies": 1,
        "compiler_projector_consumers": 1,
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
