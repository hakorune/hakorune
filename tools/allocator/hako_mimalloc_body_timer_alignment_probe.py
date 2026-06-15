#!/usr/bin/env python3
"""Compare baseline and scaled Hako/C body timing to reduce timer granularity risk."""

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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = require_key(values, key, label)
    try:
        value = int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc
    if value <= 0:
        raise SystemExit(f"{label}: {key} must be positive, got {value}")
    return value


def require_float(values: dict[str, str], key: str, label: str) -> float:
    text = require_key(values, key, label)
    try:
        value = float(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be float, got {text!r}") from exc
    if value <= 0:
        raise SystemExit(f"{label}: {key} must be positive, got {value}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--scaled", type=Path, required=True)
    parser.add_argument("--scaled-in-process-repeat", type=int, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    if args.scaled_in_process_repeat <= 0:
        raise SystemExit("--scaled-in-process-repeat must be positive")

    baseline = read_kv(args.baseline)
    scaled = read_kv(args.scaled)
    for label, values in (("baseline", baseline), ("scaled", scaled)):
        require(values, "output_contract", "hako-mimalloc-body-timing-precision-v0", label)
        require(values, "target_method", "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1", label)
        require(values, "hako_timer_family", "workload-body-env-now-ms-v0", label)
        require(values, "c_timer_family", "workload-body-monotonic-v0", label)
        require(values, "timer_family_matched", "0", label)
        require(values, "summary", "ok", label)

    scaled_hako_ns = require_int(scaled, "hako_body_elapsed_ns", "scaled")
    timer_resolution_ns = require_int(scaled, "hako_timer_resolution_ns", "scaled")
    resolution_pct = (timer_resolution_ns / scaled_hako_ns) * 100.0
    scaled_ratio = require_float(scaled, "body_elapsed_ratio_raw", "scaled")
    baseline_ratio = require_float(baseline, "body_elapsed_ratio_raw", "baseline")

    if resolution_pct <= 2.0 and scaled_ratio >= 1.5:
        owner = "runtime_boundary_direct_probe"
        confidence = "medium"
        precision_confidence = "medium"
        reason = "scaled_body_timer_resolution_small_but_gap_remains"
    else:
        owner = "repeat_alignment_retry"
        confidence = "low"
        precision_confidence = "low"
        reason = "scaled_timer_resolution_or_gap_not_conclusive"

    lines = [
        "output_contract=hako-mimalloc-body-timer-alignment-probe-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-703",
        f"baseline_body_elapsed_ratio={baseline_ratio:.3f}",
        f"scaled_in_process_repeat={args.scaled_in_process_repeat}",
        f"scaled_hako_body_elapsed_ns={scaled_hako_ns}",
        f"scaled_c_body_elapsed_ns={require_key(scaled, 'c_body_elapsed_ns', 'scaled')}",
        f"scaled_body_elapsed_ratio={scaled_ratio:.3f}",
        "hako_timer_family=workload-body-env-now-ms-v0",
        "c_timer_family=workload-body-monotonic-v0",
        "timer_family_matched=0",
        f"hako_timer_resolution_ns={timer_resolution_ns}",
        f"scaled_hako_timer_resolution_pct={resolution_pct:.3f}",
        f"body_elapsed_ratio_precision_confidence={precision_confidence}",
        f"selected_next_owner={owner}",
        f"selected_next_owner_confidence={confidence}",
        f"owner_reason={reason}",
        "implementation_started=0",
        "compiler_lowering_changed=0",
        "runtime_object_changed=0",
        "product_default_changed=0",
        "startup_lane_reopened=0",
        "source_hako_changed=0",
        "winner_claim=0",
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
