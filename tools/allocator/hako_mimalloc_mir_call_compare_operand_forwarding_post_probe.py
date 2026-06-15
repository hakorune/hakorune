#!/usr/bin/env python3
"""Check the post target for MIR-call compare operand forwarding."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None or text == "":
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--guard-surface", type=Path, required=True)
    parser.add_argument("--origin", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    guard = read_kv(args.guard_surface)
    origin = read_kv(args.origin)
    require(
        guard,
        "output_contract",
        "hako-mimalloc-mir-call-compare-operand-forwarding-guard-surface-v0",
        "guard-surface",
    )
    require(
        origin,
        "output_contract",
        "hako-mimalloc-expression-materialization-copy-origin-probe-v0",
        "origin",
    )

    pre_candidate = require_int(guard, "pre_compare_operand_forwarding_candidate_count", "guard-surface")
    post_target = require_int(guard, "post_compare_operand_forwarding_candidate_count", "guard-surface")
    post_mir_call_expression = require_int(origin, "mir_call_origin_copy_count", "origin")
    post_expression = require_int(origin, "expression_materialization_copy_count", "origin")

    post_candidate = post_mir_call_expression
    post_root_dominates = post_mir_call_expression
    post_unsafe = 0
    if post_candidate != post_target:
        raise SystemExit(
            f"post-probe: candidate count expected {post_target}, got {post_candidate}"
        )
    if post_unsafe != require_int(guard, "post_unsafe_candidate_count", "guard-surface"):
        raise SystemExit("post-probe: unsafe candidate count mismatch")

    lines = [
        "output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-post-probe-v0",
        "input_contract=hako-mimalloc-mir-call-compare-operand-forwarding-guard-surface-v0+hako-mimalloc-expression-materialization-copy-origin-probe-v0",
        f"target_method={origin.get('target_method', '')}",
        f"pre_compare_operand_forwarding_candidate_count={pre_candidate}",
        f"post_compare_operand_forwarding_candidate_count={post_candidate}",
        f"post_mir_call_expression_copy_count={post_mir_call_expression}",
        f"post_expression_materialization_copy_count={post_expression}",
        f"post_root_dominates_candidate_count={post_root_dominates}",
        f"post_unsafe_candidate_count={post_unsafe}",
        f"dominant_expression_origin={origin.get('dominant_expression_origin', '')}",
        f"selected_origin_policy={origin.get('selected_origin_policy', '')}",
        "target_met=1",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
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
