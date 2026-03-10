#!/usr/bin/env python3
"""Canonicalize MIR(JSON v0) for narrow G1 semantic comparison."""

from __future__ import annotations

import copy
import json
import sys
from pathlib import Path


VALUE_KEYS = {"dst", "lhs", "rhs", "src", "box", "cond"}
TARGET_KEYS = {"target", "then", "else"}


def _is_copy_inst(inst: object) -> bool:
    return isinstance(inst, dict) and inst.get("op") == "copy" and isinstance(inst.get("dst"), int)


def _assign_param_ranks(params: list[object]) -> dict[int, int]:
    ranks: dict[int, int] = {}
    next_rank = 0
    for param in params:
        if isinstance(param, int) and param not in ranks:
            ranks[param] = next_rank
            next_rank += 1
    return ranks


def _normalize_copy_bundle(bundle: list[dict], ranks: dict[int, int]) -> list[dict]:
    if len(bundle) <= 1:
        return bundle

    bundle_dsts = {
        inst.get("dst")
        for inst in bundle
        if isinstance(inst.get("dst"), int)
    }
    if None in bundle_dsts:
        return bundle

    for inst in bundle:
        src = inst.get("src")
        if not isinstance(src, int):
            return bundle
        if src in bundle_dsts:
            return bundle
        if src not in ranks:
            return bundle

    return sorted(bundle, key=lambda inst: (ranks[inst["src"]], inst["dst"]))


def _assign_defined_value_rank(
    inst: object, ranks: dict[int, int], next_rank: int
) -> int:
    if not isinstance(inst, dict):
        return next_rank
    dst = inst.get("dst")
    if isinstance(dst, int) and dst not in ranks:
        ranks[dst] = next_rank
        return next_rank + 1
    return next_rank


def _normalize_block_instructions(
    instructions: list[object], ranks: dict[int, int], next_rank: int
) -> tuple[list[object], dict[int, int], int]:
    normalized: list[object] = []
    local_ranks = dict(ranks)
    idx = 0
    while idx < len(instructions):
        inst = instructions[idx]
        if not _is_copy_inst(inst):
            normalized.append(inst)
            next_rank = _assign_defined_value_rank(inst, local_ranks, next_rank)
            idx += 1
            continue

        end = idx
        bundle: list[dict] = []
        while end < len(instructions) and _is_copy_inst(instructions[end]):
            bundle.append(instructions[end])
            end += 1
        normalized_bundle = _normalize_copy_bundle(bundle, local_ranks)
        normalized.extend(normalized_bundle)
        for item in normalized_bundle:
            next_rank = _assign_defined_value_rank(item, local_ranks, next_rank)
        idx = end
    return normalized, local_ranks, next_rank


def _normalize_blocks_with_fixed_point(
    blocks: list[dict], params: list[object]
) -> tuple[list[dict], dict[int, int]]:
    current = copy.deepcopy(blocks)
    for _ in range(8):
        ranks = _assign_param_ranks(params)
        next_rank = len(ranks)
        next_blocks: list[dict] = []
        changed = False

        for block in current:
            block_copy = copy.deepcopy(block)
            instructions = block_copy.get("instructions", [])
            normalized_instructions, ranks, next_rank = _normalize_block_instructions(
                instructions, ranks, next_rank
            )
            if normalized_instructions != instructions:
                changed = True
            block_copy["instructions"] = normalized_instructions
            next_blocks.append(block_copy)

        current = next_blocks
        if not changed:
            return current, ranks

    final_ranks = _assign_param_ranks(params)
    next_rank = len(final_ranks)
    for block in current:
        for inst in block.get("instructions", []):
            if isinstance(inst, dict):
                dst = inst.get("dst")
                if isinstance(dst, int) and dst not in final_ranks:
                    final_ranks[dst] = next_rank
                    next_rank += 1
    return current, final_ranks


def _map_value_id(value: object, ranks: dict[int, int]) -> object:
    if isinstance(value, int):
        return ranks.get(value, value)
    return value


