#!/usr/bin/env python3
"""Inventory LocalSSA block-entry / PHI-edge copy candidates conservatively."""

from __future__ import annotations

import argparse
from collections import Counter
from pathlib import Path
from typing import Any

from mir_local_ssa_copy_position_probe import (
    alias_values,
    block_instructions,
    classify_copy,
    collect_call_attributed_copy_dsts,
    collect_route_carrier_roles,
    find_function,
    load_json,
)


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


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
        raise SystemExit(f"{label}: {key} must be integer, got {text!r}") from exc


def role_key(roles: set[str]) -> str:
    if not roles:
        return "none"
    if "field_set_value" in roles:
        return "field_set_value"
    if "field_base" in roles:
        return "field_base"
    if "call_operand" in roles:
        return "call_operand"
    if "call_result" in roles:
        return "call_result"
    return "+".join(sorted(roles))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--design", type=Path, required=True)
    parser.add_argument("--position", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=12)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    design = read_kv(args.design)
    position = read_kv(args.position)
    require(
        design,
        "output_contract",
        "hako-mimalloc-local-ssa-block-entry-phi-edge-copy-design-v0",
        "design",
    )
    require(design, "candidate_probe_required", "1", "design")
    require(design, "implementation_allowed", "0", "design")
    require(position, "output_contract", "hako-mimalloc-local-ssa-copy-position-probe-v0", "position")

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    category_counts: Counter[str] = Counter()
    block_entry_role_counts: Counter[str] = Counter()
    phi_edge_role_counts: Counter[str] = Counter()
    samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        call_attributed = collect_call_attributed_copy_dsts(insts)
        copies = [inst for inst in insts if inst.get("op") == "copy"]
        src_to_dsts: dict[Any, set[Any]] = {}
        for inst in copies:
            src_to_dsts.setdefault(inst.get("src"), set()).add(inst.get("dst"))

        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            dst = inst.get("dst")
            src = inst.get("src")
            aliases = alias_values(dst, src_to_dsts)
            roles = collect_route_carrier_roles(aliases, insts, src_to_dsts)
            category = classify_copy(dst, src, inst_index, insts, phi_dsts, call_attributed)
            category_counts[category] += 1
            if category == "block_entry":
                block_entry_role_counts[role_key(roles)] += 1
            elif category == "phi_edge":
                phi_edge_role_counts[role_key(roles)] += 1
            if category in {"block_entry", "phi_edge"}:
                samples.append(
                    {
                        "category": category,
                        "block_id": block_id,
                        "inst_index": inst_index,
                        "dst": dst,
                        "src": src,
                        "role": role_key(roles),
                    }
                )

    phi_edge_count = category_counts["phi_edge"]
    block_entry_count = category_counts["block_entry"]
    if phi_edge_count != require_int(position, "phi_edge_copy_count", "position"):
        raise SystemExit("position/probe mismatch: phi_edge_copy_count")
    if block_entry_count != require_int(position, "block_entry_copy_count", "position"):
        raise SystemExit("position/probe mismatch: block_entry_copy_count")

    block_entry_route_none_count = block_entry_role_counts["none"]
    block_entry_route_carrier_count = block_entry_count - block_entry_route_none_count
    block_entry_field_set_value_count = block_entry_role_counts["field_set_value"]
    block_entry_field_base_count = block_entry_role_counts["field_base"]
    block_entry_call_operand_count = block_entry_role_counts["call_operand"]

    # Conservative row: route-none block-entry copies still cross freshness /
    # binding boundaries not proven by this probe. A later row may reopen a
    # subset only after it adds explicit freshness proof.
    safe_candidate_count = 0
    next_task = "local_ssa_block_entry_phi_edge_no_safe_candidate_closeout"
    selected_policy = "none"
    if safe_candidate_count > 0:
        next_task = "local_ssa_block_entry_copy_guard_surface"
        selected_policy = "block_entry_route_none_with_freshness_proof"

    lines = [
        "output_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-candidate-probe-v0",
        "input_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-design-v0+hako-mimalloc-local-ssa-copy-position-probe-v0",
        f"target_method={function.get('name', args.method)}",
        "source_evidence=296x-749",
        f"copy_count={require_key(position, 'copy_count', 'position')}",
        f"phi_edge_copy_count={phi_edge_count}",
        f"block_entry_copy_count={block_entry_count}",
        f"block_entry_route_none_count={block_entry_route_none_count}",
        f"block_entry_route_carrier_count={block_entry_route_carrier_count}",
        f"block_entry_field_set_value_count={block_entry_field_set_value_count}",
        f"block_entry_field_base_count={block_entry_field_base_count}",
        f"block_entry_call_operand_count={block_entry_call_operand_count}",
        f"phi_edge_route_none_count={phi_edge_role_counts['none']}",
        f"phi_edge_route_carrier_count={phi_edge_count - phi_edge_role_counts['none']}",
        f"safe_candidate_count={safe_candidate_count}",
        f"selected_policy={selected_policy}",
        "phi_edge_optimization_allowed=0",
        "block_entry_route_carrier_optimization_allowed=0",
        "block_entry_route_none_optimization_allowed=0",
        "freshness_proof_available=0",
        "phi_lifecycle_changed=0",
        "cfg_changed=0",
        "copy_emission_ssot_preserved=1",
        f"next_task={next_task}",
        "implementation_allowed=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    for idx, sample in enumerate(samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_category={sample['category']}",
                f"{prefix}_block=block_{sample['block_id']}",
                f"{prefix}_inst_index={sample['inst_index']}",
                f"{prefix}_dst={sample['dst']}",
                f"{prefix}_src={sample['src']}",
                f"{prefix}_role={sample['role']}",
            ]
        )

    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
