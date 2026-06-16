#!/usr/bin/env python3
"""List known fast-path consumer families and their reachability status.

This is a small current-lane inventory. It is intentionally not a source-code
scanner and does not prove a route is active for a particular MIR file. Use the
reachability ledger for active-front route selection.
"""

from __future__ import annotations

import argparse
import json
from dataclasses import asdict, dataclass


@dataclass(frozen=True)
class ConsumerRow:
    family: str
    owner: str
    metadata_surface: str
    backend_consumer: str
    status: str
    winner_claim_allowed: str
    followup_required: str


CONSUMERS: tuple[ConsumerRow, ...] = (
    ConsumerRow(
        family="exact_seed",
        owner="function_level_exact_seed",
        metadata_surface="metadata.exact_seed_backend_route",
        backend_consumer="hako_llvmc_ffi_pure_compile",
        status="selected_route_family",
        winner_claim_allowed="ledger_dependent",
        followup_required="exact_seed_retire_inventory_before_retire",
    ),
    ConsumerRow(
        family="local_fastpath_fact",
        owner="LocalFastPathFact",
        metadata_surface="metadata.local_fastpath_facts",
        backend_consumer="python_llvm_backend_local_fastpath_resolver",
        status="positive_fact_surface",
        winner_claim_allowed="reachable_fact_only",
        followup_required="none",
    ),
    ConsumerRow(
        family="local_i64_map_entry_table",
        owner="LocalFastPathFact_plus_LocalI64MapEntryTable",
        metadata_surface="metadata.local_i64_map_entry_value_tracking_plans",
        backend_consumer="python_llvm_backend_local_i64_entry_table",
        status="landed_reachable_closed",
        winner_claim_allowed="front_measurement_dependent",
        followup_required="none",
    ),
    ConsumerRow(
        family="string_dead_text_region",
        owner="StringDeadTextRegionPlan",
        metadata_surface="metadata.string_dead_text_region_plans",
        backend_consumer="cabi_string_dead_text_region_consumer",
        status="backend_consumer_exists_reachability_blocked",
        winner_claim_allowed="0",
        followup_required="route_selection_or_exact_seed_retire_row",
    ),
    ConsumerRow(
        family="runtime_helper_fallback",
        owner="product_runtime_fallback",
        metadata_surface="none",
        backend_consumer="runtime_helper_call",
        status="fallback_not_fastpath",
        winner_claim_allowed="0",
        followup_required="none",
    ),
)


def build_report() -> dict[str, object]:
    return {
        "output_contract": "hako-fastpath-consumer-inventory-v0",
        "consumer_count": str(len(CONSUMERS)),
        "consumer_inventory_kind": "current_lane_known_families",
        "backend_consumer_code_is_not_reachability": "1",
        "winner_claim_requires_reachable_consumer": "1",
        "unknown_consumer_winner_claim_allowed": "0",
        "summary": "ok",
        "consumers": [asdict(row) for row in CONSUMERS],
    }


def emit_kv(report: dict[str, object]) -> None:
    for key, value in report.items():
        if key == "consumers":
            continue
        print(f"{key}={value}")
    consumers = report.get("consumers")
    if isinstance(consumers, list):
        for idx, row in enumerate(consumers):
            if not isinstance(row, dict):
                continue
            for key, value in row.items():
                print(f"consumer_{idx}_{key}={value}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--format", choices=("kv", "json"), default="kv")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report = build_report()
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        emit_kv(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
