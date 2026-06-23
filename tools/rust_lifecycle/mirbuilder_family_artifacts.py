#!/usr/bin/env python3
"""Spec-backed MirBuilder family artifact generators."""

from __future__ import annotations

from pathlib import Path
from textwrap import dedent

from extract_binding_context_facts import (
    SOURCE as BINDING_CONTEXT_SOURCE,
    extract_facts as extract_binding_context_facts,
)
from extract_box_compilation_context_facts import (
    SOURCE as BOX_COMPILATION_CONTEXT_SOURCE,
    extract_facts as extract_box_compilation_context_facts,
)
from extract_variable_context_simple_map_facts import (
    SOURCE as VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE,
    extract_facts as extract_variable_context_simple_map_facts,
)
from extract_variable_context_snapshot_restore_facts import (
    extract_facts as extract_variable_context_snapshot_restore_facts,
)
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from mirbuilder_core_context_artifacts import (
    CORE_CONTEXT_SOURCE,
    core_context_spec,
    extract_core_context_facts,
    validate_core_context,
)
from mirbuilder_family_validators import (
    validate_binding_context,
    validate_box_compilation_context,
    validate_variable_context_immutable_borrow,
    validate_variable_context_simple_map,
    validate_variable_context_snapshot_restore,
)
from verified_hako_family_ir import op
from shared_family_generator import read_json, run_validated_family_generator


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
VARIABLE_CONTEXT_SOURCE = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
VARIABLE_CONTEXT_FAMILY_ID = "hakorune_mir_builder::variable_context"


def _lines(text: str) -> list[str]:
    return dedent(text).strip("\n").splitlines()

