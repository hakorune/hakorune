#!/usr/bin/env python3
"""Typed Python projection of the Language v1 grammar registry."""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
import pathlib
from typing import Any, Iterable

import tomllib


ROOT = pathlib.Path(__file__).resolve().parents[2]
REGISTRY = ROOT / "grammar/language-v1-registry.toml"
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
    status: str
    normalization_mode: NormalizationMode
    normalized_shape: str
    stable_reject_tag: str
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


def _string(row: dict[str, Any], field: str) -> str:
    value = row.get(field)
    if not isinstance(value, str):
        raise ValueError(f"grammar registry row has invalid {field}")
    return value


def _profile_contract(source_row: dict[str, Any], profile_key: str) -> dict[str, Any]:
    value = source_row.get(profile_key)
    if not isinstance(value, dict):
        row_id = _string(source_row, "row_id")
        raise ValueError(f"grammar registry row {row_id} missing {profile_key} contract")
    return value


def load_registry_rows(path: pathlib.Path = REGISTRY) -> tuple[GrammarRegistryRow, ...]:
    with path.open("rb") as handle:
        document = tomllib.load(handle)
    raw_rows = document.get("rows")
    if not isinstance(raw_rows, list):
        raise ValueError("grammar registry rows are missing")

    rows = []
    for raw_row in raw_rows:
        if not isinstance(raw_row, dict):
            raise ValueError("grammar registry row must be a table")
        row_id = _string(raw_row, "row_id")
        for profile_key, profile in (("canonical", "Canonical"), ("compat2025", "Compat2025")):
            contract = _profile_contract(raw_row, profile_key)
            try:
                normalization_mode = NormalizationMode(_string(contract, "normalization_mode"))
            except ValueError as error:
                raise ValueError(
                    f"grammar registry row {row_id} has invalid {profile_key} normalization mode"
                ) from error
            status = _string(contract, "status")
            normalized_shape = _string(contract, "normalized_shape")
            _string(contract, "semantic_owner")
            stable_reject_tag = _string(contract, "stable_reject_tag")
            rows.append(
                GrammarRegistryRow(
                    row_id=row_id,
                    profile=profile,
                    status=status,
                    normalization_mode=normalization_mode,
                    normalized_shape=normalized_shape,
                    stable_reject_tag=stable_reject_tag,
                    positive_fixture_ids=_strings(contract, "positive_fixture_ids"),
                    negative_fixture_ids=_strings(contract, "negative_fixture_ids"),
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
