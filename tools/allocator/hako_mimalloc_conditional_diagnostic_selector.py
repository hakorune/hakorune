#!/usr/bin/env python3
"""Select the next hako mimalloc diagnostic from gap taxonomy evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


DIAGNOSTIC_BY_OWNER = {
    "benchmark_harness": "measurement_hygiene_refresh",
    "hako_runtime_baseline": "empty_workload_or_repeat_scaling_runtime_diagnostic",
    "compiler_lowering": "mir_or_body_shape_diagnostic",
    "allocator_algorithm": "operation_repeat_scaling_or_allocator_counter_diagnostic",
    "c_abi_memory_bridge": "c_runner_api_or_load_boundary_diagnostic",
    "provider_wrapper": "provider_explicit_call_overhead_diagnostic",
}


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


def select_diagnostic(owner: str, quality: str, suggested: str) -> tuple[str, str, str]:
    if owner not in DIAGNOSTIC_BY_OWNER:
        raise SystemExit(f"input: unsupported gap_owner {owner!r}")
    selected = "measurement_hygiene_refresh" if quality == "noisy" else DIAGNOSTIC_BY_OWNER[owner]
    if owner == "benchmark_harness":
        selected = "measurement_hygiene_refresh"
    if suggested and suggested != selected:
        return selected, "0", f"suggested_mismatch:{suggested}"
    return selected, "1", "accepted"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    values = read_kv(args.input)
    require(values, "output_contract", "hako-mimalloc-gap-taxonomy-v0", "input")
    require(values, "winner_claim", "0", "input")
    require(values, "provider_active", "0", "input")
    require(values, "replacement_active", "0", "input")
    require(values, "hook_installed", "0", "input")
    require(values, "global_allocator", "0", "input")
    require(values, "summary", "ok", "input")

    owner = require_key(values, "gap_owner", "input")
    quality = require_key(values, "evidence_quality", "input")
    confidence = require_key(values, "gap_confidence", "input")
    if quality not in {"stable", "noisy"}:
        raise SystemExit("input: evidence_quality must be stable or noisy")
    if confidence not in {"low", "medium", "high"}:
        raise SystemExit("input: gap_confidence must be low, medium, or high")

    selected, suggestion_match, selection_reason = select_diagnostic(
        owner,
        quality,
        require_key(values, "next_diagnostic", "input"),
    )
    measurement_hygiene_required = "1" if selected == "measurement_hygiene_refresh" else "0"
    optimization_allowed = "0"
    if quality == "stable" and confidence != "low" and owner in {"compiler_lowering", "allocator_algorithm"}:
        optimization_allowed = require_key(values, "next_optimization_allowed", "input")

    lines = [
        "output_contract=hako-mimalloc-conditional-diagnostic-selection-v0",
        "input_contract=hako-mimalloc-gap-taxonomy-v0",
        f"workload_id={require_key(values, 'workload_id', 'input')}",
        f"measurement_profile={require_key(values, 'measurement_profile', 'input')}",
        f"gap_owner={owner}",
        f"evidence_quality={quality}",
        f"gap_confidence={confidence}",
        f"outlier_observed={require_key(values, 'outlier_observed', 'input')}",
        f"selected_diagnostic={selected}",
        f"next_diagnostic={selected}",
        f"next_diagnostic_suggestion_match={suggestion_match}",
        f"selection_reason={selection_reason}",
        f"measurement_hygiene_required={measurement_hygiene_required}",
        f"next_optimization_allowed={optimization_allowed}",
        "selected_next_row=HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001",
        "body_elapsed_ns_primary=0",
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
