"""Pure metadata and report models for the read-only inspect surface."""

from __future__ import annotations

from pathlib import Path
from typing import Any


def root_list(data: dict[str, Any], key: str) -> list[dict[str, Any]]:
    value = data.get(key)
    if not isinstance(value, list):
        return []
    return [row for row in value if isinstance(row, dict)]


def function_metadata_rows(data: dict[str, Any], key: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for function in root_list(data, "functions"):
        metadata = function.get("metadata")
        if not isinstance(metadata, dict):
            continue
        values = metadata.get(key)
        if not isinstance(values, list):
            continue
        for row in values:
            if isinstance(row, dict):
                copied = dict(row)
                copied.setdefault("function", function.get("name", "unknown"))
                rows.append(copied)
    return rows


def function_metadata_object_rows(
    data: dict[str, Any], key: str
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for function in root_list(data, "functions"):
        metadata = function.get("metadata")
        if not isinstance(metadata, dict):
            continue
        value = metadata.get(key)
        if not isinstance(value, dict):
            continue
        copied = dict(value)
        copied.setdefault("function", function.get("name", "unknown"))
        rows.append(copied)
    return rows


def typed_object_route_rows(data: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for function in root_list(data, "functions"):
        metadata = function.get("metadata")
        if not isinstance(metadata, dict):
            continue
        values = metadata.get("route_decisions")
        if not isinstance(values, list):
            continue
        for row in values:
            if not isinstance(row, dict):
                continue
            if row.get("source_plan_kind") != "TypedObjectExactSlotRoute":
                continue
            copied = dict(row)
            copied.setdefault("function", function.get("name", "unknown"))
            rows.append(copied)
    return rows


def read_bundle_report(path: Path) -> dict[str, str]:
    if path.is_dir():
        path = path / "report.kv"
    if not path.is_file():
        raise SystemExit(f"missing report artifact: {path}")
    rows: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        rows[key.strip()] = value.strip()
    return rows


def route_counts(mir: dict[str, Any]) -> dict[str, int]:
    route_decisions = typed_object_route_rows(mir)
    array_text_state_routes = function_metadata_object_rows(
        mir, "array_text_state_residence_route"
    )
    array_text_sessions = function_metadata_rows(mir, "array_text_residence_sessions")
    array_text_observer_routes = function_metadata_rows(mir, "array_text_observer_routes")

    return {
        "typed_object_exact_route_decision_count": len(route_decisions),
        "array_text_state_residence_route_count": len(array_text_state_routes),
        "array_text_selected_route_count": sum(
            1
            for row in array_text_state_routes
            if str(row.get("selected_route", "")).startswith("hako.array_text.")
        ),
        "array_text_selected_bridge_symbol_count": sum(
            1
            for row in array_text_state_routes
            if str(row.get("selected_bridge_symbol", "")).startswith("hako.array_text.")
        ),
        "array_text_compat_string_indexof_hisi_count": sum(
            1
            for row in array_text_state_routes
            if str(row.get("fallback_route", "")).startswith(
                "nyash.array.string_indexof_"
            )
        ),
        "array_text_session_count": len(array_text_sessions),
        "array_text_session_begin_count": sum(
            1
            for row in array_text_sessions
            if str(row.get("begin_block", "")).isdigit()
            or str(row.get("begin_block", ""))
        ),
        "array_text_session_end_count": sum(
            1
            for row in array_text_sessions
            if str(row.get("end_block", "")).isdigit()
            or str(row.get("end_block", ""))
        ),
        "array_text_publication_in_selected_region_count": sum(
            1
            for row in array_text_sessions
            if str(row.get("publication_boundary", "none")).lower() != "none"
        ),
        "array_text_registry_carrier_in_selected_region_count": sum(
            1
            for row in array_text_sessions
            if "registry" in str(row.get("carrier", "")).lower()
        ),
        "array_text_silent_fallback_after_selected_route_count": sum(
            1
            for row in array_text_state_routes
            if str(row.get("fallback_policy", "")) != "fail_fast"
        ),
        "array_text_observer_route_count": len(array_text_observer_routes),
        "array_text_observer_selected_route_count": sum(
            1
            for row in array_text_observer_routes
            if str(row.get("selected_route", "")).startswith("hako.array_text.")
        ),
        "array_text_observer_selected_bridge_symbol_count": sum(
            1
            for row in array_text_observer_routes
            if str(row.get("selected_bridge_symbol", "")).startswith("hako.array_text.")
        ),
    }


def selected_route_rows(mir: dict[str, Any]) -> list[dict[str, Any]]:
    rows = typed_object_route_rows(mir)
    rows.extend(function_metadata_object_rows(mir, "array_text_state_residence_route"))
    rows.extend(function_metadata_rows(mir, "array_text_residence_sessions"))
    rows.extend(function_metadata_rows(mir, "array_text_observer_routes"))
    return rows


def manifest_contract(
    selector_kind: str,
    source_file: Path,
    source_hash: str,
    region_id: str,
    function_name: str,
    backend: str,
    emit_mir: bool,
    emit_mir_json: bool,
    emit_llvm: bool,
    emit_asm: bool,
    source_to_mir_mapping: str,
    mir_to_llvm_mapping: str,
    llvm_to_asm_mapping: str,
    summary: str,
) -> dict[str, Any]:
    return {
        "output_contract": "hako-inspect-scope-bundle-v0",
        "tool_surface": "hako_check_inspect_scope",
        "observation_only": True,
        "rewrite_executed": False,
        "keeper_selection": False,
        "source_file": str(source_file),
        "source_hash": f"sha256:{source_hash}",
        "selector_kind": selector_kind,
        "region_id": region_id,
        "function": function_name,
        "backend": backend,
        "emit_mir": emit_mir,
        "emit_mir_json": emit_mir_json,
        "emit_llvm": emit_llvm,
        "emit_asm": emit_asm,
        "source_to_mir_mapping": source_to_mir_mapping,
        "mir_to_llvm_mapping": mir_to_llvm_mapping,
        "llvm_to_asm_mapping": llvm_to_asm_mapping,
        "summary": summary,
    }


def format_report(rows: list[tuple[str, Any]]) -> str:
    return "\n".join(f"{key}={value}" for key, value in rows) + "\n"


def bundle_report_rows(
    selector_kind: str,
    source_file: Path,
    source_hash: str,
    region_id: str,
    function_name: str,
    backend: str,
    emit_mir: bool,
    emit_mir_json: bool,
    emit_llvm: bool,
    emit_asm: bool,
    source_to_mir_mapping: str,
    mir_to_llvm_mapping: str,
    llvm_to_asm_mapping: str,
    route_count_rows: dict[str, int],
    summary: str,
) -> list[tuple[str, Any]]:
    rows: list[tuple[str, Any]] = [
        ("output_contract", "hako-check-inspect-scope-v0"),
        ("tool_surface", "hako_check_inspect_scope"),
        ("observation_only", "1"),
        ("rewrite_executed", "0"),
        ("keeper_selection", "0"),
        ("source_file", str(source_file)),
        ("source_hash", f"sha256:{source_hash}"),
        ("selector_kind", selector_kind),
        ("region_id", region_id),
        ("function", function_name),
        ("backend", backend),
        ("emit_mir", "1" if emit_mir else "0"),
        ("emit_mir_json", "1" if emit_mir_json else "0"),
        ("emit_llvm", "1" if emit_llvm else "0"),
        ("emit_asm", "1" if emit_asm else "0"),
        ("source_to_mir_mapping", source_to_mir_mapping),
        ("mir_to_llvm_mapping", mir_to_llvm_mapping),
        ("llvm_to_asm_mapping", llvm_to_asm_mapping),
    ]
    rows.extend((key, str(value)) for key, value in route_count_rows.items())
    rows.append(("summary", summary))
    return rows
