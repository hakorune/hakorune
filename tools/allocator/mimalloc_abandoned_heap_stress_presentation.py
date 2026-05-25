#!/usr/bin/env python3
"""Format abandoned-heap stress evidence into a stable presentation contract."""

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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    report = read_kv(args.report)
    require(report, "output_contract", "mimalloc-comparison-abandoned-heap-stress-evidence-v0")
    require(report, "evidence_pair", "abandoned-owner-policy+abandoned-reclaim-inventory")
    require(report, "proof_pair_summary", "ok")
    require(report, "summary", "ok")
    require(report, "winner_claim", "0")

    lines = [
        "mimalloc_abandoned_heap_stress_presentation=1",
        "output_contract=mimalloc-comparison-abandoned-heap-stress-presentation-v0",
        "input_contract=mimalloc-comparison-abandoned-heap-stress-evidence-v0",
        "presentation_only=1",
        f"evidence_pair={report['evidence_pair']}",
        f"remote_same={report['remote_same']}",
        f"remote_remote={report['remote_remote']}",
        f"remote_abandoned={report['remote_abandoned']}",
        f"remote_pending={report['remote_pending']}",
        f"remote_counts={report['remote_counts']}",
        f"remote_mailbox={report['remote_mailbox']}",
        f"remote_shape={report['remote_shape']}",
        f"reclaim_missing={report['reclaim_missing']}",
        f"reclaim_active_owner={report['reclaim_active_owner']}",
        f"reclaim_same_owner={report['reclaim_same_owner']}",
        f"reclaim_remote_pending={report['reclaim_remote_pending']}",
        f"reclaim_decommitted={report['reclaim_decommitted']}",
        f"reclaim_live={report['reclaim_live']}",
        f"reclaim_retired={report['reclaim_retired']}",
        f"reclaim_would={report['reclaim_would']}",
        f"reclaim_counts={report['reclaim_counts']}",
        "provider_activation=0",
        "host_replacement=0",
        "hook_installed=0",
        "global_allocator_installed=0",
        "winner_claim=0",
        "summary=ok",
    ]
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
