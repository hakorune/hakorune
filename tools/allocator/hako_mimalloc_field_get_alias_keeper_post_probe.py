#!/usr/bin/env python3
"""Post-keeper count for field_get-origin alias forwarding candidates."""

from __future__ import annotations

import argparse
import importlib.util
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
ALLOCATOR_TOOLS = ROOT / "tools/allocator"
sys.path.insert(0, str(ALLOCATOR_TOOLS))

ORIGIN_PROBE = ALLOCATOR_TOOLS / "hako_mimalloc_expression_materialization_copy_origin_probe.py"
REFRESH_PROBE = ALLOCATOR_TOOLS / "hako_mimalloc_field_get_direct_consumer_refresh_probe.py"
DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load module: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    origin = load_module("hako_expr_origin_probe", ORIGIN_PROBE)
    refresh = load_module("hako_field_get_refresh_probe", REFRESH_PROBE)
    function = origin.find_function(origin.load_json(args.mir_json), args.method)
    blocks = origin.block_instructions(function)
    producers, _, _ = refresh.producer_maps(blocks)
    phi_dsts = {
        inst.get("dst")
        for _, insts in blocks
        for inst in insts
        if inst.get("op") == "phi" and inst.get("dst") is not None
    }

    copy_count = 0
    expression_count = 0
    field_get_expression_count = 0
    forwarding_candidate_count = 0
    samples: list[str] = []

    for block_id, insts in blocks:
        call_attributed = origin.collect_call_attributed_copy_dsts(insts)
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        for inst in insts:
            if inst.get("op") == "copy":
                copy_count += 1
            for value in origin.value_uses(inst):
                consumers[value].append(inst)

        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            category = origin.classify_copy(
                inst.get("dst"),
                inst.get("src"),
                inst_index,
                insts,
                phi_dsts,
                call_attributed,
            )
            if category != "expression_materialization":
                continue
            expression_count += 1
            origin_kind, origin_detail, chain_len = origin.origin_label(inst.get("src"), producers)
            if origin_kind != "field_get":
                continue
            field_get_expression_count += 1
            sinks = sorted(set(origin.sink_labels(inst.get("dst"), consumers)))
            if chain_len > 0 and refresh.is_real_consumer(sinks):
                forwarding_candidate_count += 1
                if len(samples) < 8:
                    samples.append(
                        f"block_{block_id}:inst_{inst_index}:dst_{inst.get('dst')}:"
                        f"field_{origin_detail}:chain_{chain_len}:sink_{'+'.join(sinks)}"
                    )

    lines = [
        "output_contract=hako-mimalloc-field-get-alias-keeper-post-probe-v0",
        f"target_method={function.get('name', args.method)}",
        f"copy_count={copy_count}",
        f"expression_materialization_copy_count={expression_count}",
        f"field_get_expression_copy_count={field_get_expression_count}",
        f"forwarding_candidate_copy_count={forwarding_candidate_count}",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for idx, sample in enumerate(samples):
        lines.append(f"sample_{idx}={sample}")
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
