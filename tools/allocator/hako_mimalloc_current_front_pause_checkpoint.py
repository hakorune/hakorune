#!/usr/bin/env python3
"""Checkpoint the current mimalloc body-timing front as paused."""

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
    parser.add_argument("--owner-selection-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    selection = read_kv(args.owner_selection_report)
    require(
        selection,
        "output_contract",
        "hako-mimalloc-next-owner-after-local-known-receiver-closeout-v0",
    )
    require(selection, "hako_slower_current_front", "0")
    require(selection, "selected_next_owner", "none_current_front_not_hako_slower")
    require(selection, "implementation_started", "0")
    require(selection, "new_backend_lowering_code_added", "0")
    require(selection, "storage_direct_enabled", "0")
    require(selection, "hosthandle_bypass_enabled", "0")
    require(selection, "arc_retirement_enabled", "0")
    require(selection, "product_default_changed", "0")
    require(selection, "summary", "ok")

    lines = [
        "output_contract=hako-mimalloc-current-front-optimization-pause-checkpoint-v0",
        "source_evidence=296x-823,296x-822,296x-821",
        "target_front=object_lifecycle_body",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        f"body_elapsed_ratio={require_key(selection, 'current_body_elapsed_ratio')}",
        "current_front_paused=1",
        "pause_reason=current_front_not_hako_slower",
        "local_known_receiver_direct_call_lane_closed=1",
        "implementation_owner_selected=0",
        "implementation_started=0",
        "new_backend_lowering_code_added=0",
        "storage_direct_enabled=0",
        "hosthandle_bypass_enabled=0",
        "arc_retirement_enabled=0",
        "product_default_changed=0",
        "fresh_front_selection_allowed=1",
        "remeasure_if_environment_changes=1",
        "no_current_front_patch_without_new_evidence=1",
        "selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001",
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
