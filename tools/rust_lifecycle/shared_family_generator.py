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


def rust_manifest_file_entry(*, path: Path, root: Path) -> dict[str, Any]:
    return {"path": str(path.relative_to(root)), "sha256": sha256_file(path)}


def rust_manifest_inputs(*entries: tuple[str, Path], root: Path) -> dict[str, Any]:
    return {
        name: rust_manifest_file_entry(path=path, root=root)
        for name, path in entries
    }
