#!/usr/bin/env python3
"""Emit the exact-slot NativeDirect guard surface from readiness inventory."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
READINESS = ROOT / "tools" / "allocator" / "typed_object_exact_slot_nativedirect_readiness_inventory.py"


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    with tempfile.TemporaryDirectory(prefix="hakorune_typed_object_nativedirect_guard_surface.") as tmp:
        readiness_report = Path(tmp) / "readiness.out"
        subprocess.run(
            ["python3", str(READINESS), "--out", str(readiness_report)],
            cwd=ROOT,
            check=True,
        )
        values = read_kv(readiness_report)

    tools_dir = ROOT / "tools" / "hako_check"
    import sys

    if str(tools_dir) not in sys.path:
        sys.path.insert(0, str(tools_dir))

    from typed_object_exact_slot_inventory import (
        typed_object_exact_slot_nativedirect_guard_surface_inventory,
    )

    report = typed_object_exact_slot_nativedirect_guard_surface_inventory(values)

    text = "\n".join(f"{key}={value}" for key, value in report.items()) + "\n"
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(text, encoding="utf-8")
    print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
