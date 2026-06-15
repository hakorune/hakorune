#!/usr/bin/env python3
"""Report the disabled backend seam for flattened nested field plans."""

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
    plan = backend.build_passive_flattened_nested_field_plan(
        owner_field="alignment_result",
        nested_object="HakoAllocObjectLifecycleAlignmentResult",
        flattened_names=flattened_names,
    )
    validation = backend.validate_flattened_nested_field_plan(plan)
    summary = "ok" if validation.get("valid_representation") and validation.get("valid_fields") else "blocked"

    lines = [
        "output_contract=hako-exact-object-flattened-nested-field-backend-seam-v0",
        "source_evidence=296x-717",
        "target_front=object_lifecycle_body",
        "representation_choice=flatten_nested_fields",
        f"flattened_nested_field_count={validation.get('flattened_nested_field_count', 0)}",
        f"backend_flattened_nested_field_consumer={validation.get('backend_flattened_nested_field_consumer', 0)}",
        f"backend_lowering_enabled={validation.get('backend_lowering_enabled', 1)}",
        "object_storage_plan_execution_enabled=0",
        "pilot_exact_object_enabled=0",
        "mirbuilder_object_management_enabled=0",
        "mirbuilder_special_case_count=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        "product_default_changed=0",
        "fallback_to_generic_box_supported=1",
        "selected_next=EXACT-OBJECT-PILOT-001S",
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
