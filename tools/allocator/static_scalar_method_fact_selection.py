#!/usr/bin/env python3
"""Select the first narrow static-scalar method fact boundary."""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


METHOD_RE = re.compile(
    r"(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\(\s*\)\s*\{\s*return\s+(?P<value>-?[0-9]+)\s*\}",
    re.MULTILINE,
)


@dataclass(frozen=True)
class Candidate:
    name: str
    value: str

    @property
    def symbol(self) -> str:
        return f"HakoAllocObjectLifecycleFacadeReason.{self.name}/0"


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as exc:
        raise SystemExit(f"failed to read source: {path}: {exc}") from exc


def require_static_box(source: str) -> None:
    if "static box HakoAllocObjectLifecycleFacadeReason" not in source:
        raise SystemExit("source must contain static box HakoAllocObjectLifecycleFacadeReason")


def candidates(source: str) -> list[Candidate]:
    return [Candidate(match.group("name"), match.group("value")) for match in METHOD_RE.finditer(source)]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = read_text(args.source)
    require_static_box(source)
    selected = candidates(source)
    if not selected:
        raise SystemExit("no zero-arg return-literal candidates found")

    lines = [
        "output_contract=static-scalar-method-fact-selection-v0",
        "input_contract=mir-builder-single-eval-surface-sweep-v0",
        "candidate_family=object_lifecycle_facade_reason_zero_arg_return_literal_i64",
        "selection=verified_static_method_return_literal_shape",
        "scope=same_source_static_box_only",
        "generic_cse=0",
        "whole_box_pure=0",
        "const_lowering=0",
        "failure_mode=keep_call",
        f"candidate_count={len(selected)}",
    ]
    for idx, candidate in enumerate(selected):
        lines.append(f"candidate_{idx}_symbol={candidate.symbol}")
        lines.append(f"candidate_{idx}_return_literal={candidate.value}")
        lines.append(f"candidate_{idx}_verified=1")

    lines.extend(
        [
            "selected_next=static_scalar_method_fact_inference",
            "summary=ok",
        ]
    )
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
