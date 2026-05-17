#!/usr/bin/env python3
"""Run a small declarative guard spec without shell eval."""

from __future__ import annotations

import argparse
import os
from pathlib import Path
import subprocess
import sys
import tomllib
from typing import Any


def fail(tag: str, message: str) -> None:
    print(f"[{tag}] ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def require_string(data: dict[str, Any], key: str, tag: str) -> str:
    value = data.get(key)
    if not isinstance(value, str) or not value:
        fail(tag, f"spec field must be a non-empty string: {key}")
    return value


def require_string_list(value: Any, field: str, tag: str) -> list[str]:
    if not isinstance(value, list) or not all(isinstance(item, str) and item for item in value):
        fail(tag, f"spec field must be a non-empty string array: {field}")
    return value


def require_rg(tag: str) -> None:
    try:
        subprocess.run(["rg", "--version"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
    except (OSError, subprocess.CalledProcessError):
        fail(tag, "rg is required")


def check_contains(root: Path, tag: str, entries: Any) -> None:
    if entries is None:
        return
    if not isinstance(entries, list):
        fail(tag, "contains must be an array of tables")
    for idx, entry in enumerate(entries, start=1):
        if not isinstance(entry, dict):
            fail(tag, f"contains[{idx}] must be a table")
        file_name = require_string(entry, "file", tag)
        pattern = require_string(entry, "pattern", tag)
        message = require_string(entry, "message", tag)
        file_path = root / file_name
        if not file_path.is_file():
            fail(tag, f"contains target missing: {file_name}")
        result = subprocess.run(
            ["rg", "-q", "--", pattern, file_name],
            cwd=root,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        if result.returncode != 0:
            fail(tag, message)


def check_forbidden(root: Path, tag: str, entries: Any) -> None:
    if entries is None:
        return
    if not isinstance(entries, list):
        fail(tag, "forbidden must be an array of tables")
    for idx, entry in enumerate(entries, start=1):
        if not isinstance(entry, dict):
            fail(tag, f"forbidden[{idx}] must be a table")
        paths = require_string_list(entry.get("paths"), f"forbidden[{idx}].paths", tag)
        pattern = require_string(entry, "pattern", tag)
        message = require_string(entry, "message", tag)
        missing = [path for path in paths if not (root / path).exists()]
        if missing:
            fail(tag, f"forbidden target missing: {', '.join(missing)}")
        result = subprocess.run(
            ["rg", "-n", "--", pattern, *paths],
            cwd=root,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=False,
        )
        if result.returncode == 0:
            sys.stderr.write(result.stdout)
            fail(tag, message)
        if result.returncode not in (0, 1):
            sys.stderr.write(result.stderr)
            fail(tag, f"forbidden rg failed for pattern: {pattern}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument("--spec", required=True, help="guard spec TOML path")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    spec_path = root / args.spec
    if not spec_path.is_file():
        print(f"[guard-spec-runner] ERROR: spec missing: {args.spec}", file=sys.stderr)
        return 1

    data = tomllib.loads(spec_path.read_text(encoding="utf-8"))
    tag = require_string(data, "tag", "guard-spec-runner")
    message = data.get("message")
    if message is not None and not isinstance(message, str):
        fail(tag, "message must be a string when present")

    require_rg(tag)

    required_files = require_string_list(data.get("require_files", []), "require_files", tag)
    for path in required_files:
        if not (root / path).is_file():
            fail(tag, f"required file missing: {path}")

    required_exec = require_string_list(data.get("require_exec_files", []), "require_exec_files", tag)
    for path in required_exec:
        target = root / path
        if not target.is_file() or not os.access(target, os.X_OK):
            fail(tag, f"file missing or not executable: {path}")

    if message:
        print(f"[{tag}] {message}")

    check_contains(root, tag, data.get("contains"))
    check_forbidden(root, tag, data.get("forbidden"))

    print(f"[{tag}] ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
