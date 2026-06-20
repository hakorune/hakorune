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
}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--family",
        required=True,
        choices=sorted(FAMILY_SCRIPTS),
        help="family to convert with lightweight facts",
    )
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()

    script = ROOT / FAMILY_SCRIPTS[args.family]
    cmd = [sys.executable, str(script)]
    if args.check:
        cmd.append("--check")

    completed = subprocess.run(cmd, cwd=ROOT, check=False)
    return completed.returncode


if __name__ == "__main__":
    raise SystemExit(main())
