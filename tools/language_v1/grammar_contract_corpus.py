#!/usr/bin/env python3
"""Typed access to the shared Language v1 grammar fixture corpus."""

from __future__ import annotations

import json
import pathlib
from typing import Any

import tomllib


ROOT = pathlib.Path(__file__).resolve().parents[2]
CORPUS = ROOT / "grammar/language-v1-grammar-contract-corpus.toml"


def _validate_normalized_node(node: Any, *, allow_empty: bool) -> None:
    if not isinstance(node, dict):
        raise ValueError("grammar normalized_form node must be a table")
    if set(node) - {"kind", "value", "children"}:
        raise ValueError("grammar normalized_form contains unsupported fields")
    kind = node.get("kind")
    value = node.get("value")
    children = node.get("children")
    if not isinstance(kind, str) or not isinstance(children, list):
        raise ValueError("grammar normalized_form requires kind and children")
    if value is not None and not isinstance(value, str):
        raise ValueError("grammar normalized_form value must be a string")
    if not kind and (not allow_empty or value is not None or children):
        raise ValueError("empty grammar normalized_form cannot carry data")
    for child in children:
        _validate_normalized_node(child, allow_empty=False)


def load_corpus() -> dict[str, Any]:
    with CORPUS.open("rb") as handle:
        manifest = tomllib.load(handle)
    fragments = manifest.get("fragments")
    if not isinstance(fragments, list) or not all(isinstance(path, str) for path in fragments):
        raise ValueError("grammar contract corpus manifest has invalid fragments")

    merged: dict[str, Any] = {"parser_inventory": {}, "fixtures": []}
    for relative_path in fragments:
        fragment_path = ROOT / relative_path
        with fragment_path.open("rb") as handle:
            fragment = tomllib.load(handle)
        inventories = fragment.get("parser_inventory", {})
        fixtures = fragment.get("fixtures", [])
        if not isinstance(inventories, dict) or not isinstance(fixtures, list):
            raise ValueError(f"grammar contract corpus fragment is invalid: {relative_path}")
        overlap = set(merged["parser_inventory"]).intersection(inventories)
        if overlap:
            raise ValueError(f"duplicate parser inventory in corpus fragments: {sorted(overlap)}")
        merged["parser_inventory"].update(inventories)
        merged["fixtures"].extend(fixtures)
    return merged


def parser_inventory_by_id(inventory_id: str) -> dict[str, Any]:
    inventories = load_corpus().get("parser_inventory", {})
    inventory = inventories.get(inventory_id)
    if not isinstance(inventory, dict) or not isinstance(inventory.get("enums"), list):
        raise KeyError(f"unknown parser inventory id: {inventory_id}")
    return inventory


def parser_inventory_json_by_id(inventory_id: str) -> str:
    if not inventory_id:
        return "[]"
    inventory = parser_inventory_by_id(inventory_id)
    enums = []
    for enum in inventory["enums"]:
        projected = dict(enum)
        projected["variants"] = [
            {"name": variant["name"], "payload_type": variant.get("payload_type")}
            for variant in enum["variants"]
        ]
        enums.append(projected)
    return json.dumps(enums, separators=(",", ":"))


def parser_inventory_source_by_id(inventory_id: str) -> str:
    if not inventory_id:
        return ""
    declarations = []
    for enum in parser_inventory_by_id(inventory_id)["enums"]:
        type_parameters = enum.get("type_parameters", [])
        generic = f"<{', '.join(type_parameters)}>" if type_parameters else ""
        variants = []
        for variant in enum["variants"]:
            payload_type = variant.get("payload_type")
            variants.append(
                f"{variant['name']}({payload_type})" if payload_type else variant["name"]
            )
        declarations.append(
            f"enum {enum['name']}{generic} {{ {', '.join(variants)} }}"
        )
    return "\n".join(declarations) + "\n"


def fixtures_by_id() -> dict[str, dict[str, Any]]:
    corpus = load_corpus()
    fixtures: dict[str, dict[str, Any]] = {}
    for raw_fixture in corpus["fixtures"]:
        fixture = dict(raw_fixture)
        _validate_normalized_node(fixture.get("normalized_form"), allow_empty=True)
        inventory_id = fixture.get("parser_inventory_id")
        if inventory_id:
            fixture["parser_inventory_json"] = parser_inventory_json_by_id(inventory_id)
            fixture["parser_inventory_source"] = parser_inventory_source_by_id(inventory_id)
        fixtures[fixture["fixture_id"]] = fixture
    return fixtures
