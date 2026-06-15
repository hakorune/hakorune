#!/usr/bin/env python3
"""Classify exact-object pilot measurement after ny-llvmc boundary reachability."""

from __future__ import annotations

import argparse
from pathlib import Path


HISTORICAL_RATIO_BEFORE = "114.326"


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if "=" not in line:
            continue
        key, value = line.strip().split("=", 1)
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
    parser.add_argument("--pair-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    pair = read_kv(args.pair_report)
    require(pair, "output_contract", "hako-mimalloc-object-lifecycle-body-timing-pair-v0")
    require(pair, "workload_id", "representative-object-lifecycle-small-block-v0")
    require(pair, "body_elapsed_comparable", "1")
    require(pair, "summary", "ok")

    ratio_after = require_key(pair, "body_elapsed_ratio")
    try:
        before_value = float(HISTORICAL_RATIO_BEFORE)
        after_value = float(ratio_after)
    except ValueError as exc:
        raise SystemExit("body_elapsed_ratio values must be numeric") from exc
    winner_claim = int(after_value < before_value)
    selected_next = (
        "EXACT-OBJECT-PILOT-CLOSEOUT-001"
        if winner_claim == 0
        else "EXACT-OBJECT-PILOT-CLOSEOUT-001"
    )

    lines = [
        "output_contract=hako-exact-object-pilot-measurement-002-v0",
        "source_evidence=296x-729",
        "target_front=object_lifecycle_body",
        "pilot_exact_object_enabled=1",
        "product_default_changed=0",
        "global_arc_retirement_claim=0",
        f"body_elapsed_ratio_before={HISTORICAL_RATIO_BEFORE}",
        f"body_elapsed_ratio_after={ratio_after}",
        f"hako_body_elapsed_ns_after={require_key(pair, 'hako_body_elapsed_ns')}",
        f"c_body_elapsed_ns_after={require_key(pair, 'c_body_elapsed_ns')}",
        f"measurement_pair_report={args.pair_report}",
        f"winner_claim={winner_claim}",
        f"selected_next={selected_next}",
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
