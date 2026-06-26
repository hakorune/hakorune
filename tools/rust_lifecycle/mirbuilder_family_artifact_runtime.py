#!/usr/bin/env python3
"""Runtime wiring for spec-backed MirBuilder family artifact generators."""

from __future__ import annotations

from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import FamilyArtifactSpec
from shared_family_generator import run_validated_family_generator


@dataclass(frozen=True)
class FamilyArtifactGenerator:
    spec_factory: Callable[[], FamilyArtifactSpec]
    validator: Callable[[dict[str, Any], dict[str, Any], dict[str, Any]], None]
    facts_loader: Callable[[FamilyArtifactSpec], dict[str, Any]]
    unchanged_label: str


def run_family_artifact_generator_from_registry(
    *,
    name: str,
    check: bool,
    generators: dict[str, FamilyArtifactGenerator],
) -> None:
    try:
        generator = generators[name]
    except KeyError as exc:
        raise SystemExit(f"unknown MirBuilder family artifact generator: {name}") from exc

    spec = generator.spec_factory()
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
    outputs.extend(
        [
            (spec.hako_path, hako_text),
            (spec.root / spec.artifact_manifest, manifest_text),
        ]
    )
    run_validated_family_generator(
        check=check,
        root=spec.root,
        unchanged_label=generator.unchanged_label,
        load_facts=lambda: generator.facts_loader(spec),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=generator.validator,
        outputs_factory=lambda: outputs,
    )
