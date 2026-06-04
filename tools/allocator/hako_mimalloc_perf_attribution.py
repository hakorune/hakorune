#!/usr/bin/env python3
"""Summarize whether a mimalloc perf artifact can select the next owner.

This tool intentionally separates two attribution levels:

* symbol attribution from `perf report`
* instruction attribution from `perf annotate`

The current AOT direct-exact app often collapses hot samples into `ny_main`.
That is useful evidence, but it is not enough to claim a DirectArray or
PageModel-specific perf delta. The report below makes that boundary explicit.
"""

from __future__ import annotations

import argparse
from pathlib import Path

from hako_mimalloc_perf_attribution_report import emit_report
from hako_mimalloc_perf_attribution_support import (
    I64_SIGNED_MAX,
    INLINE_OWNER_NEXT_BRIDGE,
    AnnotatedInstruction,
    ObjdumpInstruction,
    PerfSymbol,
    _append_unique,
    _context_fields_for_address,
    _context_for_address,
    _count_bucket,
    _count_public_or_proof,
    _dominant_inline_owner,
    _dominant_store_bucket,
    _field_hints_for_asm,
    _field_names_for_asm,
    _has_checked_public_accumulator_barrier,
    _instruction_category,
    _is_arithmetic_or_compare,
    _is_branch,
    _is_call,
    _is_memory,
    _is_store_like,
    _kv_bool,
    _select_backend_store_shape,
    _select_directarray_owner_instruction_shape,
    _select_inline_owner_for_fields,
    _store_bucket_weights,
    _inline_owner_weights,
    _public_proof_accumulator_fields,
    _sum_matching,
    _read,
    parse_annotated_instructions,
    parse_layout_field_offsets,
    parse_objdump_instructions,
    parse_perf_symbols,
)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path)
    parser.add_argument("--perf-annotate", type=Path)
    parser.add_argument("--objdump", type=Path)
    parser.add_argument("--mir-json", type=Path)
    parser.add_argument("--layout-box", default="")
    parser.add_argument("--layout-base-offset", type=lambda s: int(s, 0), default=0x20)
    parser.add_argument("--layout-field-stride", type=lambda s: int(s, 0), default=0x10)
    parser.add_argument("--symbol", default="ny_main")
    parser.add_argument("--collapse-threshold", type=float, default=90.0)
    parser.add_argument("--hot-limit", type=int, default=8)
    parser.add_argument("--context-radius", type=int, default=3)
    parser.add_argument("--observed-requested-bytes", type=int)
    args = parser.parse_args()
    if args.hot_limit < 1:
        parser.error("--hot-limit must be >= 1")
    if args.context_radius < 0:
        parser.error("--context-radius must be >= 0")
    print(emit_report(args), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
