#!/usr/bin/env python3
"""Explain the visible MIR optimizer schedule as a read-only hako_check report."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


OUTPUT_CONTRACT = "hako-check-optimizer-schedule-v0"
TOOL_SURFACE = "hako_check_optimizer_schedule"
TRUTH_SOURCE = "src/mir/optimizer/core.rs::MIR_OPT_PIPELINE_GROUPS"
EXPECTED_GROUPS = [
    "normalize_frontend_surface",
    "placement_effect_pre",
    "canonical_simplification",
    "memory_cleanup_wave",
    "placement_effect_post",
    "late_call_and_inline",
    "optional_and_diagnostics",
]


def repo_root_from_script() -> Path:
    return Path(__file__).resolve().parents[2]


def read_schedule_groups(repo_root: Path) -> list[str]:
    source = repo_root / "src/mir/optimizer/core.rs"
    text = source.read_text(encoding="utf-8")
    match = re.search(
        r"pub const MIR_OPT_PIPELINE_GROUPS:\s*&\[&str\]\s*=\s*&\[(?P<body>.*?)\];",
        text,
        flags=re.S,
    )
    if not match:
        raise RuntimeError(f"missing MIR_OPT_PIPELINE_GROUPS in {source}")
    return re.findall(r'"([^"]+)"', match.group("body"))


def report_lines(groups: list[str]) -> list[str]:
    lines = [
        f"output_contract={OUTPUT_CONTRACT}",
        f"tool_surface={TOOL_SURFACE}",
        "observation_only=1",
        "rewrite_executed=0",
        "keeper_selection=0",
        f"optimizer_schedule_truth_source={TRUTH_SOURCE}",
        "hako_check_optimizer_truth_count=0",
        "optimizer_behavior_changed=0",
        "optimizer_physical_pass_merge_count=0",
        "optimizer_schedule_group_source=rust_const",
        f"visible_group_count={len(groups)}",
        f"expected_visible_group_count={len(EXPECTED_GROUPS)}",
        f"visible_group_order_matches_expected={1 if groups == EXPECTED_GROUPS else 0}",
    ]
    for idx, group in enumerate(groups):
        lines.append(f"schedule_group[{idx}]={group}")
    lines.append("summary=ok" if groups == EXPECTED_GROUPS else "summary=fail")
    return lines


def summary_lines(groups: list[str]) -> list[str]:
    status = "ok" if groups == EXPECTED_GROUPS else "fail"
    lines = [
        f"Optimizer schedule: {status}",
        f"truth_source: {TRUTH_SOURCE}",
        f"visible_groups: {len(groups)}",
    ]
    lines.extend(f"{idx}. {group}" for idx, group in enumerate(groups))
    return lines


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=repo_root_from_script())
    parser.add_argument("--out", type=Path)
    parser.add_argument("--format", choices=["kv", "summary"], default="kv")
    args = parser.parse_args()

    groups = read_schedule_groups(args.repo_root)
    lines = report_lines(groups) if args.format == "kv" else summary_lines(groups)
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0 if groups == EXPECTED_GROUPS else 1


if __name__ == "__main__":
    raise SystemExit(main())
