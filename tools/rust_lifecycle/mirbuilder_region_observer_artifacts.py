#!/usr/bin/env python3
"""Spec and validation for RegionObserver variable-map read-fold artifact."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_region_observer_variable_map_facts import extract_facts
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_ordered_read_fold_converter import compile_ordered_read_fold
from mirbuilder_ordering_capability import RUST_STRING_ORD_V1
from shared_family_generator import read_json, run_validated_family_generator
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
SOURCE = ROOT / "src/mir/region/observer.rs"


PLAN = {
    "borrow_use_id": "RegionObserver::classify_slots_from_variable_map",
    "source": "builder.variable_ctx.variable_map",
    "destination": "slots",
    "comparator_capabilities": {
        RUST_STRING_ORD_V1: {
            "proof": "VmExeAotAccepted",
            "required_tiers": ["VM", "EXE", "AOT"],
        },
    },
    "output_capabilities": {
        "Vec<SlotMetadata>": {
            "proof": "Accepted",
            "semantic_shape": "OwnedSequence<OwnedProduct>",
            "sequence_transport": "ArrayBox",
            "element_transport": "SlotMetadataBox",
        }
    },
}


def validate_region_observer(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    fixture_facts = read_json(FIXTURES / "region-observer-variable-map-facts-v0.json")
    if facts != fixture_facts:
        raise SystemExit("RegionObserver live facts differ from fixture")
    if plan != read_json(FIXTURES / "region-observer-variable-map-plan-v0.json"):
        raise SystemExit("RegionObserver plan fixture mismatch")
    if oracle.get("subject") != "mir::region::observer::classify_slots_from_variable_map":
        raise SystemExit("RegionObserver oracle subject mismatch")
    compiled = compile_ordered_read_fold(facts, plan)
    if compiled != [
        {
            "kind": "ReadFoldOwnedOutput",
            "source": "builder.variable_ctx.variable_map",
            "destination": "slots",
            "order": facts["borrow_use_facts"][0]["order"],
        }
    ]:
        raise SystemExit("RegionObserver ordered read-fold compile mismatch")


def region_observer_spec() -> FamilyArtifactSpec:
    facts = extract_facts()
    plan = read_json(FIXTURES / "region-observer-variable-map-plan-v0.json")
    ref_kind_groups = [
        {
            "variants": [
                {"name": "Box", "payload_var": "_box_payload"},
                {"name": "Array", "payload_var": "_array_payload"},
                {"name": "Future", "payload_var": "_future_payload"},
            ],
            "returns": "RefSlotKind::StrongRoot()",
        },
        {"variants": ["WeakRef"], "returns": "RefSlotKind::WeakRoot()"},
    ]
    missing_value_fallback = {
        "input": "name",
        "string_set": ["args", "src", "body_src", "bundles", "bundle_names", "bundle_srcs", "require_mods"],
        "matched": "RefSlotKind::StrongRoot()",
        "unmatched": "RefSlotKind::NonRef()",
    }
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_region_observer_slot_metadata_artifact.py",
        generator_version="region-observer-slot-metadata-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/region_observer_slot_metadata.artifact.json",
        family_comment="mir::region::observer::classify_slots_from_variable_map",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="RegionObserverArtifactProbe", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="SlotMetadataBox",
                fields=[
                    FieldSpec(name="name", field_type="StringBox", initializer="null"),
                    FieldSpec(name="ref_kind", field_type="RefSlotKind", initializer="RefSlotKind::NonRef()"),
                ],
            )
        ],
        enum_declarations=[
            {
                "name": "MirType",
                "variants": [
                    "Integer",
                    "Float",
                    "Bool",
                    "String",
                    {"name": "Box", "payload": "StringBox"},
                    {"name": "Array", "payload": "MirType"},
                    {"name": "Future", "payload": "MirType"},
                    "WeakRef",
                    "Void",
                    "Unknown",
                ],
            },
            {"name": "RefSlotKind", "variants": ["StrongRoot", "WeakRoot", "Borrowed", "NonRef"]},
        ],
        static_boxes=[
            StaticBoxSpec(
                name="SlotClassifierApi",
                methods=[
                    ApiMethodSpec(
                        signature="classify(type_opt: Option<MirType>, name: StringBox): RefSlotKind",
                        operations=[
                            op(
                                "ClassifyEnumVariants",
                                type_source="type_opt",
                                source_enum="MirType",
                                variant_groups=ref_kind_groups,
                                default_return="RefSlotKind::NonRef()",
                                missing_value_fallback=missing_value_fallback,
                            ).to_json()
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="RegionObserverApi",
                methods=[
                    ApiMethodSpec(
                        signature="classify_slots_from_variable_map(variable_ctx: OrderedMapBox, type_ctx: MapBox): ArrayBox",
                        operations=[
                            op("NewLocalArray", target="slots").to_json(),
                            op(
                                "ReadFoldSlotMetadata",
                                source="variable_ctx",
                                type_map="type_ctx",
                                destination="slots",
                                classifier="SlotClassifierApi.classify",
                                oracle_slots=[
                                    {"name": "a", "ref_kind": "RefSlotKind::StrongRoot()", "fail_code": 6},
                                    {"name": "args", "ref_kind": "RefSlotKind::StrongRoot()", "fail_code": 8},
                                    {"name": "b", "ref_kind": "RefSlotKind::WeakRoot()", "fail_code": 10},
                                ],
                            ).to_json(),
                            op("ReturnSource", source="slots").to_json(),
                        ],
                    )
                ],
            ),
        ],
        main_operations=[
            op("StaticCall", target="variable_ctx", callee="OrderedMap.create"),
            op("NewBox", target="type_ctx", box="MapBox"),
            op("MethodCall", receiver="variable_ctx", method="set", args=[{"literal": "b"}, 2]),
            op("MethodCall", receiver="variable_ctx", method="set", args=[{"literal": "a"}, 1]),
            op("MethodCall", receiver="variable_ctx", method="set", args=[{"literal": "args"}, 3]),
            op("MethodCall", receiver="type_ctx", method="set", args=[1, {"expr": 'MirType::Box("CompletelyDifferentBox")'}]),
            op("MethodCall", receiver="type_ctx", method="set", args=[2, {"expr": "MirType::WeakRef()"}]),
            op("StaticCall", target="box_a", callee="SlotClassifierApi.classify", args=[{"expr": 'Option::Some(MirType::Box("A"))'}, {"literal": "x"}]),
            op("AssertEq", left="box_a", right={"expr": "RefSlotKind::StrongRoot()"}, fail_message="slot_classifier_box_a=fail", fail_code=1),
            op("StaticCall", target="box_b", callee="SlotClassifierApi.classify", args=[{"expr": 'Option::Some(MirType::Box("B"))'}, {"literal": "x"}]),
            op("AssertEq", left="box_b", right={"expr": "RefSlotKind::StrongRoot()"}, fail_message="slot_classifier_box_b=fail", fail_code=2),
            op("StaticCall", target="unknown_args", callee="SlotClassifierApi.classify", args=[{"expr": "Option::Some(MirType::Unknown())"}, {"literal": "args"}]),
            op("AssertEq", left="unknown_args", right={"expr": "RefSlotKind::NonRef()"}, fail_message="slot_classifier_unknown_args=fail", fail_code=3),
            op("StaticCall", target="none_args", callee="SlotClassifierApi.classify", args=[{"expr": "Option::None()"}, {"literal": "args"}]),
            op("AssertEq", left="none_args", right={"expr": "RefSlotKind::StrongRoot()"}, fail_message="slot_classifier_none_args=fail", fail_code=4),
            op("StaticCall", target="slots", callee="RegionObserverApi.classify_slots_from_variable_map", args=["variable_ctx", "type_ctx"]),
            op("MethodCall", target="slot_count", receiver="slots", method="length"),
            op("AssertEq", left="slot_count", right=3, fail_message="region_observer_slot_count=fail", fail_code=5),
            op("MethodCall", receiver="variable_ctx", method="set", args=[{"literal": "a"}, 99]),
            op("MethodCall", target="slot_count_after_mutation", receiver="slots", method="length"),
            op("AssertEq", left="slot_count_after_mutation", right=3, fail_message="region_observer_output_alias=fail", fail_code=12),
            op("Print", text="region_observer_slot_metadata_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="mir::region::observer",
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "region_observer_slot_metadata.hako",
        facts_path=FIXTURES / "region-observer-variable-map-facts-v0.json",
        plan_path=FIXTURES / "region-observer-variable-map-plan-v0.json",
        oracle_path=FIXTURES / "region-observer-variable-map-oracle-v0.json",
        recipe_path=FIXTURES / "region-observer-variable-map-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "region-observer-variable-map-derived-artifact-verifier-result-v0.json",
        pilot_scope="RegionObserver_variable_map_slot_metadata_only",
        recipe_subject="mir::region::observer::classify_slots_from_variable_map",
        selected_body_count="region_observer_variable_map_read_fold_only",
        methods=[
            BehaviorMethodSpec(id="RegionObserver::classify_slots_from_variable_map", rust_operation="variable_map().iter()", hako_operation="ReadFoldOwnedOutput", emits="RegionObserverApi.classify_slots_from_variable_map(variable_ctx, type_ctx)"),
            BehaviorMethodSpec(id="Region::classify_ref_kind", rust_operation="match MirType", hako_operation="ClassifyEnumVariants", emits="SlotClassifierApi.classify(type_opt, name)"),
        ],
        excluded_methods=["classify_slots_from_registry", "observe_control_form", "observe_function_region", "pop_function_region"],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_region_observer_claim": 0, "runtime_fallback": 0, "rust_bootstrap_retained": 1},
        verifier_checks={"rust_facts_input": "verified", "borrow_lowering_decision": "ElideToReadFold", "order": facts["borrow_use_facts"][0]["order"], "output_transport": "ArrayBox<SlotMetadataBox>", "raw_aggregate_return": 0, "region_observer_backend_branch": 0, "mirtype_backend_branch": 0},
        verified_operations=["ReadFoldOwnedOutput", "ClassifyEnumVariants", "ConstructOwnedProduct"],
        transport_notes={"semantic_shape": "OwnedSequence<OwnedProduct>", "sequence_transport": "ArrayBox", "element_transport": "SlotMetadataBox"},
        denied_boundaries=["VariableContext::variable_map standalone returned borrow"],
        extra_manifest_fields={"route_plan": PLAN, "oracle_vectors": read_json(FIXTURES / "region-observer-variable-map-oracle-v0.json")["vectors"]},
    )


def run_region_observer_artifact_generator(*, check: bool) -> None:
    spec = region_observer_spec()
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
        unchanged_label="generated_region_observer_slot_metadata_artifact=unchanged",
        load_facts=extract_facts,
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_region_observer,
        outputs_factory=lambda: outputs,
    )
