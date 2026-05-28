#!/usr/bin/env python3
"""Join Hako exact-EXE and C mimalloc object-lifecycle body timing evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


WORKLOAD = "representative-object-lifecycle-small-block-v0"
OPERATION_FAMILY = "small-block"
OPERATION_SEQUENCE = "representative-object-lifecycle-small-block-v0-seq"
FREE_ORDER = "even-odd-release-v0"
IN_PROCESS_REPEAT = 8192
ALLOCATION_COUNT = 524288
FREE_COUNT = 524288
REQUESTED_BYTES = 272416768


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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = require_key(values, key, label)
    try:
        value = int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc
    if value < 0:
        raise SystemExit(f"{label}: {key} must be non-negative, got {value}")
    return value


def require_positive_int(values: dict[str, str], key: str, label: str) -> int:
    value = require_int(values, key, label)
    if value <= 0:
        raise SystemExit(f"{label}: {key} must be positive, got {value}")
    return value


def require_common_workload(values: dict[str, str], label: str) -> None:
    require(values, "workload", WORKLOAD, label)
    require(values, "operation_family", OPERATION_FAMILY, label)
    require(values, "operation_sequence_id", OPERATION_SEQUENCE, label)
    require(values, "free_order_id", FREE_ORDER, label)
    require(values, "in_process_operation_repeat", str(IN_PROCESS_REPEAT), label)
    require(values, "allocation_count", str(ALLOCATION_COUNT), label)
    require(values, "free_count", str(FREE_COUNT), label)
    require(values, "requested_bytes", str(REQUESTED_BYTES), label)
    require(values, "summary", "ok", label)


def ratio(num: int, den: int) -> str:
    if den <= 0:
        raise SystemExit("ratio denominator must be positive")
    return f"{num / den:.3f}"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hako-report", type=Path, required=True)
    parser.add_argument("--c-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    hako = read_kv(args.hako_report)
    c = read_kv(args.c_report)

    require(hako, "output_contract", "hako-exe-memory-evidence-v0", "hako")
    require(c, "output_contract", "allocator-comparison-c-mimalloc-explicit-runner-v0", "c")
    require_common_workload(hako, "hako")
    require_common_workload(c, "c")
    require(hako, "hako_body_timing_available", "1", "hako")
    require(c, "c_body_timing_available", "1", "c")
    require(hako, "body_timing_repeat_kind", "workload-body-env-now-ms-v0", "hako")
    require(c, "body_timing_repeat_kind", "workload-body-monotonic-v0", "c")
    require(hako, "body_timing_is_process_timing", "0", "hako")
    require(c, "body_timing_is_process_timing", "0", "c")
    require(hako, "provider_activation", "0", "hako")
    require(hako, "host_replacement", "0", "hako")
    require(hako, "hook_installed", "0", "hako")
    require(hako, "global_allocator_installed", "0", "hako")
    require(c, "process_replacement_executed", "0", "c")
    require(c, "hook_installed", "0", "c")
    require(c, "global_allocator_installed", "0", "c")

    hako_body_ns = require_positive_int(hako, "body_elapsed_ns", "hako")
    c_body_ns = require_positive_int(c, "body_elapsed_ns", "c")
    hako_external_ms = require_positive_int(hako, "external_elapsed_ms", "hako")
    c_external_ms = require_positive_int(c, "external_elapsed_ms", "c")
    hako_rss = require_positive_int(hako, "external_peak_rss_bytes", "hako")
    c_rss = require_positive_int(c, "external_peak_rss_bytes", "c")

    lines = [
        "output_contract=hako-mimalloc-object-lifecycle-body-timing-pair-v0",
        "input_contract=object-lifecycle-hako-c-body-timing-v0",
        "measurement_profile=phase296x-object-lifecycle-body-timing-pair-v0",
        f"workload_id={WORKLOAD}",
        f"operation_family={OPERATION_FAMILY}",
        f"operation_sequence_id={OPERATION_SEQUENCE}",
        f"free_order_id={FREE_ORDER}",
        f"in_process_operation_repeat={IN_PROCESS_REPEAT}",
        f"allocation_count={ALLOCATION_COUNT}",
        f"free_count={FREE_COUNT}",
        f"requested_bytes={REQUESTED_BYTES}",
        "hako_subject=hako_exact_exe_object_lifecycle",
        "c_subject=c_mimalloc_explicit_object_lifecycle",
        "body_elapsed_role=primary_hot_loop_diagnostic",
        "external_elapsed_role=secondary_process_runtime_evidence",
        "body_elapsed_comparable=1",
        "hako_body_timing_available=1",
        "c_body_timing_available=1",
        "hako_body_timing_repeat_kind=workload-body-env-now-ms-v0",
        "c_body_timing_repeat_kind=workload-body-monotonic-v0",
        f"hako_body_elapsed_ns={hako_body_ns}",
        f"c_body_elapsed_ns={c_body_ns}",
        f"body_elapsed_gap_ns={hako_body_ns - c_body_ns}",
        f"body_elapsed_ratio={ratio(hako_body_ns, c_body_ns)}",
        f"hako_external_elapsed_ms={hako_external_ms}",
        f"c_external_elapsed_ms={c_external_ms}",
        f"external_elapsed_gap_ms={hako_external_ms - c_external_ms}",
        f"external_elapsed_ratio={ratio(hako_external_ms, c_external_ms)}",
        f"hako_external_peak_rss_bytes={hako_rss}",
        f"c_external_peak_rss_bytes={c_rss}",
        f"external_peak_rss_gap_bytes={hako_rss - c_rss}",
        "next_diagnostic=object_lifecycle_body_timing_gap_taxonomy",
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
