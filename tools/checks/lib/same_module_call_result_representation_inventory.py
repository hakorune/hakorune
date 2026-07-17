#!/usr/bin/env python3
"""Build/check the same-module call-result representation M0 inventory."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


FIXTURE = Path(
    "tools/checks/fixtures/"
    "same_module_call_result_representation_m0_inventory_v1.json"
)


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        raise SystemExit(f"[same-module-call-result-inventory] missing: {relative}")
    return path.read_text(encoding="utf-8")


def code_only(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//.*", "", text)


def count(text: str, needle: str) -> int:
    return code_only(text).count(needle)


def require_order(text: str, anchors: list[str], label: str) -> None:
    cursor = -1
    for anchor in anchors:
        position = text.find(anchor, cursor + 1)
        if position < 0:
            raise SystemExit(
                f"[same-module-call-result-inventory] missing {label}: {anchor}"
            )
        cursor = position


def build(root: Path) -> dict[str, object]:
    index_path = "src/mir/builder/declaration_indexer.rs"
    catalog_path = "src/mir/builder/callable_declaration_catalog/catalog.rs"
    lifecycle_path = "src/mir/builder/module_lifecycle.rs"
    static_call_path = "src/mir/builder/method_call_handlers.rs"
    emitter_path = "src/mir/builder/calls/unified_emitter.rs"
    annotation_path = "src/mir/builder/calls/annotation.rs"
    lowering_path = "src/mir/builder/calls/lowering.rs"
    hints_path = "src/mir/builder/type_hint_providers.rs"
    global_path = "src/mir/global_call_route_plan.rs"
    publication_path = "src/mir/global_call_route_plan/value_type_publish.rs"
    route_path = "src/mir/route_fixpoint.rs"
    semantic_path = "src/mir/semantic_refresh.rs"
    compiler_path = "src/mir/compiler/mod.rs"
    proof_path = "tools/checks/lib/same_module_call_result_representation_proof.py"
    caller_path = "lang/src/compiler/parser/parser_box.hako"
    callee_path = "lang/src/compiler/parser/scan/parser_string_utils_box.hako"

    index = read(root, index_path)
    catalog = read(root, catalog_path)
    lifecycle = read(root, lifecycle_path)
    static_call = read(root, static_call_path)
    emitter = read(root, emitter_path)
    annotation = read(root, annotation_path)
    lowering = read(root, lowering_path)
    hints = read(root, hints_path)
    global_plan = read(root, global_path)
    publication = read(root, publication_path)
    route = read(root, route_path)
    semantic = read(root, semantic_path)
    compiler = read(root, compiler_path)
    proof = read(root, proof_path)
    caller = read(root, caller_path)
    callee = read(root, callee_path)

    require_order(
        lifecycle,
        [
            "VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&snapshot)",
            "install_callable_declaration_catalog(callable_catalog)",
            "declaration_indexer::index_declarations(self, &snapshot)",
            "deferred_static_boxes.push((name.clone(), methods.clone()))",
            "for (name, methods) in deferred_static_boxes",
        ],
        "catalog seal/install before declaration index and static lowering",
    )
    require_order(
        static_call,
        [
            "let arg_values = self.build_call_args(arguments)?",
            "let dst = self.next_value_id()",
            "self.emit_unified_call(Some(dst), CallTarget::Global(func_name), arg_values)?",
        ],
        "static call result allocation and emission",
    )
    require_order(
        emitter,
        [
            "let res = builder.emit_instruction(call_inst)",
            "annotate_call_result_from_func_name(builder, dst, &func_name)",
        ],
        "call emission before current-module annotation",
    )
    require_order(
        lowering,
        [
            "TypePropagationPipeline::run(f, &mut self.type_ctx.value_types)?",
            "annotate_missing_result_types_from_calls_and_await(",
            "f.metadata.value_types = self.type_ctx.value_types.clone()",
        ],
        "function-local finalization",
    )
    require_order(
        semantic,
        [
            "refresh_all_functions_semantic_metadata(module, &module_metadata)",
            "refresh_module_route_convergence(module)",
        ],
        "complete-module semantic refresh",
    )
    require_order(
        compiler,
        [
            "let verification_result = self.verifier.verify_module(&module)",
            "refresh_module_semantic_metadata(&mut module)",
        ],
        "whole-module refresh after build verification",
    )

    callable_declaration_fields = {
        "params": count(catalog, "params: Box<[String]>,"),
        "param_decls": count(catalog, "param_decls: Box<[ParamDecl]>,"),
        "body": count(catalog, "body: Box<[ASTNode]>,"),
    }
    callable_declaration_return_spelling_count = count(
        catalog, "return_type_name: Option<Box<str>>"
    )
    builder_sources = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
    )
    old_store_occurrences = {
        symbol: builder_sources.count(symbol)
        for symbol in ("static_method_index", "LoweredMethodAst", "lowered_method_asts")
    }
    source_rows = {
        "forward_direct": (1, "missing"),
        "forward_copy": (1, "missing"),
        "reverse_direct": (6, "exact"),
        "typed_forward": (1, "missing"),
        "valid_numeric_control": (6, "exact"),
    }
    proof_rows = {
        name: {
            "expected_returncode": returncode,
            "expected_transient_state": outcome.capitalize(),
            "proof_row_count": count(
                proof,
                f'"{name}": {{"expected_rc": {returncode}, '
                f'"outcome": "{outcome}"}}',
            ),
        }
        for name, (returncode, outcome) in source_rows.items()
    }

    caller_method = caller[
        caller.index("\n  static_const_parse_add(text, pos) {") :
        caller.index("\n  static_const_parse_mul(text, pos) {")
    ]
    caller_before_loop, caller_loop_tail = caller_method.split(
        "loop(pos < text.length()", maxsplit=1
    )

    lowering_time_exact_owner_count = builder_sources.count(
        "pub(crate) struct VerifiedSameModuleCallableResultCatalogV1"
    )
    classification = (
        "CANONICAL-PRODUCER-PUBLICATION-REQUIRED"
        if lowering_time_exact_owner_count == 1
        else "CALLEE-REPRESENTATION-AUTHORITY-ABSENT"
    )

    return {
        "schema_version": 1,
        "row": "R0-SAME-MODULE-CALL-RESULT-REP0-M0",
        "classification": classification,
        "production_behavior_delta": 0,
        "production_type_publisher_delta": 0,
        "source_observation": {
            "rows": proof_rows,
            "debug_release_parity_check_count": count(
                proof, 'if modes["debug"] != modes["release"]'
            ),
            "final_metadata_authority": "diagnostic-only",
        },
        "actual_blocker": {
            "caller": "ParserBox.static_const_parse_add/2",
            "callee": "ParserStringUtilsBox.skip_ws/2",
            "selected_init": "ValueId(28)",
            "selected_pre_loop_call_count": caller_before_loop.count(
                "pos = ParserStringUtilsBox.skip_ws(text, me.static_const_eval_pos(ret))"
            ),
            "loop_body_refresh_call_count": caller_loop_tail.count(
                "pos = ParserStringUtilsBox.skip_ws(text, me.static_const_eval_pos(rhs))"
            ),
            "caller_following_loop_count": caller_method.count(
                "loop(pos < text.length()"
            ),
            "callee_definition_count": len(
                re.findall(r"^\s*skip_ws\s*\(src, i\)\s*\{", callee, flags=re.M)
            ),
            "callee_return_annotation_count": len(
                re.findall(r"skip_ws\s*\([^)]*\)\s*:\s*", callee)
            ),
        },
        "declaration_index": {
            "callable_declaration_body_fields": callable_declaration_fields,
            "callable_declaration_return_spelling_count": callable_declaration_return_spelling_count,
            "catalog_seal_root_calls": count(
                lifecycle,
                "VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&snapshot)",
            ),
            "catalog_install_calls": count(
                lifecycle, "install_callable_declaration_catalog(callable_catalog)"
            ),
            "old_store_occurrences": old_store_occurrences,
            "static_registration_count": count(
                index, "builder.comp_ctx.register_lowered_method_ast("
            ),
            "source_order_static_lowering_count": count(
                lifecycle, "for (name, methods) in deferred_static_boxes"
            ),
            "generic_result_representation_owner_count": lowering_time_exact_owner_count,
        },
        "lowering_time": {
            "static_call_result_allocations": count(
                static_call, "let dst = self.next_value_id()"
            ),
            "post_emit_annotation_calls": count(
                emitter,
                "annotate_call_result_from_func_name(builder, dst, &func_name)",
            ),
            "current_module_signature_lookups": count(
                annotation, "module.functions.get(name)"
            ),
            "known_name_heuristic_branches": count(annotation, 'if name == "JsonParser.'),
            "function_finalize_hint_provider_calls": count(
                lowering, "annotate_missing_result_types_from_calls_and_await("
            ),
            "hint_provider_module_lookups": count(
                hints, ".functions\n                                .get(name)"
            ),
            "hint_provider_unknown_fallbacks": count(
                hints, ".unwrap_or(MirType::Unknown)"
            ),
            "final_metadata_snapshots": count(
                lowering, "f.metadata.value_types = self.type_ctx.value_types.clone()"
            ),
        },
        "late_complete_module_publication": {
            "semantic_route_convergence_calls": count(
                semantic, "refresh_module_route_convergence(module)"
            ),
            "route_fixpoint_global_refresh_calls": count(
                route, "refresh_module_global_call_routes(module)"
            ),
            "global_result_publisher_calls": count(
                global_plan, "publish_global_call_route_result_value_types(module)"
            ),
            "metadata_result_publisher_definitions": count(
                publication,
                "fn publish_global_call_route_result_value_types(module: &mut MirModule)",
            ),
            "publication_surface": "MirFunction.metadata.value_types",
            "lowering_time_authority": False,
        },
        "nonauthority": {
            "generic_loop_representation_defaults": 0,
            "source_reordering": 0,
            "source_return_annotation_workarounds": 0,
            "final_metadata_lowering_time_reads": 0,
            "new_persistent_value_type_maps": 0,
            "fallback_or_retry": 0,
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    report = build(root)
    if args.check_reference:
        expected = json.loads(read(root, FIXTURE.as_posix()))
        if report != expected:
            print(json.dumps(report, indent=2, sort_keys=True))
            raise SystemExit("[same-module-call-result-inventory] reference drift")
        print("[same-module-call-result-inventory] reference=green")
    else:
        print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
