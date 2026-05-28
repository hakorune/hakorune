#!/usr/bin/env python3
"""Select the next MIR body owner from body-gap taxonomy and attribution."""

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


def require_key(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"{label}: missing {key}")
    return value


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = require_key(values, key, label)
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def choose_secondary_owner(attribution: dict[str, str]) -> str:
    page_hotpath = require_int(attribution, "page_hotpath_helpers_attributed_copy_count", "attribution")
    receiver = require_int(attribution, "owner_receiver_materialization_copy_count", "attribution")
    result = require_int(attribution, "owner_result_materialization_copy_count", "attribution")
    if page_hotpath >= receiver and page_hotpath >= result:
        return "page_hotpath_helper_result_chain"
    if receiver >= result:
        return "receiver_materialization"
    return "result_materialization"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--taxonomy", type=Path, required=True)
    parser.add_argument("--attribution", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    taxonomy = read_kv(args.taxonomy)
    attribution = read_kv(args.attribution)
    require(taxonomy, "output_contract", "hako-mimalloc-object-lifecycle-body-timing-gap-taxonomy-v0", "taxonomy")
    require(taxonomy, "gap_owner", "compiler_lowering", "taxonomy")
    require(taxonomy, "summary", "ok", "taxonomy")
    require(attribution, "output_contract", "hako-mimalloc-callsite-copy-attribution-v0", "attribution")
    require(attribution, "target_method", "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1", "attribution")
    require(attribution, "summary", "ok", "attribution")
    require(attribution, "dominant_copy_owner", "local_ssa_copy_materialization", "attribution")

    local_ssa = require_int(attribution, "owner_local_ssa_copy_materialization_copy_count", "attribution")
    receiver = require_int(attribution, "owner_receiver_materialization_copy_count", "attribution")
    page_hotpath = require_int(attribution, "page_hotpath_helpers_attributed_copy_count", "attribution")
    copy_count = require_int(attribution, "copy_count", "attribution")
    if local_ssa <= 0:
        raise SystemExit("attribution: local SSA copy count must be positive")
    confidence = "medium"
    reason = "dominant_copy_owner_under_large_body_gap"
    if local_ssa >= receiver * 2 and local_ssa >= page_hotpath * 2:
        confidence = "high"
        reason = "local_ssa_copy_owner_strongly_dominant_under_large_body_gap"

    lines = [
        "output_contract=hako-mimalloc-object-lifecycle-mir-body-owner-selection-v0",
        "input_contract=hako-mimalloc-object-lifecycle-body-timing-gap-taxonomy-v0+hako-mimalloc-callsite-copy-attribution-v0",
        f"workload_id={require_key(taxonomy, 'workload_id', 'taxonomy')}",
        f"target_method={require_key(attribution, 'target_method', 'attribution')}",
        f"body_gap_owner={require_key(taxonomy, 'gap_owner', 'taxonomy')}",
        f"body_gap_confidence={require_key(taxonomy, 'gap_confidence', 'taxonomy')}",
        f"body_elapsed_ratio={require_key(taxonomy, 'body_elapsed_ratio', 'taxonomy')}",
        f"instruction_count={require_key(attribution, 'instruction_count', 'attribution')}",
        f"copy_count={copy_count}",
        f"call_count={require_key(attribution, 'call_count', 'attribution')}",
        f"phi_count={require_key(attribution, 'phi_count', 'attribution')}",
        f"local_ssa_copy_count={local_ssa}",
        f"receiver_copy_count={require_key(attribution, 'receiver_copy_count', 'attribution')}",
        f"page_hotpath_helpers_attributed_copy_count={page_hotpath}",
        f"top_callsite_callee={require_key(attribution, 'callsite_0_callee', 'attribution')}",
        f"top_callsite_family={require_key(attribution, 'callsite_0_callee_family', 'attribution')}",
        f"top_callsite_attributed_copy_count={require_key(attribution, 'callsite_0_attributed_copy_count', 'attribution')}",
        "selected_mir_body_owner=local_ssa_copy_materialization",
        f"selected_owner_confidence={confidence}",
        f"selected_owner_reason={reason}",
        f"secondary_mir_body_owner={choose_secondary_owner(attribution)}",
        "rejected_recent_nonkeeper=local_ssa_same_block_field_get_reuse",
        "rejected_reason=prior_structural_win_regressed_exact_exe_body",
        "next_diagnostic=local_ssa_dynamic_weight_probe",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
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
