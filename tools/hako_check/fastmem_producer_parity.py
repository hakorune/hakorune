#!/usr/bin/env python3
"""Check producer-neutral parity between FastMemory replacement-front reports."""

from __future__ import annotations

import argparse
from pathlib import Path

from report_kv import read_kv

BASELINE_PRODUCER = "python_template_c_bridge"
CANDIDATE_PRODUCER = "mir_to_llvm_lowering"
CANDIDATE_ARTIFACTS = {"llvm_ir", "object", "exe"}

REQUIRED_ZERO_FIELDS = (
    "replacement_front_python_template_c_semantic_ssot",
    "replacement_front_mirbuilder_route_decision_count",
    "type_abi_hot_lookup_count",
    "type_abi_hot_path_lookup_count",
    "provider_abi_hot_dispatch_count",
    "provider_dispatch_hot_path",
    "product_activation",
    "hook_install",
    "hook_installed",
    "global_allocator_claim",
    "global_allocator_product_claim",
    "winner_claim",
)

REQUIRED_ONE_FIELDS = (
    "replacement_front_producer_taxonomy_v0",
    "replacement_front_python_template_c_retirement_required",
    "replacement_front_mirbuilder_representation_only",
)

PARITY_FIELDS = (
    "replacement_front_source_truth",
    "replacement_front_python_template_c_semantic_ssot",
    "replacement_front_python_template_c_retirement_required",
    "replacement_front_mirbuilder_representation_only",
    "replacement_front_mirbuilder_route_decision_count",
    "replacement_front_is_full_hako_algorithm",
    "hako_mimalloc_algorithm_claim",
    "type_abi_hot_lookup_count",
    "type_abi_hot_path_lookup_count",
    "provider_abi_hot_dispatch_count",
    "provider_dispatch_hot_path",
    "product_activation",
    "hook_install",
    "hook_installed",
    "global_allocator_claim",
    "global_allocator_product_claim",
    "winner_claim",
)

INT_DEFAULTS: dict[str, int] = {
    "type_abi_hot_lookup_count": 0,
    "type_abi_hot_path_lookup_count": 0,
    "provider_abi_hot_dispatch_count": 0,
    "provider_dispatch_hot_path": 0,
    "product_activation": 0,
    "hook_install": 0,
    "hook_installed": 0,
    "global_allocator_claim": 0,
    "global_allocator_product_claim": 0,
    "winner_claim": 0,
}


def int_count(rows: dict[str, str], key: str) -> int:
    value = normalized_value(rows, key)
    try:
        return int(float(value))
    except (TypeError, ValueError):
        return 0


def normalized_value(rows: dict[str, str], key: str) -> str:
    value = rows.get(key)
    if value is not None:
        return value
    if key in INT_DEFAULTS:
        return str(INT_DEFAULTS[key])
    return ""


def add_failure(reasons: list[str], reason: str) -> None:
    if reason not in reasons:
        reasons.append(reason)


def validate_role(
    rows: dict[str, str],
    *,
    role: str,
    reasons: list[str],
) -> None:
    prefix = f"{role}_"
    producer = rows.get("replacement_front_producer", "unknown")
    artifact = rows.get("replacement_front_backend_artifact", "unknown")
    if role == "baseline":
        if producer != BASELINE_PRODUCER:
            add_failure(reasons, prefix + "replacement_front_producer")
        if artifact != "c":
            add_failure(reasons, prefix + "replacement_front_backend_artifact")
        if int_count(rows, "replacement_front_mir_memop_enabled") != 0:
            add_failure(reasons, prefix + "replacement_front_mir_memop_enabled")
        if int_count(rows, "replacement_front_mir_fastmem_region_enabled") != 0:
            add_failure(reasons, prefix + "replacement_front_mir_fastmem_region_enabled")
    elif role == "candidate":
        if producer != CANDIDATE_PRODUCER:
            add_failure(reasons, prefix + "replacement_front_producer")
        if artifact not in CANDIDATE_ARTIFACTS:
            add_failure(reasons, prefix + "replacement_front_backend_artifact")
        if int_count(rows, "replacement_front_mir_memop_enabled") != 1:
            add_failure(reasons, prefix + "replacement_front_mir_memop_enabled")
        if int_count(rows, "replacement_front_mir_fastmem_region_enabled") != 1:
            add_failure(reasons, prefix + "replacement_front_mir_fastmem_region_enabled")
    else:
        raise ValueError(f"unknown parity role: {role}")

    for key in REQUIRED_ZERO_FIELDS:
        if int_count(rows, key) != 0:
            add_failure(reasons, prefix + key)
    for key in REQUIRED_ONE_FIELDS:
        if int_count(rows, key) != 1:
            add_failure(reasons, prefix + key)


