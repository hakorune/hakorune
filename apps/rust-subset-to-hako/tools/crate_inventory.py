#!/usr/bin/env python3
"""Inventory a RustSubset crate handoff bundle.

The tool reads an existing RustSubsetCrateManifest plus its per-module
RustSubsetModule artifacts. It does not parse Rust source and does not invoke
the syn adapter; that keeps parser/crate graph ownership outside the
Hakorune-owned converter core.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON {path}: {exc}") from exc


def require_object(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def validate_manifest(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    if manifest.get("schema_version") != 0:
        raise SystemExit("manifest schema_version must be 0")
    if manifest.get("kind") != "RustSubsetCrateManifest":
        raise SystemExit("manifest kind must be RustSubsetCrateManifest")
    modules = manifest.get("modules")
    if not isinstance(modules, list):
        raise SystemExit("manifest modules must be an array")
    seen_modules: set[str] = set()
    seen_artifacts: set[str] = set()
    out: list[dict[str, Any]] = []
    for index, entry in enumerate(modules):
        entry = require_object(entry, f"manifest module entry {index}")
        module = require_string(entry.get("module"), f"modules[{index}].module")
        artifact = require_string(entry.get("artifact_path"), f"modules[{index}].artifact_path")
        source = require_string(entry.get("source_path"), f"modules[{index}].source_path")
        if module in seen_modules:
            raise SystemExit(f"duplicate module id: {module}")
        if artifact in seen_artifacts:
            raise SystemExit(f"duplicate artifact path: {artifact}")
        if Path(artifact).is_absolute() or ".." in Path(artifact).parts:
            raise SystemExit(f"unsafe artifact path: {artifact}")
        seen_modules.add(module)
        seen_artifacts.add(artifact)
        out.append({"module": module, "artifact_path": artifact, "source_path": source})
    return out


def require_string(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value:
        raise SystemExit(f"{label} must be a non-empty string")
    return value


def iter_nodes(value: Any):
    yield value
    if isinstance(value, dict):
        for child in value.values():
            yield from iter_nodes(child)
    elif isinstance(value, list):
        for child in value:
            yield from iter_nodes(child)


def inventory_module(path: Path, expected_module: str) -> Counter[str]:
    module = require_object(load_json(path), f"module artifact {path}")
    if module.get("schema_version") != 0:
        raise SystemExit(f"{path}: schema_version must be 0")
    if module.get("kind") != "RustSubsetModule":
        raise SystemExit(f"{path}: kind must be RustSubsetModule")
    if module.get("module") != expected_module:
        raise SystemExit(
            f"{path}: module mismatch, expected {expected_module!r}, got {module.get('module')!r}"
        )

    counts: Counter[str] = Counter()
    items = module.get("items")
    if not isinstance(items, list):
        raise SystemExit(f"{path}: items must be an array")
    counts["item_total"] += len(items)

    for node in iter_nodes(module):
        if not isinstance(node, dict):
            continue
        kind = node.get("kind")
        if isinstance(kind, str):
            counts[f"kind.{kind}"] += 1
            if kind == "Unsupported":
                counts["unsupported_total"] += 1
                rust_kind = node.get("rust_kind")
                code = node.get("code")
                if isinstance(rust_kind, str) and rust_kind:
                    counts[f"unsupported_rust_kind.{rust_kind}"] += 1
                else:
                    counts["unsupported_rust_kind.<missing>"] += 1
                if isinstance(code, str) and code:
                    counts[f"unsupported_code.{code}"] += 1

        if "source_path" in node and isinstance(node["source_path"], list):
            counts["source_path_node_total"] += 1
        if "emitted_name" in node:
            counts["emitted_name_node_total"] += 1
        if "source_name" in node:
            counts["source_name_node_total"] += 1

    return counts


def merge_counts(total: Counter[str], module_counts: Counter[str]) -> None:
    for key, value in module_counts.items():
        total[key] += value


def print_kv(manifest_path: Path, manifest: dict[str, Any], entries: list[dict[str, Any]]) -> None:
    root = manifest_path.parent
    total: Counter[str] = Counter()
    print("output_contract=rust-subset-crate-inventory-v0")
    print(f"manifest_path={manifest_path}")
    print(f"crate_name={manifest.get('crate_name', '')}")
    target = manifest.get("target") if isinstance(manifest.get("target"), dict) else {}
    print(f"target_kind={target.get('kind', '')}")
    print(f"target_name={target.get('name', '')}")
    print(f"module_count={len(entries)}")

    for index, entry in enumerate(entries):
        path = root / entry["artifact_path"]
        counts = inventory_module(path, entry["module"])
        merge_counts(total, counts)
        print(f"module_{index}_id={entry['module']}")
        print(f"module_{index}_source_path={entry['source_path']}")
        print(f"module_{index}_artifact_path={entry['artifact_path']}")
        print(f"module_{index}_item_total={counts['item_total']}")
        print(f"module_{index}_unsupported_total={counts['unsupported_total']}")

    for key in sorted(total):
        print(f"total_{key}={total[key]}")

    blocker_count = total["unsupported_total"]
    print(f"unsupported_total={blocker_count}")
    print(f"selection_ready={1 if blocker_count == 0 else 0}")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--manifest",
        type=Path,
        required=True,
        help="Path to crate-manifest.json",
    )
    args = parser.parse_args()

    manifest = require_object(load_json(args.manifest), "manifest")
    entries = validate_manifest(manifest)
    print_kv(args.manifest, manifest, entries)


if __name__ == "__main__":
    main()
