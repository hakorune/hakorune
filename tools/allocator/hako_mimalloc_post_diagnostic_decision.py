#!/usr/bin/env python3
"""Decide the next row after hako mimalloc owner diagnostics."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"{label}: missing {key}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    values = read_kv(args.input)
    require(values, "output_contract", "hako-mimalloc-owner-narrow-diagnostic-v0", "input")
    require(values, "winner_claim", "0", "input")
    require(values, "provider_active", "0", "input")
    require(values, "replacement_active", "0", "input")
    require(values, "hook_installed", "0", "input")
    require(values, "global_allocator", "0", "input")
    require(values, "summary", "ok", "input")

    owner = require_key(values, "gap_owner", "input")
    diagnostic = require_key(values, "diagnostic_kind", "input")
    next_optimization_allowed = require_key(values, "next_optimization_allowed", "input")
    if next_optimization_allowed not in {"0", "1"}:
        raise SystemExit("input: next_optimization_allowed must be 0 or 1")

    if next_optimization_allowed == "1" and owner in {"compiler_lowering", "allocator_algorithm"}:
        decision = "enter_first_keeper_optimization"
        selected_next_row = "HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001"
    elif diagnostic == "measurement_hygiene_refresh":
        decision = "refresh_gap_taxonomy_after_hygiene"
        selected_next_row = "HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001"
    else:
        decision = "select_additional_owner_diagnostic"
        selected_next_row = "HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001"

    lines = [
        "output_contract=hako-mimalloc-post-diagnostic-decision-v0",
        "input_contract=hako-mimalloc-owner-narrow-diagnostic-v0",
        f"front={require_key(values, 'front', 'input')}",
        f"workload_id={require_key(values, 'workload_id', 'input')}",
        f"gap_owner={owner}",
        f"diagnostic_kind={diagnostic}",
        f"next_optimization_allowed={next_optimization_allowed}",
        f"decision={decision}",
        f"selected_next_row={selected_next_row}",
        "optimization_started=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
