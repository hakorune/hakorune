#!/usr/bin/env python3
"""Typed Design Authority Registry loader seam (DESIGN-REGISTRY-V1 L0).

One in-memory representation with typed, fail-fast structural errors.
V0 (embedded INDEX.md block) is the only production source; a passive
V1 manifest/shard reader is added by S0 and cut over by C0.
"""

from __future__ import annotations

import argparse
import hashlib
from dataclasses import dataclass, field
import re
import tempfile
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

V0_BLOCK = re.compile(
    r"<!-- design-registry-v0:begin -->\s*```toml\s*(.*?)\s*```\s*"
    r"<!-- design-registry-v0:end -->",
    re.DOTALL,
)

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


class RegistryBlockMissing(RegistryLoadError):
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
    source: str = "v0"
    raw: dict = field(default_factory=dict)

    def paths(self) -> list[str]:
        return [row.get("path", "") for row in self.documents]


def load_v0(index_path: Path = DESIGN_INDEX) -> Registry:
    """Parse the embedded V0 block. Structural failures raise typed
    errors; semantic rule violations are produced by `validate`."""
    if not index_path.is_file():
        raise RegistryNotFound("design registry INDEX.md is missing")
    match = V0_BLOCK.search(index_path.read_text(encoding="utf-8"))
    if not match:
        raise RegistryBlockMissing("design registry typed block is missing")
    try:
        data = tomllib.loads(match.group(1))
    except tomllib.TOMLDecodeError as exc:
        raise RegistryMalformed(f"design registry TOML is malformed: {exc}")
    return Registry(
        schema_version=data.get("schema_version", -1),
        mode=data.get("mode", ""),
        unregistered_baseline=data.get("unregistered_baseline", 0),
        documents=data.get("documents", []),
        source="v0",
        raw=data,
    )


def validate(registry: Registry, direct_files: set[str]) -> list[str]:
    """Warning-mode validation. Violation strings and order are the V0
    parity contract — do not reorder."""
    violations: list[str] = []
    rows = registry.documents
    if registry.schema_version != 0:
        violations.append("design registry schema_version must be 0")
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
    """Production entry point — V0 adapter only. Returns the legacy
    `(registry_dict, violations)` shape for consumer parity."""
    try:
        registry = load_v0()
    except RegistryNotFound as exc:
        return {"mode": "warning", "unregistered_baseline": 0, "documents": []}, [
            str(exc)
        ]
    except RegistryBlockMissing as exc:
        return {"mode": "warning", "unregistered_baseline": 0, "documents": []}, [
            str(exc)
        ]
    except RegistryMalformed as exc:
        return {"mode": "warning", "unregistered_baseline": 0, "documents": []}, [
            str(exc)
        ]
    violations = validate(registry, direct_files)
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


def emit_manifest(registry: Registry) -> str:
    lines = [
        f"schema_version = {V1_SCHEMA_VERSION}",
        f'mode = "{_toml_escape(registry.mode)}"',
        f"unregistered_baseline = {registry.unregistered_baseline}",
        f'shard_algorithm = "{V1_SHARD_ALGORITHM}"',
        "shards = [",
    ]
    lines += [f'  "shards/{nid}.toml",' for nid in V1_SHARD_IDS]
    lines.append("]")
    return "\n".join(lines) + "\n"


def emit_shard(nid: str, rows: list[dict]) -> str:
    lines = [f"schema_version = {V1_SCHEMA_VERSION}", f'shard_id = "{nid}"', ""]
    for row in sorted(rows, key=lambda r: r["path"]):
        lines.append("[[documents]]")
        lines += [f"{key} = {_toml_value(row[key])}" for key in V1_ROW_FIELDS]
        lines.append("")
    return "\n".join(lines).rstrip("\n") + "\n"


def generate_v1(out_dir: Path, registry: Registry | None = None) -> list[Path]:
    """Deterministic V0->V1 migration generator (G0). Writes the
    manifest plus the exact shard set under `out_dir`. Callers choose
    the destination; tracked writes are an explicit command."""
    if registry is None:
        registry = load_v0()
    buckets: dict[str, list[dict]] = {nid: [] for nid in V1_SHARD_IDS}
    for row in registry.documents:
        normalized = _normalize_row(row)
        buckets[shard_key(normalized["path"])].append(normalized)
    shards_dir = out_dir / "shards"
    shards_dir.mkdir(parents=True, exist_ok=True)
    written = [out_dir / "manifest.toml"]
    written[0].write_text(emit_manifest(registry), encoding="utf-8")
    for nid in V1_SHARD_IDS:
        shard_path = shards_dir / f"{nid}.toml"
        shard_path.write_text(emit_shard(nid, buckets[nid]), encoding="utf-8")
        written.append(shard_path)
    return written


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("check", help="load + validate the production V0 registry")
    gen = sub.add_parser(
        "generate", help="emit V1 manifest+shards (temporary dir unless --output)"
    )
    gen.add_argument("--output", type=Path, default=None)
    args = parser.parse_args(argv)

    if args.command == "check":
        registry = load_v0()
        direct_files = {
            p.name for p in (ROOT / "docs/development/current/main/design").iterdir()
            if p.is_file()
        }
        violations = validate(registry, direct_files)
        for violation in violations:
            print(violation)
        print(
            f"design-registry check: {len(registry.documents)} rows, "
            f"{len(violations)} violations"
        )
        return 1 if violations and registry.mode == "strict" else 0
    if args.command == "generate":
        out_dir = args.output or Path(tempfile.mkdtemp(prefix="design-registry-v1-"))
        written = generate_v1(out_dir)
        print(f"generated {len(written)} files under {out_dir}")
        return 0
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
