#!/usr/bin/env python3
"""Refresh gap taxonomy after rolling back the LocalSSA same-block non-keeper."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-mimalloc-post-rollback-gap-taxonomy-refresh-v0",
        "input_contract=rollback-local-ssa-same-block-reuse-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "current_hako_external_elapsed_median_ms=550",
        "current_hako_external_elapsed_source=row163_checkpoint_restored_by_row169",
        "current_c_exact_pair_available=0",
        "current_c_exact_pair_reason=object_lifecycle_c_runner_missing",
        "hako_body_elapsed_available=0",
        "c_body_elapsed_available=0",
        "body_elapsed_comparable=0",
        "body_elapsed_primary=0",
        "mir_shape_timing_correlation=weak",
        "mir_shape_timing_evidence=row167_structural_win_row168_timing_regression",
        "hako_source_suspicion=possible",
        "hako_source_suspicion_reason=facade_result_capsules_and_page_hotpath_helpers_remain_but_not_isolated",
        "compiler_lowering_suspicion=possible",
        "compiler_lowering_suspicion_reason=copy_call_field_surface_remains_but_copy_reduction_was_not_sufficient",
        "runtime_baseline_suspicion=possible",
        "runtime_baseline_suspicion_reason=external_elapsed_only_currently_drives_keeper_decisions",
        "selected_gap_owner=measurement_contract_gap",
        "gap_confidence=high",
        "owner_reason=missing_exact_c_object_lifecycle_pair_and_missing_hako_body_timing",
        "next_diagnostic=object_lifecycle_body_timing_and_exact_c_pair_contract",
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
