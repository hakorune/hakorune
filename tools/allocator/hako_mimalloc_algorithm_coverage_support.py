"""Shared helpers for mimalloc algorithm coverage reports."""

from __future__ import annotations

import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]
HAKO_ALLOC = ROOT / "lang/src/hako_alloc/memory"
REPLACEMENT_FRONT = ROOT / "tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py"
REPLACEMENT_TEMPLATES = ROOT / "tools/allocator/replacement_front_templates.py"


@dataclass(frozen=True)
class CoverageRow:
    area: str
    hako_model: int
    replacement_front: int
    status: str
    evidence: str
    next_bridge: str


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        return ""


def read_kv_report(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    data: dict[str, str] = {}
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except FileNotFoundError:
        return {}
    for line in lines:
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        data[key.strip()] = value.strip()
    return data


def read_fastpath_counts(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    try:
        text = path.read_text(encoding="utf-8")
    except FileNotFoundError:
        return {}
    try:
        payload = json.loads(text)
    except json.JSONDecodeError:
        return read_kv_report(path)
    counts = payload.get("counts") if isinstance(payload, dict) else None
    if not isinstance(counts, dict):
        return {}
    return {str(key): str(value) for key, value in counts.items()}


def int_field(data: dict[str, str], key: str, default: int = 0) -> int:
    try:
        return int(data.get(key, str(default)))
    except ValueError:
        return default


def str_field(data: dict[str, str], key: str, default: str = "0") -> str:
    value = data.get(key, default)
    return value if value else default


def page_model_field_names(page_box: str) -> list[str]:
    names: list[str] = []
    field_re = r"^\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*:"
    for match in re.finditer(field_re, page_box, re.MULTILINE):
        name = match.group("name")
        if name not in names:
            names.append(name)
    return names


def has_file(path: Path) -> bool:
    return path.exists() and path.is_file()


def has_all(text: str, needles: Iterable[str]) -> bool:
    return all(needle in text for needle in needles)


def hako_file(name: str) -> Path:
    return HAKO_ALLOC / name


def count_member_calls(text: str, field: str, method: str) -> int:
    """Count direct `field.method(` and `me.field.method(` source calls.

    This is a static readiness scan, not semantic alias analysis. The leading
    boundary avoids counting `free.set(...)` inside `local_free.set(...)`.
    """

    pattern = rf"(?<![A-Za-z0-9_])(?:me\.)?{re.escape(field)}\.{method}\s*\("
    return len(re.findall(pattern, text))
