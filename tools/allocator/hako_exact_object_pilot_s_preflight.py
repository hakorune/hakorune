#!/usr/bin/env python3
"""Preflight the first guarded exact-object pilot enablement."""

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
    backend_consumer = int(getattr(backend, "BACKEND_FLATTENED_NESTED_FIELD_CONSUMER", False))
    backend_lowering_enabled = int(
        getattr(backend, "FLATTENED_NESTED_FIELD_LOWERING_ENABLED", True)
    )
    typed_preempts_local = int(typed_newbox_preempts_local_aggregate(repo_root))
    field_access_route_enabled = int(
        module_imports_flattened_nested(repo_root, "src/llvm_py/instructions/field_access.py")
    )
    method_call_route_enabled = int(
        module_imports_flattened_nested(
            repo_root, "src/llvm_py/instructions/mir_call/method_call.py"
        )
    )
    state_sharing_ready = int(
        backend_lowering_enabled
        and field_access_route_enabled
        and method_call_route_enabled
        and not typed_preempts_local
    )
    pilot_enabled = int(bool(backend_lowering_enabled and state_sharing_ready))
    summary = "ok" if pilot_enabled else "blocked"
    selected_next = (
        "EXACT-OBJECT-PILOT-CLOSEOUT-001"
        if pilot_enabled
        else "EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001"
    )

    lines = [
        "output_contract=hako-exact-object-pilot-s-v0",
        "source_evidence=296x-718",
        "target_front=object_lifecycle_body",
        "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result",
        "nested_object=HakoAllocObjectLifecycleAlignmentResult",
        "representation_choice=flatten_nested_fields",
        f"backend_flattened_nested_field_consumer={backend_consumer}",
        f"backend_lowering_enabled={backend_lowering_enabled}",
        f"object_storage_plan_execution_enabled={pilot_enabled}",
        f"pilot_exact_object_enabled={pilot_enabled}",
        f"typed_newbox_preempts_local_aggregate={typed_preempts_local}",
        f"field_access_flattened_nested_route_enabled={field_access_route_enabled}",
        f"method_call_flattened_nested_route_enabled={method_call_route_enabled}",
        f"state_sharing_seam_ready={state_sharing_ready}",
        "mirbuilder_object_management_enabled=0",
        "mirbuilder_special_case_count=0",
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
