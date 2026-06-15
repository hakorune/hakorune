#!/usr/bin/env python3
"""Preflight exact-object pilot retry after ny-llvmc boundary consumer wiring."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path


BOUNDARY_TOOL = Path(
    "tools/allocator/hako_exact_object_flattened_nested_field_boundary_consumer.py"
)


def parse_lines(text: str) -> dict[str, str]:
    result: dict[str, str] = {}
    for line in text.splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        result[key] = value
    return result


def run_boundary_tool(repo_root: Path) -> dict[str, str]:
    cmd = [sys.executable, str(repo_root / BOUNDARY_TOOL), "--repo-root", str(repo_root)]
    proc = subprocess.run(cmd, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        raise RuntimeError(proc.stdout + proc.stderr)
    return parse_lines(proc.stdout)


def flag(values: dict[str, str], key: str) -> int:
    return 1 if values.get(key) == "1" else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    repo_root = args.repo_root.resolve()
    boundary = run_boundary_tool(repo_root)
    boundary_ready = int(
        boundary.get("summary") == "ok"
        and flag(boundary, "boundary_driver_flattened_nested_consumer")
        and flag(boundary, "field_access_lowering_connected")
        and flag(boundary, "nested_method_lowering_connected")
        and flag(boundary, "generated_artifact_reachability_proven")
    )
    pilot_enabled = boundary_ready
    summary = "ok" if pilot_enabled else "blocked"

    lines = [
        "output_contract=hako-exact-object-pilot-v-v0",
        "source_evidence=296x-728",
        "target_front=object_lifecycle_body",
        "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result",
        "nested_object=HakoAllocObjectLifecycleAlignmentResult",
        "representation_choice=flattened_nested_fields",
        f"boundary_driver_flattened_nested_consumer={flag(boundary, 'boundary_driver_flattened_nested_consumer')}",
        f"field_access_lowering_connected={flag(boundary, 'field_access_lowering_connected')}",
        f"nested_method_lowering_connected={flag(boundary, 'nested_method_lowering_connected')}",
        f"generated_artifact_reachability_proven={flag(boundary, 'generated_artifact_reachability_proven')}",
        f"backend_lowering_enabled={flag(boundary, 'backend_lowering_enabled')}",
        f"object_storage_plan_execution_enabled={pilot_enabled}",
        f"pilot_exact_object_enabled={pilot_enabled}",
        "mirbuilder_object_management_enabled=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        "product_default_changed=0",
        "fallback_to_generic_box_supported=1",
        "selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-002",
        f"summary={summary}",
    ]
    text_out = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text_out, encoding="utf-8")
    else:
        print(text_out, end="")
    return 0 if summary == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
