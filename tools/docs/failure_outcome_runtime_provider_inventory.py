#!/usr/bin/env python3
"""Build and validate the S2 runtime/provider evidence inventory.

The manifest records evidence occurrences only.  It never classifies a shared
carrier, enables a provider, or changes runtime/backend behavior.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
S1_GRAPH = ROOT / "tools/checks/manifests/failure_outcome_semantic_site_graph_v0.json"
S2_BINDING = ROOT / "tools/checks/manifests/failure_outcome_projection_binding_v0.json"
OUTPUT = ROOT / "tools/checks/manifests/failure_outcome_runtime_provider_inventory_v0.json"

FAMILIES = (
    "value_conversion",
    "weak_upgrade",
    "null_like_box",
    "provider_status",
    "missing_result_synthesis",
    "ffi_boundary",
)
PENDING_REASONS = frozenset(
    {
        "S1SiteReferenceMissing",
        "ProviderContractMissing",
        "BoundaryKindPending",
    }
)
RESOLUTIONS = frozenset({"Pending", "Linked"})


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def relative(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def evidence_id(family: str, path: str, line: int, token: str) -> str:
    return f"runtime.{family}.{path}:{line}:{token}"


def source_files(prefixes: tuple[str, ...]) -> Iterable[Path]:
    roots = [ROOT / prefix for prefix in prefixes]
    for root in roots:
        if root.is_file():
            yield root
            continue
        if root.is_dir():
            yield from sorted(path for path in root.rglob("*") if path.is_file())


def scan(
    family: str,
    prefixes: tuple[str, ...],
    patterns: tuple[tuple[str, str], ...],
    *,
    path_filter: re.Pattern[str] | None = None,
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in source_files(prefixes):
        path_text = relative(path)
        if path_filter and not path_filter.search(path_text):
            continue
        try:
            lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        except OSError:
            continue
        for line_number, line in enumerate(lines, start=1):
            for token, pattern in patterns:
                if not re.search(pattern, line):
                    continue
                ref = known_site_ref(path_text, line)
                row = {
                    "runtime_evidence_id": evidence_id(family, path_text, line_number, token),
                    "family": family,
                    "source_path": path_text,
                    "line": line_number,
                    "token": token,
                    "evidence_kind": token,
                    "evidence": line.strip(),
                    "site_ref": ref,
                    "resolution": "Linked" if ref else "Pending",
                    "pending_reason": None if ref else pending_reason(family, token),
                }
                rows.append(row)
    return rows


def known_site_ref(path: str, line: str) -> str | None:
    if "hako_mem_free" in line or "extern.hako_mem.free" in line:
        return "runtime_backend.extern.hako_mem_free.success"
    return None


def pending_reason(family: str, token: str) -> str:
    if family == "provider_status" and token == "provider_missing_fallback":
        return "ProviderContractMissing"
    if family == "ffi_boundary":
        return "BoundaryKindPending"
    return "S1SiteReferenceMissing"


def build_manifest() -> dict[str, Any]:
    rows: list[dict[str, Any]] = []
    rows += scan(
        "value_conversion",
        ("src/backend", "src/mir"),
        (
            ("const_to_vm_void", r"ConstValue::(?:Null|Void).*VMValue::Void"),
            ("vm_void_conversion", r"VMValue::Void =>"),
        ),
        path_filter=re.compile(r"(?:vm_types|arithmetic|global_call_route_plan)"),
    )
    rows += scan(
        "weak_upgrade",
        ("src",),
        (("weak_to_strong", r"weak_to_strong"), ("weak_upgrade", r"\bupgrade\b")),
        path_filter=re.compile(r"weak|Weak|method_call_handlers"),
    )
    rows += scan(
        "null_like_box",
        ("src",),
        (("null_like_box", r"\b(?:MissingBox|NullBox|VoidBox)\b"),),
    )
    rows += scan(
        "provider_status",
        ("src",),
        (("provider_missing_fallback", r"unwrap_or\(Ok\(VMValue::Void\)\)"),),
        path_filter=re.compile(r"backend/.*externals\.rs$"),
    )
    rows += scan(
        "missing_result_synthesis",
        ("src/backend", "src/mir", "src/runner"),
        (
            ("wasm_i32_zero", r"i32\.const 0"),
            ("void_sentinel_i64_zero", r"void_sentinel_i64_zero"),
            ("null_or_void_projection", r"ConstValue::Null \| ConstValue::Void"),
        ),
    )
    rows += scan(
        "ffi_boundary",
        ("src", "crates/nyash_kernel/src", "docs/reference/abi"),
        (
            ("c_abi_function", r'extern "C"'),
            ("foreign_pointer", r"\bc_void\b"),
            ("nullable_boundary", r"\b(?:nullable|null pointer|NULL)\b"),
            ("status_boundary", r"\b(?:status|error code|result code)\b"),
        ),
        path_filter=re.compile(r"(?:ffi|exports|abi|extern)"),
    )
    rows.sort(key=lambda row: row["runtime_evidence_id"])
    return {
        "schema_version": 0,
        "status": "runtime_provider_evidence_inventory",
        "semantic_activation": 0,
        "source_manifests": [S1_GRAPH.relative_to(ROOT).as_posix(), S2_BINDING.relative_to(ROOT).as_posix()],
        "families": list(FAMILIES),
        "pending_reasons": sorted(PENDING_REASONS),
        "runtime_provider_evidence": rows,
    }


def validate(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if manifest.get("schema_version") != 0:
        errors.append("schema_version must be 0")
    if manifest.get("semantic_activation") != 0:
        errors.append("semantic activation must remain 0")
    if tuple(manifest.get("families", ())) != FAMILIES:
        errors.append("runtime/provider family vocabulary drift")
    graph = read_json(S1_GRAPH)
    binding = read_json(S2_BINDING)
    known_sites = {site["site_id"] for site in graph["semantic_sites"]}
    known_sites.update(site["site_id"] for site in binding["operation_outcome_sites"])
    rows = manifest.get("runtime_provider_evidence", [])
    ids = [row.get("runtime_evidence_id") for row in rows]
    if len(ids) != len(set(ids)):
        errors.append("duplicate runtime evidence id")
    seen_families = set()
    for row in rows:
        family = row.get("family")
        seen_families.add(family)
        if family not in FAMILIES:
            errors.append(f"unknown runtime evidence family: {family}")
        if not row.get("source_path") or not row.get("line") or not row.get("token"):
            errors.append(f"runtime evidence location incomplete: {row.get('runtime_evidence_id')}")
        if not row.get("evidence"):
            errors.append(f"runtime evidence text missing: {row.get('runtime_evidence_id')}")
        if row.get("resolution") not in RESOLUTIONS:
            errors.append(f"unknown runtime evidence resolution: {row.get('runtime_evidence_id')}")
        if row.get("resolution") == "Linked":
            if row.get("site_ref") not in known_sites:
                errors.append(f"runtime evidence site reference unknown: {row.get('runtime_evidence_id')}")
            if row.get("pending_reason") is not None:
                errors.append(f"linked evidence cannot be pending: {row.get('runtime_evidence_id')}")
        else:
            if row.get("pending_reason") not in PENDING_REASONS:
                errors.append(f"unknown pending reason: {row.get('runtime_evidence_id')}")
            if row.get("site_ref") is not None:
                errors.append(f"pending evidence has semantic site reference: {row.get('runtime_evidence_id')}")
        if family == "provider_status" and row.get("token") == "provider_missing_fallback":
            if row.get("resolution") != "Pending" or row.get("pending_reason") != "ProviderContractMissing":
                errors.append(f"provider missing fallback was classified: {row.get('runtime_evidence_id')}")
    missing = set(FAMILIES) - seen_families
    if missing:
        errors.append(f"runtime evidence families missing: {sorted(missing)}")
    return errors


def main() -> int:
    args = parse_args()
    expected = json.dumps(build_manifest(), ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[failure-outcome-runtime-provider] wrote {args.output}")
        return 0
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[failure-outcome-runtime-provider] drift detected")
        return 1
    errors = validate(json.loads(actual))
    if errors:
        for error in errors:
            print(f"[failure-outcome-runtime-provider] {error}")
        return 1
    rows = json.loads(actual)["runtime_provider_evidence"]
    counts = {family: sum(row["family"] == family for row in rows) for family in FAMILIES}
    print(f"[failure-outcome-runtime-provider] rows={len(rows)} counts={counts}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
