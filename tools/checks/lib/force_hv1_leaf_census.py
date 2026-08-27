#!/usr/bin/env python3
"""Derive the finite force-hv1 leaf observation from checked-in shell bodies.

This module is deliberately an observer: it does not execute a smoke, choose a
fate, or infer success from an exit status.  The manifest supplies the reviewed
inventory; this census verifies that inventory against the bounded phase2*
source surface and derives only syntactic route facts.
"""

from __future__ import annotations

import hashlib
import re
from pathlib import Path
from typing import Any


ENTRY_RE = re.compile(r"(?<![A-Za-z0-9_])(verify_v1_inline_file|verify_mir_rc)(?![A-Za-z0-9_])")
WRAPPER_RE = re.compile(
    r"(?<![A-Za-z0-9_])run_verify_mir_canary_and_expect_rc[ \t]*(?:\\[ \t]*\n[ \t]*)*([A-Za-z0-9_]+)"
)
ALLOWED_WRAPPER_PREFIX = "run_verify_mir_via_hakovm_"
DYNAMIC_NAMES = frozenset(
    {
        "mirbuilder_jsonfrag_normalizer_rc_parity_if_canary_vm.sh",
        "mirbuilder_jsonfrag_normalizer_rc_parity_binop_canary_vm.sh",
        "mirbuilder_jsonfrag_normalizer_rc_parity_loop_canary_vm.sh",
        "mirbuilder_internal_return_logical_varvar_core_exec_canary_vm.sh",
    }
)


def _strip_comments(text: str) -> str:
    """Blank shell comments without changing line/column offsets."""

    chars = list(text)
    single = double = escaped = False
    comment = False
    for index, char in enumerate(chars):
        if comment:
            if char == "\n":
                comment = False
            elif char != "\r":
                chars[index] = " "
            continue
        if escaped:
            escaped = False
            continue
        if char == "\\" and not single:
            escaped = True
            continue
        if char == "'" and not double:
            single = not single
            continue
        if char == '"' and not single:
            double = not double
            continue
        if char == "#" and not single and not double:
            comment = True
            chars[index] = " "
    if single or double:
        raise ValueError("unterminated shell quote")
    return "".join(chars)


