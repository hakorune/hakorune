"""Retired Python-template C replacement-front bridge guard.

The bridge is kept only as an explicit diagnostic baseline while the
replacement-front producer moves to `.hako fastmem -> MIR MemOp -> LLVM`.
Normal runtime paths must not generate it implicitly.
"""

from __future__ import annotations

import argparse

ALLOW_FLAG = "--allow-python-template-c-bridge-baseline"
PRODUCER = "python_template_c_bridge"

RETIREMENT_MESSAGE = (
    "Python-template C replacement front is retired from normal runs; "
    f"pass {ALLOW_FLAG} for an explicit diagnostic baseline"
)


def add_baseline_flag(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        ALLOW_FLAG,
        action="store_true",
        help=(
            "diagnostic-only: explicitly allow the retired Python-template C "
            "replacement-front bridge as a comparison baseline"
        ),
    )


def require_explicit_baseline(allowed: bool) -> None:
    if not allowed:
        raise SystemExit(RETIREMENT_MESSAGE)
