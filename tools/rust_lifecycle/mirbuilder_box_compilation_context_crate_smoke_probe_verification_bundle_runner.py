#!/usr/bin/env python3
"""Run the landed BoxCompilationContext crate-smoke consultation bundle."""

from __future__ import annotations

import json
import os
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
BUNDLE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-probe-verification-bundle-v0.json"


def main() -> int:
    bundle = json.loads(BUNDLE.read_text())
    env = os.environ.copy()
    env["MIRBUILDER_CRATE_SMOKE_BUNDLE_MODE"] = "1"
    executed = 0
    for command in bundle["bundle_commands"]:
        subprocess.run(command, shell=True, check=True, cwd=ROOT, env=env)
        executed += 1

    print("output_contract=rust-mirbuilder-box-compilation-context-crate-smoke-probe-verification-bundle-v0")
    print("subject=BoxCompilationContext")
    print("crate_level_probe_candidate=BoxCompilationContext")
    print("decision=probe_verification_bundle_only")
    print(f"bundle_commands_executed={executed}")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
