#!/usr/bin/env python3
"""Emit a read-only inventory of remaining semantic refresh duplicate entry seams."""

from __future__ import annotations

import argparse
from pathlib import Path


OUTPUT_CONTRACT = "hako-check-semantic-refresh-inventory-v0"
TOOL_SURFACE = "hako_check_semantic_refresh_inventory"


INVENTORY_ROWS = [
    {
        "id": "compiler_pre_verification_contracts",
        "kind": "intentional_timing_seam",
        "owner": "src/mir/compiler/mod.rs",
        "status": "keep",
        "next": "move only under separate verifier-timing card",
    },
    {
        "id": "compiler_post_rc_semantic_refresh",
        "kind": "canonical_entry",
        "owner": "src/mir/compiler/mod.rs",
        "status": "keep",
        "next": "canonical final module semantic refresh",
    },
    {
        "id": "compiler_post_callsite_canonicalize_refresh",
        "kind": "conditional_canonical_entry",
        "owner": "src/mir/compiler/mod.rs",
        "status": "keep",
        "next": "required after MIR mutation by callsite canonicalize",
    },
    {
        "id": "builder_decl_layout_timing",
        "kind": "intentional_timing_seam",
        "owner": "src/mir/builder/module_lifecycle.rs",
        "status": "keep",
        "next": "declaration-derived subset remains before function-local metadata",
    },
    {
        "id": "json_v0_decl_layout_timing",
        "kind": "intentional_timing_seam",
        "owner": "src/runner/json_v0_bridge/lowering.rs",
        "status": "keep",
        "next": "uses shared record/packed layout helper",
    },
    {
        "id": "json_v0_post_canonicalize_metadata_subset",
        "kind": "resolved_helper",
        "owner": "src/runner/json_v0_bridge/core.rs",
        "status": "resolved",
        "next": "owned by refresh_module_json_v0_post_canonicalize_metadata",
    },
    {
        "id": "rune_immediate_attr_refresh",
        "kind": "intentional_timing_seam",
        "owner": "builder/json_v0_bridge/optimizer",
        "status": "keep",
        "next": "rune attrs mutate before full module refresh and inline consumes fresh plans",
    },
    {
        "id": "string_corridor_local_mutation_refresh",
        "kind": "intentional_local_mutation_seam",
        "owner": "src/mir/passes/string_corridor_sink/*",
        "status": "keep",
        "next": "function-local pass mutates MIR and refreshes only affected metadata",
    },
]


def contract_lines() -> list[str]:
    rows = INVENTORY_ROWS
    candidates = [row for row in rows if row["kind"] == "remaining_duplicate_candidate"]
    resolved = [row for row in rows if row["kind"] == "resolved_helper"]
    keep_rows = [row for row in rows if row["status"] == "keep"]
    lines = [
        f"output_contract={OUTPUT_CONTRACT}",
        f"tool_surface={TOOL_SURFACE}",
        "observation_only=1",
        "rewrite_executed=0",
        "keeper_selection=0",
        "semantic_refresh_truth_source=src/mir/semantic_refresh.rs",
        "semantic_refresh_inventory_source=docs/development/current/main/design/compiler-pipeline-thinning-ssot.md",
        f"semantic_refresh_inventory_row_count={len(rows)}",
        f"semantic_refresh_remaining_duplicate_candidate_count={len(candidates)}",
        f"semantic_refresh_resolved_helper_count={len(resolved)}",
        f"semantic_refresh_intentional_timing_seam_count={len(keep_rows)}",
        "semantic_refresh_behavior_changed=0",
        "semantic_refresh_order_changed=0",
    ]
    for idx, row in enumerate(rows):
        prefix = f"semantic_refresh_inventory[{idx}]"
        lines.extend(
            [
                f"{prefix}.id={row['id']}",
                f"{prefix}.kind={row['kind']}",
                f"{prefix}.owner={row['owner']}",
                f"{prefix}.status={row['status']}",
                f"{prefix}.next={row['next']}",
            ]
        )
    lines.append("summary=ok")
    return lines


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    report = "\n".join(contract_lines()) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
