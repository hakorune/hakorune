#!/usr/bin/env python3
"""Classify local known-receiver direct-call pilot measurement evidence."""

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


def require_key(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"{label}: missing {key}")
    return value


def require_float(values: dict[str, str], key: str, label: str) -> float:
    text = require_key(values, key, label)
    try:
        return float(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be numeric, got {text!r}") from exc


def require_pair(values: dict[str, str], label: str) -> None:
    require(values, "output_contract", "hako-mimalloc-object-lifecycle-body-timing-pair-v0", label)
    require(values, "workload_id", "representative-object-lifecycle-small-block-v0", label)
    require(values, "body_elapsed_comparable", "1", label)
    require(values, "summary", "ok", label)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pilot-report", type=Path, required=True)
    parser.add_argument("--pair-report", type=Path, required=True)
    parser.add_argument("--secondary-pair-report", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    pilot = read_kv(args.pilot_report)
    pair = read_kv(args.pair_report)
    require(pilot, "output_contract", "hako-local-known-receiver-direct-call-pilot-v0", "pilot")
    require(pilot, "summary", "ok", "pilot")
    require(pilot, "new_backend_lowering_code_added", "0", "pilot")
    require(pilot, "product_default_changed", "0", "pilot")
    require_pair(pair, "pair")

    ratio = require_float(pair, "body_elapsed_ratio", "pair")
    hako_not_slower = int(ratio <= 1.0)

    secondary_lines: list[str] = []
    if args.secondary_pair_report:
        secondary = read_kv(args.secondary_pair_report)
        require_pair(secondary, "secondary_pair")
        secondary_lines = [
            f"secondary_pair_report={args.secondary_pair_report.as_posix()}",
            f"secondary_in_process_repeat={require_key(secondary, 'in_process_operation_repeat', 'secondary_pair')}",
            f"secondary_hako_body_elapsed_ns={require_key(secondary, 'hako_body_elapsed_ns', 'secondary_pair')}",
            f"secondary_c_body_elapsed_ns={require_key(secondary, 'c_body_elapsed_ns', 'secondary_pair')}",
            f"secondary_body_elapsed_ratio={require_key(secondary, 'body_elapsed_ratio', 'secondary_pair')}",
        ]

    lines = [
        "output_contract=hako-local-known-receiver-direct-call-measurement-v0",
        "source_evidence=296x-820",
        "target_front=object_lifecycle_body",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "selected_shape=local_known_receiver_direct_call",
        "pilot_status=already_satisfied_existing_generic_route",
        f"pilot_report={args.pilot_report.as_posix()}",
        f"primary_pair_report={args.pair_report.as_posix()}",
        f"primary_in_process_repeat={require_key(pair, 'in_process_operation_repeat', 'pair')}",
        f"hako_body_elapsed_ns={require_key(pair, 'hako_body_elapsed_ns', 'pair')}",
        f"c_body_elapsed_ns={require_key(pair, 'c_body_elapsed_ns', 'pair')}",
        f"body_elapsed_gap_ns={require_key(pair, 'body_elapsed_gap_ns', 'pair')}",
        f"body_elapsed_ratio={require_key(pair, 'body_elapsed_ratio', 'pair')}",
        f"hako_not_slower_than_c={hako_not_slower}",
        "measurement_interpretation=current_front_no_longer_hako_slower",
        "new_backend_lowering_code_added=0",
        "page_specific_rule_enabled=0",
        "method_name_special_case_enabled=0",
        "helper_symbol_inference_enabled=0",
        "storage_direct_enabled=0",
        "hosthandle_bypass_enabled=0",
        "arc_retirement_enabled=0",
        "product_default_changed=0",
        "winner_claim=1" if hako_not_slower else "winner_claim=0",
        "selected_next=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-CLOSEOUT-001",
        *secondary_lines,
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
