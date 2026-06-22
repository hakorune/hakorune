#!/usr/bin/env python3
"""Carrier snapshot artifact generator for the MirBuilder family."""

from __future__ import annotations

from pathlib import Path
from textwrap import dedent
from typing import Any

from mirbuilder_carrier_snapshot_converter import (
    compile_carrier_snapshot_methods,
    compile_explicit_carrier_snapshot_methods,
)
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FamilyArtifactSpec, StaticBoxSpec
from extract_variable_context_carrier_snapshot_facts import (
    SOURCE as CARRIER_SNAPSHOT_SOURCE,
    extract_facts as extract_variable_context_carrier_snapshot_facts,
)
from extract_variable_context_explicit_carrier_snapshot_facts import (
    SOURCE as EXPLICIT_CARRIER_SNAPSHOT_SOURCE,
    extract_facts as extract_variable_context_explicit_carrier_snapshot_facts,
)
from shared_family_generator import read_json, run_validated_family_generator
from verified_hako_family_ir import HakoMethodIR


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
VARIABLE_CONTEXT_SOURCE = ROOT / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs"


def _lines(text: str) -> list[str]:
    return dedent(text).strip("\n").splitlines()


def _api_methods_from_compiled(methods: list[HakoMethodIR]) -> list[ApiMethodSpec]:
    return [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in methods
    ]


def _require_kinds(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any], *, subject: str) -> None:
    if facts.get("kind") != "RustLifecycleFacts":
        raise SystemExit("unexpected facts kind")
    if plan.get("kind") != "HakoLifecyclePlan":
        raise SystemExit("unexpected plan kind")
    if oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected oracle kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("subject mismatch")


def _write_validated_family_artifact(
    *,
    check: bool,
    unchanged_label: str,
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
    validate: Any,
    spec_factory: Any,
) -> None:
    validate(facts, plan, oracle)
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
    outputs = []
    if recipe_text is not None and spec.recipe_path is not None:
        outputs.append((spec.recipe_path, recipe_text))
    if verifier_text is not None and spec.verifier_path is not None:
        outputs.append((spec.verifier_path, verifier_text))
    outputs.extend([(spec.hako_path, hako_text), (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text)])
    return run_validated_family_generator(
        check=check,
        root=ROOT,
        unchanged_label=unchanged_label,
        load_facts=lambda: facts,
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=lambda loaded_facts, loaded_plan, loaded_oracle: validate(loaded_facts, loaded_plan, loaded_oracle),
        outputs_factory=lambda: outputs,
    )