def parity_failures(
    baseline: dict[str, str],
    candidate: dict[str, str],
) -> tuple[list[str], int, int, int]:
    reasons: list[str] = []
    mismatch_count = 0
    missing_count = 0
    compared_count = 0

    validate_role(baseline, role="baseline", reasons=reasons)
    validate_role(candidate, role="candidate", reasons=reasons)

    for key in PARITY_FIELDS:
        baseline_has = key in baseline or key in INT_DEFAULTS
        candidate_has = key in candidate or key in INT_DEFAULTS
        if not baseline_has or not candidate_has:
            missing_count += 1
            add_failure(reasons, f"missing:{key}")
            continue
        compared_count += 1
        if normalized_value(baseline, key) != normalized_value(candidate, key):
            mismatch_count += 1
            add_failure(reasons, f"mismatch:{key}")

    if mismatch_count:
        add_failure(reasons, "producer_neutral_mismatch_count")
    if missing_count:
        add_failure(reasons, "producer_neutral_missing_field_count")
    return reasons, compared_count, mismatch_count, missing_count


def render_kv(
    baseline: dict[str, str],
    candidate: dict[str, str],
    reasons: list[str],
    compared_count: int,
    mismatch_count: int,
    missing_count: int,
) -> str:
    schema_ok = 0 if missing_count else 1
    parity_pass = 0 if reasons else 1
    runtime_dependency_count = 0 if parity_pass else 1
    lines = [
        "output_contract=hako-check-fastmem-producer-parity-v0",
        "tool_surface=hako_check_fastmem_producer_parity",
        "observation_only=1",
        "rewrite_executed=0",
        "source_rewrite_executed=0",
        "benchmark_run_executed=0",
        "keeper_selection=0",
        f"baseline_replacement_front_producer={baseline.get('replacement_front_producer', 'unknown')}",
        f"candidate_replacement_front_producer={candidate.get('replacement_front_producer', 'unknown')}",
        f"baseline_replacement_front_backend_artifact={baseline.get('replacement_front_backend_artifact', 'unknown')}",
        f"candidate_replacement_front_backend_artifact={candidate.get('replacement_front_backend_artifact', 'unknown')}",
        f"producer_neutral_report_schema={schema_ok}",
        f"producer_neutral_parity_pass={parity_pass}",
        f"producer_neutral_compared_field_count={compared_count}",
        f"producer_neutral_mismatch_count={mismatch_count}",
        f"producer_neutral_missing_field_count={missing_count}",
        f"python_template_c_bridge_runtime_dependency_count={runtime_dependency_count}",
        "product_activation=0",
        "hook_install=0",
        "global_allocator_claim=0",
        "winner_claim=0",
        f"failure_count={len(reasons)}",
    ]
    for idx, reason in enumerate(reasons):
        lines.append(f"failure_{idx}_reason={reason}")
    lines.append(f"summary={'ok' if parity_pass else 'failed'}")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--format", choices=("kv",), default="kv")
    args = parser.parse_args()

    baseline = read_kv(args.baseline)
    candidate = read_kv(args.candidate)
    reasons, compared_count, mismatch_count, missing_count = parity_failures(
        baseline,
        candidate,
    )
    print(render_kv(baseline, candidate, reasons, compared_count, mismatch_count, missing_count), end="")
    return 0 if not reasons else 1


if __name__ == "__main__":
    raise SystemExit(main())
