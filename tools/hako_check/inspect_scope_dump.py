#!/usr/bin/env python3
"""Read-only scope/route/mark inspect bundles for hako_check.

This tool is an artifact/query surface, not an optimizer. It consumes source
files, optional MIR JSON artifacts, and optional route metadata to produce
stable inspect bundles for humans and automation.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
EMIT_ROUTE = ROOT / "tools" / "smokes" / "v2" / "lib" / "emit_mir_route.sh"
TRACE_BUNDLE = ROOT / "tools" / "perf" / "trace_optimization_bundle.sh"
NYASH_BIN = Path(os.environ.get("HAKORUNE_BIN") or os.environ.get("NYASH_BIN") or ROOT / "target" / "release" / "hakorune")


def load_json(path: Path) -> dict[str, Any]:
    if not path.is_file():
        raise SystemExit(f"missing JSON file: {path}")
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise SystemExit(f"JSON root must be an object: {path}")
    return data


def write_json(path: Path, data: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


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


def function_metadata_object_rows(data: dict[str, Any], key: str) -> list[dict[str, Any]]:
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


def bool_text(value: bool) -> str:
    return "1" if value else "0"


def parse_span(value: str) -> tuple[Path, int, int]:
    parts = value.rsplit(":", 2)
    if len(parts) != 3:
        raise SystemExit(f"invalid span selector (expected path:start:end): {value}")
    path_str, start_str, end_str = parts
    try:
        start = int(start_str)
        end = int(end_str)
    except ValueError as exc:
        raise SystemExit(f"invalid span selector line numbers: {value}") from exc
    if start < 1 or end < start:
        raise SystemExit(f"invalid span selector range: {value}")
    return Path(path_str), start, end


def source_lines(path: Path) -> list[str]:
    if not path.is_file():
        raise SystemExit(f"source file not found: {path}")
    return path.read_text(encoding="utf-8", errors="replace").splitlines()


def source_slice(lines: list[str], start: int, end: int) -> str:
    return "\n".join(lines[start - 1 : end]) + "\n"


def find_anchor_region(lines: list[str], region_id: str) -> tuple[int, int]:
    begin = f"// hako:inspect begin {region_id}"
    end = f"// hako:inspect end {region_id}"
    begin_line = -1
    end_line = -1
    for idx, line in enumerate(lines, start=1):
        if begin_line < 0 and line.strip() == begin:
            begin_line = idx
            continue
        if begin_line > 0 and line.strip() == end:
            end_line = idx
            break
    if begin_line < 0 or end_line < 0:
        raise SystemExit(f"inspect anchor not found: {region_id}")
    return begin_line, end_line


def emit_mir_json(source_file: Path, timeout_secs: int) -> tuple[Path, str]:
    if not source_file.is_file():
        raise SystemExit(f"source file not found: {source_file}")
    tmp_dir = Path(tempfile.mkdtemp(prefix="hako_inspect.", dir=os.environ.get("TMPDIR", "/tmp")))
    mir_json = tmp_dir / "mir.json"
    cmd = [
        "bash",
        str(EMIT_ROUTE),
        "--route",
        "direct",
        "--out",
        str(mir_json),
        "--input",
        str(source_file),
        "--timeout-secs",
        str(timeout_secs),
    ]
    env = os.environ.copy()
    env["NYASH_DISABLE_PLUGINS"] = "1"
    env.setdefault("NYASH_VM_USE_FALLBACK", "0")
    env.setdefault("NYASH_VM_HAKO_PREFER_STRICT_DEV", "0")
    result = subprocess.run(cmd, env=env, check=False, capture_output=True, text=True)
    if result.returncode != 0:
        raise SystemExit(
            "emit_mir_route failed (rc=%s)\n%s%s"
            % (
                result.returncode,
                result.stdout,
                result.stderr,
            )
        )
    if not mir_json.is_file():
        raise SystemExit(f"emit_mir_route did not produce MIR JSON: {mir_json}")
    return mir_json, result.stdout + result.stderr


def emit_llvm_asm_bundle(
    mir_json_path: Path,
    function_name: str,
    timeout_secs: int,
) -> tuple[Path, str]:
    if not mir_json_path.is_file():
        raise SystemExit(f"missing MIR JSON artifact for backend emit: {mir_json_path}")
    tmp_dir = Path(tempfile.mkdtemp(prefix="hako_inspect.trace.", dir=os.environ.get("TMPDIR", "/tmp")))
    cmd = [
        "bash",
        str(TRACE_BUNDLE),
        "--mir-json",
        str(mir_json_path),
        "--microasm-runs",
        "0",
        "--out-dir",
        str(tmp_dir),
    ]
    if function_name:
        cmd.extend(["--function", function_name])
    env = os.environ.copy()
    env.setdefault("NYASH_LLVM_ROUTE_TRACE", "1")
    env.setdefault("NYASH_LLVM_SKIP_BUILD", "1")
    result = subprocess.run(
        cmd,
        env=env,
        check=False,
        capture_output=True,
        text=True,
        timeout=max(180, timeout_secs * 8 if timeout_secs > 0 else 180),
    )
    if result.returncode != 0:
        raise SystemExit(
            "trace_optimization_bundle failed (rc=%s)\n%s%s"
            % (
                result.returncode,
                result.stdout,
                result.stderr,
            )
        )
    ll_dump = tmp_dir / "lowered.ll"
    objdump = tmp_dir / "objdump.txt"
    if not ll_dump.is_file():
        raise SystemExit(f"trace bundle did not produce LLVM IR: {ll_dump}")
    if not objdump.is_file():
        exe_path = tmp_dir / "bundle.exe"
        if not exe_path.is_file():
            raise SystemExit(f"trace bundle did not produce executable for assembly dump: {exe_path}")
        objdump_cmd = ["objdump", "-d", "--demangle", str(exe_path)]
        objdump_result = subprocess.run(
            objdump_cmd,
            check=False,
            capture_output=True,
            text=True,
        )
        if objdump_result.returncode != 0:
            raise SystemExit(
                "objdump failed (rc=%s)\n%s%s"
                % (
                    objdump_result.returncode,
                    objdump_result.stdout,
                    objdump_result.stderr,
                )
            )
        objdump.write_text(objdump_result.stdout, encoding="utf-8")
    return tmp_dir, result.stdout + result.stderr


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
            if str(row.get("fallback_route", "")).startswith("nyash.array.string_indexof_")
        ),
        "array_text_session_count": len(array_text_sessions),
        "array_text_session_begin_count": sum(
            1
            for row in array_text_sessions
            if str(row.get("begin_block", "")).isdigit() or str(row.get("begin_block", ""))
        ),
        "array_text_session_end_count": sum(
            1
            for row in array_text_sessions
            if str(row.get("end_block", "")).isdigit() or str(row.get("end_block", ""))
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


def resolve_objdump_symbol(objdump_text: str, function_name: str) -> tuple[str, int | None]:
    import re

    preferred = [
        function_name,
        function_name.replace("/", "_"),
        "ny_main",
        "main",
    ]
    lines = objdump_text.splitlines()
    label_pattern = re.compile(r"^\s*[0-9a-fA-F]+\s+<([^>]+)>:\s*$")
    indexed_labels: list[tuple[str, int]] = []
    for idx, line in enumerate(lines, start=1):
        m = label_pattern.match(line)
        if m:
            indexed_labels.append((m.group(1), idx))
    if not indexed_labels:
        return ("unknown", None)
    label_map = {label: line_no for label, line_no in indexed_labels}
    for candidate in preferred:
        if candidate and candidate in label_map:
            return candidate, label_map[candidate]
    return indexed_labels[0]


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
    return "\n".join(f"{k}={v}" for k, v in rows) + "\n"


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


@dataclass
class ScopeSelector:
    kind: str
    source_file: Path
    region_id: str
    start_line: int
    end_line: int


def resolve_scope_selector(
    source_file: Path | None,
    span: str | None,
    region: str | None,
    function_name: str | None,
) -> ScopeSelector:
    if span:
        span_path, start_line, end_line = parse_span(span)
        source_file = source_file or span_path
        region_id = region or f"{span_path.stem}_{start_line}_{end_line}"
        return ScopeSelector("span", source_file, region_id, start_line, end_line)
    if region:
        if source_file is None:
            raise SystemExit("--source-file is required when using --region")
        lines = source_lines(source_file)
        start_line, end_line = find_anchor_region(lines, region)
        return ScopeSelector("comment_anchor", source_file, region, start_line, end_line)
    if function_name:
        if source_file is None:
            raise SystemExit("--source-file is required when using --function")
        lines = source_lines(source_file)
        start_line = 1
        end_line = len(lines)
        return ScopeSelector("function", source_file, function_name, start_line, end_line)
    raise SystemExit("choose one selector: --span, --region, or --function")


def determine_artifacts(emits: Iterable[str]) -> tuple[bool, bool, bool, bool]:
    if isinstance(emits, str):
        emit_iterable = emits.split(",")
    else:
        emit_iterable = emits
    emit_set = {item.strip() for item in emit_iterable if item.strip()}
    if not emit_set:
        emit_set = {"mir", "mir-json", "report"}
    return (
        "mir" in emit_set,
        "mir-json" in emit_set,
        "llvm" in emit_set,
        "asm" in emit_set,
    )


def bundle_scope(args: argparse.Namespace) -> int:
    selector = resolve_scope_selector(args.source_file, args.span, args.region, args.function)
    lines = source_lines(selector.source_file)
    selected_source = source_slice(lines, selector.start_line, selector.end_line)
    source_hash = sha256_file(selector.source_file)

    out_dir = args.out or (ROOT / "target" / "hako-inspect" / selector.region_id)
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    mir_json_path: Path | None = args.mir_json
    mir_json_text = ""
    emit_log = ""
    if mir_json_path is None:
        if args.app is None:
            args.app = selector.source_file
        mir_json_path, emit_log = emit_mir_json(Path(args.app), args.emit_timeout_secs)
    mir = load_json(mir_json_path)
    mir_json_text = json.dumps(mir, indent=2, sort_keys=True) + "\n"

    route_rows = route_counts(mir)
    _emit_mir_requested, _emit_mir_json_requested, emit_llvm_requested, emit_asm_requested = determine_artifacts(args.emit)
    emit_mir_actual = True
    emit_mir_json_actual = True
    emit_llvm_actual = emit_llvm_requested
    emit_asm_actual = emit_asm_requested
    backend_trace_log = ""
    backend_trace_dir: Path | None = None
    llvm_ir_text = ""
    asm_text = ""
    asm_map: dict[str, Any] = {}
    if emit_llvm_requested or emit_asm_requested:
        try:
            backend_trace_dir, backend_trace_log = emit_llvm_asm_bundle(
                mir_json_path,
                args.function or "",
                args.emit_timeout_secs,
            )
            llvm_ir_path = backend_trace_dir / "lowered.ll"
            asm_path = backend_trace_dir / "objdump.txt"
            llvm_ir_text = llvm_ir_path.read_text(encoding="utf-8", errors="replace")
            asm_text = asm_path.read_text(encoding="utf-8", errors="replace")
            emit_llvm_actual = True
            emit_asm_actual = True
            symbol_name, symbol_line = resolve_objdump_symbol(asm_text, args.function or "")
            asm_map = {
                "output_contract": "hako-inspect-asm-map-v0",
                "tool_surface": "hako_check_inspect_scope",
                "source_file": str(selector.source_file),
                "region_id": selector.region_id,
                "function": args.function or "",
                "function_symbol": symbol_name,
                "mapping_quality": "symbol",
                "asm_file": "asm.s",
                "symbol_line": symbol_line,
                "notes": [
                    "assembly mapping is symbol-level evidence from trace bundle output",
                ],
            }
        except SystemExit:
            if not args.allow_unavailable_artifacts:
                raise
            emit_llvm_actual = False
            emit_asm_actual = False
            backend_trace_log = ""
            backend_trace_dir = None
            llvm_ir_text = ""
            asm_text = ""
            asm_map = {}

    manifest = manifest_contract(
        selector.kind,
        selector.source_file,
        source_hash,
        selector.region_id,
        args.function or "",
        args.backend,
        emit_mir_actual,
        emit_mir_json_actual,
        emit_llvm_actual,
        emit_asm_actual,
        "exact" if mir_json_path is not None else "missing",
        "block" if emit_llvm_actual else "missing",
        "symbol" if emit_asm_actual else "missing",
        "ok",
    )
    manifest["selector"] = {
        "kind": selector.kind,
        "start_line": selector.start_line,
        "end_line": selector.end_line,
    }
    if emit_log:
        manifest["emit_log"] = emit_log
    if backend_trace_log:
        manifest["backend_trace_log"] = backend_trace_log
    if backend_trace_dir is not None:
        manifest["backend_trace_dir"] = str(backend_trace_dir)

    report_rows = bundle_report_rows(
        selector.kind,
        selector.source_file,
        source_hash,
        selector.region_id,
        args.function or "",
        args.backend,
        emit_mir_actual,
        emit_mir_json_actual,
        emit_llvm_actual,
        emit_asm_actual,
        "exact" if mir_json_path is not None else "missing",
        "block" if emit_llvm_actual else "missing",
        "symbol" if emit_asm_actual else "missing",
        route_rows,
        "ok",
    )
    report_text = format_report(report_rows)
    summary_lines = [
        f"# Inspect: {selector.region_id}",
        f"- source: {selector.source_file}:{selector.start_line}-{selector.end_line}",
        f"- function: {args.function or 'unknown'}",
        f"- selector: {selector.kind}",
        f"- MIR JSON: {mir_json_path}",
        f"- selected routes: {route_rows['array_text_selected_route_count']}",
        f"- compat helper calls: {route_rows['array_text_compat_string_indexof_hisi_count']}",
        f"- mapping: source->MIR exact, MIR->LLVM {'block' if emit_llvm_actual else 'missing'}, LLVM->ASM {'symbol' if emit_asm_actual else 'missing'}",
    ]

    source_slice_path = out_dir / "source.slice.hako"
    source_map_path = out_dir / "source.map.json"
    mir_raw_path = out_dir / "mir.raw.json"
    mir_raw_txt_path = out_dir / "mir.raw.txt"
    mir_planned_path = out_dir / "mir.planned.json"
    mir_planned_txt_path = out_dir / "mir.planned.txt"
    route_decisions_path = out_dir / "route_decisions.json"
    verifier_path = out_dir / "verifier.json"
    report_path = out_dir / "report.kv"
    summary_path = out_dir / "summary.md"
    manifest_path = out_dir / "manifest.json"
    llvm_ir_path = out_dir / "llvm.ir"
    asm_path = out_dir / "asm.s"
    asm_map_path = out_dir / "asm.map.json"

    write_text(source_slice_path, selected_source)
    write_json(
        source_map_path,
        {
            "source_file": str(selector.source_file),
            "selector_kind": selector.kind,
            "start_line": selector.start_line,
            "end_line": selector.end_line,
            "source_hash": f"sha256:{source_hash}",
        },
    )
    write_json(mir_raw_path, mir)
    write_text(mir_raw_txt_path, mir_json_text)
    write_json(mir_planned_path, mir)
    write_text(mir_planned_txt_path, mir_json_text)
    write_json(route_decisions_path, selected_route_rows(mir))
    write_json(
        verifier_path,
        {
            "output_contract": "hako-inspect-scope-bundle-v0",
            "summary": "ok",
            "source_hash": f"sha256:{source_hash}",
        },
    )
    if emit_llvm_actual:
        write_text(llvm_ir_path, llvm_ir_text)
    if emit_asm_actual:
        write_text(asm_path, asm_text)
        write_json(asm_map_path, asm_map)
    write_text(report_path, report_text)
    write_json(manifest_path, manifest)
    write_text(summary_path, "\n".join(summary_lines) + "\n")

    print(f"inspect scope: {out_dir}")
    print(report_text, end="")
    return 0


def bundle_route(args: argparse.Namespace) -> int:
    if args.mir_json is None and args.app is None:
        raise SystemExit("choose one selector: --mir-json or --app")
    if args.mir_json is None:
        if args.app is None:
            raise SystemExit("--app required when --mir-json is absent")
        mir_json, _emit_log = emit_mir_json(Path(args.app), args.emit_timeout_secs)
    else:
        mir_json = args.mir_json
    mir = load_json(mir_json)
    out_dir = args.out or (ROOT / "target" / "hako-inspect" / "route")
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    route_rows = route_counts(mir)
    selected_rows = selected_route_rows(mir)
    selected_route = args.selected_route or ""
    selected_rows = [
        row
        for row in selected_rows
        if selected_route == ""
        or str(row.get("selected_route", "")) == selected_route
        or str(row.get("fallback_route", "")) == selected_route
    ]
    report_rows = [
        ("output_contract", "hako-check-inspect-route-v0"),
        ("tool_surface", "hako_check_inspect_route"),
        ("observation_only", "1"),
        ("rewrite_executed", "0"),
        ("keeper_selection", "0"),
        ("mir_json", str(mir_json)),
        ("selected_route_filter", selected_route or "all"),
        ("route_row_count", str(len(selected_rows))),
        ("typed_object_exact_route_decision_count", str(route_rows["typed_object_exact_route_decision_count"])),
        ("array_text_state_residence_route_count", str(route_rows["array_text_state_residence_route_count"])),
        ("array_text_observer_route_count", str(route_rows["array_text_observer_route_count"])),
        ("summary", "ok"),
    ]
    for idx, row in enumerate(selected_rows[: max(0, args.topn)]):
        prefix = f"route_{idx}"
        report_rows.extend(
            [
                (f"{prefix}_function", row.get("function", "unknown")),
                (f"{prefix}_selected_route", row.get("selected_route", row.get("route", "unknown"))),
                (f"{prefix}_selected_bridge_symbol", row.get("selected_bridge_symbol", "unknown")),
                (f"{prefix}_fallback_route", row.get("fallback_route", "unknown")),
                (f"{prefix}_fallback_policy", row.get("fallback_policy", "unknown")),
            ]
        )
    report_text = format_report(report_rows)
    write_text(out_dir / "report.kv", report_text)
    write_json(out_dir / "route_rows.json", selected_rows)
    write_json(out_dir / "manifest.json", {"output_contract": "hako-check-inspect-route-v0", "summary": "ok"})
    print(report_text, end="")
    return 0


def bundle_mark(args: argparse.Namespace) -> int:
    if args.source_file is None:
        raise SystemExit("--source-file is required for mark inspection")
    lines = source_lines(args.source_file)
    needle = f'__mir__.mark("{args.label}")'
    line_no = -1
    for idx, line in enumerate(lines, start=1):
        if needle in line.replace(" ", "") or needle in line:
            line_no = idx
            break
    if line_no < 0:
        raise SystemExit(f"mark label not found: {args.label}")
    window = max(0, args.window)
    start = max(1, line_no - window)
    end = min(len(lines), line_no + window)
    region_id = args.label.replace(" ", "_")
    out_dir = args.out or (ROOT / "target" / "hako-inspect" / region_id)
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    snippet = source_slice(lines, start, end)
    write_text(out_dir / "source.slice.hako", snippet)
    write_json(
        out_dir / "manifest.json",
        {
            "output_contract": "hako-check-inspect-mark-v0",
            "tool_surface": "hako_check_inspect_mark",
            "observation_only": True,
            "rewrite_executed": False,
            "keeper_selection": False,
            "source_file": str(args.source_file),
            "label": args.label,
            "line": line_no,
            "window": window,
            "summary": "ok",
        },
    )
    write_text(
        out_dir / "report.kv",
        format_report(
            [
                ("output_contract", "hako-check-inspect-mark-v0"),
                ("tool_surface", "hako_check_inspect_mark"),
                ("observation_only", "1"),
                ("rewrite_executed", "0"),
                ("keeper_selection", "0"),
                ("source_file", str(args.source_file)),
                ("label", args.label),
                ("line", str(line_no)),
                ("window", str(window)),
                ("summary", "ok"),
            ]
        ),
    )
    print(f"inspect mark: {out_dir}")
    return 0


def bundle_diff(args: argparse.Namespace) -> int:
    before = read_bundle_report(args.before)
    after = read_bundle_report(args.after)
    all_keys = sorted(set(before) | set(after))
    changed = [
        (key, before.get(key, ""), after.get(key, ""))
        for key in all_keys
        if before.get(key, "") != after.get(key, "")
    ]
    out_dir = args.out or (ROOT / "target" / "hako-inspect" / "diff")
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    report_lines = [
        "output_contract=hako-check-inspect-diff-v0",
        "tool_surface=hako_check_inspect_diff",
        "observation_only=1",
        "rewrite_executed=0",
        "keeper_selection=0",
        f"before={args.before}",
        f"after={args.after}",
        f"changed_count={len(changed)}",
        "summary=ok",
    ]
    write_text(out_dir / "report.kv", "\n".join(report_lines) + "\n")
    write_json(
        out_dir / "diff.json",
        {
            "output_contract": "hako-check-inspect-diff-v0",
            "before": str(args.before),
            "after": str(args.after),
            "changed": [
                {"key": key, "before": before_value, "after": after_value}
                for key, before_value, after_value in changed
            ],
        },
    )
    write_text(
        out_dir / "summary.md",
        "\n".join(
            [
                "# Inspect Diff",
                f"- before: {args.before}",
                f"- after: {args.after}",
                f"- changed: {len(changed)}",
            ]
        )
        + "\n",
    )
    print("\n".join(report_lines))
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    scope = sub.add_parser("scope", help="Inspect a source span / comment anchor")
    scope.add_argument("--source-file", type=Path, help="Source file to inspect")
    scope.add_argument("--span", help="Span selector PATH:START:END")
    scope.add_argument("--region", help="Comment anchor region id")
    scope.add_argument("--function", help="Function name label for the bundle")
    scope.add_argument("--app", type=Path, help="App/source path used when emitting MIR JSON")
    scope.add_argument("--mir-json", type=Path, help="Existing MIR JSON artifact to inspect")
    scope.add_argument("--emit", default="mir,mir-json,report", help="Comma separated artifacts to emit")
    scope.add_argument("--out", type=Path, help="Output directory")
    scope.add_argument("--backend", default="mir", help="Backend label for manifests")
    scope.add_argument("--emit-timeout-secs", type=int, default=20, help="Timeout for MIR emission")
    scope.add_argument(
        "--allow-unavailable-artifacts",
        action="store_true",
        help="Do not fail when llvm/asm artifacts are unavailable yet",
    )
    scope.set_defaults(func=bundle_scope)

    route = sub.add_parser("route", help="Inspect route metadata rows")
    route.add_argument("--app", type=Path, help="App/source path used when emitting MIR JSON")
    route.add_argument("--mir-json", type=Path, help="Existing MIR JSON artifact to inspect")
    route.add_argument("--selected-route", help="Filter by selected route")
    route.add_argument("--topn", type=int, default=8)
    route.add_argument("--out", type=Path)
    route.add_argument("--emit-timeout-secs", type=int, default=20)
    route.set_defaults(func=bundle_route)

    mark = sub.add_parser("mark", help="Inspect a source anchor around __mir__.mark")
    mark.add_argument("--source-file", type=Path, required=True)
    mark.add_argument("--label", required=True)
    mark.add_argument("--window", type=int, default=12)
    mark.add_argument("--out", type=Path)
    mark.set_defaults(func=bundle_mark)

    diff = sub.add_parser("diff", help="Compare two inspect bundle reports")
    diff.add_argument("--before", type=Path, required=True)
    diff.add_argument("--after", type=Path, required=True)
    diff.add_argument("--out", type=Path)
    diff.set_defaults(func=bundle_diff)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
