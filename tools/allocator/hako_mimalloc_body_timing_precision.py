#!/usr/bin/env python3
"""Classify Hako/C body-timing precision after compiler-lowering checkpoint."""

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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkpoint", type=Path, required=True)
    parser.add_argument("--pair-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    checkpoint = read_kv(args.checkpoint)
    pair = read_kv(args.pair_report)
    require(
        checkpoint,
        "output_contract",
        "hako-mimalloc-compiler-lowering-optimization-checkpoint-v0",
        "checkpoint",
    )
    require(checkpoint, "compiler_lowering_optimization_pause", "1", "checkpoint")
    require(checkpoint, "summary", "ok", "checkpoint")
    require(pair, "output_contract", "hako-mimalloc-object-lifecycle-body-timing-pair-v0", "pair")
    require(pair, "body_elapsed_comparable", "1", "pair")
    require(pair, "summary", "ok", "pair")

    lines = [
        "output_contract=hako-mimalloc-body-timing-precision-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-701",
        f"hako_timer_family={require_key(pair, 'hako_body_timing_repeat_kind', 'pair')}",
        f"c_timer_family={require_key(pair, 'c_body_timing_repeat_kind', 'pair')}",
        "timer_family_matched=0",
        "hako_timer_resolution_ns=1000000",
        "c_timer_resolution_ns=unknown",
        f"hako_body_elapsed_ns={require_key(pair, 'hako_body_elapsed_ns', 'pair')}",
        f"c_body_elapsed_ns={require_key(pair, 'c_body_elapsed_ns', 'pair')}",
        f"body_elapsed_ratio_raw={require_key(pair, 'body_elapsed_ratio', 'pair')}",
        "body_elapsed_ratio_precision_confidence=low",
        "measurement_boundary_confidence=low",
        "selected_next_owner=runtime_boundary_inventory",
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
