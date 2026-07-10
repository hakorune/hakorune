#!/usr/bin/env python3
"""Typed Python projection of the Language v1 grammar registry."""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
import pathlib
from typing import Any, Iterable

import tomllib


ROOT = pathlib.Path(__file__).resolve().parents[2]
REGISTRY = ROOT / "grammar/unified-grammar.toml"
HAKO_TRANSPORT_EXCLUSION_TAG = "parser/hako_transport_row_excluded"
RUST_MIGRATION_TRANSPORT_OWNER = "RustMigrationToolingOnly"


class NormalizationMode(str, Enum):
    CANONICAL_SHAPE = "canonical_shape"
    COMPATIBILITY_ALIAS = "compatibility_alias"
    COMPATIBILITY_TRANSPORT = "compatibility_transport"
    NONE = "none"


@dataclass(frozen=True)
class GrammarRegistryRow:
    row_id: str
    profile: str
    normalization_mode: NormalizationMode
    positive_fixture_ids: tuple[str, ...]
    negative_fixture_ids: tuple[str, ...]

    @property
    def fixture_ids(self) -> tuple[str, ...]:
        return self.positive_fixture_ids + self.negative_fixture_ids

    @property
    def excluded_from_hako_semantic_conformance(self) -> bool:
        return self.normalization_mode is NormalizationMode.COMPATIBILITY_TRANSPORT


def _strings(row: dict[str, Any], field: str) -> tuple[str, ...]:
    values = row.get(field)
    if not isinstance(values, list) or not all(isinstance(value, str) for value in values):
        raise ValueError(f"grammar registry row has invalid {field}")
    return tuple(values)


def load_registry_rows(path: pathlib.Path = REGISTRY) -> tuple[GrammarRegistryRow, ...]:
    with path.open("rb") as handle:
        document = tomllib.load(handle)
    raw_rows = document.get("language_v1_grammar_contract", {}).get("rows")
    if not isinstance(raw_rows, list):
        raise ValueError("grammar registry rows are missing")

    rows = []
    for raw_row in raw_rows:
        if not isinstance(raw_row, dict):
            raise ValueError("grammar registry row must be a table")
        try:
            row_id = raw_row["row_id"]
            profile = raw_row["profile"]
            normalization_mode = NormalizationMode(raw_row["normalization_mode"])
        except KeyError as error:
            raise ValueError(f"grammar registry row missing {error.args[0]}") from error
        if not all(isinstance(value, str) for value in (row_id, profile)):
            raise ValueError("grammar registry row identity fields must be strings")
        rows.append(
            GrammarRegistryRow(
                row_id=row_id,
                profile=profile,
                normalization_mode=normalization_mode,
                positive_fixture_ids=_strings(raw_row, "positive_fixture_ids"),
                negative_fixture_ids=_strings(raw_row, "negative_fixture_ids"),
            )
        )
    return tuple(rows)


def registry_rows_by_key(
    rows: Iterable[GrammarRegistryRow] | None = None,
) -> dict[tuple[str, str], GrammarRegistryRow]:
    indexed: dict[tuple[str, str], GrammarRegistryRow] = {}
    for row in load_registry_rows() if rows is None else rows:
        key = (row.row_id, row.profile)
        if key in indexed:
            raise ValueError(f"duplicate grammar registry row: {key}")
        indexed[key] = row
    return indexed


def hako_transport_fixture_ids(
    rows: Iterable[GrammarRegistryRow] | None = None,
) -> tuple[str, ...]:
    fixture_ids = []
    for row in load_registry_rows() if rows is None else rows:
        if row.excluded_from_hako_semantic_conformance:
            fixture_ids.extend(row.fixture_ids)
    if len(fixture_ids) != len(set(fixture_ids)):
        raise ValueError("transport fixture belongs to multiple registry rows")
    return tuple(fixture_ids)


def fixture_ids_for_row(
    row_id: str,
    profile: str,
    rows: Iterable[GrammarRegistryRow] | None = None,
) -> tuple[str, ...]:
    row = registry_rows_by_key(rows).get((row_id, profile))
    if row is None:
        raise KeyError((row_id, profile))
    return row.fixture_ids