def validate_variable_context_carrier_snapshot(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::variable_context::CarrierInfo.from_variable_map"
    _require_kinds(facts, plan, oracle, subject=subject)
    if facts.get("base_facts") != ["variable-context-simple-map-facts-v0.json", "variable-context-snapshot-restore-facts-v0.json"]:
        raise SystemExit("unexpected base facts")
    method = facts["method_fact"]
    if method.get("id") != "CarrierInfo::from_variable_map" or method.get("operation") != "CarrierSnapshotFromOwnedMap":
        raise SystemExit("unexpected carrier snapshot method")
    if method.get("input_snapshot", {}).get("ownership") != "OwnedReadSnapshotProjection":
        raise SystemExit("missing owned snapshot projection")
    if method.get("input_snapshot", {}).get("access") != "read" or method.get("input_snapshot", {}).get("escapes") is not False:
        raise SystemExit("unexpected carrier snapshot input facts")
    if method.get("map_requirements", {}).get("deterministic_order_required") is not True:
        raise SystemExit("missing deterministic order requirement")
    if method.get("map_requirements", {}).get("value_drop_fact") != "TrivialMemory":
        raise SystemExit("unexpected value drop fact")
    if method.get("output", {}).get("owns_carrier_names") is not True or method.get("output", {}).get("copies_value_ids") is not True:
        raise SystemExit("unexpected output ownership facts")
    if method.get("output", {}).get("value_id_copy_kind") != "ImmediateValue" or method.get("output", {}).get("join_id_initialized") is not False:
        raise SystemExit("unexpected carrier output facts")

    denied = set(facts.get("denied_followups", []))
    for item in ["CarrierInfo::with_explicit_carriers", "join_id lifecycle", "promoted_body_locals lifecycle", "trim_helper lifecycle", "PHI planner integration"]:
        if item not in denied:
            raise SystemExit(f"missing denied followup: {item}")

    denied_methods = {row["id"]: row for row in facts.get("denied_methods", [])}
    if denied_methods.get("VariableContext::variable_map", {}).get("deny_reason") != "ReturnedReadBorrow":
        raise SystemExit("VariableContext::variable_map must deny returned read borrow")

    entry = plan["plans"][0]
    if entry.get("plan_kind") != "CarrierSnapshotFromOwnedMap" or entry.get("mutation_policy") != "none":
        raise SystemExit("unexpected carrier snapshot plan")
    if entry.get("publication_policy") != "does_not_publish_variable_map":
        raise SystemExit("unexpected publication policy")
    output_policy = entry.get("output_policy", {})
    if output_policy.get("carrier_names") != "owned_strings" or output_policy.get("host_id") != "copied_ValueId":
        raise SystemExit("unexpected carrier output policy")
    if output_policy.get("join_id") != "None_uninitialized" or output_policy.get("role") != "LoopState" or output_policy.get("init") != "FromHost":
        raise SystemExit("unexpected carrier initialization policy")
    for fact in ["input_snapshot.ownership=OwnedReadSnapshotProjection", "map_requirements.deterministic_order_required=true", "output.value_id_copy_kind=ImmediateValue"]:
        if fact not in set(entry.get("required_facts", [])):
            raise SystemExit(f"missing required fact: {fact}")
    behavior = plan.get("behavior", {})
    for key in ["general_resolver_implemented", "converter_emission_added", "rust_lifetime_syntax_added", "phi_join_id_claim", "full_variable_context_claim"]:
        if behavior.get(key, False) is not False:
            raise SystemExit(f"unexpected behavior flag: {key}")

    vector = oracle["vectors"][0]
    if vector.get("loop_var_name") != "i" or vector.get("expect", {}).get("loop_var_id") != 5:
        raise SystemExit("unexpected oracle loop var")
    if vector.get("expect", {}).get("carrier_count") != 2:
        raise SystemExit("unexpected oracle carrier count")
    if [row["name"] for row in vector.get("expect", {}).get("carriers", [])] != ["count", "sum"]:
        raise SystemExit("unexpected oracle carrier order")
    if [row["host_id"] for row in vector.get("expect", {}).get("carriers", [])] != [11, 10]:
        raise SystemExit("unexpected oracle carrier hosts")
    requires = set(vector.get("requires", []))
    for item in ["owned_read_snapshot_projection", "deterministic_order_required=true", "ValueId.copy_kind=ImmediateValue"]:
        if item not in requires:
            raise SystemExit(f"missing oracle requirement: {item}")
    denied_vectors = set(oracle.get("denied_vectors", []))
    for item in ["with_explicit_carriers", "join_id_assignment", "promoted_body_locals", "trim_helper", "phi_planner_integration"]:
        if item not in denied_vectors:
            raise SystemExit(f"missing denied oracle vector: {item}")
    scope = oracle.get("promotion_scope", {})
    if scope.get("hako_authority") != "CarrierInfo::from_variable_map snapshot only" or scope.get("phi_join_id_claim") is not False:
        raise SystemExit("unexpected carrier promotion scope")


def carrier_snapshot_spec(carrier_api_methods: list[ApiMethodSpec]) -> FamilyArtifactSpec:
    excluded = [
        "VariableContext::variable_map_mut",
        "VariableContext::variable_map",
        "VariableContext::restore",
        "CarrierInfo::with_explicit_carriers",
        "join_id lifecycle",
        "promoted_body_locals lifecycle",
        "trim_helper lifecycle",
        "PHI planner integration",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py",
        generator_version="variable-context-carrier-snapshot-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="VariableContext", field_name="variable_map", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
        main_lines=_lines("""
            local ctx = new VariableContext()
            ctx.variable_map.set("count", 11)
            ctx.variable_map.set("i", 5)
            ctx.variable_map.set("sum", 10)

            local snapshot = VariableContextApi.snapshot(ctx)
            local carrier_names = new ArrayBox()
            local carrier_host_ids = new ArrayBox()
            if CarrierInfoApi.from_snapshot("i", snapshot, carrier_names, carrier_host_ids) != 0 {
                print("carrier_snapshot_status=fail")
                return 1
            }
            ctx.variable_map.set("sum", 99)
            snapshot.set("count", 77)
            if carrier_names.length() != 2 {
                print("carrier_snapshot_carrier_names_len=fail")
                return 2
            }
            if BoxHelpers.array_get(carrier_names, 0) != "count" or BoxHelpers.array_get(carrier_names, 1) != "sum" {
                print("carrier_snapshot_carrier_names_order=fail")
                return 3
            }
            if BoxHelpers.array_get(carrier_host_ids, 0) != 11 or BoxHelpers.array_get(carrier_host_ids, 1) != 10 {
                print("carrier_snapshot_carrier_hosts=fail")
                return 4
            }
            local count_values = ctx.variable_map.values()
            if BoxHelpers.array_get(count_values, 0) != 11 {
                print("carrier_snapshot_ctx_alias=fail")
                return 5
            }

            print("variable_context_carrier_snapshot_derived_artifact=ok")
            return 0
        """),
        family_id="hakorune_mir_builder::variable_context",
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_carrier_snapshot.hako",
        facts_path=FIXTURES / "variable-context-carrier-snapshot-facts-v0.json",
        plan_path=FIXTURES / "variable-context-carrier-snapshot-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-carrier-snapshot-oracle-vectors-v0.json",
        recipe_path=FIXTURES / "variable-context-carrier-snapshot-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "variable-context-carrier-snapshot-derived-artifact-verifier-result-v0.json",
        pilot_scope="VariableContext_carrier_snapshot_only",
        static_boxes=[
            StaticBoxSpec(
                name="VariableContextApi",
                methods=[
                    ApiMethodSpec(signature="snapshot(ctx): OrderedMapBox", operations=[{"kind": "CloneOwnedMap", "field": "variable_map"}]),
                ],
            ),
            StaticBoxSpec(
                name="CarrierInfoApi",
                methods=carrier_api_methods,
            ),
        ],
        recipe_subject="hakorune_mir_builder::variable_context::CarrierInfo.from_variable_map",
        selected_body_count="carrier_snapshot_methods_only",
        methods=[
            BehaviorMethodSpec(
                id="CarrierInfo::from_variable_map",
                rust_operation="CarrierSnapshotFromOwnedMap",
                hako_operation="CarrierInfoBox.from_snapshot",
                emits="CarrierInfoApi.from_snapshot(carrier_data, loop_var_name, snapshot)",
            )
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_variable_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "carrier_snapshot_methods_only", "full_variable_context_claim": 0, "carrier_behavior_generated": 1, "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["CarrierSnapshotFromOwnedMap", "CloneOwnedMap", "OrderedMap.create", "OrderedMapBox.set", "OrderedMapBox.get", "OrderedMapBox.key_at", "OrderedMapBox.length", "ArrayBox.push"],
        transport_notes={"carrier_info_transport": "caller-owned OrderedMapBox carrier_data", "loop_var_id_transport": "i64", "carrier_names_transport": "ArrayBox", "carrier_host_ids_transport": "ArrayBox"},
        denied_boundaries=excluded,
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_variable_context_carrier_snapshot_artifact_generator(*, check: bool) -> None:
    facts = extract_variable_context_carrier_snapshot_facts(CARRIER_SNAPSHOT_SOURCE)
    plan = read_json(FIXTURES / "variable-context-carrier-snapshot-plan-v0.json")
    oracle = read_json(FIXTURES / "variable-context-carrier-snapshot-oracle-vectors-v0.json")
    _write_validated_family_artifact(
        check=check,
        unchanged_label="generated_variable_context_carrier_snapshot_artifact=unchanged",
        facts=facts,
        plan=plan,
        oracle=oracle,
        validate=validate_variable_context_carrier_snapshot,
        spec_factory=lambda: carrier_snapshot_spec(_api_methods_from_compiled(compile_carrier_snapshot_methods(facts, plan))),
    )


def validate_variable_context_explicit_carrier_snapshot(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::variable_context::CarrierInfo.with_explicit_carriers"
    _require_kinds(facts, plan, oracle, subject=subject)
    if facts.get("base_facts") != ["variable-context-carrier-snapshot-facts-v0.json", "variable-context-snapshot-restore-facts-v0.json"]:
        raise SystemExit("unexpected explicit carrier base facts")
    method = facts["method_fact"]
    if method.get("id") != "CarrierInfo::with_explicit_carriers" or method.get("operation") != "ExplicitCarrierSnapshotFromOwnedMap":
        raise SystemExit("unexpected explicit carrier snapshot method")
    if method.get("input_snapshot", {}).get("ownership") != "OwnedReadSnapshotProjection":
        raise SystemExit("missing owned snapshot projection")
    if method.get("input_snapshot", {}).get("access") != "read" or method.get("input_snapshot", {}).get("escapes") is not False:
        raise SystemExit("unexpected explicit carrier snapshot facts")
    if method.get("carrier_names", {}).get("ownership") != "owned_strings" or method.get("carrier_names", {}).get("missing_carrier_policy") != "fail_fast":
        raise SystemExit("unexpected explicit carrier name facts")
    if method.get("map_requirements", {}).get("deterministic_order_required") is not True:
        raise SystemExit("missing explicit carrier deterministic order requirement")
    if method.get("map_requirements", {}).get("value_drop_fact") != "TrivialMemory":
        raise SystemExit("unexpected explicit carrier value drop fact")
    if method.get("output", {}).get("owns_carrier_names") is not True or method.get("output", {}).get("copies_value_ids") is not True:
        raise SystemExit("unexpected explicit carrier output ownership facts")
    if method.get("output", {}).get("value_id_copy_kind") != "ImmediateValue" or method.get("output", {}).get("join_id_initialized") is not False:
        raise SystemExit("unexpected explicit carrier output facts")

    denied = set(facts.get("denied_followups", []))
    for item in ["join_id lifecycle", "promoted_body_locals lifecycle", "trim_helper lifecycle", "PHI planner integration"]:
        if item not in denied:
            raise SystemExit(f"missing denied followup: {item}")

    denied_methods = {row["id"]: row for row in facts.get("denied_methods", [])}
    if denied_methods.get("VariableContext::variable_map", {}).get("deny_reason") != "ReturnedReadBorrow":
        raise SystemExit("VariableContext::variable_map must deny returned read borrow")

    entry = plan["plans"][0]
    if entry.get("plan_kind") != "ExplicitCarrierSnapshotFromOwnedMap" or entry.get("mutation_policy") != "none":
        raise SystemExit("unexpected explicit carrier plan")
    if entry.get("publication_policy") != "does_not_publish_variable_map":
        raise SystemExit("unexpected explicit carrier publication policy")
    output_policy = entry.get("output_policy", {})
    if output_policy.get("carrier_names") != "owned_strings" or output_policy.get("host_id") != "copied_ValueId":
        raise SystemExit("unexpected explicit carrier output policy")
    if output_policy.get("join_id") != "None_uninitialized" or output_policy.get("role") != "LoopState" or output_policy.get("init") != "FromHost":
        raise SystemExit("unexpected explicit carrier initialization policy")
    for fact in [
        "input_snapshot.ownership=OwnedReadSnapshotProjection",
        "input_snapshot.escapes=false",
        "carrier_names.ownership=owned_strings",
        "carrier_names.missing_carrier_policy=fail_fast",
        "map_requirements.value_drop_fact=TrivialMemory",
        "output.value_id_copy_kind=ImmediateValue",
    ]:
        if fact not in set(entry.get("required_facts", [])):
            raise SystemExit(f"missing required fact: {fact}")
    behavior = plan.get("behavior", {})
    for key in ["general_resolver_implemented", "converter_emission_added", "rust_lifetime_syntax_added", "phi_join_id_claim", "full_variable_context_claim"]:
        if behavior.get(key, False) is not False:
            raise SystemExit(f"unexpected explicit carrier behavior flag: {key}")

    vectors = {row["id"]: row for row in oracle["vectors"]}
    ok = vectors["loop_var_i_with_requested_carriers"]
    if ok.get("loop_var_name") != "i" or ok.get("loop_var_id") != 5:
        raise SystemExit("unexpected explicit carrier oracle loop var")
    if ok.get("carrier_names") != ["sum", "count"]:
        raise SystemExit("unexpected explicit carrier requested names")
    if ok.get("expect", {}).get("carrier_count") != 2:
        raise SystemExit("unexpected explicit carrier count")
    if [row["name"] for row in ok.get("expect", {}).get("carriers", [])] != ["count", "sum"]:
        raise SystemExit("unexpected explicit carrier order")
    if [row["host_id"] for row in ok.get("expect", {}).get("carriers", [])] != [11, 10]:
        raise SystemExit("unexpected explicit carrier host ids")
    requires = set(ok.get("requires", []))
    for item in ["owned_read_snapshot_projection", "requested_names_owned", "missing_carrier_fail_fast", "ValueId.copy_kind=ImmediateValue"]:
        if item not in requires:
            raise SystemExit(f"missing explicit carrier oracle requirement: {item}")
    missing = vectors["missing_requested_carrier_fails"]
    if missing.get("expect_error") != "Carrier variable 'missing' not found in variable_map":
        raise SystemExit("unexpected explicit carrier missing-carrier error")
    denied_vectors = set(oracle.get("denied_vectors", []))
    for item in ["join_id_assignment", "promoted_body_locals", "trim_helper", "phi_planner_integration"]:
        if item not in denied_vectors:
            raise SystemExit(f"missing explicit carrier denied vector: {item}")
    scope = oracle.get("promotion_scope", {})
    if scope.get("hako_authority") != "CarrierInfo::with_explicit_carriers snapshot only" or scope.get("phi_join_id_claim") is not False:
        raise SystemExit("unexpected explicit carrier promotion scope")


def explicit_carrier_snapshot_spec(carrier_api_methods: list[ApiMethodSpec]) -> FamilyArtifactSpec:
    excluded = [
        "VariableContext::variable_map_mut",
        "VariableContext::variable_map",
        "VariableContext::restore",
        "CarrierInfo::from_variable_map",
        "join_id lifecycle",
        "promoted_body_locals lifecycle",
        "trim_helper lifecycle",
        "PHI planner integration",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_variable_context_explicit_carrier_snapshot_artifact.py",
        generator_version="variable-context-explicit-carrier-snapshot-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/variable_context_explicit_carrier_snapshot.artifact.json",
        family_comment="hakorune_mir_builder::variable_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="VariableContext", field_name="variable_map", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
        main_lines=_lines("""
            local ctx = new VariableContext()
            ctx.variable_map.set("count", 11)
            ctx.variable_map.set("i", 5)
            ctx.variable_map.set("sum", 10)
            ctx.variable_map.set("tmp", 12)

            local snapshot = VariableContextApi.snapshot(ctx)
            local requested_names = new ArrayBox()
            requested_names.push("sum")
            requested_names.push("count")
            local carrier_names = new ArrayBox()
            local carrier_host_ids = new ArrayBox()
            if CarrierInfoApi.with_explicit_carriers_from_snapshot(5, requested_names, snapshot, carrier_names, carrier_host_ids) != 0 {
                print("explicit_carrier_snapshot_status=fail")
                return 1
            }

            ctx.variable_map.set("sum", 99)
            requested_names.push("late")
            snapshot.set("count", 77)
            if carrier_names.length() != 2 {
                print("explicit_carrier_snapshot_carrier_names_len=fail")
                return 2
            }
            if BoxHelpers.array_get(carrier_names, 0) != "count" or BoxHelpers.array_get(carrier_names, 1) != "sum" {
                print("explicit_carrier_snapshot_carrier_names_order=fail")
                return 3
            }
            if BoxHelpers.array_get(carrier_host_ids, 0) != 11 or BoxHelpers.array_get(carrier_host_ids, 1) != 10 {
                print("explicit_carrier_snapshot_carrier_hosts=fail")
                return 4
            }
            local count_values = ctx.variable_map.values()
            if BoxHelpers.array_get(count_values, 0) != 11 {
                print("explicit_carrier_snapshot_ctx_alias=fail")
                return 5
            }

            print("variable_context_explicit_carrier_snapshot_derived_artifact=ok")
            return 0
        """),
        family_id="hakorune_mir_builder::variable_context",
        state="DerivedShadow",
        source_rust_file=VARIABLE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "variable_context_explicit_carrier_snapshot.hako",
        facts_path=FIXTURES / "variable-context-explicit-carrier-snapshot-facts-v0.json",
        plan_path=FIXTURES / "variable-context-explicit-carrier-snapshot-plan-v0.json",
        oracle_path=FIXTURES / "variable-context-explicit-carrier-snapshot-oracle-vectors-v0.json",
        recipe_path=FIXTURES / "variable-context-explicit-carrier-snapshot-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "variable-context-explicit-carrier-snapshot-derived-artifact-verifier-result-v0.json",
        pilot_scope="VariableContext_explicit_carrier_snapshot_only",
        static_boxes=[
            StaticBoxSpec(
                name="VariableContextApi",
                methods=[
                    ApiMethodSpec(
                        signature="snapshot(ctx): OrderedMapBox",
                        operations=[{"kind": "CloneOwnedMap", "field": "variable_map"}],
                    )
                ],
            ),
            StaticBoxSpec(
                name="CarrierInfoApi",
                methods=carrier_api_methods,
            )
        ],
        recipe_subject="hakorune_mir_builder::variable_context::CarrierInfo.with_explicit_carriers",
        selected_body_count="explicit_carrier_snapshot_methods_only",
        methods=[
            BehaviorMethodSpec(
                id="CarrierInfo::with_explicit_carriers",
                rust_operation="ExplicitCarrierSnapshotFromOwnedMap",
                hako_operation="CarrierInfoBox.with_explicit_carriers_from_snapshot",
                emits="CarrierInfoApi.with_explicit_carriers_from_snapshot(loop_var_id, requested_names, snapshot, carrier_names, carrier_host_ids)",
            )
        ],
        excluded_methods=excluded,
        claims={
            "generated_hako_manual_edit": 0,
            "mainline_selected": 0,
            "full_variable_context_claim": 0,
            "rust_bootstrap_retained": 1,
            "backend_behavior_changed": 0,
            "source_selfhost_claim": 0,
        },
        verifier_checks={
            "rust_facts_input": "verified",
            "hako_lifecycle_plan": "verified",
            "hako_behavior_recipe": "verified",
            "selected_body_count": "explicit_carrier_snapshot_methods_only",
            "full_variable_context_claim": 0,
            "carrier_behavior_generated": 1,
            "requested_names_owned": 1,
            "missing_carrier_fail_fast": 1,
            "unmapped_thir_nodes": 0,
            "unmapped_mir_side_effects": 0,
            "unresolved_call_targets": 0,
            "unclassified_drop_obligations": 0,
            "mainline_selected": 0,
            "rust_bootstrap_retained": 1,
            "backend_behavior_changed": 0,
        },
        verified_operations=[
            "ExplicitCarrierSnapshotFromOwnedMap",
            "CloneOwnedMap",
            "OrderedMapBox.key_at",
            "OrderedMapBox.length",
            "ArrayBox.push",
            "ArrayBox.get",
        ],
        transport_notes={
            "loop_var_id_transport": "i64",
            "requested_names_transport": "ArrayBox",
            "carrier_names_transport": "ArrayBox",
            "carrier_host_ids_transport": "ArrayBox",
        },
        denied_boundaries=excluded,
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_variable_context_explicit_carrier_snapshot_artifact_generator(*, check: bool) -> None:
    facts = extract_variable_context_explicit_carrier_snapshot_facts(EXPLICIT_CARRIER_SNAPSHOT_SOURCE)
    plan = read_json(FIXTURES / "variable-context-explicit-carrier-snapshot-plan-v0.json")
    oracle = read_json(FIXTURES / "variable-context-explicit-carrier-snapshot-oracle-vectors-v0.json")
    _write_validated_family_artifact(
        check=check,
        unchanged_label="generated_variable_context_explicit_carrier_snapshot_artifact=unchanged",
        facts=facts,
        plan=plan,
        oracle=oracle,
        validate=validate_variable_context_explicit_carrier_snapshot,
        spec_factory=lambda: explicit_carrier_snapshot_spec(_api_methods_from_compiled(compile_explicit_carrier_snapshot_methods(facts, plan))),
    )
