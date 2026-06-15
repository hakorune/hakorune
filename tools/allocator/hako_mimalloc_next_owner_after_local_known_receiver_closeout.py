#!/usr/bin/env python3
"""Select the next owner after local known-receiver direct-call closeout."""

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


def require_float(values: dict[str, str], key: str) -> float:
    text = require_key(values, key)
    try:
        return float(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be numeric, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--closeout-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    closeout = read_kv(args.closeout_report)
    require(closeout, "output_contract", "hako-local-known-receiver-direct-call-closeout-v0")
    require(closeout, "lane_closed", "1")
    require(closeout, "new_speedup_claim", "0")
    require(closeout, "new_backend_lowering_code_added", "0")
    require(closeout, "storage_direct_enabled", "0")
    require(closeout, "hosthandle_bypass_enabled", "0")
    require(closeout, "arc_retirement_enabled", "0")
    require(closeout, "product_default_changed", "0")
    require(closeout, "summary", "ok")

    ratio = require_float(closeout, "body_elapsed_ratio")
    hako_slower = int(ratio > 1.0)
    selected_owner = "none_current_front_not_hako_slower" if not hako_slower else "requires_fresh_inventory"
    selected_confidence = "high" if not hako_slower else "low"
    next_task = (
        "MIMALLOC-CURRENT-FRONT-OPTIMIZATION-PAUSE-CHECKPOINT-001"
        if not hako_slower
        else "MIMALLOC-BODY-TIMING-FRESH-OWNER-INVENTORY-001"
    )

    lines = [
        "output_contract=hako-mimalloc-next-owner-after-local-known-receiver-closeout-v0",
        "source_evidence=296x-822,296x-821",
        "target_front=object_lifecycle_body",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "local_known_receiver_direct_call_lane_closed=1",
        f"current_body_elapsed_ratio={require_key(closeout, 'body_elapsed_ratio')}",
        f"hako_slower_current_front={hako_slower}",
        "current_front_winner_from_previous=1",
        f"selected_next_owner={selected_owner}",
        f"selected_owner_confidence={selected_confidence}",
        "implementation_started=0",
        "new_backend_lowering_code_added=0",
        "storage_direct_enabled=0",
        "hosthandle_bypass_enabled=0",
        "arc_retirement_enabled=0",
        "product_default_changed=0",
        "startup_lane_reopened=0",
        "source_hako_changed=0",
        "winner_claim=0",
        f"next_task={next_task}",
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
