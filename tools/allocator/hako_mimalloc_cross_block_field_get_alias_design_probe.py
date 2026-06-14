#!/usr/bin/env python3
"""Design probe for cross-block field_get alias forwarding."""

from __future__ import annotations

import argparse
import importlib.util
import sys
from collections import Counter, defaultdict
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


def cfg(blocks: list[tuple[Any, list[dict[str, Any]]]]) -> tuple[list[Any], dict[Any, list[Any]], dict[Any, list[Any]]]:
    ids = [block_id for block_id, _ in blocks]
    successors: dict[Any, list[Any]] = {block_id: [] for block_id in ids}
    predecessors: dict[Any, list[Any]] = {block_id: [] for block_id in ids}
    for block_id, insts in blocks:
        for inst in insts:
            op = inst.get("op")
            if op == "jump":
                target = inst.get("target")
                if target in successors:
                    successors[block_id].append(target)
            elif op == "branch":
                for target in (inst.get("then"), inst.get("else")):
                    if target in successors:
                        successors[block_id].append(target)
    for source, targets in successors.items():
        for target in targets:
            predecessors[target].append(source)
    return ids, successors, predecessors


def dominators(ids: list[Any], predecessors: dict[Any, list[Any]]) -> dict[Any, set[Any]]:
    if not ids:
        return {}
    entry = ids[0]
    dom: dict[Any, set[Any]] = {block_id: set(ids) for block_id in ids}
    dom[entry] = {entry}
    changed = True
    while changed:
        changed = False
        for block_id in ids[1:]:
            preds = predecessors.get(block_id, [])
            pred_intersection = set.intersection(*(dom[pred] for pred in preds)) if preds else set()
            new = {block_id} | pred_intersection
            if new != dom[block_id]:
                dom[block_id] = new
                changed = True
    return dom


def simple_paths(
    start: Any,
    end: Any,
    successors: dict[Any, list[Any]],
    *,
    max_depth: int = 64,
) -> list[list[Any]]:
    paths: list[list[Any]] = []

    def walk(current: Any, path: list[Any]) -> None:
        if len(path) > max_depth:
            return
        if current == end:
            paths.append(path[:])
            return
        for nxt in successors.get(current, []):
            if nxt in path:
                continue
            walk(nxt, path + [nxt])

    walk(start, [start])
    return paths


def same_field_sets_on_path(
    path: list[Any],
    block_map: dict[Any, list[dict[str, Any]]],
    *,
    field: str,
    origin_block: Any,
    origin_index: int | None,
    candidate_block: Any,
    candidate_index: int,
) -> list[tuple[Any, int]]:
    hits: list[tuple[Any, int]] = []
    for block_id in path:
        for inst_index, inst in enumerate(block_map.get(block_id, [])):
            if block_id == origin_block and origin_index is not None and inst_index <= origin_index:
                continue
            if block_id == candidate_block and inst_index >= candidate_index:
                continue
            if inst.get("op") == "field_set" and str(inst.get("field", "unknown")) == field:
                hits.append((block_id, inst_index))
    return hits


def collect_candidates(
    origin: Any,
    refresh: Any,
    blocks: list[tuple[Any, list[dict[str, Any]]]],
) -> list[dict[str, Any]]:
    producers, producer_blocks, producer_indices = refresh.producer_maps(blocks)
    phi_dsts = {
        inst.get("dst")
        for _, insts in blocks
        for inst in insts
        if inst.get("op") == "phi" and inst.get("dst") is not None
    }
    candidates: list[dict[str, Any]] = []
    for block_id, insts in blocks:
        call_attributed = origin.collect_call_attributed_copy_dsts(insts)
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        for inst in insts:
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
            root_inst, origin_block, origin_index, chain_len, chain = refresh.root_field_get(
                inst.get("src"),
                producers,
                producer_blocks,
                producer_indices,
            )
            if root_inst is None or chain_len <= 0:
                continue
            sinks = sorted(set(origin.sink_labels(inst.get("dst"), consumers)))
            if not refresh.is_real_consumer(sinks):
                continue
            candidates.append(
                {
                    "origin_field": str(root_inst.get("field", "unknown")),
                    "origin_box": root_inst.get("box"),
                    "origin_block": origin_block,
                    "origin_index": origin_index,
                    "candidate_block": block_id,
                    "candidate_index": inst_index,
                    "candidate_dst": inst.get("dst"),
                    "candidate_src": inst.get("src"),
                    "copy_chain_len": chain_len,
                    "consumer_family": "+".join(sinks),
                    "copy_chain": chain,
                }
            )
    return candidates


