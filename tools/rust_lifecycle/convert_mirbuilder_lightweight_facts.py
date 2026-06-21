#!/usr/bin/env python3
"""One-command entrypoint for the MirBuilder lightweight-facts converters."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
FAMILY_SCRIPTS = {
    "binding-context": "tools/rust_lifecycle/generate_binding_context_artifact.py",
    "variable-context-simple-map": "tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py",
    "variable-context-snapshot-restore": "tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py",
    "variable-context-carrier-snapshot": "tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py",
    "variable-context-explicit-carrier-snapshot": "tools/rust_lifecycle/generate_variable_context_explicit_carrier_snapshot_artifact.py",
}
FAMILY_ORDER = tuple(FAMILY_SCRIPTS)


def _run_family(name: str, *, check: bool) -> int:
    script = ROOT / FAMILY_SCRIPTS[name]
    cmd = [sys.executable, str(script)]
    if check:
        cmd.append("--check")
    completed = subprocess.run(cmd, cwd=ROOT, check=False)
    return completed.returncode


def main() -> int:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "--family",
        choices=sorted(FAMILY_SCRIPTS),
        help="family to convert with lightweight facts",
    )
    group.add_argument("--all", action="store_true", help="run every lightweight converter family")
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()

    if args.all:
        for name in FAMILY_ORDER:
            return_code = _run_family(name, check=args.check)
            if return_code != 0:
                return return_code
        return 0
    return _run_family(args.family, check=args.check)


if __name__ == "__main__":
    raise SystemExit(main())
