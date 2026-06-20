#!/usr/bin/env python3
"""Data-only specs for bounded Rust-derived Hako family generators."""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from shared_family_generator import (
    build_common_rust_derived_inputs,
    build_common_rust_derived_manifest,
    build_derived_artifact_verifier_result,
    build_hako_behavior_recipe,
)
from shared_mirbuilder_emitter import emit_verified_family_hako


@dataclass(frozen=True)
class BoxSpec:
    name: str
    field_name: str
    field_type: str
    initializer: str


@dataclass(frozen=True)
class ApiMethodSpec:
    signature: str
    body_lines: list[str]


@dataclass(frozen=True)
class StaticBoxSpec:
    name: str
    methods: list[ApiMethodSpec]
    trailing_blank_line: bool = False


@dataclass(frozen=True)
class BehaviorMethodSpec:
    id: str
    rust_operation: str
    hako_operation: str
    emits: str


@dataclass(frozen=True)
class FamilyArtifactSpec:
    root: Path
    generated_by: str
    generator_version: str
    artifact_manifest: str
    family_comment: str
    using_module: str
    box: BoxSpec
    main_lines: list[str]
    family_id: str
    state: str
    source_rust_file: Path
    hako_path: Path
    facts_path: Path
    plan_path: Path
    oracle_path: Path
    recipe_path: Path | None = None
    verifier_path: Path | None = None
    pilot_scope: str | None = None
    api_name: str | None = None
    api_methods: list[ApiMethodSpec] = field(default_factory=list)
    api_trailing_blank_line: bool = False
    static_boxes: list[StaticBoxSpec] = field(default_factory=list)
    recipe_subject: str | None = None
    source_plan: str | None = None
    source_oracle: str | None = None
    selected_body_count: str | None = None
    methods: list[BehaviorMethodSpec] = field(default_factory=list)
    excluded_methods: list[str] = field(default_factory=list)
    claims: dict[str, Any] = field(default_factory=dict)
    verifier_checks: dict[str, Any] = field(default_factory=dict)
    verified_operations: list[str] | None = None
    transport_notes: dict[str, Any] | None = None
    denied_boundaries: list[str] | None = None
    extra_manifest_fields: dict[str, Any] = field(default_factory=dict)


def build_family_artifact_recipe(spec: FamilyArtifactSpec) -> dict[str, Any] | None:
    if not spec.methods:
        return None
    if spec.recipe_subject is None or spec.source_plan is None or spec.source_oracle is None or spec.selected_body_count is None:
        raise ValueError("recipe fields are required when methods are present")
    return build_hako_behavior_recipe(
        family_id=spec.family_id,
        pilot_scope=spec.pilot_scope,
        subject=spec.recipe_subject,
        source_plan=spec.source_plan,
        source_oracle=spec.source_oracle,
        selected_body_count=spec.selected_body_count,
        methods=[
            {
                "id": method.id,
                "rust_operation": method.rust_operation,
                "hako_operation": method.hako_operation,
                "emits": method.emits,
            }
            for method in spec.methods
        ],
        excluded_methods=spec.excluded_methods or None,
    )


def build_family_artifact_verifier(spec: FamilyArtifactSpec, recipe: dict[str, Any] | None) -> dict[str, Any] | None:
    if not spec.methods:
        return None
    if recipe is None:
        raise ValueError("recipe is required when methods are present")
    if spec.recipe_subject is None or spec.source_plan is None or spec.source_oracle is None or spec.recipe_path is None:
        raise ValueError("verifier fields are required when methods are present")
    return build_derived_artifact_verifier_result(
        family_id=spec.family_id,
        pilot_scope=spec.pilot_scope,
        subject=spec.recipe_subject,
        source_facts=spec.facts_path.name,
        source_plan=spec.plan_path.name,
        source_oracle=spec.oracle_path.name,
        source_recipe=spec.recipe_path.name if spec.recipe_path is not None else "",
        checks=spec.verifier_checks,
        verified_operations=spec.verified_operations
        if spec.verified_operations is not None
        else [method.hako_operation for method in spec.methods],
        transport_notes=spec.transport_notes,
        denied_boundaries=spec.denied_boundaries,
    )


def build_family_artifact_hako(spec: FamilyArtifactSpec) -> str:
    verified_ir: dict[str, Any] = {
        "generated_by": spec.generated_by,
        "artifact_manifest": spec.artifact_manifest,
        "family_comment": spec.family_comment,
        "pilot_scope": spec.pilot_scope,
        "using_module": spec.using_module,
        "box": {
            "name": spec.box.name,
            "field_name": spec.box.field_name,
            "field_type": spec.box.field_type,
            "initializer": spec.box.initializer,
        },
        "main": {"lines": spec.main_lines},
    }
    if spec.api_name is not None:
        verified_ir["api"] = {
            "name": spec.api_name,
            "trailing_blank_line": spec.api_trailing_blank_line,
            "methods": [
                {"signature": method.signature, "body_lines": method.body_lines}
                for method in spec.api_methods
            ],
        }
    if spec.static_boxes:
        verified_ir["static_boxes"] = [
            {
                "name": static_box.name,
                "methods": [
                    {"signature": method.signature, "body_lines": method.body_lines}
                    for method in static_box.methods
                ],
                "trailing_blank_line": static_box.trailing_blank_line,
            }
            for static_box in spec.static_boxes
        ]
    return emit_verified_family_hako(verified_ir)


def build_family_artifact_manifest(
    spec: FamilyArtifactSpec,
    *,
    hako_text: str,
    recipe_text: str | None = None,
    verifier_text: str | None = None,
) -> dict[str, Any]:
    inputs = build_common_rust_derived_inputs(
        root=spec.root,
        facts=spec.facts_path,
        plan=spec.plan_path,
        oracle=spec.oracle_path,
        recipe=(spec.recipe_path, recipe_text) if recipe_text is not None and spec.recipe_path is not None else None,
        verifier=(spec.verifier_path, verifier_text) if verifier_text is not None and spec.verifier_path is not None else None,
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
