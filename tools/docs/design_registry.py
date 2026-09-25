#!/usr/bin/env python3
"""Typed Design Authority Registry loader seam (DESIGN-REGISTRY-V1 L0).

One in-memory representation with typed, fail-fast structural errors.
V0 (embedded INDEX.md block) is the only production source; a passive
V1 manifest/shard reader is added by S0 and cut over by C0.
"""

from __future__ import annotations

from dataclasses import dataclass, field
import re
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

REQUIRED_ROW_FIELDS = (
    "path",
    "role",
    "owner",
    "precedence_parent",
    "sidecars",
    "supersedes",
    "superseded_by",
    "retire_when",
)


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
