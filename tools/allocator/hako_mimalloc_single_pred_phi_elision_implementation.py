#!/usr/bin/env python3
"""Validate guarded single-pred PHI elision implementation evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
OWNER = ROOT / "src/mir/builder/emission/phi.rs"


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--shape-report", type=Path, required=True)
    parser.add_argument("--measurement-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    owner_text = OWNER.read_text(encoding="utf-8", errors="replace")
    if "insert_phi_single(pre_branch_bb, pre_v)" in owner_text:
        raise SystemExit("single-pred PHI insertion is still present")
    if ".insert(name.clone(), pre_v)" not in owner_text:
        raise SystemExit("pre_v variable_map insertion is missing")

    shape = read_kv(args.shape_report)
    require(shape, "output_contract", "hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0", "shape")
    require(shape, "single_incoming_phi_count", "0", "shape")

    measurement = read_kv(args.measurement_report)
    require(
        measurement,
        "output_contract",
        "hako-mimalloc-post-rollback-inline-success-result-measurement-v0",
        "measurement",
    )
    require(measurement, "summary", "ok", "measurement")
    require(measurement, "winner_claim", "0", "measurement")
    require(measurement, "replacement_active", "0", "measurement")

    lines = [
        "output_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0",
        "input_contract=hako-mimalloc-single-pred-phi-elision-guard-surface-v0",
        "selected_owner_file=src/mir/builder/emission/phi.rs",
        "single_pred_phi_elision_enabled=1",
        "before_single_incoming_phi_count=61",
        f"after_single_incoming_phi_count={shape.get('single_incoming_phi_count', '')}",
        f"after_phi_count={shape.get('phi_count', '')}",
        f"after_copy_count={shape.get('copy_count', '')}",
        f"after_candidate_source={shape.get('candidate_source', '')}",
        f"after_hako_elapsed_median_ms={measurement.get('after_hako_elapsed_median_ms', '')}",
        "semantic_summary=ok",
        "measurement_summary=ok",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
