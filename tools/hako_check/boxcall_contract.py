#!/usr/bin/env python3
"""Emit the hako_check BoxCallable / TypeAbiCatalog contract report."""

from __future__ import annotations

import argparse
from pathlib import Path


def contract_lines() -> list[str]:
    return [
        "output_contract=hako-check-boxcall-contract-v0",
        "tool_surface=hako_check_boxcall_contract",
        "observation_only=1",
        "rewrite_executed=0",
        "keeper_selection=0",
        "type_abi_catalog_is_truth=0",
        "type_abi_pack_is_truth=0",
        "type_abi_existing_refresh_preserved=1",
        "type_abi_refresh_truth_trait_enabled=0",
        "type_abi_catalog_from_refreshed_world=1",
        "type_abi_catalog_refresh_owner_count=0",
        "type_abi_catalog_hot_lookup_count=0",
        "type_abi_pack_used_by_planner_count=0",
        "generic_typeabi_generate_plans_count=0",
        "box_callable_registry_enabled=1",
        "box_callable_registry_truth_owner=1",
        "box_callable_common_key_enabled=1",
        "method_slot_id_space=internal_vtable_slot",
        "plugin_method_id_space=plugin_typebox_method_id",
        "lifecycle_id_space=plugin_lifecycle_method_id",
        "id_space_mixed_count=0",
        "duplicate_callable_truth_count=0",
        "plugin_loader_callable_provider_only=1",
        "type_registry_callable_provider_only=1",
        "box_callable_provider_source_stored=1",
        "string_surface_catalog_provider_rows=1",
        "array_surface_catalog_provider_rows=1",
        "map_surface_catalog_provider_rows=1",
        "buffer_surface_catalog_required_before_provider_rows=1",
        "buffer_surface_catalog_exists=1",
        "buffer_surface_catalog_visible_methods_named=1",
        "buffer_provider_rows_not_added_before_catalog=1",
        "buffer_storage_mechanics_owner=substrate",
        "buffer_surface_catalog_provider_rows=1",
        "buffer_vm_handler_dispatch_owner=1",
        "buffer_visible_semantics_changed=0",
        "type_registry_execution_truth_owner=0",
        "type_registry_slot_vocabulary_provider=1",
        "type_registry_dispatch_behavior_owner=0",
        "vm_dispatch_by_slot_behavior_owner=1",
        "wasm_dispatch_by_slot_behavior_owner=1",
        "type_abi_catalog_projection_only=1",
        "typeabi_catalog_execution_route_count=0",
        "plugin_loader_provider_snapshot_only=1",
        "plugin_loader_registry_snapshot_entrypoint_count=1",
        "plugin_callable_export_contains_fn_pointer_count=0",
        "typebox_abi_v2_changed=0",
        "plugin_lifecycle_snapshot_filtered_count=1",
        "method_plan_direct_provider_seed_count=0",
        "lifecycle_plan_direct_provider_seed_count=0",
        "registry_snapshot_cache_required_count=0",
        "runtime_invoke_boundary_module_count=1",
        "route_resolver_invoke_contract_count=0",
        "runtime_invoke_boundary_derives_fn_pointer_count=1",
        "callable_route_truth_from_invoke_boundary_count=0",
        "runtime_invoke_boundary_owns_method_id_count=0",
        "runtime_invoke_boundary_owns_lifecycle_id_count=0",
        "runtime_invoke_boundary_typeabi_lookup_count=0",
        "runtime_invoke_boundary_function_pointer_binding_count=1",
        "plugin_catalog_projection_chain_documented=1",
        "plugin_loader_to_typeabi_direct_truth_count=0",
        "type_abi_catalog_as_plugin_route_truth_count=0",
        "plugin_snapshot_catalog_projection_helper_count=1",
        "plugin_snapshot_catalog_reads_loader_directly=0",
        "registry_snapshot_cache_default_enabled=0",
        "plugin_catalog_tooling_consumer_count=1",
        "plugin_catalog_routeplan_consumer_count=0",
        "plugin_catalog_hot_path_consumer_count=0",
        "route_plan_type_abi_hot_lookup_count=0",
        "route_plan_semantic_data_only=1",
        "route_plan_executable_pointer_count=0",
        "runtime_invoke_boundary_executable_pointer_owner=1",
        "provider_executable_pointer_count=0",
        "catalog_executable_pointer_count=0",
        "route_plan_uses_registry_entry_target=1",
        "route_plan_uses_provider_source_as_execution_route=0",
        "boxcall_foundation_closeout_ready=1",
        "provider_rows_cover_builtin_plugin_surface=1",
        "boxcall_next_lane_requires_selection=1",
        "summary=ok",
    ]


def plugin_catalog_sample_lines() -> list[str]:
    return [
        "plugin_catalog_sample_contract=hako-check-boxcall-plugin-catalog-sample-v0",
        "plugin_catalog_tooling_example_count=1",
        "plugin_catalog_sample_source=fixture_plugin_callable_exports",
        "plugin_catalog_sample_chain=PluginCallableExport>BoxCallableRegistry>TypeAbiCatalog",
        "plugin_catalog_sample_entry_count=3",
        "plugin_catalog_sample_method_entry_count=1",
        "plugin_catalog_sample_lifecycle_entry_count=2",
        "plugin_catalog_sample_method_name=run",
        "plugin_catalog_sample_lifecycle_names=birth,fini",
        "plugin_catalog_sample_routeplan_consumer_count=0",
        "plugin_catalog_sample_hot_path_consumer_count=0",
        "plugin_catalog_sample_executes_plugin_loader_count=0",
        "plugin_loader_to_typeabi_direct_truth_count=0",
        "type_abi_catalog_as_plugin_route_truth_count=0",
        "route_plan_type_abi_hot_lookup_count=0",
        "boxcall_contract_split_required_count=0",
        "boxcall_sample_subcommand_required_count=0",
        "boxcall_contract_optional_sample_flag_count=1",
    ]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    parser.add_argument(
        "--include-plugin-catalog-sample",
        action="store_true",
        help="append an observation-only plugin catalog projection sample",
    )
    args = parser.parse_args()

    lines = contract_lines()
    if args.include_plugin_catalog_sample:
        lines.extend(plugin_catalog_sample_lines())
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
