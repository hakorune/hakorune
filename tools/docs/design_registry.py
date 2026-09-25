#!/usr/bin/env python3
"""Typed Design Authority Registry loader (DESIGN-REGISTRY-V1).

One in-memory representation with typed, fail-fast structural errors.
V1 (`design/registry/manifest.toml` + `shards/{0..f}.toml`) is the sole
authority. The V0 embedded-block reader was removed in R0.
"""

from __future__ import annotations

import argparse
import hashlib
from dataclasses import dataclass, field
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DESIGN_DIR = ROOT / "docs/development/current/main/design"
DESIGN_INDEX = DESIGN_DIR / "INDEX.md"
REGISTRY_DIR = DESIGN_DIR / "registry"
MANIFEST_PATH = REGISTRY_DIR / "manifest.toml"

DESIGN_ROLES = {
    "authority",
    "navigation",
    "supporting",
    "status-ledger",
    "superseded",
}

V1_SCHEMA_VERSION = 1
V1_SHARD_ALGORITHM = "sha256-utf8-first-nybble-v1"
V1_SHARD_IDS = tuple("0123456789abcdef")

# Complete V1 row schema — every field is explicit, including empty
# fields ("" / []). Order is the deterministic emission order.
V1_ROW_FIELDS = (
    "path",
    "role",
    "owner",
    "precedence_parent",
    "classification_basis",
    "sidecars",
    "supersedes",
    "superseded_by",
    "retire_when",
)

V1_MANIFEST_FIELDS = {
    "schema_version",
    "mode",
    "unregistered_baseline",
    "shard_algorithm",
    "shards",
}

V1_SHARD_FIELDS = {"schema_version", "shard_id", "documents"}


class RegistryLoadError(Exception):
    """Typed structural load failure (missing file/block, malformed
    TOML, bad schema). Validation rule violations stay on the warning
    list for V0 parity."""


class RegistryNotFound(RegistryLoadError):
    pass


class RegistryMalformed(RegistryLoadError):
    pass


@dataclass
class Registry:
    """Typed in-memory registry. `documents` keeps the raw row dicts so
    warning-mode consumers see byte-identical shapes."""

    schema_version: int
    mode: str
    unregistered_baseline: int
    documents: list[dict]
    source: str = "v1"
    raw: dict = field(default_factory=dict)

    def paths(self) -> list[str]:
        return [row.get("path", "") for row in self.documents]


def validate(
    registry: Registry, direct_files: set[str], expected_schema: int = 0
) -> list[str]:
    """Warning-mode validation. Violation strings and order are the V0
    parity contract — do not reorder."""
    violations: list[str] = []
    rows = registry.documents
    if registry.schema_version != expected_schema:
        violations.append(
            f"design registry schema_version must be {expected_schema}"
        )
    if registry.mode not in {"warning", "strict"}:
        violations.append("design registry mode must be warning or strict")
    paths = registry.paths()
    if len(paths) != len(set(paths)):
        violations.append("design registry contains duplicate paths")
    sidecar_owners: dict[str, str] = {}
    row_by_path = {row.get("path", ""): row for row in rows}
    for row in rows:
        path = row.get("path", "")
        role = row.get("role", "")
        if path not in direct_files:
            violations.append(f"registered design file is missing: {path}")
        if role not in DESIGN_ROLES:
            violations.append(f"invalid design role for {path}: {role}")
        if not row.get("owner"):
            violations.append(f"design row owner is missing: {path}")
        if not row.get("retire_when"):
            violations.append(f"design row retire_when is missing: {path}")
        if role == "superseded" and not row.get("superseded_by"):
            violations.append(f"superseded_by is required: {path}")
        for sidecar in row.get("sidecars", []):
            if sidecar not in direct_files:
                violations.append(f"design sidecar is missing: {path} -> {sidecar}")
            if sidecar in row_by_path:
                violations.append(
                    f"design sidecar also has a document row: {sidecar}"
                )
            previous_owner = sidecar_owners.setdefault(sidecar, path)
            if previous_owner != path:
                violations.append(
                    f"design sidecar has multiple owners: {sidecar}"
                )
    for path in paths:
        seen: set[str] = set()
        current = path
        while current in row_by_path:
            if current in seen:
                violations.append(f"design precedence cycle includes: {current}")
                break
            seen.add(current)
            current = row_by_path[current].get("precedence_parent", "")
    readme = DESIGN_INDEX.parent.joinpath("README.md").read_text(encoding="utf-8")
    if "INDEX.md" not in readme or "navigation-only" not in readme:
        violations.append("design README must identify INDEX.md and navigation-only role")
    return violations


