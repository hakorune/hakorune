#!/usr/bin/env python3
"""Close the local known-receiver direct-call lane after measurement."""

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


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"missing {key}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measurement-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    measurement = read_kv(args.measurement_report)
    require(
        measurement,
        "output_contract",
        "hako-local-known-receiver-direct-call-measurement-v0",
    )
    require(measurement, "winner_claim", "1")
    require(measurement, "hako_not_slower_than_c", "1")
    require(measurement, "measurement_interpretation", "current_front_no_longer_hako_slower")
    require(measurement, "new_backend_lowering_code_added", "0")
    require(measurement, "page_specific_rule_enabled", "0")
    require(measurement, "method_name_special_case_enabled", "0")
    require(measurement, "helper_symbol_inference_enabled", "0")
    require(measurement, "storage_direct_enabled", "0")
    require(measurement, "hosthandle_bypass_enabled", "0")
    require(measurement, "arc_retirement_enabled", "0")
    require(measurement, "product_default_changed", "0")
    require(measurement, "summary", "ok")

    lines = [
        "output_contract=hako-local-known-receiver-direct-call-closeout-v0",
        "source_evidence=296x-821,296x-820,296x-819",
        "target_front=object_lifecycle_body",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "closed_lane=local_known_receiver_direct_call",
        "lane_closed=1",
        "closeout_reason=current_front_no_longer_hako_slower_and_no_new_lowering_needed",
        f"measurement_report={args.measurement_report.as_posix()}",
        f"hako_body_elapsed_ns={require_key(measurement, 'hako_body_elapsed_ns')}",
        f"c_body_elapsed_ns={require_key(measurement, 'c_body_elapsed_ns')}",
        f"body_elapsed_ratio={require_key(measurement, 'body_elapsed_ratio')}",
        "winner_claim=1",
        "winner_claim_source=current_front_measurement",
        "new_speedup_claim=0",
        "new_backend_lowering_code_added=0",
        "page_specific_rule_enabled=0",
        "method_name_special_case_enabled=0",
        "helper_symbol_inference_enabled=0",
        "storage_direct_enabled=0",
        "hosthandle_bypass_enabled=0",
        "arc_retirement_enabled=0",
        "product_default_changed=0",
        "next_owner_selection_required=1",
        "selected_next=MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-AFTER-LOCAL-KNOWN-RECEIVER-CLOSEOUT-001",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    else:
        print(report, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
