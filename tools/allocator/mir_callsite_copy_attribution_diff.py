#!/usr/bin/env python3
"""Diff two MIR callsite-copy attribution reports."""

from __future__ import annotations

import argparse
from pathlib import Path


COUNT_KEYS = [
    "block_count",
    "instruction_count",
    "call_count",
    "copy_count",
    "phi_count",
    "helper_call_count",
    "helper_copy_count",
    "receiver_copy_count",
    "arg_copy_count",
    "result_copy_count",
    "local_ssa_copy_count",
    "phi_edge_copy_count",
    "unknown_copy_count",
    "owner_local_ssa_copy_materialization_copy_count",
    "owner_receiver_materialization_copy_count",
    "owner_phi_edge_copy_materialization_copy_count",
    "owner_result_materialization_copy_count",
    "owner_arg_materialization_copy_count",
    "owner_unknown_copy_materialization_copy_count",
    "page_hotpath_helpers_call_count",
    "page_hotpath_helpers_attributed_copy_count",
    "facade_result_helpers_call_count",
    "facade_result_helpers_attributed_copy_count",
    "facade_state_helpers_call_count",
    "facade_state_helpers_attributed_copy_count",
    "other_call_count",
    "other_attributed_copy_count",
]

IMPROVEMENT_KEYS = {
    "instruction_count",
    "call_count",
    "copy_count",
    "phi_count",
    "helper_call_count",
    "helper_copy_count",
    "receiver_copy_count",
    "arg_copy_count",
    "result_copy_count",
    "local_ssa_copy_count",
    "phi_edge_copy_count",
    "owner_local_ssa_copy_materialization_copy_count",
    "owner_receiver_materialization_copy_count",
    "owner_phi_edge_copy_materialization_copy_count",
    "owner_result_materialization_copy_count",
    "owner_arg_materialization_copy_count",
    "page_hotpath_helpers_attributed_copy_count",
    "facade_result_helpers_attributed_copy_count",
}


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str, default: int = 0) -> int:
    text = values.get(key)
    if text is None or text == "":
        return default
    try:
        return int(text)
    except ValueError:
        return default


def require_contract(values: dict[str, str], path: Path) -> None:
    contract = values.get("output_contract", "")
    if contract != "hako-mimalloc-callsite-copy-attribution-v0":
        raise SystemExit(f"{path}: expected hako-mimalloc-callsite-copy-attribution-v0, got {contract!r}")
    if values.get("summary") != "ok":
        raise SystemExit(f"{path}: expected summary=ok")


def classify_effect(deltas: dict[str, int]) -> str:
    improved = any(deltas.get(key, 0) < 0 for key in IMPROVEMENT_KEYS)
    regressed = any(deltas.get(key, 0) > 0 for key in IMPROVEMENT_KEYS)
    if improved and not regressed:
        return "improved"
    if regressed and not improved:
        return "regressed"
    if improved and regressed:
        return "mixed"
    return "no_effect"


def selected_owner(before: dict[str, str], deltas: dict[str, int]) -> str:
    dominant = before.get("dominant_copy_owner", "none")
    owner_key = f"owner_{dominant}_copy_count"
    if owner_key in deltas:
        return dominant
    best_key = "none"
    best_abs = 0
    for key, delta in deltas.items():
        if not key.startswith("owner_") or not key.endswith("_copy_count"):
            continue
        if abs(delta) > best_abs:
            best_key = key.removeprefix("owner_").removesuffix("_copy_count")
            best_abs = abs(delta)
    return best_key


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--candidate-id", default="unknown")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    before = read_kv(args.before)
    after = read_kv(args.after)
    require_contract(before, args.before)
    require_contract(after, args.after)

    target_before = before.get("target_method", "")
    target_after = after.get("target_method", "")
    if target_before != target_after:
        raise SystemExit(f"target_method mismatch: before={target_before!r} after={target_after!r}")

    deltas = {key: as_int(after, key) - as_int(before, key) for key in COUNT_KEYS}
    effect = classify_effect(deltas)
    owner = selected_owner(before, deltas)
    exact_exe_required = int(effect in {"improved", "mixed"})

    lines = [
        "output_contract=hako-mimalloc-callsite-copy-attribution-diff-v0",
        "input_contract=hako-mimalloc-callsite-copy-attribution-v0",
        f"candidate_id={args.candidate_id}",
        f"target_method={target_before}",
        f"before_dominant_callee_family={before.get('dominant_callee_family', 'none')}",
        f"after_dominant_callee_family={after.get('dominant_callee_family', 'none')}",
        f"before_dominant_copy_owner={before.get('dominant_copy_owner', 'none')}",
        f"after_dominant_copy_owner={after.get('dominant_copy_owner', 'none')}",
        f"selected_delta_owner={owner}",
        f"structural_effect={effect}",
        f"exact_exe_required={exact_exe_required}",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key in COUNT_KEYS:
        lines.extend(
            [
                f"before_{key}={as_int(before, key)}",
                f"after_{key}={as_int(after, key)}",
                f"delta_{key}={deltas[key]}",
            ]
        )
    lines.append("summary=ok")

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
