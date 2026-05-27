#!/usr/bin/env python3
"""Split hako mimalloc in-process small-block cost by allocator phase."""

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


def median_int(values: list[int]) -> int:
    if not values:
        raise SystemExit("missing samples")
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def load_group(paths: list[Path], workload: str, label: str) -> list[int]:
    elapsed: list[int] = []
    for index, path in enumerate(paths):
        sample_label = f"{label}[{index}]"
        values = read_kv(path)
        require(values, "output_contract", "hako-exe-memory-evidence-v0", sample_label)
        require(values, "workload", workload, sample_label)
        require(values, "runtime_config_profile", "empty", sample_label)
        require(values, "result_code", "0", sample_label)
        require(values, "run_count", "1", sample_label)
        require(values, "operation_repeat", "1", sample_label)
        require(values, "timing_repeat_kind", "process-invocation-v0", sample_label)
        require(values, "in_process_operation_repeat", "8192", sample_label)
        require(values, "app_timing_repeat_kind", "in-process-operation-loop-v0", sample_label)
        require(values, "provider_activation", "0", sample_label)
        require(values, "host_replacement", "0", sample_label)
        require(values, "hook_installed", "0", sample_label)
        require(values, "global_allocator_installed", "0", sample_label)
        require(values, "summary", "ok", sample_label)
        elapsed.append(require_int(values, "external_elapsed_ms", sample_label))
    return elapsed


def dominant_phase(reset_ms: int, alloc_ms: int, release_ms: int) -> str:
    phases = {
        "reset": reset_ms,
        "alloc": alloc_ms,
        "release": release_ms,
    }
    top_name, top_value = max(phases.items(), key=lambda item: item[1])
    second_value = sorted(phases.values())[-2]
    if top_value <= 0:
        return "mixed"
    if second_value > 0 and top_value * 100 < second_value * 125:
        return "mixed"
    return top_name


def target_for(phase: str) -> str:
    if phase == "reset":
        return "reset_to_fresh_bulk_clear_or_generation_stamp"
    if phase == "alloc":
        return "acquire_usize_fast_path_and_invariant_hoist"
    if phase == "release":
        return "release_local_fast_path_and_retire_policy"
    return "phase_pair_micro_diagnostic"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reset-only", type=Path, nargs="+", required=True)
    parser.add_argument("--reset-alloc-only", type=Path, nargs="+", required=True)
    parser.add_argument("--full", type=Path, nargs="+", required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    sample_count = len(args.reset_only)
    if len(args.reset_alloc_only) != sample_count or len(args.full) != sample_count:
        raise SystemExit("all sample groups must have the same length")

    reset_samples = load_group(
        args.reset_only,
        "representative-small-block-reset-only-v0",
        "reset-only",
    )
    reset_alloc_samples = load_group(
        args.reset_alloc_only,
        "representative-small-block-reset-alloc-only-v0",
        "reset-alloc-only",
    )
    full_samples = load_group(
        args.full,
        "representative-small-block-v0",
        "full",
    )

    reset_median = median_int(reset_samples)
    reset_alloc_median = median_int(reset_alloc_samples)
    full_median = median_int(full_samples)
    alloc_estimate = max(0, reset_alloc_median - reset_median)
    alloc_release = max(0, full_median - reset_median)
    release_estimate = max(0, full_median - reset_alloc_median)
    dominant = dominant_phase(reset_median, alloc_estimate, release_estimate)
    next_allowed = "1" if dominant != "mixed" else "0"

    lines = [
        "output_contract=hako-mimalloc-phase-cost-ablation-v0",
        "input_contract=hako-exe-memory-evidence-v0",
        "workload_id=representative-small-block-v0",
        "measurement_profile=hako-mimalloc-phase-cost-ablation-v0",
        "timing_repeat_kind=in-process-operation-loop-v0",
        "operation_repeat=8192",
        f"process_repeat={sample_count}",
        "runtime_config_profile=empty",
        "external_timing_collector_hako=usr_bin_time_elapsed",
        "hako_body_timing_available=0",
        "body_elapsed_primary=0",
        "phase_cost_method=median_difference_ablation",
        "release_only_estimated=1",
        "hako_level_vs_mirbuilder_level=hako_allocator_model_primary",
        "mirbuilder_owner=secondary_later",
        "work_shape=page_model_reset_acquire_release",
        f"reset_only_elapsed_median_ms={reset_median}",
        f"reset_alloc_only_elapsed_median_ms={reset_alloc_median}",
        f"full_elapsed_median_ms={full_median}",
        f"alloc_only_estimated_ms={alloc_estimate}",
        f"alloc_release_elapsed_median_ms={alloc_release}",
        f"release_only_elapsed_median_ms={release_estimate}",
        f"dominant_phase={dominant}",
        f"next_optimization_target={target_for(dominant)}",
        f"next_optimization_allowed={next_allowed}",
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
