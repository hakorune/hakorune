#!/usr/bin/env python3
"""Inventory typed-object exact-slot NativeDirect readiness from MIR JSON."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"


def emit_mir_json(path: Path) -> None:
    subprocess.run(
        [
            str(ROOT / "target/release/hakorune"),
            "--backend",
            "mir",
            "--emit-mir-json",
            str(path),
            str(APP),
        ],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        check=True,
    )


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be object")
    return data


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    subprocess.run(["cargo", "build", "--release", "--bin", "hakorune"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory(prefix="hakorune_typed_object_nativedirect_readiness.") as tmp:
        mir_json = args.mir_json or Path(tmp) / "app.mir.json"
        if args.mir_json is None:
            emit_mir_json(mir_json)
        mir = load_json(mir_json)

    tools_dir = ROOT / "tools" / "hako_check"
    import sys

    if str(tools_dir) not in sys.path:
        sys.path.insert(0, str(tools_dir))

    from typed_object_exact_slot_inventory import (
        typed_object_exact_slot_nativedirect_readiness_inventory,
    )

    report = typed_object_exact_slot_nativedirect_readiness_inventory(mir)
    report.update(
        {
            "output_contract": "typed-object-exact-slot-nativedirect-readiness-inventory-v0",
            "input_contract": "typed-object-exact-slot-direct-helper-measurement-v0",
            "workload_id": "representative-object-lifecycle-small-block-v0",
            "candidate_representation": "NativeDirect",
            "storage_substrate": "PinnedTypedObjectArena",
            "fallback_boundary": "explicit_materialized_view_handle",
            "selected_next": "typed_object_exact_slot_nativedirect_guard_surface",
            "optimization_open": "0",
            "winner_claim": "0",
            "replacement_active": "0",
            "hook_installed": "0",
            "global_allocator": "0",
            "summary": "ok",
        }
    )

    lines = [f"{key}={value}" for key, value in report.items()]
    text = "\n".join(lines) + "\n"
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(text, encoding="utf-8")
    print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
