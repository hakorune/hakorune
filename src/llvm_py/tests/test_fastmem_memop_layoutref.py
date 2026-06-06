#!/usr/bin/env python3
import unittest
import sys
from pathlib import Path

import llvmlite.ir as ir

_REPO_ROOT = Path(__file__).resolve().parents[3]
_LLVM_PY_ROOT = _REPO_ROOT / "src" / "llvm_py"
for _path in (str(_REPO_ROOT), str(_LLVM_PY_ROOT)):
    if _path not in sys.path:
        sys.path.insert(0, _path)

from src.llvm_py.instructions.memop import lower_memop


class _DummyResolver:
    def __init__(self):
        self.current_block_id = 0
        self.current_instruction_index = 0
        self.fastmem_access_plans_by_site = {}
        self.fastmem_layout_refs = {}
        self.global_vmap = {}


def _new_builder():
    i64 = ir.IntType(64)
    module = ir.Module(name="test_fastmem_memop_layoutref")
    fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
    bb = fn.append_basic_block("bb0")
    builder = ir.IRBuilder(bb)
    return i64, module, builder


def _verified_table_plan():
    return {
        "kind": "table_index",
        "verified": True,
        "status": "verified",
        "block": 0,
        "instruction_index": 0,
        "region": 1,
        "table_id": "page_table",
        "table": 10,
        "index": 11,
        "result": 12,
        "element_layout_id": "PageMetaLayoutV0",
        "element_repr": "pointer_to_element",
        "element_stride": 8,
        "element_size": 56,
        "length": 64,
        "alignment": 8,
        "table_length_resolved": True,
        "bounds_proof_valid": True,
        "stride_resolved": True,
        "field_offset_resolved": True,
        "overflow_proof_valid": True,
        "alignment_valid": True,
        "element_layout_verified": True,
    }


class TestFastMemMemOpLayoutRef(unittest.TestCase):
    def test_table_index_lowers_to_layout_ref_map_not_vmap(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.fastmem_access_plans_by_site[(0, 0)] = [_verified_table_plan()]
        vmap = {
            10: ir.Constant(i64, 4096),
            11: ir.Constant(i64, 1),
        }

        lower_memop(
            builder,
            {"kind": "table_index", "dst": 12, "operands": [10, 11]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        self.assertNotIn(12, vmap)
        self.assertIn(12, resolver.fastmem_layout_refs)
        ref = resolver.fastmem_layout_refs[12]
        self.assertEqual(ref["layout_id"], "PageMetaLayoutV0")
        self.assertEqual(ref["table_id"], "page_table")
        self.assertIn("fastmem_table_slot_ptr_12", str(module))
        self.assertIn("fastmem_layout_ref_ptr_12", str(module))

    def test_layout_ref_cannot_be_used_as_ordinary_operand(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.fastmem_layout_refs[12] = {
            "ptr": ir.Constant(ir.IntType(8).as_pointer(), None),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "layout-ref-as-ordinary-value"):
            lower_memop(
                builder,
                {"kind": "add", "dst": 13, "operands": [12, 12]},
                {},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_incomplete_table_plan_is_rejected(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        plan = _verified_table_plan()
        plan["overflow_proof_valid"] = False
        resolver.fastmem_access_plans_by_site[(0, 0)] = [plan]

        with self.assertRaisesRegex(RuntimeError, "incomplete-table-plan"):
            lower_memop(
                builder,
                {"kind": "table_index", "dst": 12, "operands": [10, 11]},
                {10: ir.Constant(i64, 4096), 11: ir.Constant(i64, 1)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )


if __name__ == "__main__":
    unittest.main()