def binding_context_spec() -> FamilyArtifactSpec:
    facts = extract_binding_context_facts(BINDING_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "binding-context-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("binding_context.single_ordered_map_context", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_binding_context_artifact.py",
        generator_version="binding-context-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json",
        family_comment="hakorune_mir_builder::binding_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="BindingContext",
            field_name="binding_map",
            field_type="OrderedMapBox",
            initializer_operation={"kind": "NewOrderedMap"},
        ),
        main_operations=[
            op("NewBox", target="ctx", box="BindingContext"),
            op("AssertEq", left="ctx.binding_map.keys_value.length()", right=0, fail_message="binding_context_new_empty=fail", fail_code=1),
            op("AssertEq", left="ctx.binding_map.keys_value.length()", right=0, fail_message="binding_context_new_len=fail", fail_code=2),
            op("StaticCall", callee="BindingContextApi.insert", args=["ctx", {"literal": "x"}, 1]),
            op("StaticCall", target="lookup_x", callee="BindingContextApi.lookup", args=["ctx", {"literal": "x"}]),
            op("AssertEq", left="lookup_x", right=1, fail_message="binding_context_lookup_x=fail", fail_code=3),
            op("AssertEq", left="ctx.binding_map.keys_value.length()", right=1, fail_message="binding_context_len_after_insert=fail", fail_code=4),
            op("AssertEq", left="ctx.binding_map.keys_value.length()", right=1, fail_message="binding_context_empty_after_insert=fail", fail_code=5),
            op("StaticCall", target="remove_x", callee="BindingContextApi.remove", args=["ctx", {"literal": "x"}]),
            op("AssertEq", left="remove_x", right=1, fail_message="binding_context_remove_x=fail", fail_code=6),
            op("StaticCall", target="lookup_removed", callee="BindingContextApi.lookup", args=["ctx", {"literal": "x"}]),
            op("AssertEq", left="lookup_removed", right=None, fail_message="binding_context_lookup_removed=fail", fail_code=7),
            op("AssertEq", left="ctx.binding_map.keys_value.length()", right=0, fail_message="binding_context_empty_after_remove=fail", fail_code=8),
            op("NewBox", target="contains_ctx", box="BindingContext"),
            op("StaticCall", target="contains_empty", callee="BindingContextApi.contains", args=["contains_ctx", {"literal": "x"}]),
            op("AssertEq", left="contains_empty", right=0, fail_message="binding_context_contains_empty=fail", fail_code=9),
            op("StaticCall", callee="BindingContextApi.insert", args=["contains_ctx", {"literal": "x"}, 1]),
            op("StaticCall", target="contains_x", callee="BindingContextApi.contains", args=["contains_ctx", {"literal": "x"}]),
            op("AssertEq", left="contains_x", right=1, fail_message="binding_context_contains_x=fail", fail_code=10),
            op("NewBox", target="order_ctx", box="BindingContext"),
            op("StaticCall", callee="BindingContextApi.insert", args=["order_ctx", {"literal": "b"}, 2]),
            op("StaticCall", callee="BindingContextApi.insert", args=["order_ctx", {"literal": "a"}, 1]),
            op("AssertEq", left="order_ctx.binding_map.keys_value.length()", right=2, fail_message="binding_context_order_len=fail", fail_code=11),
            op("StaticCall", target="lookup_a", callee="BindingContextApi.lookup", args=["order_ctx", {"literal": "a"}]),
            op("AssertEq", left="lookup_a", right=1, fail_message="binding_context_lookup_a=fail", fail_code=12),
            op("StaticCall", target="lookup_b", callee="BindingContextApi.lookup", args=["order_ctx", {"literal": "b"}]),
            op("AssertEq", left="lookup_b", right=2, fail_message="binding_context_lookup_b=fail", fail_code=13),
            op("NewBox", target="clear_ctx", box="BindingContext"),
            op("StaticCall", callee="BindingContextApi.insert", args=["clear_ctx", {"literal": "a"}, 1]),
            op("StaticCall", callee="BindingContextApi.clear_for_function_entry", args=["clear_ctx"]),
            op("AssertEq", left="clear_ctx.binding_map.keys_value.length()", right=0, fail_message="binding_context_clear_empty=fail", fail_code=14),
            op("Print", text="binding_context_derived_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::binding_context",
        state="DerivedShadow",
        source_rust_file=ROOT / "crates/hakorune_mir_builder/src/binding_context.rs",
        hako_path=OUT_DIR / "binding_context.hako",
        facts_path=FIXTURES / "binding-context-adapter-facts-v0.json",
        plan_path=FIXTURES / "binding-context-plan-v0.json",
        oracle_path=FIXTURES / "binding-context-oracle-vectors-v0.json",
        recipe_path=FIXTURES / "binding-context-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "binding-context-derived-artifact-verifier-result-v0.json",
        recipe_subject="hakorune_mir_builder::binding_context::BindingContext",
        selected_body_count="all_non_test_methods",
        api_name="BindingContextApi",
        api_methods=api_methods,
        api_trailing_blank_line=True,
        methods=[
            BehaviorMethodSpec(id="BindingContext::new", rust_operation="BTreeMap::new", hako_operation="OrderedMap.create", emits="birth initializes me.binding_map"),
            BehaviorMethodSpec(id="BindingContext::is_empty", rust_operation="BTreeMap::is_empty", hako_operation="OrderedMapBox.length == 0 as i64_bool_v0", emits="BindingContextApi.is_empty(ctx)"),
            BehaviorMethodSpec(id="BindingContext::len", rust_operation="BTreeMap::len", hako_operation="OrderedMapBox.length", emits="BindingContextApi.len(ctx)"),
            BehaviorMethodSpec(id="BindingContext::contains", rust_operation="BTreeMap::contains_key", hako_operation="OrderedMapBox.has as i64_bool_v0", emits="BindingContextApi.contains(ctx, name)"),
            BehaviorMethodSpec(id="BindingContext::lookup", rust_operation="BTreeMap::get(...).copied", hako_operation="OrderedMapBox.get", emits="BindingContextApi.lookup(ctx, name)"),
            BehaviorMethodSpec(id="BindingContext::insert", rust_operation="BTreeMap::insert", hako_operation="OrderedMapBox.set", emits="BindingContextApi.insert(ctx, name, binding_id)"),
            BehaviorMethodSpec(id="BindingContext::remove", rust_operation="BTreeMap::remove", hako_operation="OrderedMapBox.remove", emits="BindingContextApi.remove(ctx, name)"),
            BehaviorMethodSpec(id="BindingContext::clear_for_function_entry", rust_operation="BTreeMap::clear", hako_operation="OrderedMapBox.clear", emits="BindingContextApi.clear_for_function_entry(ctx)"),
        ],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "all_non_test_methods", "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["OrderedMap.create", "OrderedMapBox.length == 0 as i64_bool_v0", "OrderedMapBox.length", "OrderedMapBox.has as i64_bool_v0", "OrderedMapBox.get", "OrderedMapBox.set", "OrderedMapBox.remove", "OrderedMapBox.clear"],
        transport_notes={"bool_return_transport": "i64_bool_v0", "reason": "pure-first global helper ABI expects scalar i64 returns in this pilot"},
        denied_boundaries=["selfhost mainline selection", "HakoAdopted native source decision", "MirBuilder-wide lifecycle parity", "runtime try-Hako-then-Rust fallback"],
    )


def variable_context_simple_map_spec() -> FamilyArtifactSpec:
    excluded = ["VariableContext::variable_map", "VariableContext::variable_map_mut", "VariableContext::snapshot", "VariableContext::restore"]
    facts = extract_variable_context_simple_map_facts(VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE)
    plan = read_json(FIXTURES / "variable-context-simple-map-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("variable_context.single_ordered_map_context", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py",
        generator_version="variable-context-simple-map-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="VariableContext",
            field_name="variable_map",
            field_type="OrderedMapBox",
            initializer_operation={"kind": "NewOrderedMap"},
        ),
        main_operations=[
            op("NewBox", target="ctx", box="VariableContext"),
            op("StaticCall", target="new_empty", callee="VariableContextApi.is_empty", args=["ctx"]),
            op("AssertEq", left="new_empty", right=1, fail_message="variable_context_new_empty=fail", fail_code=1),
            op("StaticCall", target="new_len", callee="VariableContextApi.len", args=["ctx"]),
            op("AssertEq", left="new_len", right=0, fail_message="variable_context_new_len=fail", fail_code=2),
            op("StaticCall", callee="VariableContextApi.insert", args=["ctx", {"literal": "x"}, 42]),
            op("StaticCall", target="lookup_x", callee="VariableContextApi.lookup", args=["ctx", {"literal": "x"}]),
            op("AssertEq", left="lookup_x", right=42, fail_message="variable_context_lookup_x=fail", fail_code=3),
            op("StaticCall", target="len_after_insert", callee="VariableContextApi.len", args=["ctx"]),
            op("AssertEq", left="len_after_insert", right=1, fail_message="variable_context_len_after_insert=fail", fail_code=4),
            op("StaticCall", target="empty_after_insert", callee="VariableContextApi.is_empty", args=["ctx"]),
            op("AssertEq", left="empty_after_insert", right=0, fail_message="variable_context_empty_after_insert=fail", fail_code=5),
            op("StaticCall", target="remove_x", callee="VariableContextApi.remove", args=["ctx", {"literal": "x"}]),
            op("AssertEq", left="remove_x", right=42, fail_message="variable_context_remove_x=fail", fail_code=6),
            op("StaticCall", target="lookup_removed", callee="VariableContextApi.lookup", args=["ctx", {"literal": "x"}]),
            op("AssertEq", left="lookup_removed", right=None, fail_message="variable_context_lookup_removed=fail", fail_code=7),
            op("NewBox", target="contains_ctx", box="VariableContext"),
            op("StaticCall", target="contains_empty", callee="VariableContextApi.contains", args=["contains_ctx", {"literal": "x"}]),
            op("AssertEq", left="contains_empty", right=0, fail_message="variable_context_contains_empty=fail", fail_code=8),
            op("StaticCall", callee="VariableContextApi.insert", args=["contains_ctx", {"literal": "x"}, 1]),
            op("StaticCall", target="contains_x", callee="VariableContextApi.contains", args=["contains_ctx", {"literal": "x"}]),
            op("AssertEq", left="contains_x", right=1, fail_message="variable_context_contains_x=fail", fail_code=9),
            op("NewBox", target="ssa_ctx", box="VariableContext"),
            op("StaticCall", callee="VariableContextApi.insert", args=["ssa_ctx", {"literal": "x"}, 1]),
            op("StaticCall", callee="VariableContextApi.insert", args=["ssa_ctx", {"literal": "x"}, 2]),
            op("StaticCall", callee="VariableContextApi.insert", args=["ssa_ctx", {"literal": "x"}, 4]),
            op("StaticCall", target="ssa_lookup_x", callee="VariableContextApi.lookup", args=["ssa_ctx", {"literal": "x"}]),
            op("AssertEq", left="ssa_lookup_x", right=4, fail_message="variable_context_ssa_update=fail", fail_code=10),
            op("Print", text="variable_context_simple_map_derived_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=VARIABLE_CONTEXT_FAMILY_ID,
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_simple_map.hako",
        facts_path=FIXTURES / "variable-context-simple-map-facts-v0.json",
        plan_path=FIXTURES / "variable-context-simple-map-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-simple-map-oracle-vectors-v0.json",
        recipe_path=FIXTURES / "variable-context-simple-map-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "variable-context-simple-map-derived-artifact-verifier-result-v0.json",
        pilot_scope="VariableContext_simple_map_only",
        recipe_subject="hakorune_mir_builder::variable_context::VariableContext.simple_map",
        selected_body_count="simple_map_methods_only",
        api_name="VariableContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(id="VariableContext::lookup", rust_operation="BTreeMap::get(...).copied", hako_operation="OrderedMapBox.get", emits="VariableContextApi.lookup(ctx, name)"),
            BehaviorMethodSpec(id="VariableContext::contains", rust_operation="BTreeMap::contains_key", hako_operation="OrderedMapBox.has as i64_bool_v0", emits="VariableContextApi.contains(ctx, name)"),
            BehaviorMethodSpec(id="VariableContext::len", rust_operation="BTreeMap::len", hako_operation="OrderedMapBox.length", emits="VariableContextApi.len(ctx)"),
            BehaviorMethodSpec(id="VariableContext::is_empty", rust_operation="BTreeMap::is_empty", hako_operation="OrderedMapBox.length == 0 as i64_bool_v0", emits="VariableContextApi.is_empty(ctx)"),
            BehaviorMethodSpec(id="VariableContext::insert", rust_operation="BTreeMap::insert", hako_operation="OrderedMapBox.set", emits="VariableContextApi.insert(ctx, name, value_id)"),
            BehaviorMethodSpec(id="VariableContext::remove", rust_operation="BTreeMap::remove", hako_operation="OrderedMapBox.remove", emits="VariableContextApi.remove(ctx, name)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_variable_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "simple_map_methods_only", "full_variable_context_claim": 0, "excluded_methods": excluded, "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["OrderedMapBox.get", "OrderedMapBox.has as i64_bool_v0", "OrderedMapBox.length", "OrderedMapBox.length == 0 as i64_bool_v0", "OrderedMapBox.set", "OrderedMapBox.remove"],
        transport_notes={"bool_return_transport": "i64_bool_v0", "value_id_transport": "i64"},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def box_compilation_context_spec() -> FamilyArtifactSpec:
    facts = extract_box_compilation_context_facts(BOX_COMPILATION_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "box-compilation-context-plan-v0.json")
    oracle = read_json(FIXTURES / "box-compilation-context-oracle-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("box_compilation_context.multi_ordered_map_context", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_box_compilation_context_artifact.py",
        generator_version="box-compilation-context-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json",
        family_comment="hakorune_mir_builder::context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="BoxCompilationContext",
            fields=[
                FieldSpec(name="variable_map", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
                FieldSpec(name="value_origin_newbox", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
                FieldSpec(name="value_types", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
            ],
        ),
        main_operations=[
            op("NewBox", target="ctx", box="BoxCompilationContext"),
            op("StaticCall", target="is_empty_result", callee="BoxCompilationContextApi.is_empty", args=["ctx"]),
            op("AssertEq", left="is_empty_result", right=1, fail_message="box_compilation_context_new_empty=fail", fail_code=1),
            op("Print", text="box_compilation_context_derived_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::context",
        state="DerivedShadow",
        source_rust_file=BOX_COMPILATION_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "box_compilation_context.hako",
        facts_path=FIXTURES / "box-compilation-context-facts-v0.json",
        plan_path=FIXTURES / "box-compilation-context-plan-v0.json",
        oracle_path=FIXTURES / "box-compilation-context-oracle-v0.json",
        recipe_path=FIXTURES / "box-compilation-context-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "box-compilation-context-derived-artifact-verifier-result-v0.json",
        pilot_scope="BoxCompilationContext_ctor_is_empty_only",
        recipe_subject="hakorune_mir_builder::context::BoxCompilationContext",
        selected_body_count="constructor_is_empty_only",
        api_name="BoxCompilationContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(
                id="BoxCompilationContext::new",
                rust_operation="DefaultConstruct",
                hako_operation="BoxCompilationContext.birth",
                emits="BoxCompilationContext.birth initializes three ordered maps",
            ),
            BehaviorMethodSpec(
                id="BoxCompilationContext::is_empty",
                rust_operation="CompositeMapIsEmpty",
                hako_operation="BoxCompilationContextBox.all_fields_empty",
                emits="BoxCompilationContextApi.is_empty(ctx)",
            ),
        ],
        excluded_methods=["BoxCompilationContext::size_info"],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "constructor_is_empty_only", "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["DefaultConstruct", "NewOrderedMap", "AllFieldsMapIsEmpty"],
        transport_notes={"bool_return_transport": "i64_bool_v0", "box_birth": "three ordered maps"},
        extra_manifest_fields={"excluded_methods": ["BoxCompilationContext::size_info"]},
    )


def variable_context_immutable_borrow_spec() -> FamilyArtifactSpec:
    excluded = ["VariableContext::variable_map_mut", "VariableContext::snapshot", "VariableContext::restore"]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py",
        generator_version="variable-context-immutable-borrow-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="VariableContext", field_name="variable_map", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
        main_operations=[
            op("NewBox", target="ctx", box="VariableContext"),
            op("StaticCall", target="view", callee="VariableContextApi.variable_map", args=["ctx"]),
            op("StaticCall", target="view_empty", callee="VariableMapViewApi.is_empty", args=["view"]),
            op("AssertEq", left="view_empty", right=1, fail_message="variable_context_borrow_view_empty=fail", fail_code=1),
            op("StaticCall", target="view_len", callee="VariableMapViewApi.len", args=["view"]),
            op("AssertEq", left="view_len", right=0, fail_message="variable_context_borrow_view_len=fail", fail_code=2),
            op("StaticCall", target="view_contains", callee="VariableMapViewApi.contains", args=["view", {"literal": "x"}]),
            op("AssertEq", left="view_contains", right=0, fail_message="variable_context_borrow_view_contains=fail", fail_code=3),
            op("StaticCall", target="view_lookup", callee="VariableMapViewApi.lookup", args=["view", {"literal": "x"}]),
            op("AssertEq", left="view_lookup", right=None, fail_message="variable_context_borrow_view_lookup=fail", fail_code=4),
            op("Print", text="variable_context_immutable_borrow_derived_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=VARIABLE_CONTEXT_FAMILY_ID,
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_immutable_borrow.hako",
        facts_path=FIXTURES / "variable-context-immutable-borrow-facts-v0.json",
        plan_path=FIXTURES / "variable-context-immutable-borrow-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-immutable-borrow-oracle-vectors-v0.json",
        pilot_scope="VariableContext_immutable_borrow_only",
        static_boxes=[
            StaticBoxSpec(
                name="VariableContextApi",
                methods=[
                    ApiMethodSpec(
                        signature="variable_map(ctx)",
                        operations=[op("ReturnSource", source="ctx.variable_map").to_json()],
                    )
                ],
            ),
            StaticBoxSpec(
                name="VariableMapViewApi",
                methods=[
                    ApiMethodSpec(signature="is_empty(view): i64", operations=[op("MapIsEmpty", source="view").to_json()]),
                    ApiMethodSpec(signature="len(view): i64", operations=[op("MapLength", source="view").to_json()]),
                    ApiMethodSpec(signature="contains(view, name): i64", operations=[op("MapHas", source="view", key="name").to_json()]),
                    ApiMethodSpec(signature="lookup(view, name)", operations=[op("MapGet", source="view", key="name").to_json()]),
                ],
            ),
        ],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_variable_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def variable_context_snapshot_restore_spec() -> FamilyArtifactSpec:
    excluded = ["VariableContext::variable_map", "VariableContext::variable_map_mut", "VariableContext::lookup", "VariableContext::require", "VariableContext::insert", "VariableContext::remove", "VariableContext::contains", "VariableContext::len", "VariableContext::is_empty", "CarrierInfo::from_variable_map", "CarrierInfo::with_explicit_carriers", "PHI planner integration"]
    facts = extract_variable_context_snapshot_restore_facts(VARIABLE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "variable-context-snapshot-restore-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("variable_context.owned_ordered_map_snapshot", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py",
        generator_version="variable-context-snapshot-restore-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="VariableContext",
            field_name="variable_map",
            field_type="OrderedMapBox",
            initializer_operation={"kind": "NewOrderedMap"},
        ),
        main_operations=[
            op("NewBox", target="ctx", box="VariableContext"),
            op("StaticCall", target="snapshot", callee="VariableContextApi.snapshot", args=["ctx"]),
            op("StaticCall", callee="VariableContextApi.restore", args=["ctx", "snapshot"]),
            op("StaticCall", target="empty_after_restore", callee="VariableContextApi.is_empty", args=["ctx"]),
            op("AssertEq", left="empty_after_restore", right=1, fail_message="variable_context_snapshot_restore_empty=fail", fail_code=1),
            op("Print", text="variable_context_snapshot_restore_derived_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=VARIABLE_CONTEXT_FAMILY_ID,
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_snapshot_restore.hako",
        facts_path=FIXTURES / "variable-context-snapshot-restore-facts-v0.json",
        plan_path=FIXTURES / "variable-context-snapshot-restore-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-snapshot-restore-oracle-vectors-v0.json",
        pilot_scope="VariableContext_snapshot_restore_only",
        api_name="VariableContextApi",
        api_methods=api_methods,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_variable_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        extra_manifest_fields={"excluded_methods": excluded},
    )


_GENERATORS = {
    "binding_context": (binding_context_spec, validate_binding_context, lambda spec: extract_binding_context_facts(BINDING_CONTEXT_SOURCE), "generated_binding_context_artifact=unchanged"),
    "box_compilation_context": (box_compilation_context_spec, validate_box_compilation_context, lambda spec: extract_box_compilation_context_facts(BOX_COMPILATION_CONTEXT_SOURCE), "generated_box_compilation_context_artifact=unchanged"),
    "core_context": (core_context_spec, validate_core_context, lambda spec: extract_core_context_facts(CORE_CONTEXT_SOURCE), "generated_core_context_artifact=unchanged"),
    "variable_context_simple_map": (variable_context_simple_map_spec, validate_variable_context_simple_map, lambda spec: extract_variable_context_simple_map_facts(VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE), "generated_variable_context_simple_map_artifact=unchanged"),
    "variable_context_immutable_borrow": (variable_context_immutable_borrow_spec, validate_variable_context_immutable_borrow, lambda spec: read_json(spec.facts_path), "generated_variable_context_immutable_borrow_artifact=unchanged"),
    "variable_context_snapshot_restore": (variable_context_snapshot_restore_spec, validate_variable_context_snapshot_restore, lambda spec: extract_variable_context_snapshot_restore_facts(VARIABLE_CONTEXT_SOURCE), "generated_variable_context_snapshot_restore_artifact=unchanged"),
}


def run_mirbuilder_family_artifact_generator(name: str, *, check: bool) -> None:
    try:
        spec_factory, validator, facts_loader, unchanged_label = _GENERATORS[name]
    except KeyError as exc:
        raise SystemExit(f"unknown MirBuilder family artifact generator: {name}") from exc

    spec = spec_factory()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(
        spec,
        hako_text=hako_text,
        recipe_text=recipe_text,
        verifier_text=verifier_text,
    )
    outputs: list[tuple[Path, str]] = []
    if recipe_text is not None and spec.recipe_path is not None:
        outputs.append((spec.recipe_path, recipe_text))
    if verifier_text is not None and spec.verifier_path is not None:
        outputs.append((spec.verifier_path, verifier_text))
    outputs.extend([(spec.hako_path, hako_text), (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text)])
    run_validated_family_generator(
        check=check,
        root=ROOT,
        unchanged_label=unchanged_label,
        load_facts=lambda: facts_loader(spec),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validator,
        outputs_factory=lambda: outputs,
    )
