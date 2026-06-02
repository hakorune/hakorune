#!/usr/bin/env python3
"""Combine provider replacement pilot evidence without making a winner claim."""

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


def require_positive_int(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None:
        raise SystemExit(f"{label}: missing {key}")
    try:
        number = int(value)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {value!r}") from exc
    if number <= 0:
        raise SystemExit(f"{label}: {key} must be positive, got {value!r}")
    return value


def require_key(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"{label}: missing {key}")
    return value


def validate_hako_c(values: dict[str, str], workload_index: int) -> tuple[str, str, str, str]:
    label = "hako_c"
    require(values, "output_contract", "mimalloc-comparison-repeated-measurement-v0", label)
    require(values, "winner_claim", "0", label)
    require(values, "provider_activation", "0", label)
    require(values, "host_replacement", "0", label)
    require(values, "hook_installed", "0", label)
    require(values, "global_allocator_installed", "0", label)
    require(values, "summary", "ok", label)
    prefix = f"workload_{workload_index}"
    workload = require_key(values, f"{prefix}_id", label)
    operation_family = require_key(values, f"{prefix}_operation_family", label)
    operation_repeat = require_key(values, f"{prefix}_operation_repeat", label)
    sample_count = require_key(values, f"{prefix}_sample_count", label)
    return workload, operation_family, operation_repeat, sample_count


def validate_provider_explicit(
    values: dict[str, str],
    operation_repeat: str,
    sample_count: str,
) -> None:
    label = "provider"
    require(values, "output_contract", "hakorune-provider-explicit-repeated-measurement-v0", label)
    require(values, "summary", "ok", label)
    require(values, "operation_repeat", operation_repeat, label)
    require(values, "sample_count", sample_count, label)
    require(values, "provider_api_bound", "1", label)
    require(values, "provider_call_executed", "1", label)
    require(values, "allocator_entrypoint_called", "1", label)
    require(values, "provider_active", "0", label)
    require(values, "replacement_active", "0", label)
    require(values, "hook_installed", "0", label)
    require(values, "global_allocator", "0", label)
    require(values, "winner_claim", "0", label)
    require_positive_int(values, "sample_elapsed_median_ns", label)


def validate_ldpreload(values: dict[str, str]) -> None:
    label = "ldpreload"
    contract = require_key(values, "output_contract", label)
    if contract not in {
        "hako-mimalloc-provider-backed-hakmem-ldpreload-bench-pilot-v0",
        "hako-mimalloc-provider-backed-hakmem-ldpreload-repeated-measurement-v0",
    }:
        raise SystemExit(f"{label}: unsupported output_contract {contract!r}")
    require(values, "summary", "ok", label)
    require(values, "ld_preload_env_applied", "1", label)
    require(values, "provider_api_bound", "1", label)
    require(values, "provider_call_executed", "1", label)
    require(values, "allocator_entrypoint_called", "1", label)
    require(values, "replacement_active", "1", label)
    require(values, "replacement_product_claim", "0", label)
    require(values, "hook_installed", "0", label)
    require(values, "global_allocator", "0", label)
    require(values, "winner_claim", "0", label)
    if contract.endswith("repeated-measurement-v0"):
        require(values, "shim_runtime_real_fallback_count_total", "0", label)
        require(values, "shim_pointer_table_overflow_total", "0", label)
        require_positive_int(values, "shim_provider_alloc_count_total", label)
        require_positive_int(values, "shim_provider_free_count_total", label)
        require_positive_int(values, "throughput_median_ops_per_sec", label)
    else:
        require(values, "benchmark_exit_code", "0", label)
        require(values, "shim_runtime_real_fallback_count", "0", label)
        require(values, "shim_pointer_table_overflow", "0", label)
        require_positive_int(values, "shim_provider_alloc_count", label)
        require_positive_int(values, "shim_provider_free_count", label)
        require_positive_int(values, "throughput_ops_per_sec", label)


def validate_rust_global(values: dict[str, str]) -> None:
    label = "rust_global"
    require(
        values,
        "output_contract",
        "hako-mimalloc-provider-backed-rust-global-allocator-smoke-v0",
        label,
    )
    require(values, "summary", "ok", label)
    require(values, "rust_exit_code", "0", label)
    require(values, "provider_api_bound", "1", label)
    require(values, "provider_call_executed", "1", label)
    require(values, "allocator_entrypoint_called", "1", label)
    require(values, "replacement_active", "0", label)
    require(values, "global_allocator", "1", label)
    require(values, "global_allocator_scope", "generated-rust-smoke-process-only", label)
    require(values, "global_allocator_product_claim", "0", label)
    require(values, "hook_installed", "0", label)
    require(values, "winner_claim", "0", label)
    require(values, "rust_runtime_fallback_count", "0", label)
    require(values, "rust_pointer_table_overflow", "0", label)
    require_positive_int(values, "rust_provider_alloc_count", label)
    require_positive_int(values, "rust_provider_free_count", label)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hako-c-report", type=Path, required=True)
    parser.add_argument("--provider-report", type=Path, required=True)
    parser.add_argument("--ldpreload-report", type=Path, required=True)
    parser.add_argument("--rust-global-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--workload-index", type=int, default=0)
    args = parser.parse_args()

    hako_c = read_kv(args.hako_c_report)
    provider = read_kv(args.provider_report)
    ldpreload = read_kv(args.ldpreload_report)
    rust_global = read_kv(args.rust_global_report)

    workload, operation_family, operation_repeat, sample_count = validate_hako_c(
        hako_c,
        args.workload_index,
    )
    validate_provider_explicit(provider, operation_repeat, sample_count)
    validate_ldpreload(ldpreload)
    validate_rust_global(rust_global)

    lines = [
        "output_contract=hako-mimalloc-provider-replacement-decision-adapter-v0",
        "input_contracts=mimalloc-comparison-repeated-measurement-v0,hakorune-provider-explicit-repeated-measurement-v0,hako-mimalloc-provider-backed-hakmem-ldpreload-repeated-measurement-v0|hako-mimalloc-provider-backed-hakmem-ldpreload-bench-pilot-v0,hako-mimalloc-provider-backed-rust-global-allocator-smoke-v0",
        "decision_scope=provider_replacement_pilots_without_winner_claim",
        "measurement_domains=exact_exe_c_provider_explicit,external_ldpreload,generated_rust_global_allocator",
        f"workload_id={workload}",
        f"operation_family={operation_family}",
        f"operation_repeat={operation_repeat}",
        f"sample_count={sample_count}",
        "subject_count=5",
        "subject_0_id=hako_exact_exe",
        f"subject_0_elapsed_median_ms={require_key(hako_c, f'workload_{args.workload_index}_hako_external_elapsed_median_ms', 'hako_c')}",
        "subject_0_winner_claim=0",
        "subject_1_id=c_mimalloc_explicit_runner",
        f"subject_1_elapsed_median_ms={require_key(hako_c, f'workload_{args.workload_index}_c_external_elapsed_median_ms', 'hako_c')}",
        "subject_1_winner_claim=0",
        "subject_2_id=provider_package_explicit_alloc_free",
        f"subject_2_elapsed_median_ns={require_key(provider, 'sample_elapsed_median_ns', 'provider')}",
        "subject_2_replacement_active=0",
        "subject_2_winner_claim=0",
        "subject_3_id=provider_backed_hakmem_ldpreload",
        f"subject_3_benchmark_id={require_key(ldpreload, 'benchmark_id', 'ldpreload')}",
        f"subject_3_timing_repeat_kind={ldpreload.get('timing_repeat_kind', 'single-process-pilot-v0')}",
        f"subject_3_throughput_median_ops_per_sec={ldpreload.get('throughput_median_ops_per_sec', ldpreload.get('throughput_ops_per_sec', ''))}",
        "subject_3_replacement_active=1",
        "subject_3_replacement_product_claim=0",
        "subject_3_winner_claim=0",
        "subject_4_id=provider_backed_rust_global_allocator",
        "subject_4_global_allocator=1",
        "subject_4_global_allocator_scope=generated-rust-smoke-process-only",
        "subject_4_global_allocator_product_claim=0",
        "subject_4_winner_claim=0",
        "provider_api_bound=1",
        "provider_call_executed=1",
        "allocator_entrypoint_called=1",
        "external_process_replacement_pilot=1",
        "generated_global_allocator_pilot=1",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
        "decision=external_process_repeated_ready_no_product_default_change",
        "next_step=compare_against_external_corpus_or_scale_sample_count_before_any_winner_claim",
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
