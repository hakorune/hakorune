"""Shared helpers for the Hakozuna mixed-ws gap ladder summary."""

from __future__ import annotations

from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def as_int(values: dict[str, str], key: str, default: int = 0) -> int:
    text = values.get(key, str(default))
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def ratio(value: float, baseline: float) -> str:
    if baseline <= 0.0:
        return "nan"
    return f"{value / baseline:.3f}"


def slower_percent(value: float, baseline: float) -> str:
    if value <= 0.0:
        return "nan"
    return f"{(baseline / value - 1.0) * 100.0:.1f}"


def subject_rows(values: dict[str, str]) -> dict[str, dict[str, str]]:
    rows: dict[str, dict[str, str]] = {}
    subject_count = as_int(values, "subject_count")
    for index in range(subject_count):
        prefix = f"subject_{index}_"
        subject_id = values.get(f"{prefix}id")
        if not subject_id:
            raise SystemExit(f"missing {prefix}id")
        row: dict[str, str] = {}
        for key, value in values.items():
            if key.startswith(prefix):
                row[key.removeprefix(prefix)] = value
        rows[subject_id] = row
    return rows


def append_if_present(lines: list[str], values: dict[str, str], key: str) -> None:
    value = values.get(key)
    if value is not None:
        lines.append(f"{key}={value}")
