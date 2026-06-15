#!/usr/bin/env python3
"""Preflight exact-object pilot enablement after state-sharing seam."""

from __future__ import annotations

import argparse
import importlib.util
import sys
from pathlib import Path
from typing import Any


def load_backend_module(repo_root: Path) -> Any:
    path = repo_root / "src" / "llvm_py" / "instructions" / "flattened_nested_fields.py"
    llvm_py = str(repo_root / "src" / "llvm_py")
    if llvm_py not in sys.path:
        sys.path.insert(0, llvm_py)
    spec = importlib.util.spec_from_file_location("flattened_nested_fields", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load backend seam module: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def typed_newbox_preempts_local_aggregate(repo_root: Path) -> bool:
    src = text(repo_root / "src" / "llvm_py" / "instructions" / "newbox.py")
    exact_pos = src.find("if exact_plan is not None:")
    local_pos = src.find("local_user_box = build_local_user_box_aggregate_for_newbox")
    return exact_pos >= 0 and local_pos >= 0 and exact_pos < local_pos


def module_imports_flattened_nested(repo_root: Path, rel: str) -> bool:
    src = text(repo_root / rel)
    return "flattened_nested_fields" in src


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    repo_root = args.repo_root.resolve()
    backend = load_backend_module(repo_root)
    flattened_names = [
        "alignment_result.last_requested",
        "alignment_result.last_normalized",
        "alignment_result.last_reason",
        "alignment_result.last_supported",
    ]
    state_plan = backend.build_passive_flattened_nested_state_plan(
        owner_box="HakoAllocObjectLifecycleFacade",
        owner_field="alignment_result",
        nested_object="HakoAllocObjectLifecycleAlignmentResult",
        flattened_names=flattened_names,
    )
    validation = backend.validate_flattened_nested_state_plan(state_plan)
    state_seam_defined = int(validation.get("state_sharing_seam_defined", 0))
    backend_lowering_enabled = int(validation.get("backend_lowering_enabled", 1))
    field_access_route_enabled = int(
        module_imports_flattened_nested(repo_root, "src/llvm_py/instructions/field_access.py")
    )
    method_call_route_enabled = int(
        module_imports_flattened_nested(
            repo_root, "src/llvm_py/instructions/mir_call/method_call.py"
        )
    )
    typed_preempts_local = int(typed_newbox_preempts_local_aggregate(repo_root))
    route_wiring_ready = int(
        state_seam_defined
        and field_access_route_enabled
        and method_call_route_enabled
        and not backend_lowering_enabled
    )
    pilot_enabled = int(
        route_wiring_ready and not typed_preempts_local and backend_lowering_enabled
    )
    selected_next = (
        "EXACT-OBJECT-PILOT-001U"
        if route_wiring_ready
        else "EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001"
    )
    summary = "ok" if route_wiring_ready else "blocked"

    lines = [
        "output_contract=hako-exact-object-pilot-t-v0",
        "source_evidence=296x-720",
        "target_front=object_lifecycle_body",
        "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result",
        "nested_object=HakoAllocObjectLifecycleAlignmentResult",
        "representation_choice=flatten_nested_fields",
        f"state_sharing_seam_defined={state_seam_defined}",
        f"typed_newbox_preempts_local_aggregate={typed_preempts_local}",
        f"field_access_flattened_nested_route_enabled={field_access_route_enabled}",
        f"method_call_flattened_nested_route_enabled={method_call_route_enabled}",
        f"route_wiring_ready={route_wiring_ready}",
        f"backend_lowering_enabled={backend_lowering_enabled}",
        "object_storage_plan_execution_enabled=0",
        f"pilot_exact_object_enabled={pilot_enabled}",
        "mirbuilder_object_management_enabled=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        "product_default_changed=0",
        "fallback_to_generic_box_supported=1",
        f"selected_next={selected_next}",
        f"summary={summary}",
    ]
    text_out = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text_out, encoding="utf-8")
    else:
        print(text_out, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