def _canonicalize_instruction(
    inst: dict, ranks: dict[int, int], block_map: dict[int, int]
) -> dict:
    out: dict = {}
    for key, value in inst.items():
        if key in VALUE_KEYS:
            out[key] = _map_value_id(value, ranks)
        elif key == "value":
            out[key] = _map_value_id(value, ranks)
        elif key == "args" and isinstance(value, list):
            out[key] = [_map_value_id(item, ranks) for item in value]
        elif key == "incoming" and isinstance(value, list):
            incoming = []
            for pair in value:
                if not (isinstance(pair, list) and len(pair) == 2):
                    incoming.append(pair)
                    continue
                incoming.append(
                    [_map_value_id(pair[0], ranks), block_map.get(pair[1], pair[1])]
                )
            incoming.sort(key=lambda pair: (pair[1], pair[0]))
            out[key] = incoming
        elif key in TARGET_KEYS and isinstance(value, int):
            out[key] = block_map.get(value, value)
        else:
            out[key] = value
    return out


def _canonicalize_cfg(cfg: object, block_map: dict[int, int]) -> object:
    if not isinstance(cfg, dict):
        return cfg
    out = copy.deepcopy(cfg)
    if isinstance(out.get("entry_block"), int):
        out["entry_block"] = block_map.get(out["entry_block"], out["entry_block"])
    blocks = out.get("blocks")
    if isinstance(blocks, list):
        canonical_blocks = []
        for block in blocks:
            if not isinstance(block, dict):
                canonical_blocks.append(block)
                continue
            item = copy.deepcopy(block)
            if isinstance(item.get("id"), int):
                item["id"] = block_map.get(item["id"], item["id"])
            if isinstance(item.get("successors"), list):
                item["successors"] = [
                    block_map.get(succ, succ) for succ in item["successors"]
                ]
            canonical_blocks.append(item)
        out["blocks"] = canonical_blocks
    return out


def _canonicalize_metadata(metadata: object, ranks: dict[int, int]) -> object:
    if not isinstance(metadata, dict):
        return metadata
    out = copy.deepcopy(metadata)
    value_types = out.get("value_types")
    if isinstance(value_types, dict):
        canonical_value_types = {}
        for key, value in value_types.items():
            try:
                raw_key = int(key)
            except (TypeError, ValueError):
                canonical_value_types[key] = value
                continue
            canonical_value_types[str(ranks.get(raw_key, raw_key))] = value
        out["value_types"] = canonical_value_types
    return out


def _canonicalize_function(func: dict) -> dict:
    blocks = func.get("blocks")
    if not isinstance(blocks, list):
        return copy.deepcopy(func)

    params = func.get("params", [])
    normalized_blocks, ranks = _normalize_blocks_with_fixed_point(blocks, params)
    block_map = {
        block.get("id"): idx
        for idx, block in enumerate(normalized_blocks)
        if isinstance(block.get("id"), int)
    }

    out = copy.deepcopy(func)
    out["params"] = [_map_value_id(param, ranks) for param in params]

    canonical_blocks = []
    for idx, block in enumerate(normalized_blocks):
        item = copy.deepcopy(block)
        item["id"] = idx
        if isinstance(item.get("successors"), list):
            item["successors"] = [block_map.get(succ, succ) for succ in item["successors"]]
        instructions = item.get("instructions", [])
        if isinstance(instructions, list):
            item["instructions"] = [
                _canonicalize_instruction(inst, ranks, block_map)
                if isinstance(inst, dict)
                else inst
                for inst in instructions
            ]
        canonical_blocks.append(item)
    out["blocks"] = canonical_blocks

    if "cfg" in out:
        out["cfg"] = _canonicalize_cfg(out["cfg"], block_map)
    if "metadata" in out:
        out["metadata"] = _canonicalize_metadata(out["metadata"], ranks)
    return out


def canonicalize_module(payload: dict) -> dict:
    module = copy.deepcopy(payload)
    functions = module.get("functions")
    if isinstance(functions, list):
        module["functions"] = [
            _canonicalize_function(func) if isinstance(func, dict) else func
            for func in functions
        ]
    return module


def main(argv: list[str]) -> int:
    if len(argv) != 3 or argv[1] != "canonicalize":
        print(
            "usage: mir_canonical_compare.py canonicalize <mir.json>",
            file=sys.stderr,
        )
        return 2

    path = Path(argv[2])
    try:
        payload = json.loads(path.read_text())
    except Exception as exc:  # pragma: no cover - fail-fast helper
        print(f"[mir-canonical-compare] failed to parse {path}: {exc}", file=sys.stderr)
        return 1

    json.dump(canonicalize_module(payload), sys.stdout, sort_keys=True, separators=(",", ":"))
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
