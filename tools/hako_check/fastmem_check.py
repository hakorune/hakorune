#!/usr/bin/env python3
"""Check FastMemory capability inventory reports.

This is a verifier adapter over fastmem inventory fields. It fails when a
contract/runtime report contains unclassified MemOps, forbidden operations,
escaping memory values, or Type ABI / Provider ABI hot-path crossings.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path
from typing import Any

from replacement_front_report import read_kv

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "tools" / "hako_check" / "fastmem_capability_inventory.py"

FAIL_FIELDS = [
    "fastmem_general_rawptr_type",
    "fastmem_general_deref_outside_region",
    "fastmem_general_pointer_arithmetic_outside_region",
    "fastmem_escape_count",
    "fastmem_metadata_ptr_escape_count",
    "fastmem_closure_capture_count",
    "fastmem_box_field_store_count",
    "fastmem_array_store_count",
    "fastmem_unverified_offset_load_count",
    "fastmem_contract_runtime_lookup_count",
    "fastmem_memop_unbalanced_region_count",
    "fastmem_memop_unclassified_count",
    "fastmem_forbidden_allocation_count",
    "fastmem_forbidden_safepoint_count",
    "fastmem_forbidden_await_count",
    "fastmem_forbidden_nowait_count",
    "fastmem_forbidden_call_count",
    "fastmem_type_abi_hot_lookup_count",
    "fastmem_provider_abi_crossing_count",
    "type_abi_hot_path_lookup_count",
    "provider_dispatch_hot_path",
    "page_map_bridge_type_abi_hot_lookup_count",
    "page_map_bridge_provider_abi_hot_dispatch_count",
]


def int_count(rows: dict[str, Any], key: str) -> int:
    value = rows.get(key, "0")
    try:
        return int(float(str(value)))
    except (TypeError, ValueError):
        return 0


def run_inventory(source_flag: str, source_path: Path) -> dict[str, str]:
    cmd = [sys.executable, str(INVENTORY), source_flag, str(source_path)]
    proc = subprocess.run(cmd, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        if proc.stderr:
            sys.stderr.write(proc.stderr)
        if proc.stdout:
            sys.stderr.write(proc.stdout)
        raise SystemExit(proc.returncode)
    rows: dict[str, str] = {}
    for raw_line in proc.stdout.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        rows[key.strip()] = value.strip()
    return rows


def failure_reasons(rows: dict[str, str]) -> list[str]:
    reasons: list[str] = []
    for key in FAIL_FIELDS:
        if int_count(rows, key) > 0:
            reasons.append(key)
    return reasons


def render(rows: dict[str, str], reasons: list[str]) -> str:
    status = "OK" if not reasons else "FAILED"
    lines = [
        f"FastMemory check: {status}",
        "",
        "Contract",
        "  output_contract=hako-check-fastmem-check-v0",
        f"  source_contract={rows.get('output_contract', 'unknown')}",
        f"  tool_surface={rows.get('tool_surface', 'unknown')}",
        "",
        "Regions",
        f"  fastmem regions: {rows.get('fastmem_region_count', '0')}",
        f"  fastmem contracts: {rows.get('fastmem_contract_count', '0')}",
        f"  unclassified memops: {rows.get('fastmem_memop_unclassified_count', '0')}",
        f"  unbalanced regions: {rows.get('fastmem_memop_unbalanced_region_count', '0')}",
        "",
        "Boundaries",
        f"  type ABI hot lookup: {rows.get('type_abi_hot_path_lookup_count', '0')}",
        f"  provider hot dispatch: {rows.get('provider_dispatch_hot_path', '0')}",
        f"  fastmem runtime contract lookup: {rows.get('fastmem_contract_runtime_lookup_count', '0')}",
        "",
        "Machine",
        f"  failure_count={len(reasons)}",
    ]
    for idx, reason in enumerate(reasons):
        lines.append(f"  failure_{idx}_reason={reason}")
    lines.append("  summary=ok" if not reasons else "  summary=failed")
    return "\n".join(lines) + "\n"


def emit_kv(rows: dict[str, str], reasons: list[str]) -> str:
    out = [
        "output_contract=hako-check-fastmem-check-v0",
        "input_kind=fastmem_inventory",
        "tool_surface=hako_check_fastmem_check",
        "observation_only=1",
        "rewrite_executed=0",
        "source_rewrite_executed=0",
        "benchmark_run_executed=0",
        "keeper_selection=0",
        f"source_contract={rows.get('output_contract', 'unknown')}",
        f"failure_count={len(reasons)}",
    ]
    for idx, reason in enumerate(reasons):
        out.append(f"failure_{idx}_reason={reason}")
    out.append("summary=ok" if not reasons else "summary=failed")
    return "\n".join(out) + "\n"


def write_output(text: str, out: Path | None) -> None:
    if out is None:
        print(text, end="")
        return
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--report", type=Path, help="Read a benchmark report via inventory.")
    source.add_argument("--inventory", type=Path, help="Read an existing fastmem inventory kv file.")
    source.add_argument("--ast-json", type=Path, help="Read Rust AST JSON via inventory.")
    source.add_argument("--program-json", type=Path, help="Read Program(JSON v0) via inventory.")
    parser.add_argument("--format", choices=("kv", "text"), default="text")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    if args.report:
        rows = run_inventory("--report", args.report)
    elif args.ast_json:
        rows = run_inventory("--ast-json", args.ast_json)
    elif args.program_json:
        rows = run_inventory("--program-json", args.program_json)
    else:
        rows = read_kv(args.inventory)
    reasons = failure_reasons(rows)
    text = emit_kv(rows, reasons) if args.format == "kv" else render(rows, reasons)
    write_output(text, args.out)
    return 1 if reasons else 0


if __name__ == "__main__":
    raise SystemExit(main())
