#!/usr/bin/env python3
"""Report passive route wiring for flattened nested field state."""

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


def has_passive_route_hook(repo_root: Path, rel: str, hook_name: str) -> bool:
    src = text(repo_root / rel)
    return "flattened_nested_fields" in src and hook_name in src


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    repo_root = args.repo_root.resolve()
    backend = load_backend_module(repo_root)
    field_access_route_enabled = int(
        has_passive_route_hook(
            repo_root,
            "src/llvm_py/instructions/field_access.py",
            "_flattened_nested_field_access_route_enabled",
        )
    )
    method_call_route_enabled = int(
        has_passive_route_hook(
            repo_root,
            "src/llvm_py/instructions/mir_call/method_call.py",
            "_flattened_nested_method_call_route_enabled",
        )
    )
    state_seam_defined = int(
        getattr(backend, "FLATTENED_NESTED_FIELD_STATE_SEAM_DEFINED", False)
    )
    backend_lowering_enabled = int(
        getattr(backend, "FLATTENED_NESTED_FIELD_LOWERING_ENABLED", True)
    )
    route_wiring_ready = int(
        state_seam_defined
        and field_access_route_enabled
        and method_call_route_enabled
        and not backend_lowering_enabled
    )
    summary = "ok" if route_wiring_ready else "blocked"

    lines = [
        "output_contract=hako-exact-object-flattened-nested-field-route-wiring-v0",
        "source_evidence=296x-721",
        "target_front=object_lifecycle_body",
        "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result",
        "nested_object=HakoAllocObjectLifecycleAlignmentResult",
        f"state_sharing_seam_defined={state_seam_defined}",
        f"field_access_flattened_nested_route_enabled={field_access_route_enabled}",
        f"method_call_flattened_nested_route_enabled={method_call_route_enabled}",
        f"route_wiring_ready={route_wiring_ready}",
        f"backend_lowering_enabled={backend_lowering_enabled}",
        "object_storage_plan_execution_enabled=0",
        "pilot_exact_object_enabled=0",
        "mirbuilder_object_management_enabled=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        "product_default_changed=0",
        "fallback_to_generic_box_supported=1",
        "selected_next=EXACT-OBJECT-PILOT-001U",
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
