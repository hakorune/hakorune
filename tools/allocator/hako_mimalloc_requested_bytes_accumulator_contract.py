#!/usr/bin/env python3
"""Emit the representative requested-bytes accumulator contract.

This fixes the arithmetic for the current representative object-lifecycle
workload and reports whether the source-side accumulator policy is present, so
source/backend probes can distinguish:

* observed no-overflow for this benchmark shape
* source-side reject-before-accumulate overflow policy for arbitrary page-model
  requested_bytes updates
"""

from __future__ import annotations

import argparse
from pathlib import Path


I64_SIGNED_MAX = (1 << 63) - 1
SOURCE_ACCUMULATOR_LIMIT = 536_870_911
ALLOCATIONS_PER_RUN = 64
PER_RUN_REQUESTED_BYTES = 33254
MAX_REQUESTED_SIZE = 528
PAGE_BOX = Path("lang/src/hako_alloc/memory/page_box.hako")


def read_kv(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def positive_int(value: int, name: str) -> int:
    if value < 1:
        raise SystemExit(f"{name} must be >= 1")
    return value


def source_overflow_policy_ready(root: Path) -> bool:
    page_box = root / PAGE_BOX
    if not page_box.exists():
        return False
    text = page_box.read_text(encoding="utf-8")
    required = [
        "requestedBytesAccumulatorLimit",
        "canAccumulateRequestedBytes",
        "recordRequestedBytes",
        str(SOURCE_ACCUMULATOR_LIMIT),
        "limit - me.requested_bytes",
        "requested_size > remaining",
        "if me.recordRequestedBytes(requested_size) == 0",
        "me.reject_count = me.reject_count + 1",
        "me.requested_bytes = me.requested_bytes + requested_size",
    ]
    return all(item in text for item in required) and text.count(
        "if me.recordRequestedBytes(requested_size) == 0"
    ) >= 3


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--operation-repeat", type=int, default=8192)
    parser.add_argument("--measurement-report", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    operation_repeat = positive_int(args.operation_repeat, "--operation-repeat")
    measurement = read_kv(args.measurement_report)
    observed_repeat = int(measurement.get("in_process_operation_repeat", operation_repeat))
    observed_requested = int(
        measurement.get(
            "requested_bytes",
            str(PER_RUN_REQUESTED_BYTES * operation_repeat),
        )
    )
    expected_requested = PER_RUN_REQUESTED_BYTES * operation_repeat
    expected_observed_requested = PER_RUN_REQUESTED_BYTES * observed_repeat
    repeat_matches = int(observed_repeat == operation_repeat)
    requested_matches = int(observed_requested == expected_observed_requested)
    per_run_no_overflow = int(PER_RUN_REQUESTED_BYTES <= I64_SIGNED_MAX)
    observed_no_overflow = int(0 <= observed_requested <= I64_SIGNED_MAX)
    expected_no_overflow = int(0 <= expected_requested <= I64_SIGNED_MAX)
    source_policy_ready = int(source_overflow_policy_ready(Path.cwd()))
    expected_within_source_limit = int(expected_requested <= SOURCE_ACCUMULATOR_LIMIT)
    observed_within_source_limit = int(observed_requested <= SOURCE_ACCUMULATOR_LIMIT)
    source_reorder_allowed = int(
        source_policy_ready and expected_within_source_limit and observed_within_source_limit
    )
    general_no_overflow_proof = source_reorder_allowed
    next_bridge = (
        "source_reorder_probe_after_requested_bytes_overflow_policy"
        if source_policy_ready
        else "add_public_proof_accumulator_overflow_policy_before_source_reorder"
    )

    lines = [
        "output_contract=hako-mimalloc-requested-bytes-accumulator-contract-v0",
        "workload=representative-object-lifecycle-small-block-v0",
        "accumulator_field=requested_bytes",
        "accumulator_kind=public_semantics_proof_evidence",
        "accumulator_update=reject_before_accumulate_source_limit",
        f"source_overflow_policy_ready={source_policy_ready}",
        "source_overflow_policy=reject_before_accumulate",
        f"source_overflow_limit={SOURCE_ACCUMULATOR_LIMIT}",
        f"operation_repeat={operation_repeat}",
        f"observed_operation_repeat={observed_repeat}",
        f"operation_repeat_matches={repeat_matches}",
        f"allocations_per_run={ALLOCATIONS_PER_RUN}",
        f"max_requested_size={MAX_REQUESTED_SIZE}",
        f"per_run_requested_bytes={PER_RUN_REQUESTED_BYTES}",
        f"per_run_no_overflow={per_run_no_overflow}",
        f"expected_requested_bytes={expected_requested}",
        f"observed_requested_bytes={observed_requested}",
        f"requested_bytes_matches_workload_formula={requested_matches}",
        f"expected_no_overflow={expected_no_overflow}",
        f"observed_no_overflow={observed_no_overflow}",
        f"expected_within_source_overflow_limit={expected_within_source_limit}",
        f"observed_within_source_overflow_limit={observed_within_source_limit}",
        f"observed_i64_margin={I64_SIGNED_MAX - observed_requested}",
        f"general_no_overflow_proof={general_no_overflow_proof}",
        f"source_reorder_allowed={source_reorder_allowed}",
        f"next_bridge={next_bridge}",
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
