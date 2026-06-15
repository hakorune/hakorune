#!/usr/bin/env python3
"""Preflight one exact-object pilot and report whether lowering may open."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


TARGET_FRONT = "object_lifecycle_body"


def function_instructions(func: dict[str, Any]) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    for block in func.get("blocks", []) or []:
        for inst in block.get("instructions", []) or []:
            if isinstance(inst, dict):
                out.append(inst)
    return out


def callee_box_name(inst: dict[str, Any]) -> str | None:
    if inst.get("op") != "mir_call":
        return None
    callee = inst.get("mir_call", {}).get("callee", {})
    if not isinstance(callee, dict):
        return None
    name = callee.get("box_name")
    return name if isinstance(name, str) else None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--candidate", required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    mir = json.loads(args.mir_json.read_text(encoding="utf-8"))
    candidate = args.candidate

    facade_publication_sets = 0
    facade_publication_gets = 0
    candidate_method_call_count = 0
    candidate_birth_call_count = 0

    for func in mir.get("functions", []) or []:
        if not isinstance(func, dict):
            continue
        for inst in function_instructions(func):
            if inst.get("op") == "field_set" and inst.get("field") == "alignment_result":
                declared = inst.get("declared_type")
                if isinstance(declared, dict) and declared.get("box_type") == candidate:
                    facade_publication_sets += 1
            if inst.get("op") == "field_get" and inst.get("field") == "alignment_result":
                declared = inst.get("declared_type")
                if isinstance(declared, dict) and declared.get("box_type") == candidate:
                    facade_publication_gets += 1
            if callee_box_name(inst) == candidate:
                candidate_method_call_count += 1
                callee = inst.get("mir_call", {}).get("callee", {})
                if isinstance(callee, dict) and callee.get("name") == "birth":
                    candidate_birth_call_count += 1

    publication_boundary_count = facade_publication_sets + facade_publication_gets
    boundary_open = publication_boundary_count > 0
    pilot_enabled = 0 if boundary_open else 1
    summary = "blocked" if boundary_open else "ok"
    selected_next = (
        "EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001"
        if boundary_open
        else "EXACT-OBJECT-PILOT-IMPLEMENTATION-001"
    )

    lines = [
        "output_contract=hako-exact-object-pilot-v0",
        "source_evidence=296x-712",
        f"target_front={TARGET_FRONT}",
        f"pilot_candidate={candidate}",
        "object_storage_plan_execution_enabled=0",
        f"pilot_exact_object_enabled={pilot_enabled}",
        "closed_world_plan_required=1",
        "mirbuilder_object_management_enabled=0",
        "mirbuilder_special_case_count=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        f"observed_publication_boundary={'Facade.alignment_result_handle_field' if boundary_open else 'none'}",
        f"publication_boundary_count={publication_boundary_count}",
        f"facade_alignment_result_set_count={facade_publication_sets}",
        f"facade_alignment_result_get_count={facade_publication_gets}",
        f"candidate_method_call_count={candidate_method_call_count}",
        f"candidate_birth_call_count={candidate_birth_call_count}",
        "product_default_changed=0",
        "source_hako_changed=0",
        "runtime_object_changed=0",
        "fallback_to_generic_box_supported=1",
        f"selected_next={selected_next}",
        f"summary={summary}",
    ]
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
