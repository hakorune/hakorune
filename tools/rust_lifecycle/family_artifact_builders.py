#!/usr/bin/env python3
"""Builders for bounded Rust-derived Hako family artifacts."""

from __future__ import annotations

from typing import Any

from family_artifact_spec import FamilyArtifactSpec
from shared_family_generator import (
    build_common_rust_derived_inputs,
    build_common_rust_derived_manifest,
    build_derived_artifact_verifier_result,
    build_hako_behavior_recipe,
    RenderedTextArtifact,
    stable_json,
)
from shared_mirbuilder_emitter import emit_verified_family_hako


def _build_api_method_ir(method: Any) -> dict[str, Any]:
    data: dict[str, Any] = {"signature": method.signature}
    if method.operations is None:
        raise ValueError(f"method has no operations: {method.signature}")
    data["operations"] = method.operations
    return data


def _build_box_ir(box: Any) -> dict[str, Any]:
    data: dict[str, Any] = {"name": box.name}
    if not box.fields and box.field_name is None and box.field_type is None:
        data["fields"] = []
        return data
    if box.fields:
        data["fields"] = [
            {
                "name": field.name,
                "field_type": field.field_type,
                **({"initializer": field.initializer} if field.initializer is not None else {}),
                **(
                    {"initializer_operation": field.initializer_operation}
                    if field.initializer_operation is not None
                    else {}
                ),
            }
            for field in box.fields
        ]
        return data
    data["field_name"] = box.field_name
    data["field_type"] = box.field_type
    if box.initializer_operation is not None:
        data["initializer_operation"] = box.initializer_operation
    else:
        data["initializer"] = box.initializer
    return data


def _build_method_payloads(spec: FamilyArtifactSpec) -> list[dict[str, Any]]:
    return [
        {
            "id": method.id,
            "rust_operation": method.rust_operation,
            "hako_operation": method.hako_operation,
            "emits": method.emits,
        }
        for method in spec.methods
    ]


def _build_optional_json_text(data: dict[str, Any] | None) -> str | None:
    return None if data is None else stable_json(data)


def _build_family_artifact_recipe_object(spec: FamilyArtifactSpec) -> dict[str, Any] | None:
    if not spec.methods:
        return None
    if spec.recipe_subject is None or spec.selected_body_count is None:
        raise ValueError("recipe fields are required when methods are present")
    return build_hako_behavior_recipe(
        family_id=spec.family_id,
        pilot_scope=spec.pilot_scope,
        subject=spec.recipe_subject,
        source_plan=spec.plan_path.name,
        source_oracle=spec.oracle_path.name,
        selected_body_count=spec.selected_body_count,
        methods=_build_method_payloads(spec),
        excluded_methods=spec.excluded_methods or None,
    )


def build_family_artifact_recipe_text(spec: FamilyArtifactSpec) -> str | None:
    return _build_optional_json_text(_build_family_artifact_recipe_object(spec))


def _build_family_artifact_verifier_object(spec: FamilyArtifactSpec) -> dict[str, Any] | None:
    if not spec.methods:
        return None
    if spec.recipe_subject is None or spec.recipe_path is None:
        raise ValueError("verifier fields are required when methods are present")
    return build_derived_artifact_verifier_result(
        family_id=spec.family_id,
        pilot_scope=spec.pilot_scope,
        subject=spec.recipe_subject,
        source_facts=spec.facts_path.name,
        source_plan=spec.plan_path.name,
        source_oracle=spec.oracle_path.name,
        source_recipe=spec.recipe_path.name,
        checks=spec.verifier_checks,
        verified_operations=spec.verified_operations
        if spec.verified_operations is not None
        else [method.hako_operation for method in spec.methods],
        transport_notes=spec.transport_notes,
        denied_boundaries=spec.denied_boundaries,
    )


def build_family_artifact_verifier_text(spec: FamilyArtifactSpec) -> str | None:
    return _build_optional_json_text(_build_family_artifact_verifier_object(spec))


def _build_family_artifact_hako_object(spec: FamilyArtifactSpec) -> dict[str, Any]:
    if not spec.main_operations:
        raise ValueError("FamilyArtifactSpec.main_operations is required")
    verified_ir = {
        "generated_by": spec.generated_by,
        "artifact_manifest": spec.artifact_manifest,
        "family_comment": spec.family_comment,
        "pilot_scope": spec.pilot_scope,
        "using_module": spec.using_module,
        "extra_using_modules": spec.extra_using_modules,
        "enum_declarations": spec.enum_declarations,
        "box": _build_box_ir(spec.box),
        "additional_boxes": [_build_box_ir(box) for box in spec.additional_boxes],
        "main": {"operations": [operation.to_json() for operation in spec.main_operations]},
    }
    if spec.api_name is not None:
        verified_ir["api"] = {
            "name": spec.api_name,
            "trailing_blank_line": spec.api_trailing_blank_line,
            "methods": [_build_api_method_ir(method) for method in spec.api_methods],
        }
    if spec.static_boxes:
        verified_ir["static_boxes"] = [
            {
                "name": static_box.name,
                "methods": [_build_api_method_ir(method) for method in static_box.methods],
                "trailing_blank_line": static_box.trailing_blank_line,
            }
            for static_box in spec.static_boxes
        ]
    return verified_ir


def build_family_artifact_hako_text(spec: FamilyArtifactSpec) -> str:
    return emit_verified_family_hako(_build_family_artifact_hako_object(spec))


def _build_family_artifact_manifest_object(
    spec: FamilyArtifactSpec,
    *,
    hako_text: str,
    recipe_text: str | None = None,
    verifier_text: str | None = None,
) -> dict[str, Any]:
    if recipe_text is not None and spec.recipe_path is None:
        raise ValueError("recipe_path is required when recipe_text is provided")
    if verifier_text is not None and spec.verifier_path is None:
        raise ValueError("verifier_path is required when verifier_text is provided")
    inputs = build_common_rust_derived_inputs(
        root=spec.root,
        facts=spec.facts_path,
        plan=spec.plan_path,
        oracle=spec.oracle_path,
        recipe=RenderedTextArtifact(path=spec.recipe_path, text=recipe_text)
        if recipe_text is not None and spec.recipe_path is not None
        else None,
        verifier=RenderedTextArtifact(path=spec.verifier_path, text=verifier_text)
        if verifier_text is not None and spec.verifier_path is not None
        else None,
    )
    return build_common_rust_derived_manifest(
        root=spec.root,
        family_id=spec.family_id,
        state=spec.state,
        source_rust_file=spec.source_rust_file,
        generator_tool=spec.generated_by,
        generator_version=spec.generator_version,
        hako_path=spec.hako_path,
        hako_text=hako_text,
        claims=spec.claims,
        pilot_scope=spec.pilot_scope,
        inputs=inputs,
        extra_fields=spec.extra_manifest_fields,
    )


def build_family_artifact_manifest_text(
    spec: FamilyArtifactSpec,
    *,
    hako_text: str,
    recipe_text: str | None = None,
    verifier_text: str | None = None,
) -> str:
    return stable_json(
        _build_family_artifact_manifest_object(
            spec,
            hako_text=hako_text,
            recipe_text=recipe_text,
            verifier_text=verifier_text,
        )
    )
