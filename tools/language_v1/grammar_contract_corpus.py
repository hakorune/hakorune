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
        return tomllib.load(handle)


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
