#!/usr/bin/env python3
"""Shared helpers for bounded Rust-derived Hako family generators."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
from collections.abc import Callable, Iterable
from typing import Any


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def stable_json(data: dict[str, Any]) -> str:
    return json.dumps(data, indent=2, sort_keys=True) + "\n"


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_if_changed(path: Path, text: str) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    old = path.read_text() if path.exists() else None
    if old == text:
        return False
    path.write_text(text)
    return True


def write_outputs(outputs: Iterable[tuple[Path, str]], *, check: bool, unchanged_label: str, root: Path) -> None:
    changed: list[str] = []
    for path, text in outputs:
        if check:
            if not path.exists() or path.read_text() != text:
                changed.append(str(path.relative_to(root)))
        elif write_if_changed(path, text):
            changed.append(str(path.relative_to(root)))

    if changed:
        if check:
            raise SystemExit("generated files differ: " + ", ".join(changed))
        print("updated=" + ",".join(changed))
        return

    print(unchanged_label)


def run_family_generator(
    *,
    check: bool,
    root: Path,
    unchanged_label: str,
    outputs_factory: Callable[[], Iterable[tuple[Path, str]]],
) -> None:
    write_outputs(
        outputs_factory(),
        check=check,
        unchanged_label=unchanged_label,
        root=root,
    )


def build_rust_derived_hako_manifest(
    *,
    family_id: str,
    state: str,
    source_rust_files: list[dict[str, Any]],
    generator_tool: str,
    generator_version: str,
    hako_path: str,
    hako_sha256: str,
    claims: dict[str, Any],
    pilot_scope: str | None = None,
    inputs: dict[str, Any] | None = None,
    extra_fields: dict[str, Any] | None = None,
) -> dict[str, Any]:
    manifest: dict[str, Any] = {
        "schema_version": 0,
        "kind": "RustDerivedHakoArtifact",
        "family_id": family_id,
        "state": state,
        "source": {"rust_files": source_rust_files},
        "generator": {
            "tool": generator_tool,
            "version": generator_version,
        },
        "output": {
            "hako_path": hako_path,
            "hako_sha256": hako_sha256,
        },
        "claims": claims,
    }
    if pilot_scope is not None:
        manifest["pilot_scope"] = pilot_scope
    if inputs is not None:
        manifest["inputs"] = inputs
    if extra_fields is not None:
        manifest.update(extra_fields)
    return manifest


def build_hako_behavior_recipe(
    *,
    family_id: str,
    subject: str,
    source_plan: str,
    source_oracle: str,
    selected_body_count: str,
    methods: list[dict[str, Any]],
    pilot_scope: str | None = None,
    excluded_methods: list[str] | None = None,
) -> dict[str, Any]:
    recipe: dict[str, Any] = {
        "schema_version": 0,
        "kind": "HakoBehaviorRecipe",
        "family_id": family_id,
        "subject": subject,
        "source_plan": source_plan,
        "source_oracle": source_oracle,
        "selected_body_count": selected_body_count,
        "methods": methods,
    }
    if pilot_scope is not None:
        recipe["pilot_scope"] = pilot_scope
    if excluded_methods is not None:
        recipe["excluded_methods"] = excluded_methods
    return recipe


def build_derived_artifact_verifier_result(
    *,
    family_id: str,
    subject: str,
    source_facts: str,
    source_plan: str,
    source_oracle: str,
    source_recipe: str,
    checks: dict[str, Any],
    result: str = "VerifiedHakoFamilyIR",
    pilot_scope: str | None = None,
    verified_operations: list[str] | None = None,
    transport_notes: dict[str, Any] | None = None,
    denied_boundaries: list[str] | None = None,
) -> dict[str, Any]:
    verifier: dict[str, Any] = {
        "schema_version": 0,
        "kind": "DerivedHakoArtifactVerifierResult",
        "family_id": family_id,
        "subject": subject,
        "result": result,
        "source_facts": source_facts,
        "source_plan": source_plan,
        "source_recipe": source_recipe,
        "checks": checks,
    }
    if pilot_scope is not None:
        verifier["pilot_scope"] = pilot_scope
    if verified_operations is not None:
        verifier["verified_operations"] = verified_operations
    if transport_notes is not None:
        verifier["transport_notes"] = transport_notes
    if denied_boundaries is not None:
        verifier["denied_boundaries"] = denied_boundaries
    return verifier


def rust_manifest_file_entry(*, path: Path, root: Path) -> dict[str, Any]:
    return {"path": str(path.relative_to(root)), "sha256": sha256_file(path)}


def rust_manifest_text_entry(*, path: Path, text: str, root: Path) -> dict[str, Any]:
    return {"path": str(path.relative_to(root)), "sha256": sha256_text(text)}


def build_common_rust_derived_inputs(
    *,
    root: Path,
    facts: Path,
    plan: Path,
    oracle: Path,
    recipe: tuple[Path, str] | None = None,
    verifier: tuple[Path, str] | None = None,
) -> dict[str, Any]:
    inputs: dict[str, Any] = {
        "facts": rust_manifest_file_entry(path=facts, root=root),
        "plan": rust_manifest_file_entry(path=plan, root=root),
        "oracle": rust_manifest_file_entry(path=oracle, root=root),
    }
    if recipe is not None:
        inputs["recipe"] = rust_manifest_text_entry(path=recipe[0], text=recipe[1], root=root)
    if verifier is not None:
        inputs["verifier"] = rust_manifest_text_entry(path=verifier[0], text=verifier[1], root=root)
    return inputs


def build_common_rust_derived_manifest(
    *,
    root: Path,
    family_id: str,
    state: str,
    source_rust_file: Path,
    generator_tool: str,
    generator_version: str,
    hako_path: Path,
    hako_text: str,
    claims: dict[str, Any],
    pilot_scope: str | None = None,
    inputs: dict[str, Any] | None = None,
    extra_fields: dict[str, Any] | None = None,
) -> dict[str, Any]:
    return build_rust_derived_hako_manifest(
        family_id=family_id,
        state=state,
        source_rust_files=[
            rust_manifest_file_entry(path=source_rust_file, root=root),
        ],
        generator_tool=generator_tool,
        generator_version=generator_version,
        hako_path=str(hako_path.relative_to(root)),
        hako_sha256=sha256_text(hako_text),
        claims=claims,
        pilot_scope=pilot_scope,
        inputs=inputs,
        extra_fields=extra_fields,
    )
