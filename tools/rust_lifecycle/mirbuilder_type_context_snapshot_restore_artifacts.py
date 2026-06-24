#!/usr/bin/env python3
"""Spec and validation for TypeContext aggregate snapshot/restore conversion."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_type_context_snapshot_restore_facts import SNAPSHOT_FIELDS, SOURCE as TYPE_CONTEXT_SOURCE, extract_facts
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from shared_family_generator import read_json, run_validated_family_generator
from verified_hako_family_ir import HakoOperation, op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"


def _oracle_ops(oracle: dict[str, Any]) -> set[str]:
    return {operation["op"] for vector in oracle["vectors"] for operation in vector["operations"]}


def validate_type_context_snapshot_restore(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
) -> None:
    subject = "hakorune_mir_builder::type_context::TypeContext.snapshot_restore"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected TypeContext snapshot fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("TypeContext snapshot subject mismatch")
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    if body_facts["TypeContext::take_snapshot"].get("fields") != SNAPSHOT_FIELDS:
        raise SystemExit("TypeContext take_snapshot field coverage mismatch")
    if body_facts["TypeContext::restore_snapshot"].get("fields") != SNAPSHOT_FIELDS:
        raise SystemExit("TypeContext restore_snapshot field coverage mismatch")
    for field in facts["field_facts"]:
        if field.get("snapshot_transport") != "OpaqueOwnedMapStorage":
            raise SystemExit("TypeContext snapshot field must use opaque map storage")
        if field.get("entry_access_claim") != "none":
            raise SystemExit("TypeContext snapshot must not claim entry access")
    plans = {row["id"]: row for row in plan["plans"]}
    shape_plan = plans["TypeContext.TypeContextSnapshot"]
    if shape_plan.get("shape_rule") != "aggregate.take_restore_with_defaults":
        raise SystemExit("TypeContext snapshot must use aggregate.take_restore_with_defaults")
    if shape_plan.get("opaque_container_move") is not True:
        raise SystemExit("TypeContext snapshot must require opaque container move")
    for op_name in ["take_snapshot", "restore_snapshot", "double_restore_fail_fast"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing TypeContext snapshot oracle op: {op_name}")


def _map_fields() -> list[FieldSpec]:
    return [FieldSpec(name=field, field_type="MapBox", initializer_operation={"kind": "NewMap"}) for field in SNAPSHOT_FIELDS]


def _snapshot_box() -> BoxSpec:
    return BoxSpec(
        name="TypeContextSnapshot",
        fields=[*_map_fields(), FieldSpec(name="consumed", field_type="i64", initializer="0")],
    )


def _seed_ops() -> list[HakoOperation]:
    ops: list[HakoOperation] = []
    for index, field in enumerate(SNAPSHOT_FIELDS, start=1):
        ops.append(op("StaticCall", callee=f"TypeContextHarnessApi.seed_{field}", args=["ctx"]))
    return ops


def _assert_missing_after_take_ops() -> list[HakoOperation]:
    ops: list[HakoOperation] = []
    for index, field in enumerate(SNAPSHOT_FIELDS, start=1):
        target = f"{field}_ctx_has_after_take"
        ops.append(op("StaticCall", target=target, callee=f"TypeContextHarnessApi.has_{field}", args=["ctx"]))
        ops.append(op("AssertEq", left=target, right=0, fail_message=f"type_context_snapshot_ctx_reset_{field}=fail", fail_code=index))
    return ops


def _assert_restored_ops() -> list[HakoOperation]:
    ops: list[HakoOperation] = []
    for index, field in enumerate(SNAPSHOT_FIELDS, start=1):
        target = f"{field}_restored_has"
        ops.append(op("StaticCall", target=target, callee=f"TypeContextHarnessApi.has_{field}", args=["ctx"]))
        ops.append(op("AssertEq", left=target, right=1, fail_message=f"type_context_snapshot_restore_{field}=fail", fail_code=20 + index))
    return ops


def _assert_snapshot_moved_out_ops() -> list[HakoOperation]:
    ops: list[HakoOperation] = []
    for index, field in enumerate(SNAPSHOT_FIELDS, start=1):
        target = f"{field}_snapshot_has_after_restore"
        ops.append(op("StaticCall", target=target, callee=f"TypeContextHarnessApi.snapshot_has_{field}", args=["snapshot"]))
        ops.append(op("AssertEq", left=target, right=0, fail_message=f"type_context_snapshot_moved_out_{field}=fail", fail_code=40 + index))
    return ops


def _harness_methods() -> list[ApiMethodSpec]:
    methods: list[ApiMethodSpec] = []
    for index, field in enumerate(SNAPSHOT_FIELDS, start=1):
        methods.extend(
            [
                ApiMethodSpec(
                    signature=f"seed_{field}(ctx): i64",
                    operations=[op("MapSet", field=field, key=str(index), value=str(index * 10), storage="MapBox").to_json()],
                ),
                ApiMethodSpec(
                    signature=f"has_{field}(ctx): i64",
                    operations=[op("MapBoxHas", field=field, key=str(index)).to_json()],
                ),
                ApiMethodSpec(
                    signature=f"snapshot_has_{field}(snapshot: TypeContextSnapshot): i64",
                    operations=[op("MapBoxHas", source=f"snapshot.{field}", key=str(index)).to_json()],
                ),
            ]
        )
    methods.append(
        ApiMethodSpec(
            signature="set_replacement(ctx): i64",
            operations=[op("MapSet", field="value_types", key="99", value="99", storage="MapBox").to_json()],
        )
    )
    return methods


def type_context_snapshot_restore_spec() -> FamilyArtifactSpec:
    facts = extract_facts(TYPE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "type-context-snapshot-restore-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("aggregate.take_restore_with_defaults", facts, plan)
    ]
    excluded = [
        "TypeContext::get_type",
        "TypeContext::set_type",
        "TypeContext::try_get_kind",
        "TypeContext::get_kind",
        "TypeContext::set_kind",
        "TypeContext::get_origin_box",
        "TypeContext::set_origin_box",
        "TypeContext::clear_origin_boxes",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family type_context_snapshot_restore",
        generator_version="type-context-snapshot-restore-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/type_context_snapshot_restore.artifact.json",
        family_comment="hakorune_mir_builder::type_context::TypeContext.snapshot_restore",
        using_module="",
        extra_using_modules=[],
        box=BoxSpec(name="TypeContext", fields=_map_fields()),
        additional_boxes=[_snapshot_box()],
        main_operations=[
            op("NewBox", target="ctx", box="TypeContext"),
            *_seed_ops(),
            op("StaticCall", target="snapshot", callee="TypeContextApi.take_snapshot", args=["ctx"]),
            *_assert_missing_after_take_ops(),
            op("StaticCall", callee="TypeContextHarnessApi.set_replacement", args=["ctx"]),
            op("StaticCall", target="snapshot_value_types_has", callee="TypeContextHarnessApi.snapshot_has_value_types", args=["snapshot"]),
            op("AssertEq", left="snapshot_value_types_has", right=1, fail_message="type_context_snapshot_alias=fail", fail_code=19),
            op("StaticCall", target="restore_status", callee="TypeContextApi.restore_snapshot", args=["ctx", "snapshot"]),
            op("AssertEq", left="restore_status", right=0, fail_message="type_context_snapshot_restore_status=fail", fail_code=20),
            *_assert_restored_ops(),
            *_assert_snapshot_moved_out_ops(),
            op("AssertEq", left="snapshot.consumed", right=1, fail_message="type_context_snapshot_consumed=fail", fail_code=60),
            op("StaticCall", target="second_restore_status", callee="TypeContextApi.restore_snapshot", args=["ctx", "snapshot"]),
            op("AssertEq", left="second_restore_status", right=7, fail_message="type_context_snapshot_double_restore=fail", fail_code=61),
            op("Print", text="type_context_snapshot_restore_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::type_context",
        state="DerivedShadow",
        source_rust_file=TYPE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "type_context_snapshot_restore.hako",
        facts_path=FIXTURES / "type-context-snapshot-restore-facts-v0.json",
        plan_path=FIXTURES / "type-context-snapshot-restore-plan-v0.json",
        oracle_path=FIXTURES / "type-context-snapshot-restore-oracle-v0.json",
        recipe_path=FIXTURES / "type-context-snapshot-restore-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "type-context-snapshot-restore-derived-artifact-verifier-result-v0.json",
        pilot_scope="TypeContext_snapshot_restore_only",
        recipe_subject="hakorune_mir_builder::type_context::TypeContext.snapshot_restore",
        selected_body_count="snapshot_restore_methods_only",
        static_boxes=[
            StaticBoxSpec(name="TypeContextApi", methods=api_methods),
            StaticBoxSpec(name="TypeContextHarnessApi", methods=_harness_methods()),
        ],
        methods=[
            BehaviorMethodSpec(id="TypeContext::take_snapshot", rust_operation="std::mem::take six fields", hako_operation="MoveFieldAndResetSource", emits="TypeContextApi.take_snapshot(ctx)"),
            BehaviorMethodSpec(id="TypeContext::restore_snapshot", rust_operation="owned snapshot field restore", hako_operation="MoveFieldAndResetSource", emits="TypeContextApi.restore_snapshot(ctx, snapshot)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_type_context_claim": 0, "mirbuilder_wide_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "aggregate.take_restore_with_defaults", "selected_body_count": "snapshot_restore_methods_only", "opaque_container_move": 1, "entry_access_claim": 0, "composite_key_transport_claim": 0, "btree_iteration_parity_claim": 0, "hash_iteration_parity_claim": 0, "unresolved_call_targets": 0, "unmapped_mir_side_effects": 0},
        verified_operations=["MoveFieldAndResetSource", "AssertNotConsumed", "MarkConsumed", "MapBoxHas"],
        transport_notes={"field_transport": "OpaqueOwnedMapStorage", "map_literal_value_types_key_transport": "not_claimed", "container_cloned": False},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_type_context_snapshot_restore_artifact_generator(*, check: bool) -> None:
    spec = type_context_snapshot_restore_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(spec, hako_text=hako_text, recipe_text=recipe_text, verifier_text=verifier_text)
    outputs = [
        (spec.recipe_path, recipe_text),
        (spec.verifier_path, verifier_text),
        (spec.hako_path, hako_text),
        (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text),
    ]
    run_validated_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_type_context_snapshot_restore_artifact=unchanged",
        load_facts=lambda: extract_facts(TYPE_CONTEXT_SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_type_context_snapshot_restore,
        outputs_factory=lambda: outputs,
    )
