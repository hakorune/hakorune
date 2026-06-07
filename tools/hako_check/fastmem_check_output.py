#!/usr/bin/env python3
"""Rendering helpers for FastMemory verifier checks."""

from __future__ import annotations


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