def load_registry(direct_files: set[str]) -> tuple[dict, list[str]]:
    """Production entry point — V1 manifest/shards are the sole
    authority (C0). Returns the legacy `(registry_dict, violations)`
    shape for consumer parity. V1 failure is terminal: no V0 retry."""
    try:
        registry = load_v1()
    except RegistryLoadError as exc:
        return {"mode": "warning", "unregistered_baseline": 0, "documents": []}, [
            str(exc)
        ]
    violations = validate(registry, direct_files, V1_SCHEMA_VERSION)
    return registry.raw, violations


def shard_key(path: str) -> str:
    """Canonical V1 placement: first lowercase hex nybble of
    sha256(utf-8 path). Pure function of the exact registered path."""
    return hashlib.sha256(path.encode("utf-8")).hexdigest()[0]


def _load_toml_file(path: Path, what: str) -> dict:
    if not path.is_file():
        raise RegistryNotFound(f"design registry {what} is missing: {path}")
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as exc:
        raise RegistryMalformed(f"design registry {what} TOML is malformed: {exc}")


def load_v1(registry_dir: Path = REGISTRY_DIR) -> Registry:
    """Passive V1 manifest/shard reader (S0). Structural failures raise
    typed errors; production callers remain zero until C0."""
    manifest = _load_toml_file(registry_dir / "manifest.toml", "manifest")
    unknown = set(manifest) - V1_MANIFEST_FIELDS
    if unknown:
        raise RegistryMalformed(
            f"design registry manifest has unknown fields: {sorted(unknown)}"
        )
    if manifest.get("schema_version") != V1_SCHEMA_VERSION:
        raise RegistryMalformed("design registry manifest schema_version must be 1")
    if manifest.get("shard_algorithm") != V1_SHARD_ALGORITHM:
        raise RegistryMalformed(
            f"design registry shard_algorithm must be {V1_SHARD_ALGORITHM}"
        )
    expected_shards = [f"shards/{nid}.toml" for nid in V1_SHARD_IDS]
    if manifest.get("shards") != expected_shards:
        raise RegistryMalformed(
            "design registry manifest shards must be the exact ordered set 0..f"
        )
    documents: list[dict] = []
    seen_paths: set[str] = set()
    for nid in V1_SHARD_IDS:
        shard = _load_toml_file(
            registry_dir / "shards" / f"{nid}.toml", f"shard {nid}"
        )
        unknown = set(shard) - V1_SHARD_FIELDS
        if unknown:
            raise RegistryMalformed(
                f"design registry shard {nid} has unknown fields: {sorted(unknown)}"
            )
        if shard.get("schema_version") != V1_SCHEMA_VERSION:
            raise RegistryMalformed(f"design registry shard {nid} schema_version must be 1")
        if shard.get("shard_id") != nid:
            raise RegistryMalformed(
                f"design registry shard id mismatch: expected {nid}, got {shard.get('shard_id')!r}"
            )
        rows = shard.get("documents", [])
        shard_paths = [row.get("path", "") for row in rows]
        if shard_paths != sorted(shard_paths):
            raise RegistryMalformed(
                f"design registry shard {nid} rows are not ordered by path"
            )
        for row in rows:
            missing = [key for key in V1_ROW_FIELDS if key not in row]
            if missing:
                raise RegistryMalformed(
                    f"design registry shard {nid} row {row.get('path', '')!r} "
                    f"lacks fields: {missing}"
                )
            path = row["path"]
            if shard_key(path) != nid:
                raise RegistryMalformed(
                    f"design registry path in wrong shard: {path} (in {nid})"
                )
            if path in seen_paths:
                raise RegistryMalformed(f"design registry duplicate path: {path}")
            seen_paths.add(path)
            documents.append(row)
    return Registry(
        schema_version=V1_SCHEMA_VERSION,
        mode=manifest.get("mode", ""),
        unregistered_baseline=manifest.get("unregistered_baseline", 0),
        documents=documents,
        source="v1",
        raw={
            "schema_version": V1_SCHEMA_VERSION,
            "mode": manifest.get("mode", ""),
            "unregistered_baseline": manifest.get("unregistered_baseline", 0),
            "documents": documents,
        },
    )


def _toml_escape(text: str) -> str:
    return (
        text.replace("\\", "\\\\")
        .replace('"', '\\"')
        .replace("\t", "\\t")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
    )


def _toml_value(value: object) -> str:
    if isinstance(value, str):
        return f'"{_toml_escape(value)}"'
    if isinstance(value, int):
        return str(value)
    if isinstance(value, list):
        return "[" + ", ".join(_toml_value(item) for item in value) + "]"
    raise RegistryMalformed(f"unspelled registry field type: {type(value).__name__}")


