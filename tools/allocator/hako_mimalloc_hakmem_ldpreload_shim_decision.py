#!/usr/bin/env python3
"""Emit the hakmem LD_PRELOAD shim decision after provider evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


INPUT_CARD = "docs/development/current/main/phases/phase-296x/296x-71-HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-EXPLICIT-MEASUREMENT.md"


def require_input(root: Path) -> None:
    path = root / INPUT_CARD
    if not path.is_file():
        raise SystemExit(f"missing input card: {INPUT_CARD}")
    text = path.read_text(encoding="utf-8", errors="replace")
    for needle in (
        "output_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0",
        "provider_explicit_measurement_ready=1",
        "ld_preload_decision_ready=1",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "summary=ok",
    ):
        if needle not in text:
            raise SystemExit(f"input card missing required evidence: {needle}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    require_input(args.repo_root.resolve())

    lines = [
        "output_contract=hako-mimalloc-hakmem-ldpreload-shim-decision-v0",
        "input_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0",
        "ld_preload_shim_decision=accepted",
        "decision_scope=hakmem_compat_probe_only",
        "decision_reason=provider_explicit_measurement_ready_and_hakmem_existing_scripts_need_malloc_free_symbol_surface",
        "provider_call_evidence_ready=1",
        "provider_explicit_measurement_ready=1",
        "ld_preload_shim_build_allowed=1",
        "ld_preload_shim_ready=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE-296X-001",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
