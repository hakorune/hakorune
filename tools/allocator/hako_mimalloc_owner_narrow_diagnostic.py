#!/usr/bin/env python3
"""Emit the selected hako mimalloc owner diagnostic without optimizing."""

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
    parser.add_argument("--selection", type=Path, required=True)
    parser.add_argument("--measurement-report", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    selection = read_kv(args.selection)
    require(selection, "output_contract", "hako-mimalloc-conditional-diagnostic-selection-v0", "selection")
    require(selection, "winner_claim", "0", "selection")
    require(selection, "provider_active", "0", "selection")
    require(selection, "replacement_active", "0", "selection")
    require(selection, "hook_installed", "0", "selection")
    require(selection, "global_allocator", "0", "selection")
    require(selection, "body_elapsed_ns_primary", "0", "selection")
    require(selection, "summary", "ok", "selection")

    diagnostic = require_key(selection, "selected_diagnostic", "selection")
    hygiene_required = require_key(selection, "measurement_hygiene_required", "selection")
    if hygiene_required not in {"0", "1"}:
        raise SystemExit("selection: measurement_hygiene_required must be 0 or 1")

    sample_count = "0"
    build_compile_excluded = "0"
    body_elapsed_secondary = "0"
    measurement_contract = "none"
    if args.measurement_report is not None:
        measurement = read_kv(args.measurement_report)
        require(measurement, "output_contract", "mimalloc-comparison-repeated-measurement-v0", "measurement")
        require(measurement, "winner_claim", "0", "measurement")
        require(measurement, "provider_activation", "0", "measurement")
        require(measurement, "host_replacement", "0", "measurement")
        require(measurement, "hook_installed", "0", "measurement")
        require(measurement, "global_allocator_installed", "0", "measurement")
        require(measurement, "summary", "ok", "measurement")
        sample_count = require_key(measurement, "sample_count", "measurement")
        measurement_contract = "mimalloc-comparison-repeated-measurement-v0"
        build_compile_excluded = "1"
        body_elapsed_secondary = "1"

    if diagnostic == "measurement_hygiene_refresh":
        if args.measurement_report is None:
            raise SystemExit("measurement_hygiene_refresh requires --measurement-report")
        if sample_count not in {"5", "7"}:
            raise SystemExit("measurement_hygiene_refresh requires sample_count 5 or 7")
    elif args.measurement_report is not None:
        raise SystemExit("--measurement-report is only accepted for measurement_hygiene_refresh")

    next_optimization_allowed = "0"
    if diagnostic != "measurement_hygiene_refresh":
        next_optimization_allowed = require_key(selection, "next_optimization_allowed", "selection")

    lines = [
        "output_contract=hako-mimalloc-owner-narrow-diagnostic-v0",
        "input_contract=hako-mimalloc-conditional-diagnostic-selection-v0",
        f"front={require_key(selection, 'workload_id', 'selection')}",
        f"workload_id={require_key(selection, 'workload_id', 'selection')}",
        f"measurement_profile={require_key(selection, 'measurement_profile', 'selection')}",
        f"gap_owner={require_key(selection, 'gap_owner', 'selection')}",
        f"diagnostic_kind={diagnostic}",
        f"measurement_contract={measurement_contract}",
        f"measurement_hygiene_required={hygiene_required}",
        f"body_elapsed_ns_secondary={body_elapsed_secondary}",
        f"build_compile_excluded={build_compile_excluded}",
        f"sample_count={sample_count}",
        f"next_optimization_allowed={next_optimization_allowed}",
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
