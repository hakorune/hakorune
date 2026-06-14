#!/usr/bin/env python3
"""Pin the CFG-stable owner for dominance-required receiver operand rewrites."""

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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--implementation-rejection", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    rejected = read_kv(args.implementation_rejection)
    label = "implementation-rejection"
    require(
        rejected,
        "output_contract",
        "hako-mimalloc-call-operand-dominance-required-forwarding-implementation-rejected-v0",
        label,
    )
    require(
        rejected,
        "rejected_reason",
        "dominance_required_candidates_need_cfg_stable_rewrite_owner",
        label,
    )
    require(rejected, "source_hako_changed", "0", label)
    require(rejected, "startup_lane_reopened", "0", label)
    require(rejected, "summary", "ok", label)

    lines = [
        "output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-design-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-693",
        "selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite",
        "selected_owner_reason=callsite_canonicalize_already_owns_cfg_stable_callsite_rewrites_in_late_call_and_inline",
        "selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite",
        "pre_selected_keeper_candidate_count=13",
        "post_selected_keeper_candidate_count_target=0",
        "arg_forwarding_enabled=0",
        "requires_cfg_stable_dominance_guard=1",
        "dominance_source=final_mir_cfg_successors",
        "receiver_only_rewrite=1",
        "unknown_root_forwarding_enabled=0",
        "helper_name_special_case=0",
        "variable_map_semantics_changed=0",
        "phi_lifecycle_changed=0",
        "source_hako_changed=0",
        "startup_lane_reopened=0",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
        "next_task=call_operand_cfg_stable_receiver_rewrite_guard_surface",
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
