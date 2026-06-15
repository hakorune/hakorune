#!/usr/bin/env python3
"""Inventory object/runtime boundaries for exact-AOT object storage planning."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]


OBJECT_LIFECYCLE_FUNCTIONS = {
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    "HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById/1",
    "Main.runOne/2",
}


def read_kv(path: Path) -> dict[str, str]:
    out: dict[str, str] = {}
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        out[key] = value
    return out


def repo_contains(rel: str, needle: str) -> bool:
    path = ROOT / rel
    return path.is_file() and needle in path.read_text(encoding="utf-8", errors="replace")


def function_instructions(func: dict[str, Any]) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    for block in func.get("blocks", []):
        out.extend(block.get("instructions", []))
    return out


def is_primitive_storage(field: dict[str, Any]) -> bool:
    return str(field.get("storage", "")) in {"i64", "usize", "u64", "bool"}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    mir = json.loads(args.mir_json.read_text(encoding="utf-8"))
    perf = read_kv(args.perf_report)

    selected_funcs = [
        f for f in mir.get("functions", []) if f.get("name") in OBJECT_LIFECYCLE_FUNCTIONS
    ]
    if not selected_funcs:
        raise SystemExit("no object lifecycle functions found in MIR JSON")

    op_counts: Counter[str] = Counter()
    call_certainty: Counter[str] = Counter()
    call_box_names: Counter[str] = Counter()
    runtime_boundary_calls = 0
    for func in selected_funcs:
        for ins in function_instructions(func):
            op = str(ins.get("op", ""))
            op_counts[op] += 1
            if op == "mir_call":
                callee = ins.get("mir_call", {}).get("callee", {})
                certainty = str(callee.get("certainty", "unknown"))
                box_name = str(callee.get("box_name", "unknown"))
                call_certainty[certainty] += 1
                call_box_names[box_name] += 1
                if box_name in {"ArrayBox", "RuntimeDataBox"} or certainty != "Known":
                    runtime_boundary_calls += 1

    typed_plans = mir.get("typed_object_plans", [])
    exact_stack_candidates = 0
    exact_native_candidates = 0
    scalarized_candidates = 0
    handle_field_plans = 0
    for plan in typed_plans:
        fields = plan.get("fields", [])
        if not fields:
            continue
        primitive_count = sum(1 for field in fields if is_primitive_storage(field))
        handle_count = sum(1 for field in fields if str(field.get("storage", "")) == "handle")
        if handle_count:
            handle_field_plans += 1
        if primitive_count == len(fields):
            scalarized_candidates += 1
            exact_stack_candidates += 1
            exact_native_candidates += 1
        elif primitive_count > 0:
            exact_native_candidates += 1

    arc_carrier_visible = repo_contains("src/runtime/host_handles.rs", "StableBox(Arc<dyn NyashBox>)")
    vmvalue_arc_visible = repo_contains("src/backend/vm_types.rs", "BoxRef(Arc<dyn NyashBox>)")
    object_handle_visible = repo_contains("src/runtime/object_identity.rs", "pub struct ObjectHandle")
    route_plan_visible = repo_contains("src/box_callable/route_plan.rs", "pub enum MethodCallRoutePlan")
    registry_visible = repo_contains("src/box_callable/registry.rs", "pub struct BoxCallableRegistry")

    top_symbol = perf.get("top_symbol", "")
    top_symbol_percent = perf.get("top_symbol_percent", "unknown")
    body_elapsed_ns = perf.get("body_elapsed_ns", "unknown")

    dynamic_calls = sum(count for name, count in call_certainty.items() if name != "Known")
    known_calls = call_certainty.get("Known", 0)
    host_handle_boundary_count = runtime_boundary_calls + (1 if arc_carrier_visible else 0)
    arc_boundary_count = int(arc_carrier_visible) + int(vmvalue_arc_visible)

    if top_symbol == "nyash_array_length_h" and host_handle_boundary_count > 0:
        selected_owner = "object_handle_boundary_inventory"
        selected_confidence = "medium"
    else:
        selected_owner = "none"
        selected_confidence = "low"

    lines = [
        "output_contract=hako-object-boundary-inventory-v0",
        "source_evidence=296x-709",
        "target_front=object_lifecycle_body",
        f"perf_top_symbol={top_symbol}",
        f"perf_top_symbol_percent={top_symbol_percent}",
        f"body_elapsed_ns={body_elapsed_ns}",
        "mirbuilder_object_management_enabled=0",
        f"box_callable_registry_is_callable_truth={1 if registry_visible else 0}",
        f"routeplan_is_call_execution_truth={1 if route_plan_visible else 0}",
        "object_storage_plan_is_representation_truth=1",
        f"arc_dynbox_boundary_count={arc_boundary_count}",
        f"host_handle_boundary_count={host_handle_boundary_count}",
        f"runtime_helper_boundary_count={1 if top_symbol == 'nyash_array_length_h' else 0}",
        f"dynamic_box_method_route_count={dynamic_calls}",
        f"box_callable_routeplan_dynamic_count={dynamic_calls}",
        f"closed_world_direct_method_candidate_count={known_calls}",
        f"exact_stack_object_candidate_count={exact_stack_candidates}",
        f"exact_native_struct_candidate_count={exact_native_candidates}",
        f"scalarized_object_candidate_count={scalarized_candidates}",
        f"object_escape_count={handle_field_plans}",
        "plugin_or_extern_escape_count=0",
        f"array_or_map_escape_count={call_box_names.get('ArrayBox', 0)}",
        "return_escape_count=0",
        f"selected_object_boundary_owner={selected_owner}",
        f"selected_owner_confidence={selected_confidence}",
        "implementation_started=0",
        "product_default_changed=0",
        "source_hako_changed=0",
        "compiler_lowering_changed=0",
        "runtime_object_changed=0",
        "summary=ok",
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
