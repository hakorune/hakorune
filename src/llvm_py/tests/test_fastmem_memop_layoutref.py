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


def _verified_field_load_plan():
    return {
        "kind": "field_load",
        "verified": True,
        "status": "verified",
        "block": 0,
        "instruction_index": 1,
        "region": 1,
        "base": 12,
        "result": 13,
        "layout_id": "PageMetaLayoutV0",
        "field_id": "capacity",
        "access": "load",
        "byte_offset": 40,
        "field_size": 8,
        "field_type": "usize",
        "alignment": 8,
        "field_class": "plain_scalar",
    }


def _verified_field_store_plan():
    return {
        "kind": "field_store",
        "verified": True,
        "status": "verified",
        "block": 0,
        "instruction_index": 2,
        "region": 1,
        "base": 12,
        "value": 14,
        "layout_id": "PageMetaLayoutV0",
        "field_id": "used",
        "access": "store",
        "byte_offset": 48,
        "field_size": 8,
        "field_type": "usize",
        "alignment": 8,
        "mutability": "mutable",
        "field_class": "plain_scalar",
    }


class TestFastMemMemOpLayoutRef(unittest.TestCase):
    def test_current_alloc_owner_id_lowers_to_intrinsic_scalar_vmap(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        vmap = {}

        lower_memop(
            builder,
            {"kind": "current_alloc_owner_id", "dst": 15, "operands": []},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(vmap[15])

        self.assertIn(15, vmap)
        self.assertNotIn(15, resolver.fastmem_layout_refs)
        text = str(module)
        self.assertIn('declare i64 @"hako_fastmem_current_alloc_owner_id"()', text)
        self.assertIn('call i64 @"hako_fastmem_current_alloc_owner_id"()', text)

    def test_owner_eq_lowers_to_scalar_equality_only(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        vmap = {
            20: ir.Constant(i64, 7),
            21: ir.Constant(i64, 7),
        }

        lower_memop(
            builder,
            {"kind": "owner_eq", "dst": 22, "operands": [20, 21]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        self.assertIn(22, vmap)
        self.assertEqual(vmap[22].type.width, 1)
        self.assertNotIn(22, resolver.fastmem_layout_refs)
        self.assertIn("fastmem_owner_eq_22", str(module))

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

    def test_field_load_from_layout_ref_writes_scalar_vmap(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 1
        resolver.fastmem_access_plans_by_site[(0, 1)] = [_verified_field_load_plan()]
        base_ptr = builder.inttoptr(
            ir.Constant(i64, 8192),
            ir.IntType(8).as_pointer(),
            name="base_layout_ref",
        )
        resolver.fastmem_layout_refs[12] = {
            "ptr": base_ptr,
            "layout_id": "PageMetaLayoutV0",
            "table_id": "page_table",
            "region": 1,
        }
        vmap = {}

        lower_memop(
            builder,
            {"kind": "field_load", "dst": 13, "operands": [12]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(vmap[13])

        self.assertIn(13, vmap)
        self.assertIn("fastmem_field_ptr_13", str(module))
        self.assertIn("fastmem_field_load_13", str(module))

    def test_field_load_requires_layout_ref_base(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 1
        resolver.fastmem_access_plans_by_site[(0, 1)] = [_verified_field_load_plan()]

        with self.assertRaisesRegex(RuntimeError, "expected-layout-ref"):
            lower_memop(
                builder,
                {"kind": "field_load", "dst": 13, "operands": [12]},
                {},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_field_load_rejects_layout_mismatch(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 1
        resolver.fastmem_access_plans_by_site[(0, 1)] = [_verified_field_load_plan()]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="wrong_layout_ref",
            ),
            "layout_id": "OtherLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "layout-ref-mismatch"):
            lower_memop(
                builder,
                {"kind": "field_load", "dst": 13, "operands": [12]},
                {},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_field_load_rejects_atomic_publication_field(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 1
        plan = _verified_field_load_plan()
        plan["field_id"] = "remote_head"
        plan["field_class"] = "atomic_remote_head"
        resolver.fastmem_access_plans_by_site[(0, 1)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="atomic_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "unsupported-field-load-class"):
            lower_memop(
                builder,
                {"kind": "field_load", "dst": 13, "operands": [12]},
                {},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_field_store_from_layout_ref_emits_store(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 2
        resolver.fastmem_access_plans_by_site[(0, 2)] = [_verified_field_store_plan()]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="store_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {14: ir.Constant(i64, 7)}

        lower_memop(
            builder,
            {"kind": "field_store", "operands": [12, 14]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        self.assertIn("fastmem_field_store_ptr", str(module))
        self.assertIn("store i64 7", str(module))

    def test_field_store_rejects_immutable_field(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 2
        plan = _verified_field_store_plan()
        plan["field_id"] = "capacity"
        plan["mutability"] = "immutable_after_claim"
        resolver.fastmem_access_plans_by_site[(0, 2)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="immutable_store_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "unsupported-field-store-mutability"):
            lower_memop(
                builder,
                {"kind": "field_store", "operands": [12, 14]},
                {14: ir.Constant(i64, 1)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_field_store_rejects_local_free_head_field(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 2
        plan = _verified_field_store_plan()
        plan["field_id"] = "local_free_head"
        plan["field_class"] = "local_free_head"
        resolver.fastmem_access_plans_by_site[(0, 2)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="local_free_store_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "unsupported-field-store-class"):
            lower_memop(
                builder,
                {"kind": "field_store", "operands": [12, 14]},
                {14: ir.Constant(i64, 1)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_field_store_rejects_atomic_publication_field(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 2
        plan = _verified_field_store_plan()
        plan["field_id"] = "remote_head"
        plan["mutability"] = "atomic_only"
        plan["field_class"] = "atomic_remote_head"
        resolver.fastmem_access_plans_by_site[(0, 2)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="atomic_store_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "unsupported-field-store-mutability"):
            lower_memop(
                builder,
                {"kind": "field_store", "operands": [12, 14]},
                {14: ir.Constant(i64, 1)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )


if __name__ == "__main__":
    unittest.main()