def _line(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def _artifact_proof(body: str) -> str:
    if "run_nyash_vm" in body or "PROG_JSON" in body or "awk '/[" in body:
        return "dynamic_runtime"
    if "<<'JSON'" in body or '<<"JSON"' in body or "<<JSON" in body:
        return "static_heredoc"
    if "printf" in body or "echo" in body:
        return "static_writer"
    return "unknown"


def _route_class(path: Path, body: str, has_direct: bool, has_wrapper: bool) -> str:
    if has_direct:
        return "DirectForceSealed"
    if path.name in DYNAMIC_NAMES:
        return "DynamicArtifactOpen"
    if "HAKO_VERIFY_PRIMARY=core" in body:
        return "ExplicitCoreResidualSealed"
    if has_wrapper or "verify_mir_rc" in body:
        return "HelperForceConditional"
    raise ValueError(f"unsupported force-hv1 entry form: {path}")


def derive_leaf(root: Path, relative_path: str) -> dict[str, Any]:
    path = root / relative_path
    if not path.is_file() or path.is_symlink():
        raise ValueError(f"leaf is not a regular checked-in file: {relative_path}")
    raw = path.read_text(encoding="utf-8")
    body = _strip_comments(raw)
    entries = list(ENTRY_RE.finditer(body))
    wrappers = list(WRAPPER_RE.finditer(body))
    if any(match.group(1).startswith(ALLOWED_WRAPPER_PREFIX) is False for match in wrappers):
        raise ValueError(f"wrapper callback is outside the finite allowlist: {relative_path}")
    if not entries and not wrappers:
        raise ValueError(f"leaf has no allowed entry site: {relative_path}")
    if wrappers and entries:
        raise ValueError(f"leaf mixes wrapper and direct/helper entry forms: {relative_path}")

    sites: list[dict[str, Any]] = []
    for match in entries:
        sites.append(
            {
                "offset": match.start(),
                "line": _line(body, match.start()),
                "entry_symbol": match.group(1),
                "entry_form": "direct" if match.group(1) == "verify_v1_inline_file" else "helper",
            }
        )
    for match in wrappers:
        sites.append(
            {
                "offset": match.start(),
                "line": _line(body, match.start()),
                "entry_symbol": match.group(1),
                "entry_form": "wrapper",
            }
        )
    sites.sort(key=lambda item: item["offset"])
    if len({site["entry_form"] for site in sites}) > 1:
        raise ValueError(f"leaf mixes direct and helper entry forms: {relative_path}")
    for ordinal, site in enumerate(sites, start=1):
        site["ordinal"] = ordinal
        site.pop("offset")

    primary_modes = sorted(set(re.findall(r"HAKO_VERIFY_PRIMARY=(core|hakovm)", body)))
    if not primary_modes:
        primary_mode = "ambient"
    elif len(primary_modes) == 1:
        primary_mode = primary_modes[0]
    else:
        primary_mode = "mixed"
    direct = any(site["entry_form"] == "direct" for site in sites)
    wrapper = any(site["entry_form"] == "wrapper" for site in sites)
    route_class = _route_class(path, body, direct, wrapper)
    if route_class == "DirectForceSealed":
        ambient = "none"
    elif route_class == "ExplicitCoreResidualSealed":
        ambient = "explicit_core"
    elif route_class == "DynamicArtifactOpen":
        ambient = "unresolved"
    else:
        ambient = "inherited_force_core_or_primary"
    return {
        "path": relative_path,
        "body_sha256": hashlib.sha256(raw.encode("utf-8")).hexdigest(),
        "caller_family": "direct" if direct else "wrapper_only" if wrapper else "textual_helper",
        "sites": sites,
        "derived": {
            "entry_symbols": sorted({site["entry_symbol"] for site in sites}),
            "lexical_entry_sites": len(sites),
            "primary_mode": primary_mode,
            "environment_contract": "explicit_env_i" if direct else "inherited_parent",
            "ambient_preemption": ambient,
            "artifact_proof": _artifact_proof(body),
            "route_class": route_class,
        },
    }


def discover_leaf_paths(root: Path) -> list[str]:
    base = root / "tools/smokes/v2/profiles/integration/core"
    paths: list[str] = []
    for path in sorted(base.glob("phase2*/*.sh")):
        if path.is_symlink() or not path.is_file():
            continue
        raw = path.read_text(encoding="utf-8")
        if not ENTRY_RE.search(raw) and not WRAPPER_RE.search(raw):
            continue
        body = _strip_comments(raw)
        if ENTRY_RE.search(body) or WRAPPER_RE.search(body):
            paths.append(path.relative_to(root).as_posix())
    return paths


def derive_inventory(root: Path, paths: list[str]) -> list[dict[str, Any]]:
    if len(paths) != len(set(paths)):
        raise ValueError("manifest leaf inventory contains duplicate paths")
    discovered = discover_leaf_paths(root)
    if paths != sorted(paths):
        raise ValueError("manifest leaf inventory must be sorted")
    if discovered != paths:
        missing = sorted(set(discovered) - set(paths))
        extra = sorted(set(paths) - set(discovered))
        raise ValueError(f"leaf inventory drifted: missing={missing[:3]} extra={extra[:3]}")
    return [derive_leaf(root, path) for path in paths]


def derive_summary(observations: list[dict[str, Any]]) -> dict[str, Any]:
    """Aggregate only facts already derived from the checked-in leaf bodies."""

    route_class: dict[str, int] = {}
    route_sites: dict[str, int] = {}
    lexical_sites = 0
    for item in observations:
        route = item["derived"]["route_class"]
        sites = item["derived"]["lexical_entry_sites"]
        route_class[route] = route_class.get(route, 0) + 1
        route_sites[route] = route_sites.get(route, 0) + sites
        lexical_sites += sites
    return {
        "lexical_leaves": len(observations),
        "lexical_sites": lexical_sites,
        "route_class": route_class,
        "route_sites": route_sites,
    }
