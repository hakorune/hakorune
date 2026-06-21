#!/usr/bin/env python3
"""Data-only specs for bounded Rust-derived Hako family generators."""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class BoxSpec:
    name: str
    field_name: str
    field_type: str
    initializer: str | None = None
    initializer_operation: dict[str, Any] | None = None


@dataclass(frozen=True)
class ApiMethodSpec:
    signature: str
    body_lines: list[str] | None = None
    operations: list[dict[str, Any]] | None = None


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
    selected_body_count: str | None = None
    methods: list[BehaviorMethodSpec] = field(default_factory=list)
    excluded_methods: list[str] = field(default_factory=list)
    claims: dict[str, Any] = field(default_factory=dict)
    verifier_checks: dict[str, Any] = field(default_factory=dict)
    verified_operations: list[str] | None = None
    transport_notes: dict[str, Any] | None = None
    denied_boundaries: list[str] | None = None
    extra_manifest_fields: dict[str, Any] = field(default_factory=dict)
