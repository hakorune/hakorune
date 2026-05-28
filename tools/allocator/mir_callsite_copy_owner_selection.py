#!/usr/bin/env python3
"""Select the next optimization owner from callsite-copy attribution evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


OWNER_KEYS = {
    "local_ssa_copy_materialization": "owner_local_ssa_copy_materialization_copy_count",
    "receiver_materialization": "owner_receiver_materialization_copy_count",
    "phi_edge_copy_materialization": "owner_phi_edge_copy_materialization_copy_count",
    "result_materialization": "owner_result_materialization_copy_count",
    "arg_materialization": "owner_arg_materialization_copy_count",
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


def require_contract(values: dict[str, str], path: Path, expected: str) -> None:
    contract = values.get("output_contract", "")
    if contract != expected:
        raise SystemExit(f"{path}: expected {expected}, got {contract!r}")
    if values.get("summary") != "ok":
        raise SystemExit(f"{path}: expected summary=ok")


def choose_owner(attribution: dict[str, str], diff: dict[str, str]) -> tuple[str, str, str]:
    structural_effect = diff.get("structural_effect", "not_available")
    if structural_effect in {"improved", "mixed"}:
        owner = diff.get("selected_delta_owner", "none")
        return owner, "medium", "candidate_delta_available"

    local_ssa = as_int(attribution, OWNER_KEYS["local_ssa_copy_materialization"])
    receiver = as_int(attribution, OWNER_KEYS["receiver_materialization"])
    result = as_int(attribution, OWNER_KEYS["result_materialization"])
    page_hotpath = as_int(attribution, "page_hotpath_helpers_attributed_copy_count")

    if local_ssa >= receiver and local_ssa >= page_hotpath:
        return "local_ssa_copy_materialization", "medium", "dominant_baseline_copy_owner"
    if receiver >= result and receiver >= page_hotpath:
        return "receiver_materialization", "medium", "receiver_copy_pressure"
    if page_hotpath > 0:
        return "method_call_route_lowering", "low", "page_hotpath_helper_attribution"
    return "measurement_harness", "low", "no_dominant_mir_owner"


def next_diagnostic(owner: str) -> str:
    if owner == "local_ssa_copy_materialization":
        return "local_ssa_block_position_probe"
    if owner == "receiver_materialization":
        return "receiver_copy_chain_probe"
    if owner == "method_call_route_lowering":
        return "same_module_route_lowering_probe"
    if owner == "verified_helper_inline":
        return "helper_inline_eligibility_probe"
    return "measurement_harness_refresh"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--attribution", type=Path, required=True)
    parser.add_argument("--diff", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    attribution = read_kv(args.attribution)
    require_contract(
        attribution,
        args.attribution,
        "hako-mimalloc-callsite-copy-attribution-v0",
    )
    diff: dict[str, str] = {}
    if args.diff is not None:
        diff = read_kv(args.diff)
        require_contract(
            diff,
            args.diff,
            "hako-mimalloc-callsite-copy-attribution-diff-v0",
        )

    owner, confidence, reason = choose_owner(attribution, diff)
    diagnostic = next_diagnostic(owner)

    lines = [
        "output_contract=hako-mimalloc-callsite-copy-owner-selection-v0",
        "input_contract=hako-mimalloc-callsite-copy-attribution-v0",
        f"diff_contract={diff.get('output_contract', 'not_available')}",
        f"target_method={attribution.get('target_method', '')}",
        f"dominant_callee_family={attribution.get('dominant_callee_family', 'none')}",
        f"dominant_copy_owner={attribution.get('dominant_copy_owner', 'none')}",
        f"diff_structural_effect={diff.get('structural_effect', 'not_available')}",
        f"selected_owner={owner}",
        f"owner_confidence={confidence}",
        f"owner_reason={reason}",
        f"next_diagnostic={diagnostic}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for owner_name, key in OWNER_KEYS.items():
        lines.append(f"{owner_name}_copy_count={as_int(attribution, key)}")
    lines.extend(
        [
            f"page_hotpath_helpers_attributed_copy_count={as_int(attribution, 'page_hotpath_helpers_attributed_copy_count')}",
            f"top_callsite_callee={attribution.get('callsite_0_callee', 'none')}",
            f"top_callsite_family={attribution.get('callsite_0_callee_family', 'none')}",
            f"top_callsite_attributed_copy_count={as_int(attribution, 'callsite_0_attributed_copy_count')}",
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
