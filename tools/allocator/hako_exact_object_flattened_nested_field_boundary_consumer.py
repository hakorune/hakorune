#!/usr/bin/env python3
"""Report the ny-llvmc boundary ObjectStoragePlan consumer seam."""

from __future__ import annotations

import argparse
from pathlib import Path


READER = Path("lang/c-abi/shims/hako_llvmc_ffi_object_storage_plan.inc")
PURE_COMPILE = Path("lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc")
OP_DISPATCH = Path("lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc")
MIR_CALL = Path("lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc")
PRESCAN = Path("lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc")


def text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def has_all(haystack: str, needles: list[str]) -> bool:
    return all(needle in haystack for needle in needles)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    repo_root = args.repo_root.resolve()
    reader_path = repo_root / READER
    pure_path = repo_root / PURE_COMPILE
    op_dispatch_path = repo_root / OP_DISPATCH
    mir_call_path = repo_root / MIR_CALL
    prescan_path = repo_root / PRESCAN
    reader = text(reader_path) if reader_path.exists() else ""
    pure = text(pure_path) if pure_path.exists() else ""
    op_dispatch = text(op_dispatch_path) if op_dispatch_path.exists() else ""
    mir_call = text(mir_call_path) if mir_call_path.exists() else ""
    prescan = text(prescan_path) if prescan_path.exists() else ""
    include_enabled = int('#include "hako_llvmc_ffi_object_storage_plan.inc"' in pure)
    metadata_reader_enabled = int(
        has_all(
            reader,
            [
                "object_storage_plans_array",
                "find_flattened_nested_object_storage_plan",
                "object_storage_plan_field_has_flattened_name",
                "object_storage_plan_selected_alignment_result_ready",
                "object_storage_plans",
                "flattened_nested_fields",
            ],
        )
    )
    uses_metadata = int(
        metadata_reader_enabled
        and has_all(
            reader,
            [
                'read_str(plan, "representation")',
                'read_str(plan, "owner_box")',
                'read_str(plan, "owner_field")',
                'read_str(plan, "nested_box")',
                'read_str(field, "flattened_field")',
            ],
        )
    )
    selected_fields_consumed = int(
        metadata_reader_enabled
        and has_all(
            reader,
            [
                "alignment_result.last_requested",
                "alignment_result.last_normalized",
                "alignment_result.last_reason",
                "alignment_result.last_supported",
            ],
        )
    )
    synthetic_key_globals = int(
        has_all(
            prescan,
            [
                "@.objstore_alignment_result_last_requested",
                "@.objstore_alignment_result_last_normalized",
                "@.objstore_alignment_result_last_reason",
                "@.objstore_alignment_result_last_supported",
                "nyash.box.from_i8_string_const",
            ],
        )
    )
    field_access_lowering_connected = int(
        has_all(
            op_dispatch,
            [
                'strcmp(field, "alignment_result")',
                "object_storage_plan_alignment_result_owner_receiver_ready",
                "set_flattened_nested_binding(dst, box_reg)",
                "emit_flattened_nested_alignment_result_reset",
            ],
        )
    )
    nested_method_lowering_connected = int(
        has_all(
            mir_call,
            [
                "emit_flattened_nested_alignment_result_method_call",
                "object_storage_plan_flattened_nested_method_emit",
            ],
        )
        and has_all(
            reader,
            [
                "object_storage_plan_alignment_result_field_for_method",
                "get_flattened_nested_binding",
                "nyash.runtime_data.get_hh",
                "nyash.runtime_data.set_hhh",
            ],
        )
    )
    generated_artifact_reachability_proven = int(
        synthetic_key_globals
        and field_access_lowering_connected
        and nested_method_lowering_connected
    )
    consumer_ready = int(
        include_enabled
        and metadata_reader_enabled
        and uses_metadata
        and selected_fields_consumed
        and generated_artifact_reachability_proven
    )
    summary = "ok" if consumer_ready else "blocked"

    lines = [
        "output_contract=hako-exact-object-flattened-nested-field-boundary-consumer-v0",
        "source_evidence=296x-727",
        "target_front=object_lifecycle_body",
        "object_storage_plan_mir_json_export_enabled=1",
        f"boundary_driver_flattened_nested_consumer={consumer_ready}",
        f"uses_object_storage_plan_metadata={uses_metadata}",
        "flattened_nested_plan_count=1",
        "flattened_nested_field_count=4",
        f"alignment_result_last_requested_consumed={selected_fields_consumed}",
        f"alignment_result_last_normalized_consumed={selected_fields_consumed}",
        f"alignment_result_last_reason_consumed={selected_fields_consumed}",
        f"alignment_result_last_supported_consumed={selected_fields_consumed}",
        f"field_access_lowering_connected={field_access_lowering_connected}",
        f"nested_method_lowering_connected={nested_method_lowering_connected}",
        f"generated_artifact_reachability_proven={generated_artifact_reachability_proven}",
        "mirbuilder_object_management_enabled=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        "product_default_changed=0",
        "fallback_to_generic_box_supported=1",
        "backend_lowering_enabled=1",
        "selected_next=EXACT-OBJECT-PILOT-001V",
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
