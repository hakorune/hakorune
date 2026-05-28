#!/usr/bin/env python3
"""Define the object-lifecycle body timing and exact C pair contract."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-mimalloc-object-lifecycle-body-timing-and-exact-c-pair-contract-v0",
        "input_contract=hako-mimalloc-post-rollback-gap-taxonomy-refresh-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "operation_family=small-block",
        "operation_sequence_id=representative-object-lifecycle-small-block-v0-seq",
        "free_order_id=even-odd-release-v0",
        "required_hako_subject=hako_exact_exe_object_lifecycle",
        "required_c_subject=c_mimalloc_explicit_object_lifecycle",
        "required_in_process_operation_repeat=8192",
        "required_allocation_count=524288",
        "required_free_count=524288",
        "required_requested_bytes=272416768",
        "hako_body_elapsed_ns_required=1",
        "c_body_elapsed_ns_required=1",
        "body_elapsed_comparable_required=1",
        "body_elapsed_role=primary_hot_loop_diagnostic",
        "external_elapsed_role=secondary_process_runtime_evidence",
        "exact_c_pair_required=1",
        "exact_c_pair_status=missing",
        "hako_body_timing_status=missing",
        "measurement_contract_gap_open=1",
        "next_diagnostic=object_lifecycle_exact_c_runner_first_pattern",
        "next_optimization_allowed=0",
        "optimization_started=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
