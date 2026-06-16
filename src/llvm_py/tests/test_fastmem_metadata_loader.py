#!/usr/bin/env python3
import unittest
import sys
from pathlib import Path

_REPO_ROOT = Path(__file__).resolve().parents[3]
_LLVM_PY_ROOT = _REPO_ROOT / "src" / "llvm_py"
for _path in (str(_REPO_ROOT), str(_LLVM_PY_ROOT)):
    if _path not in sys.path:
        sys.path.insert(0, _path)

from src.llvm_py.builders.function_metadata import (
    _load_fastmem_access_plan_metadata,
    _load_local_i64_map_direct_storage_plan_metadata,
    _load_local_i64_map_entry_value_tracking_plan_metadata,
    _load_local_fastpath_fact_metadata,
    _load_local_map_storage_realization_plan_metadata,
    _load_map_lookup_fusion_metadata,
    _load_map_repr_metadata,
)


class _DummyResolver:
    def __init__(self):
        self.fastmem_access_plans_by_site = {}
        self.map_lookup_fusion_routes_by_site = {}
        self.map_repr_plans_by_site = {}
        self.local_fastpath_facts_by_site = {}
        self.local_map_storage_realization_plans_by_receiver = {}
        self.local_i64_map_direct_storage_plans_by_receiver = {}
        self.local_i64_map_entry_value_tracking_plans_by_receiver = {}


class _DummyBuilder:
    def __init__(self):
        self.resolver = _DummyResolver()


