#!/usr/bin/env python3
"""Normalize the abandoned-heap stress proof pair into one evidence report."""

from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
REMOTE_GUARD = ROOT / "tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh"
RECLAIM_GUARD = ROOT / "tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh"


def read_kv_lines(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
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


def capture_guard(path: Path, label: str) -> dict[str, str]:
    completed = subprocess.run(
        ["bash", str(path)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    if completed.returncode != 0:
        print(completed.stdout, end="")
        raise SystemExit(f"{label} guard failed: {path}")
    values = read_kv_lines(completed.stdout)
    if values.get("summary") != "ok":
        raise SystemExit(f"{label}: summary must be ok")
    return values


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--remote-guard", type=Path, default=REMOTE_GUARD)
    parser.add_argument("--reclaim-guard", type=Path, default=RECLAIM_GUARD)
    args = parser.parse_args()

    if not args.remote_guard.exists():
        raise SystemExit(f"missing remote guard: {args.remote_guard}")
    if not args.reclaim_guard.exists():
        raise SystemExit(f"missing reclaim guard: {args.reclaim_guard}")

    remote = capture_guard(args.remote_guard, "remote")
    reclaim = capture_guard(args.reclaim_guard, "reclaim")

    for key in ("same", "remote", "abandoned", "pending", "counts", "mailbox", "shape"):
        if key not in remote:
            raise SystemExit(f"remote proof missing field: {key}")
    for key in (
        "missing",
        "active_owner",
        "same_owner",
        "remote_pending",
        "decommitted",
        "live",
        "retired",
        "would",
        "counts",
    ):
        if key not in reclaim:
            raise SystemExit(f"reclaim proof missing field: {key}")

    require(remote, "same", "1,1,0", "remote")
    require(remote, "remote", "2,1,1,0", "remote")
    require(remote, "abandoned", "3,1,1,1,1", "remote")
    require(remote, "pending", "0,6,4,3", "remote")
    require(remote, "counts", "4,1,1,1,1", "remote")
    require(remote, "mailbox", "0,0,0", "remote")
    require(remote, "shape", "9", "remote")

    require(reclaim, "missing", "0,1,10,0", "reclaim")
    require(reclaim, "active_owner", "0,2,0,1", "reclaim")
    require(reclaim, "same_owner", "0,2,2,2", "reclaim")
    require(reclaim, "remote_pending", "0,3,3", "reclaim")
    require(reclaim, "decommitted", "0,4,1", "reclaim")
    require(reclaim, "live", "1,0,1,1,1,0", "reclaim")
    require(reclaim, "retired", "1,0,1,1,1", "reclaim")
    require(reclaim, "would", "0,0,0,0,0,0", "reclaim")
    require(reclaim, "counts", "7,2,5,1,2,1,1,1,1,1,16,0", "reclaim")

    lines = [
        "mimalloc_abandoned_heap_stress_evidence_runner=1",
        "output_contract=mimalloc-comparison-abandoned-heap-stress-evidence-v0",
        "evidence_pair=abandoned-owner-policy+abandoned-reclaim-inventory",
        "remote_proof_guard=k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh",
        "reclaim_proof_guard=k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh",
        f"remote_same={remote['same']}",
        f"remote_remote={remote['remote']}",
        f"remote_abandoned={remote['abandoned']}",
        f"remote_pending={remote['pending']}",
        f"remote_counts={remote['counts']}",
        f"remote_mailbox={remote['mailbox']}",
        f"remote_shape={remote['shape']}",
        f"reclaim_missing={reclaim['missing']}",
        f"reclaim_active_owner={reclaim['active_owner']}",
        f"reclaim_same_owner={reclaim['same_owner']}",
        f"reclaim_remote_pending={reclaim['remote_pending']}",
        f"reclaim_decommitted={reclaim['decommitted']}",
        f"reclaim_live={reclaim['live']}",
        f"reclaim_retired={reclaim['retired']}",
        f"reclaim_would={reclaim['would']}",
        f"reclaim_counts={reclaim['counts']}",
        "proof_pair_summary=ok",
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
