#!/usr/bin/env python3
"""Decide whether MIR method shape observation should migrate to .hako now."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-mir-method-shape-hako-migration-selection-v0",
        "input_contract=hako-source-mir-shape-join-v0",
        "python_contract_stable=0",
        "hako_migration_decision=parked",
        "park_reason=python_mir_shape_contract_needs_multi_method_use_before_hako_port",
        "selected_scope=python_adapter_continues_multi_method_observation",
        "next_row=HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001",
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
