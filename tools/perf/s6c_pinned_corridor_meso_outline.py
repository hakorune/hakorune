#!/usr/bin/env python3
"""Outline the exact lifecycle-free S6C scan graph for promotion evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import sys


OLD_HEADER = "define i64 @ny_main(i64 %r0, i64 %r1, i64 %r2, i64 %r3) {"
NEW_HEADER = (
    "define i64 @hako_s6c_meso(ptr %ptfc_subject_ptr, i64 %ptfc_subject_len, "
    "ptr %ptfc_needle_ptr, i64 %ptfc_needle_len) {"
)
REMOVED_BLOCKS = ("bb0", "bb2")
ROOT_PREFIXES = (
    "%ptfc_subject_ptr_addr =", "%ptfc_subject_ptr =",
    "%ptfc_subject_len_addr =", "%ptfc_subject_len =",
    "%ptfc_needle_ptr_addr =", "%ptfc_needle_ptr =",
    "%ptfc_needle_len_addr =", "%ptfc_needle_len =",
)
ROOT_PROJECTION = (
    "%ptfc_subject_ptr_addr = getelementptr i8, ptr %ptfc_frame, i64 32",
    "%ptfc_subject_ptr = load ptr, ptr %ptfc_subject_ptr_addr, align 8",
    "%ptfc_subject_len_addr = getelementptr i8, ptr %ptfc_frame, i64 40",
    "%ptfc_subject_len = load i64, ptr %ptfc_subject_len_addr, align 8",
    "%ptfc_needle_ptr_addr = getelementptr i8, ptr %ptfc_frame, i64 48",
    "%ptfc_needle_ptr = load ptr, ptr %ptfc_needle_ptr_addr, align 8",
    "%ptfc_needle_len_addr = getelementptr i8, ptr %ptfc_frame, i64 56",
    "%ptfc_needle_len = load i64, ptr %ptfc_needle_len_addr, align 8",
)
FINISH = "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)"
EXPECTED_COUNTS = {"blocks": 20, "instructions": 92, "edges": 35, "phis": 5, "returns": 2}


def reject(message: str) -> None:
    raise ValueError(message)


def extract_function(module: str) -> tuple[list[str], str, str]:
    lines = module.splitlines()
    starts = [index for index, line in enumerate(lines) if line == OLD_HEADER]
    if len(starts) != 1:
        reject("exact real ny_main signature is missing or duplicated")
    start = starts[0]
    end = next((index for index in range(start + 1, len(lines)) if lines[index] == "}"), -1)
    if end < 0:
        reject("real function is incomplete")
    layout = next((line for line in lines if line.startswith("target datalayout = ")), "")
    triple = next((line for line in lines if line.startswith("target triple = ")), "")
    if not layout or not triple:
        reject("target/data layout is missing")
    return lines[start : end + 1], layout, triple


def split_blocks(function: list[str]) -> tuple[list[str], dict[str, list[str]]]:
    order: list[str] = []
    blocks: dict[str, list[str]] = {}
    current = ""
    for line in function[1:-1]:
        match = re.match(r"^([A-Za-z0-9_.-]+):", line)
        if match:
            current = match.group(1)
            order.append(current)
            blocks[current] = [line]
        elif current:
            blocks[current].append(line)
        elif line.strip():
            reject("instruction appeared before the first block")
    if len(order) != len(set(order)):
        reject("duplicate block label")
    return order, blocks


def instruction_records(lines: list[str]) -> list[str]:
    records: list[str] = []
    switch: list[str] | None = None
    for raw in lines[1:]:
        clean = raw.split(";", 1)[0].strip()
        if not clean:
            continue
        if switch is not None:
            switch.append(clean)
            if clean == "]":
                records.append(" ".join(switch))
                switch = None
            continue
        if re.match(r"^(?:%[^ ]+\s*=\s*)?switch\b", clean):
            switch = [clean]
        else:
            records.append(re.sub(r"\s+", " ", clean))
    if switch is not None:
        reject("unterminated switch")
    return records


def graph(block_order: list[str], blocks: dict[str, list[str]]) -> dict[str, object]:
    graph_blocks: list[dict[str, object]] = []
    counts = {"blocks": len(block_order), "instructions": 0, "edges": 0, "phis": 0, "returns": 0}
    for name in block_order:
        instructions = instruction_records(blocks[name])
        successors: list[str] = []
        for instruction in instructions:
            opcode = instruction.split("=", 1)[-1].strip().split(" ", 1)[0]
            if opcode in ("br", "switch"):
                successors.extend(re.findall(r"label %([A-Za-z0-9_.-]+)", instruction))
            counts["phis"] += int(opcode == "phi")
            counts["returns"] += int(opcode == "ret")
        counts["instructions"] += len(instructions)
        counts["edges"] += len(successors)
        graph_blocks.append({"id": name, "instructions": instructions, "successors": successors})
    return {"counts": counts, "blocks": graph_blocks}


def graph_digest(value: dict[str, object]) -> str:
    encoded = json.dumps(value, separators=(",", ":"), sort_keys=True).encode()
    return hashlib.sha256(encoded).hexdigest()


def validate_shell(order: list[str], blocks: dict[str, list[str]]) -> None:
    expected_bb0 = [
        "%ptfc_pairs = alloca [2 x { i64, i64 }], align 8",
        "%ptfc_pair_0 = getelementptr [2 x { i64, i64 }], ptr %ptfc_pairs, i64 0, i64 0",
        "store i64 %r0, ptr %ptfc_pair_0, align 8",
        "%ptfc_pair_0_gen = getelementptr { i64, i64 }, ptr %ptfc_pair_0, i32 0, i32 1",
        "store i64 %r1, ptr %ptfc_pair_0_gen, align 8",
        "%ptfc_pair_1 = getelementptr [2 x { i64, i64 }], ptr %ptfc_pairs, i64 0, i64 1",
        "store i64 %r2, ptr %ptfc_pair_1, align 8",
        "%ptfc_pair_1_gen = getelementptr { i64, i64 }, ptr %ptfc_pair_1, i32 0, i32 1",
        "store i64 %r3, ptr %ptfc_pair_1_gen, align 8",
        "%ptfc_frame = alloca i8, i64 64, align 8",
        "%ptfc_status = call i32 @hako_text_formal_residence_enter_v1(ptr %ptfc_pairs, i32 2, ptr %ptfc_frame, i32 64)",
        "%ptfc_enter_ok = icmp eq i32 %ptfc_status, 0",
        "br i1 %ptfc_enter_ok, label %bb1, label %bb2",
    ]
    if order[:3] != ["bb0", "bb1", "bb2"] or instruction_records(blocks["bb0"]) != expected_bb0:
        reject("Enter/lane shell grammar drift")
    if instruction_records(blocks["bb2"]) != ["call void @llvm.trap()", "unreachable"]:
        reject("Trap shell grammar drift")
    bb1 = instruction_records(blocks["bb1"])
    if len(bb1) < 10 or tuple(bb1[:8]) != ROOT_PROJECTION:
        reject("root projection grammar drift")
    for exit_block in ("bb5", "bb6"):
        rows = instruction_records(blocks.get(exit_block, []))
        if len(rows) < 2 or rows[-2] != FINISH or not rows[-1].startswith("ret i64 "):
            reject(f"Finish/Return shell drift at {exit_block}")


def outline(module: str, expected_digest: str) -> tuple[str, dict[str, object]]:
    function, layout, triple = extract_function(module)
    order, blocks = split_blocks(function)
    validate_shell(order, blocks)
    kept_order = [name for name in order if name not in REMOVED_BLOCKS]
    kept: dict[str, list[str]] = {name: list(blocks[name]) for name in kept_order}
    kept["bb1"] = [kept["bb1"][0]] + [
        line for line in kept["bb1"][1:]
        if not line.strip().startswith(ROOT_PREFIXES)
    ]
    for exit_block in ("bb5", "bb6"):
        kept[exit_block] = [line for line in kept[exit_block] if line.strip() != FINISH]
    retained_text = "\n".join(line for name in kept_order for line in kept[name])
    forbidden = ("%r0", "%r1", "%r2", "%r3", "%ptfc_frame", "%ptfc_status", "%ptfc_pairs")
    if any(re.search(rf"(?<![A-Za-z0-9_]){re.escape(token)}(?![0-9])", retained_text) for token in forbidden):
        reject("retained scan depends on lifecycle/lane shell")
    if any(re.search(rf"label %{name}\b", retained_text) for name in REMOVED_BLOCKS):
        reject("retained scan points to a removed shell block")
    retained_graph = graph(kept_order, kept)
    if retained_graph["counts"] != EXPECTED_COUNTS:
        reject(f"retained scan census drift: {retained_graph['counts']}")
    actual_digest = graph_digest(retained_graph)
    if actual_digest != expected_digest:
        reject(f"retained scan digest drift: {actual_digest}")
    output_lines = [layout, triple, "", NEW_HEADER]
    for name in kept_order:
        label = re.sub(r";.*$", "", kept[name][0]).rstrip()
        output_lines.append(label)
        output_lines.extend(kept[name][1:])
    output_lines.append("}")
    output = "\n".join(output_lines) + "\n"
    out_function, _, _ = extract_outlined_function(output)
    out_order, out_blocks = split_blocks(out_function)
    outlined_graph = graph(out_order, out_blocks)
    if outlined_graph != retained_graph:
        reject("outlined graph differs from retained real scan graph")
    manifest = {
        "schema": "s6c-pinned-corridor-meso-outline-evidence-v1",
        "authority": "promotion-evidence-only",
        "removed": {"blocks": 2, "instructions": 25},
        "retained": retained_graph["counts"],
        "retained_graph_sha256": actual_digest,
        "source_module_sha256": hashlib.sha256(module.encode()).hexdigest(),
        "outlined_module_sha256": hashlib.sha256(output.encode()).hexdigest(),
    }
    return output, manifest


def extract_outlined_function(module: str) -> tuple[list[str], str, str]:
    replaced = module.replace(NEW_HEADER, OLD_HEADER, 1)
    return extract_function(replaced)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--ir", required=True, type=Path)
    parser.add_argument("--expected-scan-sha256", required=True)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    args = parser.parse_args()
    output_tmp = args.output.with_name(args.output.name + ".tmp")
    manifest_tmp = args.manifest.with_name(args.manifest.name + ".tmp")
    try:
        output, manifest = outline(args.ir.read_text(), args.expected_scan_sha256)
        output_tmp.write_text(output)
        manifest_tmp.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
        output_tmp.replace(args.output)
        manifest_tmp.replace(args.manifest)
    except (OSError, ValueError) as error:
        for path in (args.output, output_tmp, args.manifest, manifest_tmp):
            path.unlink(missing_ok=True)
        print(f"[s6c-pinned-corridor-meso-outline] ERROR: {error}", file=sys.stderr)
        return 1
    print("[s6c-pinned-corridor-meso-outline] ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
