#!/usr/bin/env python3
"""Select the narrow LocalSSA same-block reuse owner for field_get copy chains."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-mimalloc-local-ssa-same-block-reuse-selection-v0",
        "input_contract=hako-mimalloc-field-get-result-chain-follow-on-probe-v0",
        "selected_owner=local_ssa_same_block_reuse",
        "selected_file=src/mir/builder/ssa/local.rs",
        "selected_function=ensure_inner",
        "selected_rule=return_original_value_when_def_block_is_current_block",
        "selected_scope=all_local_ssa_kinds_with_current_block_definition",
        "guarded_boundary=non_dominating_and_cross_block_values_keep_existing_copy_path",
        "rejected_owner=phi_incoming_copy_cleanup",
        "rejected_reason=phi_incoming_is_consumer_not_origin;same_block_origin_count_equals_field_get_chain_count",
        "rejected_owner_2=source_hako_rewrite",
        "rejected_reason_2=remaining_surface_is_compiler_local_ssa_same_block_materialization",
        "implementation_open=0",
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