class TestFastMemMetadataLoader(unittest.TestCase):
    def test_fastmem_access_plan_loader_preserves_size_fields(self):
        builder = _DummyBuilder()
        func_data = {
            "metadata": {
                "fastmem_access_plans": [
                    {
                        "block": "0",
                        "instruction_index": "1",
                        "region": "2",
                        "kind": "field_load",
                        "base": "10",
                        "result": "11",
                        "byte_offset": "40",
                        "field_size": "8",
                        "alignment": "8",
                    },
                    {
                        "block": "0",
                        "instruction_index": "2",
                        "region": "2",
                        "kind": "table_index",
                        "table": "20",
                        "index": "21",
                        "result": "22",
                        "element_stride": "8",
                        "element_size": "56",
                        "length": "64",
                        "alignment": "8",
                    },
                ]
            }
        }

        _load_fastmem_access_plan_metadata(builder, func_data)

        by_site = builder.resolver.fastmem_access_plans_by_site
        field_plan = by_site[(0, 1)][0]
        table_plan = by_site[(0, 2)][0]
        self.assertEqual(field_plan["field_size"], 8)
        self.assertEqual(table_plan["element_size"], 56)

    def test_map_lookup_fusion_loader_indexes_both_sites(self):
        builder = _DummyBuilder()
        func_data = {
            "metadata": {
                "map_lookup_fusion_routes": [
                    {
                        "block": "7",
                        "get_instruction_index": "11",
                        "has_instruction_index": "13",
                        "receiver_value": "20",
                        "key_value": "21",
                        "key_const": "-1",
                        "stored_value_proof": "scalar_i64_const",
                        "stored_value_const": "1",
                        "stored_value_known_nonzero": "true",
                    }
                ]
            }
        }

        _load_map_lookup_fusion_metadata(builder, func_data)

        by_site = builder.resolver.map_lookup_fusion_routes_by_site
        self.assertIn((7, 11), by_site)
        self.assertIn((7, 13), by_site)
        route = by_site[(7, 11)][0]
        self.assertEqual(route["receiver_value"], 20)
        self.assertEqual(route["key_const"], -1)
        self.assertTrue(route["stored_value_known_nonzero"])

    def test_map_repr_loader_indexes_sites(self):
        builder = _DummyBuilder()
        func_data = {
            "metadata": {
                "map_repr_plans": [
                    {
                        "route_id": "map_repr.generic_hash_runtime",
                        "repr_kind": "generic_hash_runtime",
                        "source_route_id": "generic_method.set",
                        "source_route_kind": "map_store_any",
                        "source_helper_symbol": "nyash.map.slot_store_hhh",
                        "block": "3",
                        "instruction_index": "9",
                        "surface_box_name": "MapBox",
                        "receiver_origin_box": "MapBox",
                        "method": "set",
                        "receiver_value": "20",
                        "key_value": "21",
                        "result_value": "22",
                        "key_route": "i64_const",
                        "value_demand": "write_any",
                        "proof_tag": "set_surface_policy",
                    }
                ]
            }
        }

        _load_map_repr_metadata(builder, func_data)

        by_site = builder.resolver.map_repr_plans_by_site
        self.assertIn((3, 9), by_site)
        plan = by_site[(3, 9)][0]
        self.assertEqual(plan["route_id"], "map_repr.generic_hash_runtime")
        self.assertEqual(plan["receiver_value"], 20)
        self.assertEqual(plan["key_value"], 21)
        self.assertEqual(plan["result_value"], 22)

    def test_local_fastpath_fact_loader_indexes_sites(self):
        builder = _DummyBuilder()
        func_data = {
            "metadata": {
                "local_fastpath_facts": [
                    {
                        "route_id": "local_fastpath.known_receiver_direct_call",
                        "fact_kind": "local_fastpath_fact",
                        "backend_kind": "known_receiver_direct_call",
                        "route_plan": "map_repr.generic_hash_runtime",
                        "site_id": "99",
                        "block": "5",
                        "instruction_index": "8",
                        "receiver_value": "20",
                        "key_value": "21",
                        "object_id": "20",
                        "alias_class": "3",
                        "route_plan_id": "4",
                        "storage_plan_id": "5",
                        "fallback_reason": None,
                    }
                ]
            }
        }

        _load_local_fastpath_fact_metadata(builder, func_data)

        by_site = builder.resolver.local_fastpath_facts_by_site
        self.assertIn((5, 8), by_site)
        fact = by_site[(5, 8)][0]
        self.assertEqual(fact["route_id"], "local_fastpath.known_receiver_direct_call")
        self.assertEqual(fact["receiver_value"], 20)
        self.assertEqual(fact["key_value"], 21)
        self.assertEqual(fact["site_id"], 99)
        self.assertEqual(fact["alias_class"], 3)

    def test_local_map_storage_realization_plan_loader_indexes_receivers(self):
        builder = _DummyBuilder()
        func_data = {
            "metadata": {
                "local_map_storage_realization_plans": [
                    {
                        "receiver_value": "20",
                        "representation": "local_i64_key_map",
                        "candidate_set_count": "3",
                        "candidate_scalar_get_count": "2",
                        "publication_materialization_required": "true",
                        "backend_lowering_enabled": "false",
                        "runtime_helper_enabled": False,
                    }
                ]
            }
        }

        _load_local_map_storage_realization_plan_metadata(builder, func_data)

        by_receiver = builder.resolver.local_map_storage_realization_plans_by_receiver
        self.assertIn(20, by_receiver)
        plan = by_receiver[20][0]
        self.assertEqual(plan["representation"], "local_i64_key_map")
        self.assertEqual(plan["candidate_set_count"], 3)
        self.assertEqual(plan["candidate_scalar_get_count"], 2)
        self.assertTrue(plan["publication_materialization_required"])
        self.assertFalse(plan["backend_lowering_enabled"])
        self.assertFalse(plan["runtime_helper_enabled"])

    def test_local_i64_map_direct_storage_plan_loader_indexes_receivers(self):
        builder = _DummyBuilder()
        func_data = {
            "metadata": {
                "local_i64_map_direct_storage_plans": [
                    {
                        "receiver_value": "20",
                        "representation": "closed_world_i64_key_value_table",
                        "known_i64_key_set_count": "3",
                        "scalar_get_count": "2",
                        "entry_value_tracking_enabled": "false",
                        "publication_materialization_required": "true",
                        "backend_lowering_enabled": "false",
                        "runtime_helper_enabled": False,
                    }
                ]
            }
        }

        _load_local_i64_map_direct_storage_plan_metadata(builder, func_data)

        by_receiver = builder.resolver.local_i64_map_direct_storage_plans_by_receiver
        self.assertIn(20, by_receiver)
        plan = by_receiver[20][0]
        self.assertEqual(plan["representation"], "closed_world_i64_key_value_table")
        self.assertEqual(plan["known_i64_key_set_count"], 3)
        self.assertEqual(plan["scalar_get_count"], 2)
        self.assertFalse(plan["entry_value_tracking_enabled"])
        self.assertTrue(plan["publication_materialization_required"])
        self.assertFalse(plan["backend_lowering_enabled"])
        self.assertFalse(plan["runtime_helper_enabled"])

    def test_local_i64_map_entry_value_tracking_plan_loader_indexes_receivers(self):
        builder = _DummyBuilder()
        func_data = {
            "metadata": {
                "local_i64_map_entry_value_tracking_plans": [
                    {
                        "receiver_value": "20",
                        "set_block": "3",
                        "set_instruction_index": "9",
                        "key_value": "21",
                        "value_value": "22",
                        "key_const_if_known": "-1",
                        "value_const_if_known": "7",
                        "backend_lowering_enabled": "false",
                        "runtime_helper_enabled": False,
                    }
                ]
            }
        }

        _load_local_i64_map_entry_value_tracking_plan_metadata(builder, func_data)

        by_receiver = builder.resolver.local_i64_map_entry_value_tracking_plans_by_receiver
        self.assertIn(20, by_receiver)
        plan = by_receiver[20][0]
        self.assertEqual(plan["receiver_value"], 20)
        self.assertEqual(plan["set_block"], 3)
        self.assertEqual(plan["set_instruction_index"], 9)
        self.assertEqual(plan["key_value"], 21)
        self.assertEqual(plan["value_value"], 22)
        self.assertEqual(plan["key_const_if_known"], -1)
        self.assertEqual(plan["value_const_if_known"], 7)
        self.assertFalse(plan["backend_lowering_enabled"])
        self.assertFalse(plan["runtime_helper_enabled"])


if __name__ == "__main__":
    unittest.main()
