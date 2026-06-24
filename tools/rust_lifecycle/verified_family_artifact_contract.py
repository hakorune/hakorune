#!/usr/bin/env python3
"""Verified family artifact contract projection helpers.

This module keeps semantic facts upstream of generated artifacts. Manifests and
verifier expectations may consume this contract, but they do not become the
semantic authority.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any


@dataclass(frozen=True)
class StableDeny:
    method_id: str
    reason: str
    detail: str | None = None


@dataclass(frozen=True)
class ArtifactIdentity:
    family_id: str
    api_name: str
    pilot_scope: str
    artifact_path: str
    manifest_path: str
    schema_version: int = 0


@dataclass(frozen=True)
class VerifiedFamilyArtifactContractV1:
    family_id: str
    method_universe: tuple[str, ...]
    selected_method_ids: tuple[str, ...]
    denials: tuple[StableDeny, ...]
    semantic_transports: dict[str, Any]
    artifact: ArtifactIdentity
    selected_body_count_label: str
    expected_fields: tuple[str, ...] = field(default_factory=tuple)

    def __post_init__(self) -> None:
        if self.family_id != self.artifact.family_id:
            raise ValueError("ArtifactIdentity family_id mismatch")
        selected = set(self.selected_method_ids)
        denied = {deny.method_id for deny in self.denials}
        universe = set(self.method_universe)
        if selected & denied:
            raise ValueError(f"selected/denied overlap: {sorted(selected & denied)}")
        if selected | denied != universe:
            missing = sorted(universe - (selected | denied))
            extra = sorted((selected | denied) - universe)
            raise ValueError(f"method partition mismatch: missing={missing} extra={extra}")
        if len(self.selected_method_ids) != len(selected):
            raise ValueError("duplicate selected method id")
        if len(self.semantic_transports) != len(set(self.semantic_transports)):
            raise ValueError("duplicate semantic transport id")

    @property
    def denied_method_ids(self) -> tuple[str, ...]:
        return tuple(deny.method_id for deny in self.denials)

    @property
    def selected_api_method_names(self) -> tuple[str, ...]:
        return tuple(method_id.split("::", 1)[1] for method_id in self.selected_method_ids)

    @property
    def selected_api_symbols(self) -> tuple[str, ...]:
        return tuple(f"{self.artifact.api_name}.{name}" for name in self.selected_api_method_names)

    def require_selected_recipe_methods(self, method_ids: list[str]) -> None:
        if tuple(method_ids) != self.selected_method_ids:
            raise ValueError(
                "selected recipe methods do not match contract: "
                f"expected={self.selected_method_ids} actual={tuple(method_ids)}"
            )

    def manifest_extra_fields(self) -> dict[str, Any]:
        return {"excluded_methods": list(self.denied_method_ids)}

    def verifier_checks(self, base: dict[str, Any]) -> dict[str, Any]:
        data = dict(base)
        data["selected_body_count"] = self.selected_body_count_label
        data["excluded_methods"] = list(self.denied_method_ids)
        return data

    def transport_notes(self, base: dict[str, Any]) -> dict[str, Any]:
        data = dict(base)
        data.update(self.semantic_transports)
        return data