def safe_key(text: str) -> str:
    out: list[str] = []
    for ch in text:
        if ch.isalnum() or ch == "_":
            out.append(ch)
        else:
            out.append("_")
    return "".join(out).strip("_") or "unknown"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--refresh", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=10)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    refresh_report = read_kv(args.refresh)
    require(
        refresh_report,
        "output_contract",
        "hako-mimalloc-field-get-direct-consumer-refresh-v2",
        "refresh",
    )
    require(refresh_report, "selected_owner", "cross_block_field_get_alias_copy_chain", "refresh")
    require(refresh_report, "optimization_open", "0", "refresh")

    origin = load_module("hako_expr_origin_probe", ORIGIN_PROBE)
    refresh = load_module("hako_field_get_refresh_probe", REFRESH_PROBE)
    function = origin.find_function(origin.load_json(args.mir_json), args.method)
    blocks = origin.block_instructions(function)
    block_map = {block_id: insts for block_id, insts in blocks}
    ids, successors, predecessors = cfg(blocks)
    dom = dominators(ids, predecessors)
    candidates = collect_candidates(origin, refresh, blocks)

    dominance_ok = 0
    same_block = 0
    cross_block = 0
    mutation_path_count = 0
    all_path_count = 0
    safe_alias_count = 0
    field_counts: Counter[str] = Counter()
    sink_counts: Counter[str] = Counter()
    samples: list[dict[str, Any]] = []

    for candidate in candidates:
        origin_block = candidate["origin_block"]
        candidate_block = candidate["candidate_block"]
        dominates = origin_block in dom.get(candidate_block, set())
        if dominates:
            dominance_ok += 1
        if origin_block == candidate_block:
            same_block += 1
        else:
            cross_block += 1
        paths = simple_paths(origin_block, candidate_block, successors)
        path_mutations = 0
        for path in paths:
            all_path_count += 1
            hits = same_field_sets_on_path(
                path,
                block_map,
                field=candidate["origin_field"],
                origin_block=origin_block,
                origin_index=candidate["origin_index"],
                candidate_block=candidate_block,
                candidate_index=candidate["candidate_index"],
            )
            if hits:
                path_mutations += 1
        if path_mutations:
            mutation_path_count += 1
        if dominates and path_mutations == 0:
            safe_alias_count += 1
        field_counts[candidate["origin_field"]] += 1
        sink_counts[candidate["consumer_family"]] += 1
        samples.append(
            {
                **candidate,
                "dominates": int(dominates),
                "path_count": len(paths),
                "same_field_mutation_path_count": path_mutations,
                "safe_alias_candidate": int(dominates and path_mutations == 0),
            }
        )

    keeper_shape = "no_keeper"
    confidence = "high"
    next_task = "return_to_kernel_front_selection"
    if safe_alias_count:
        keeper_shape = "dominance_alias"
        confidence = "medium"
        next_task = "cross_block_field_get_alias_forwarding_keeper"

    lines = [
        "output_contract=hako-mimalloc-cross-block-field-get-alias-forwarding-design-v0",
        "input_contract=hako-mimalloc-field-get-direct-consumer-refresh-v2",
        f"target_method={function.get('name', args.method)}",
        f"forwarding_candidate_copy_count={len(candidates)}",
        f"same_block_candidate_count={same_block}",
        f"cross_block_candidate_count={cross_block}",
        f"root_dominates_candidate_count={dominance_ok}",
        f"same_field_mutation_candidate_count={mutation_path_count}",
        f"same_field_mutation_path_count={mutation_path_count}",
        f"safe_alias_candidate_count={safe_alias_count}",
        f"all_candidate_path_count={all_path_count}",
        f"dominant_candidate_field={max(field_counts, key=field_counts.get) if field_counts else 'none'}",
        f"dominant_candidate_sink={max(sink_counts, key=sink_counts.get) if sink_counts else 'none'}",
        f"keeper_shape={keeper_shape}",
        f"selected_owner=cross_block_field_get_alias_copy_chain",
        f"selected_owner_confidence={confidence}",
        "dominance_required=1",
        "same_field_mutation_guard_required=1",
        "same_receiver_alias_guard_required=1",
        "ssa_visibility_guard_required=1",
        "arbitrary_copy_coalescing_allowed=0",
        f"next_task={next_task}",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for key, count in field_counts.most_common(8):
        lines.append(f"candidate_field_{safe_key(key)}_count={count}")
    for key, count in sink_counts.most_common(8):
        lines.append(f"candidate_sink_{safe_key(key)}_count={count}")
    for idx, sample in enumerate(samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_origin_field={sample['origin_field']}",
                f"{prefix}_origin_block=block_{sample['origin_block']}",
                f"{prefix}_candidate_block=block_{sample['candidate_block']}",
                f"{prefix}_dominates={sample['dominates']}",
                f"{prefix}_path_count={sample['path_count']}",
                f"{prefix}_same_field_mutation_path_count={sample['same_field_mutation_path_count']}",
                f"{prefix}_safe_alias_candidate={sample['safe_alias_candidate']}",
                f"{prefix}_consumer_family={sample['consumer_family']}",
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
