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
    _load_map_lookup_fusion_metadata,
)


class _DummyResolver:
    def __init__(self):
        self.fastmem_access_plans_by_site = {}
        self.map_lookup_fusion_routes_by_site = {}


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


if __name__ == "__main__":
    unittest.main()
