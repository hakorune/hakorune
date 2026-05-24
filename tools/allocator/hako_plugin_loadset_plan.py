#!/usr/bin/env python3
"""Emit a no-dlopen plugin loadset preflight plan.

This tool is deliberately diagnostic-only. It parses the selected config and
reports which plugin libraries belong to the selected loadset, but it never
loads a shared library or executes provider/plugin code.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import tomllib
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]

OUTPUT_CONTRACT = "hako-plugin-loadset-plan-v0"
PLUGIN_LOAD_POLICY = "eager_selected"


def default_config_path() -> Path:
    hako = ROOT / "hako.toml"
    if hako.exists():
        return hako
    return ROOT / "nyash.toml"


def load_config(path: Path) -> dict[str, Any]:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"config not found: {path}") from exc
    except tomllib.TOMLDecodeError as exc:
        raise SystemExit(f"invalid TOML config {path}: {exc}") from exc


def search_paths(config: dict[str, Any]) -> list[Path]:
    paths: list[Path] = []
    raw = config.get("plugin_paths", {}).get("search_paths", [])
    if isinstance(raw, list):
        for item in raw:
            if isinstance(item, str):
                paths.append(resolve_against_root(Path(item)))
    envp = os.environ.get("NYASH_PLUGIN_PATHS", "")
    if envp:
        sep = ";" if os.name == "nt" else ":"
        for item in envp.split(sep):
            if item:
                paths.append(resolve_against_root(Path(item)))
    return paths


def resolve_against_root(path: Path) -> Path:
    if path.is_absolute():
        return path
    return ROOT / path


def candidate_paths(configured_path: str, config_dir: Path, config: dict[str, Any]) -> list[Path]:
    base = Path(configured_path)
    candidates: list[Path] = []

    def add(path: Path) -> None:
        if path not in candidates:
            candidates.append(path)

    # Match the Rust loader's broad shape: configured path first, then platform
    # extension candidate, then search_paths by basename.
    add(resolve_against_root(base))
    if not base.is_absolute():
        add(config_dir / base)

    if os.name == "nt":
        ext_candidates = [base.with_suffix(".dll")]
        name = base.name
        if name.startswith("lib"):
            ext_candidates.append(base.with_name(name[3:]).with_suffix(".dll"))
    elif sys.platform == "darwin":
        ext_candidates = [base.with_suffix(".dylib")]
    else:
        ext_candidates = [base.with_suffix(".so")]

    for item in ext_candidates:
        add(resolve_against_root(item))
        if not item.is_absolute():
            add(config_dir / item)

    names = {path.name for path in candidates if path.name}
    for search_path in search_paths(config):
        for name in sorted(names):
            add(search_path / name)

    return candidates


def resolve_library_path(configured_path: str, config_dir: Path, config: dict[str, Any]) -> tuple[Path | None, list[str]]:
    candidates = candidate_paths(configured_path, config_dir, config)
    for path in candidates:
        if path.exists():
            return path, [str(item) for item in candidates]
    return None, [str(item) for item in candidates]


def library_rows(config_path: Path, config: dict[str, Any], selected: list[tuple[str, dict[str, Any]]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    config_dir = config_path.resolve().parent
    for name, lib_def in selected:
        configured_path = str(lib_def.get("path", name))
        resolved, candidates = resolve_library_path(configured_path, config_dir, config)
        boxes = lib_def.get("boxes", [])
        if not isinstance(boxes, list):
            boxes = []
        rows.append(
            {
                "name": name,
                "configured_path": configured_path,
                "resolved_path": str(resolved) if resolved else "",
                "path_exists": 1 if resolved else 0,
                "box_count": len(boxes),
                "boxes": [str(box) for box in boxes],
                "candidate_paths": candidates,
            }
        )
    return rows


def select_libraries(loadset: str, config: dict[str, Any]) -> list[tuple[str, dict[str, Any]]]:
    libraries = config.get("libraries", {})
    if not isinstance(libraries, dict):
        raise SystemExit("[libraries] must be a table")

    normalized = loadset.replace("-", "_")
    if normalized in ("empty", "no_plugins"):
        return []
    if normalized in ("root", "default", "all"):
        return sorted(
            ((str(name), lib_def) for name, lib_def in libraries.items() if isinstance(lib_def, dict)),
            key=lambda item: item[0],
        )
    raise SystemExit(
        f"unsupported loadset {loadset!r}; supported today: root, default, all, empty, no_plugins"
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", type=Path, default=default_config_path())
    parser.add_argument(
        "--loadset",
        default="root",
        help="root/default/all or empty/no_plugins. app/core are reserved by the SSOT.",
    )
    parser.add_argument("--out", type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    config_path = resolve_against_root(args.config)
    config = load_config(config_path)
    selected = select_libraries(args.loadset, config)
    rows = library_rows(config_path, config, selected)
    missing_count = sum(1 for row in rows if row["path_exists"] == 0)
    loadset = args.loadset.replace("-", "_")

    report = {
        "output_contract": OUTPUT_CONTRACT,
        "config_path": str(config_path),
        "selected_loadset": loadset,
        "plugin_load_policy": PLUGIN_LOAD_POLICY,
        "library_count": len(rows),
        "missing_library_count": missing_count,
        "preflight_ok": 1 if missing_count == 0 else 0,
        "provider_activation": 0,
        "host_replacement": 0,
        "hook_installed": 0,
        "global_allocator_installed": 0,
        "winner_claim": 0,
        "libraries": rows,
    }

    text = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
