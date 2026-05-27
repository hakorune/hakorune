#!/usr/bin/env python3
"""Refresh in-process taxonomy after the third hako mimalloc keeper."""

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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None or text == "":
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--current", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    current = read_kv(args.current)
    label = "current"
    require(current, "output_contract", "hako-mimalloc-in-process-operation-repeat-measurement-v0", label)
    require(current, "measurement_profile", "hako-mimalloc-in-process-operation-repeat-v0", label)
    require(current, "timing_repeat_kind", "in-process-operation-loop-v0", label)
    require(current, "workload_id", "representative-small-block-v0", label)
    require(current, "operation_repeat", "8192", label)
    require(current, "process_repeat", "3", label)
    require(current, "same_workload", "1", label)
    require(current, "same_operation_count", "1", label)
    require(current, "process_invocation_repeat", "0", label)
    require(current, "winner_claim", "0", label)
    require(current, "provider_active", "0", label)
    require(current, "replacement_active", "0", label)
    require(current, "hook_installed", "0", label)
    require(current, "global_allocator", "0", label)
    require(current, "summary", "ok", label)

    cur_hako = require_int(current, "hako_external_elapsed_median_ms", label)
    cur_c = require_int(current, "c_external_elapsed_median_ms", label)
    cur_gap = require_int(current, "external_elapsed_median_gap_ms", label)
    if cur_gap != cur_hako - cur_c:
        raise SystemExit("current: external_elapsed_median_gap_ms mismatch")

    lines = [
        "output_contract=hako-mimalloc-post-third-keeper-taxonomy-refresh-v0",
        "input_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0",
        "optimization_kind=known_active_small_cycle_fast_path",
        "workload_id=representative-small-block-v0",
        "measurement_profile=hako-mimalloc-in-process-operation-repeat-v0",
        "timing_repeat_kind=in-process-operation-loop-v0",
        "operation_repeat=8192",
        "process_repeat=3",
        "hako_compile_build_excluded=1",
        "c_compile_build_excluded=1",
        "external_timing_collectors_same=0",
        "hako_body_timing_available=0",
        "c_body_timing_available=1",
        "body_elapsed_comparable=0",
        "body_elapsed_primary=0",
        "hako_work_shape=page_model_known_active_small_cycle",
        "c_work_shape=mi_malloc_memset_mi_free",
        "same_workload_semantics=partial",
        "interpretation_scope=operation-count-parity-only",
        f"current_hako_external_elapsed_median_ms={cur_hako}",
        f"current_c_external_elapsed_median_ms={cur_c}",
        f"remaining_gap_ms={cur_gap}",
        "gap_owner=allocator_algorithm",
        "gap_confidence=high",
        "optimization_checkpoint=small_model_fast_path_plateau",
        "next_diagnostic=port_feature_gap_inventory",
        "next_optimization_allowed=0",
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
