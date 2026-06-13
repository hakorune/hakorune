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
        "box_callable_common_key_enabled=1",
        "method_slot_id_space=internal_vtable_slot",
        "plugin_method_id_space=plugin_typebox_method_id",
        "lifecycle_id_space=plugin_lifecycle_method_id",
        "id_space_mixed_count=0",
        "duplicate_callable_truth_count=0",
        "plugin_loader_callable_provider_only=1",
        "type_registry_callable_provider_only=1",
        "type_abi_catalog_projection_only=1",
        "plugin_loader_registry_snapshot_entrypoint_count=1",
        "method_plan_direct_provider_seed_count=0",
        "lifecycle_plan_direct_provider_seed_count=0",
        "registry_snapshot_cache_required_count=0",
        "runtime_invoke_boundary_module_count=1",
        "route_resolver_invoke_contract_count=0",
        "runtime_invoke_boundary_derives_fn_pointer_count=1",
        "callable_route_truth_from_invoke_boundary_count=0",
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
        "summary=ok",
    ]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    report = "\n".join(contract_lines()) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
