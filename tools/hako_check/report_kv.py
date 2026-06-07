#!/usr/bin/env python3
"""Shared key/value report readers for hako_check tools."""

from __future__ import annotations

from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    if not path.is_file():
        raise SystemExit(f"missing report file: {path}")
    rows: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        rows[key.strip()] = value.strip()
    return rows


def read_expected_kv(path: Path) -> dict[str, str]:
    """Read an expected-key manifest.

    The format intentionally matches report `.kv` files so shell grep
    expectations can move to data files without introducing a second syntax.
    """

    return read_kv(path)


def expected_kv_mismatches(
    rows: dict[str, str],
    expected: dict[str, str],
) -> list[tuple[str, str, str]]:
    """Return `(key, expected, actual)` rows for exact expectation failures."""

    mismatches: list[tuple[str, str, str]] = []
    for key, expected_value in expected.items():
        actual_value = rows.get(key, "")
        if actual_value != expected_value:
            mismatches.append((key, expected_value, actual_value))
    return mismatches


def format_expected_kv_mismatches(
    mismatches: list[tuple[str, str, str]],
) -> list[str]:
    return [
        f"{key}: expected={expected} actual={actual}"
        for key, expected, actual in mismatches
    ]


def first_value(rows: dict[str, str], keys: list[str], default: str = "") -> str:
    for key in keys:
        value = rows.get(key)
        if value is not None and value != "":
            return value
    return default


def int_value(rows: dict[str, str], keys: list[str], default: int = 0) -> int:
    value = first_value(rows, keys)
    if value == "":
        return default
    try:
        return int(float(value))
    except ValueError:
        return default


def float_value(rows: dict[str, str], keys: list[str], default: float = 0.0) -> float:
    value = first_value(rows, keys)
    if value == "":
        return default
    try:
        return float(value)
    except ValueError:
        return default


def subject_indices(rows: dict[str, str]) -> list[int]:
    indices: set[int] = set()
    for key in rows:
        if not key.startswith("subject_"):
            continue
        parts = key.split("_", 2)
        if len(parts) < 3:
            continue
        try:
            indices.add(int(parts[1]))
        except ValueError:
            continue
    return sorted(indices)


def find_subject(rows: dict[str, str], front_class: str, fallback: int) -> int:
    for idx in subject_indices(rows):
        if rows.get(f"subject_{idx}_benchmark_front_class") == front_class:
            return idx
    return fallback


def prefixed(rows: dict[str, str], subject_idx: int, suffix: str, default: str = "") -> str:
    return first_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def prefixed_int(rows: dict[str, str], subject_idx: int, suffix: str, default: int = 0) -> int:
    return int_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def prefixed_float(
    rows: dict[str, str],
    subject_idx: int,
    suffix: str,
    default: float = 0.0,
) -> float:
    return float_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def ratio(numerator: float, denominator: float) -> float:
    if denominator == 0.0:
        return 0.0
    return numerator / denominator
