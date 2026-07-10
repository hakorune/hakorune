#!/usr/bin/env python3
"""Typed access to the shared Language v1 grammar fixture corpus."""

from __future__ import annotations

import json
import pathlib
from typing import Any

import tomllib


ROOT = pathlib.Path(__file__).resolve().parents[2]
CORPUS = ROOT / "grammar/language-v1-grammar-contract-corpus.toml"


def load_corpus() -> dict[str, Any]:
    with CORPUS.open("rb") as handle:
        manifest = tomllib.load(handle)
    fragments = manifest.get("fragments")
    if not isinstance(fragments, list) or not all(isinstance(path, str) for path in fragments):
        raise ValueError("grammar contract corpus manifest has invalid fragments")

    merged: dict[str, Any] = {"hako_inventory": {}, "fixtures": []}
    for relative_path in fragments:
        fragment_path = ROOT / relative_path
        with fragment_path.open("rb") as handle:
            fragment = tomllib.load(handle)
        inventories = fragment.get("hako_inventory", {})
        fixtures = fragment.get("fixtures", [])
        if not isinstance(inventories, dict) or not isinstance(fixtures, list):
            raise ValueError(f"grammar contract corpus fragment is invalid: {relative_path}")
        overlap = set(merged["hako_inventory"]).intersection(inventories)
        if overlap:
            raise ValueError(f"duplicate Hako inventory in corpus fragments: {sorted(overlap)}")
        merged["hako_inventory"].update(inventories)
        merged["fixtures"].extend(fixtures)
    return merged


def inventory_json_by_id(inventory_id: str) -> str:
    if not inventory_id:
        return "[]"
    inventories = load_corpus().get("hako_inventory", {})
    inventory = inventories.get(inventory_id)
    if not isinstance(inventory, dict) or not isinstance(inventory.get("enums"), list):
        raise KeyError(f"unknown Hako inventory id: {inventory_id}")
    enums = []
    for enum in inventory["enums"]:
        projected = dict(enum)
        projected["variants"] = [
            {"name": variant["name"], "payload_type": variant.get("payload_type")}
            for variant in enum["variants"]
        ]
        enums.append(projected)
    return json.dumps(enums, separators=(",", ":"))


def fixtures_by_id() -> dict[str, dict[str, Any]]:
    corpus = load_corpus()
    fixtures: dict[str, dict[str, Any]] = {}
    for raw_fixture in corpus["fixtures"]:
        fixture = dict(raw_fixture)
        inventory_id = fixture.get("hako_inventory_id")
        if inventory_id:
            fixture["hako_inventory_json"] = inventory_json_by_id(inventory_id)
        fixtures[fixture["fixture_id"]] = fixture
    return fixtures