def _normalize_row(row: dict) -> dict:
    """Project a row onto the complete V1 schema; absent fields become
    explicit empty fields (never invented values)."""
    normalized: dict = {}
    for key in V1_ROW_FIELDS:
        value = row.get(key)
        if value is None:
            value = [] if key in {"sidecars", "supersedes"} else ""
        normalized[key] = value
    return normalized


def emit_shard(nid: str, rows: list[dict]) -> str:
    lines = [f"schema_version = {V1_SCHEMA_VERSION}", f'shard_id = "{nid}"', ""]
    for row in sorted(rows, key=lambda r: r["path"]):
        lines.append("[[documents]]")
        lines += [f"{key} = {_toml_value(row[key])}" for key in V1_ROW_FIELDS]
        lines.append("")
    return "\n".join(lines).rstrip("\n") + "\n"


def _direct_files() -> set[str]:
    return {
        p.name for p in DESIGN_DIR.iterdir() if p.is_file()
    }


def _rewrite_shard(registry_dir: Path, nid: str, documents: list[dict]) -> None:
    """Rewrite exactly one shard file with deterministic spelling."""
    rows = [row for row in documents if shard_key(row["path"]) == nid]
    (registry_dir / "shards" / f"{nid}.toml").write_text(
        emit_shard(nid, rows), encoding="utf-8"
    )


def helper_add(registry_dir: Path, row: dict) -> str:
    """H0 helper add: insert one complete row into its canonical shard.
    All fields are caller-supplied; nothing is inferred."""
    registry = load_v1(registry_dir)
    path = row["path"]
    if path in set(registry.paths()):
        raise RegistryMalformed(f"design registry already contains: {path}")
    normalized = _normalize_row(row)
    nid = shard_key(path)
    documents = registry.documents + [normalized]
    _rewrite_shard(registry_dir, nid, documents)
    return nid


def helper_update(registry_dir: Path, path: str, updates: dict) -> str:
    """H0 helper update: rewrite fields of one existing row in place.
    `path` itself cannot change (that is remove + add)."""
    registry = load_v1(registry_dir)
    if "path" in updates and updates["path"] != path:
        raise RegistryMalformed("update cannot change path; use add")
    row = next((r for r in registry.documents if r.get("path") == path), None)
    if row is None:
        raise RegistryNotFound(f"design registry row is missing: {path}")
    unknown = set(updates) - set(V1_ROW_FIELDS)
    if unknown:
        raise RegistryMalformed(f"unknown row fields: {sorted(unknown)}")
    row.update(updates)
    nid = shard_key(path)
    _rewrite_shard(registry_dir, nid, registry.documents)
    return nid


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    chk = sub.add_parser("check", help="load + validate the V1 registry")
    loc = sub.add_parser("locate", help="print the canonical shard for a path")
    loc.add_argument("path")
    add = sub.add_parser("add", help="add one complete row to the V1 store")
    for key in V1_ROW_FIELDS:
        required = key in {"path", "role", "owner", "retire_when"}
        if key in {"sidecars", "supersedes"}:
            add.add_argument(f"--{key.replace('_', '-')}", nargs="*", default=[])
        else:
            add.add_argument(
                f"--{key.replace('_', '-')}", required=required, default=""
            )
    upd = sub.add_parser("update", help="update fields of one V1 row")
    upd.add_argument("path")
    upd.add_argument("--set", action="append", default=[], metavar="KEY=VALUE")
    args = parser.parse_args(argv)

    if args.command == "check":
        registry = load_v1()
        violations = validate(registry, _direct_files(), V1_SCHEMA_VERSION)
        for violation in violations:
            print(violation)
        print(
            f"design-registry check: "
            f"{len(registry.documents)} rows, {len(violations)} violations"
        )
        return 1 if violations and registry.mode == "strict" else 0
    if args.command == "locate":
        nid = shard_key(args.path)
        print(f"{args.path} -> shards/{nid}.toml")
        return 0
    if args.command == "add":
        row = {
            key: getattr(args, key) for key in V1_ROW_FIELDS
        }
        nid = helper_add(REGISTRY_DIR, row)
        print(f"added {row['path']} -> shards/{nid}.toml")
        return 0
    if args.command == "update":
        updates: dict = {}
        for item in args.set:
            key, _, value = item.partition("=")
            if not key:
                raise RegistryMalformed(f"bad --set item: {item!r}")
            updates[key.replace("-", "_")] = value
        nid = helper_update(REGISTRY_DIR, args.path, updates)
        print(f"updated {args.path} -> shards/{nid}.toml")
        return 0
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
