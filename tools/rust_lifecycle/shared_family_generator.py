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
