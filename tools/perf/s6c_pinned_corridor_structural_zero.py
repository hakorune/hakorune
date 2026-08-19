#!/usr/bin/env python3
"""Offline structural evidence for the caller-zero S6C pinned-Text candidate."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import sys


SCHEMA = "s6c-pinned-corridor-structural-zero-evidence-v1"
CANDIDATE = "hako_s6c_candidate"
IR_CANDIDATE = "ny_main"
ENTER = "hako_text_formal_residence_enter_v1"
FINISH = "hako_text_formal_residence_finish_or_abort_v1"
TRAP = "llvm.trap"


def reject(message: str) -> None:
    raise ValueError(message)


def digest(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def extract_ir_function(text: str) -> str:
    start = re.search(r'^define\s+i64\s+@"?ny_main"?\([^\n]*\)\s*\{\s*$', text, re.M)
    if not start:
        reject("exact final IR candidate is missing")
    lines = text[start.start() :].splitlines()
    body: list[str] = []
    depth = 0
    for line in lines:
        depth += line.count("{") - line.count("}")
        body.append(line)
        if depth == 0:
            break
    if not body or depth != 0:
        reject("final IR candidate body is incomplete")
    return "\n".join(body) + "\n"


def call_targets_ir(function: str) -> list[str]:
    targets: list[str] = []
    for line in function.splitlines():
        if re.search(r"\b(?:call|invoke|callbr)\b", line):
            match = re.search(r'@"?([^"(\s]+)"?\s*\(', line)
            if not match:
                reject(f"indirect or unreadable IR call: {line.strip()}")
            targets.append(match.group(1))
    return targets


def block_for_lines(function: str) -> dict[int, str]:
    result: dict[int, str] = {}
    block = ""
    for index, line in enumerate(function.splitlines()):
        label = re.match(r"^([A-Za-z0-9_.-]+):", line)
        if label:
            block = label.group(1)
        result[index] = block
    return result


def verify_ir(text: str) -> dict[str, object]:
    if 'target triple = "' not in text or 'target datalayout = "' not in text:
        reject("final IR target/data layout is missing")
    function = extract_ir_function(text)
    calls = call_targets_ir(function)
    allowed = {ENTER, FINISH, TRAP}
    if any(target not in allowed for target in calls):
        reject(f"unexpected candidate IR call: {calls}")
    for required in allowed:
        if required not in calls:
            reject(f"required candidate IR call is missing: {required}")

    forbidden = (
        " invoke ", " callbr ", "landingpad", "resume ", "catchswitch",
        "catchpad", "cleanuppad", "cleanupret", " noalias ", "memcmp",
        "eq_hh", "substring", "registry", "lease", "handle", "publication",
        "malloc", "calloc", "realloc", "@free", "retain", "release_strong",
    )
    lowered = f" {function.lower()} "
    for token in forbidden:
        if token in lowered:
            reject(f"forbidden candidate IR structure: {token.strip()}")

    lines = function.splitlines()
    blocks = block_for_lines(function)
    labels = [re.match(r"^([A-Za-z0-9_.-]+):", line) for line in lines]
    first_label = next((match.group(1) for match in labels if match), None)
    if not first_label:
        reject("candidate IR has no physical blocks")
    alloca_blocks = {
        blocks[index] for index, line in enumerate(lines) if " = alloca " in line
    }
    if alloca_blocks != {first_label}:
        reject(f"allocation escaped the entry block: {sorted(alloca_blocks)}")

    roots = (
        "%ptfc_subject_ptr = load ptr",
        "%ptfc_subject_len = load i64",
        "%ptfc_needle_ptr = load ptr",
        "%ptfc_needle_len = load i64",
    )
    root_positions: list[int] = []
    root_blocks: set[str] = set()
    for root in roots:
        positions = [index for index, line in enumerate(lines) if root in line]
        if len(positions) != 1:
            reject(f"root projection must occur once: {root}")
        root_positions.extend(positions)
        root_blocks.add(blocks[positions[0]])
    if len(root_blocks) != 1:
        reject("root ptr/len projections are not co-located")

    byte_positions: list[int] = []
    for index, line in enumerate(lines):
        if re.search(r"%ptfc_(?:byte|[lr]\d+_\d+)_\d+\s*=\s*load", line):
            byte_positions.append(index)
            if "load i8" not in line or "align 1" not in line:
                reject(f"wide or unaligned scalar read: {line.strip()}")
    if not byte_positions or max(root_positions) >= min(byte_positions):
        reject("root projections were not hoisted ahead of scalar reads")
    if any("<" in line and " x i" in line for line in lines if "load" in line):
        reject("vector load appeared in the candidate")

    widths: dict[int, set[int]] = {width: set() for width in range(1, 5)}
    for line in lines:
        match = re.search(r"%ptfc_[lr]ptr_([1-4])_([0-9]+)_", line)
        if match:
            widths[int(match.group(1))].add(int(match.group(2)))
    for width, offsets in widths.items():
        if offsets != set(range(width)):
            reject(f"width-{width} read offsets drifted: {sorted(offsets)}")

    triple = re.search(r'^target triple = "([^"]+)"$', text, re.M)
    layout = re.search(r'^target datalayout = "([^"]+)"$', text, re.M)
    return {
        "target_triple": triple.group(1) if triple else "",
        "data_layout": layout.group(1) if layout else "",
        "candidate_call_targets": sorted(set(calls)),
        "root_projection_block_count": len(root_blocks),
        "scalar_read_count": len(byte_positions),
        "exact_widths": sorted(widths),
    }


def verify_assembly(text: str) -> dict[str, object]:
    headers = re.findall(rf"^[0-9a-f]+\s+<{re.escape(CANDIDATE)}>:$", text, re.M)
    if len(headers) != 1:
        reject("linked candidate symbol is not unique")
    instruction_lines = [
        line for line in text.splitlines() if re.match(r"^\s*[0-9a-f]+:\s", line)
    ]
    if not instruction_lines:
        reject("linked candidate disassembly is empty")
    calls: list[str] = []
    for line in instruction_lines:
        match = re.search(r"\bcallq?\s+(.+)$", line)
        if not match:
            continue
        target = match.group(1).strip()
        calls.append(target)
        if "*" in target or "@plt" in target:
            reject(f"indirect or PLT call in candidate: {target}")
        if ENTER not in target and FINISH not in target:
            reject(f"unexpected linked candidate call: {target}")
    if not any(ENTER in target for target in calls):
        reject("linked candidate has no Residence Enter")
    if not any(FINISH in target for target in calls):
        reject("linked candidate has no Residence Finish")

    lowered = "\n".join(instruction_lines).lower()
    forbidden = (
        "lock ", "cmpxchg", "xadd", "memcmp", "eq_hh", "substring",
        "registry", "lease", "handle", "publication", "malloc", "calloc",
        "realloc", " free", "%xmm", "%ymm", "%zmm",
    )
    for token in forbidden:
        if token in lowered:
            reject(f"forbidden linked candidate structure: {token.strip()}")
    return {
        "candidate_symbol": CANDIDATE,
        "observed_call_target_kinds": sorted(
            {ENTER if ENTER in target else FINISH for target in calls}
        ),
        "indirect_call_count": 0,
        "plt_call_count": 0,
        "vector_register_use_count": 0,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--ir", required=True, type=pathlib.Path)
    parser.add_argument("--assembly", required=True, type=pathlib.Path)
    parser.add_argument("--binary", required=True, type=pathlib.Path)
    parser.add_argument("--report", required=True, type=pathlib.Path)
    parser.add_argument("--commit", required=True)
    args = parser.parse_args()
    try:
        ir_text = args.ir.read_text()
        assembly_text = args.assembly.read_text()
        if not args.binary.is_file() or args.binary.stat().st_size == 0:
            reject("linked candidate binary is missing")
        report = {
            "schema": SCHEMA,
            "commit": args.commit,
            "authority": "promotion-evidence-only",
            "ir_sha256": digest(args.ir),
            "assembly_sha256": digest(args.assembly),
            "binary_sha256": digest(args.binary),
            "ir": verify_ir(ir_text),
            "linked_assembly": verify_assembly(assembly_text),
            "verdict": "green",
        }
        temporary = args.report.with_suffix(args.report.suffix + ".tmp")
        temporary.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
        temporary.replace(args.report)
    except (OSError, ValueError) as error:
        print(f"[s6c-pinned-corridor-structural-zero] ERROR: {error}", file=sys.stderr)
        return 1
    print("[s6c-pinned-corridor-structural-zero] ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
