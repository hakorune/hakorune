#!/usr/bin/env python3
"""Check FastPath / RouteDecision profile reports.

This is a thin CI adapter over `fastpath_explain.py --format json`. It does
not infer routes from source or MIR by itself.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
EXPLAIN = ROOT / "tools" / "hako_check" / "fastpath_explain.py"


def int_count(counts: dict[str, Any], key: str) -> int:
    value = counts.get(key, "0")
    try:
        return int(value)
    except (TypeError, ValueError):
        return 0


def run_explain(args: argparse.Namespace) -> dict[str, Any]:
    cmd = [
        sys.executable,
        str(EXPLAIN),
        "--mir-json",
        str(args.mir_json),
        "--format",
        "json",
        "--profile",
        args.profile,
    ]
    if args.group:
        cmd.extend(["--group", args.group])
    if args.method:
        cmd.extend(["--method", args.method])
    proc = subprocess.run(cmd, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        if proc.stderr:
            sys.stderr.write(proc.stderr)
        if proc.stdout:
            sys.stderr.write(proc.stdout)
        raise SystemExit(proc.returncode)
    return json.loads(proc.stdout)


def failure_reasons(counts: dict[str, Any]) -> list[str]:
    reasons: list[str] = []
    if int_count(counts, "route_tier_failed_count") > 0:
        reasons.append("route_tier_failed_count")
    if int_count(counts, "fastpath_obligation_failed_count") > 0:
        reasons.append("fastpath_obligation_failed_count")
    if int_count(counts, "direct_exact_plan_lowered_to_fallback_count") > 0:
        reasons.append("direct_exact_plan_lowered_to_fallback_count")
    return reasons


def render(counts: dict[str, Any], reasons: list[str]) -> str:
    status = "OK" if not reasons else "FAILED"
    matched = int_count(counts, "route_decision_count") > 0
    lines = [
        f"FastPath check: {status}",
        "",
        "Profile",
        f"  profile: {counts.get('selected_profile', 'unknown')}",
        f"  group: {counts.get('selected_group', 'unknown')}",
        f"  policy: {counts.get('profile_policy', 'unknown')}",
        f"  required tier: {counts.get('default_required_tier', 'unknown')}",
        f"  severity: {counts.get('default_severity', 'unknown')}",
        "",
        "Route Tiers",
        f"  decisions: {counts.get('route_decision_count', '0')}",
        f"  ok: {counts.get('route_tier_ok_count', '0')}",
        f"  failed: {counts.get('route_tier_failed_count', '0')}",
        f"  slow selected: {counts.get('route_decision_slow_selected_count', '0')}",
        f"  selected checked direct: {counts.get('selected_tier_checked_direct_count', '0')}",
        f"  selected proved direct: {counts.get('selected_tier_proved_direct_count', '0')}",
        f"  selected static exact: {counts.get('selected_tier_static_exact_call_count', '0')}",
        f"  selected replacement thin: {counts.get('selected_tier_replacement_thin_count', '0')}",
        "",
        "Fallbacks",
        f"  failed obligations: {counts.get('fastpath_obligation_failed_count', '0')}",
        f"  lowering fallback: {counts.get('direct_exact_plan_lowered_to_fallback_count', '0')}",
        f"  generic method dispatch: {counts.get('generic_method_dispatch_count', '0')}",
        f"  dynamic route: {counts.get('dynamic_route_count', '0')}",
        f"  boxed fallback: {counts.get('boxed_fallback_count', '0')}",
    ]
    if reasons:
        lines.extend(["", "Failures"])
        for idx, reason in enumerate(reasons):
            lines.append(f"  {idx + 1}. {reason}")
    if not matched:
        lines.extend(
            [
                "",
                "Note",
                "  no RouteDecision rows matched this profile/group selection",
            ]
        )
    lines.extend(
        [
            "",
            "Machine",
            "  output_contract=hako-check-fastpath-check-v0",
            f"  failure_count={len(reasons)}",
        ]
    )
    for idx, reason in enumerate(reasons):
        lines.append(f"  failure_{idx}_reason={reason}")
    lines.append("  summary=ok" if not reasons else "  summary=failed")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument(
        "--profile",
        choices=("default", "hot-report", "direct-memory", "direct-exact", "replacement-front"),
        default="replacement-front",
    )
    parser.add_argument("--group")
    parser.add_argument("--method")
    args = parser.parse_args()

    payload = run_explain(args)
    counts = payload.get("counts")
    if not isinstance(counts, dict):
        raise SystemExit("fastpath explain JSON missing counts object")
    reasons = failure_reasons(counts)
    print(render(counts, reasons), end="")
    return 1 if reasons else 0


if __name__ == "__main__":
    raise SystemExit(main())
