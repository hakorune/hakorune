#!/usr/bin/env python3
import sys
from pathlib import Path
import unittest

_REPO_ROOT = Path(__file__).resolve().parents[3]
_LLVM_PY_ROOT = _REPO_ROOT / "src" / "llvm_py"
for _path in (str(_REPO_ROOT), str(_LLVM_PY_ROOT)):
    if _path not in sys.path:
        sys.path.insert(0, _path)

from src.llvm_py.tests.test_fastmem_memop_layoutref import (
    _DummyResolver,
    _new_builder,
    _verified_atomic_remote_head_drain_plan,
    _verified_atomic_remote_head_push_plan,
    _verified_drain_remote_list_to_local_plan,
    _verified_free_head_push_plan,
    _verified_local_free_pop_plan,
    _verified_local_free_push_plan,
)
from src.llvm_py.instructions.memop import lower_memop

import llvmlite.ir as ir


class TestFastMemMemOpLayoutRefFreeAndRemote(unittest.TestCase):
    def test_local_free_push_lowers_verified_plan_only(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 3
        resolver.fastmem_access_plans_by_site[(0, 3)] = [_verified_local_free_push_plan()]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="local_free_page_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {15: ir.Constant(i64, 12288)}

        lower_memop(
            builder,
            {"kind": "local_free_push", "operands": [12, 15]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        text = str(module)
        self.assertIn("fastmem_local_free_old_head", text)
        self.assertIn("fastmem_local_free_block_next_ptr", text)
        self.assertIn("store i64 12288", text)
        self.assertNotIn(12, vmap)

    def test_local_free_push_rejects_missing_block_next_proof(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 3
        plan = _verified_local_free_push_plan()
        plan["block_next_proof_valid"] = False
        resolver.fastmem_access_plans_by_site[(0, 3)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="local_free_reject_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "block-next-proof-missing"):
            lower_memop(
                builder,
                {"kind": "local_free_push", "operands": [12, 15]},
                {15: ir.Constant(i64, 12288)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )
    def test_local_free_pop_lowers_verified_plan_only(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 4
        resolver.fastmem_access_plans_by_site[(0, 4)] = [_verified_local_free_pop_plan()]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="local_free_pop_page_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {}

        lower_memop(
            builder,
            {"kind": "local_free_pop", "dst": 16, "operands": [12]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        text = str(module)
        self.assertIn("fastmem_local_free_pop_old_head", text)
        self.assertIn("fastmem_local_free_pop_block_next_ptr", text)
        self.assertIn("fastmem_local_free_pop_next_head", text)
        self.assertIn(16, vmap)
        self.assertNotIn(12, vmap)

    def test_local_free_pop_rejects_missing_non_empty_proof(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 4
        plan = _verified_local_free_pop_plan()
        plan["non_empty_proof_valid"] = False
        resolver.fastmem_access_plans_by_site[(0, 4)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="local_free_pop_reject_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "non-empty-proof-missing"):
            lower_memop(
                builder,
                {"kind": "local_free_pop", "dst": 16, "operands": [12]},
                {},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_free_head_push_lowers_verified_plan_only(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 5
        resolver.fastmem_access_plans_by_site[(0, 5)] = [_verified_free_head_push_plan()]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="free_head_push_page_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {15: ir.Constant(i64, 12288)}

        lower_memop(
            builder,
            {"kind": "free_head_push", "operands": [12, 15]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        text = str(module)
        self.assertIn("fastmem_free_head_push_old_head", text)
        self.assertIn("fastmem_free_head_push_block_next_ptr", text)
        self.assertIn("store i64 12288", text)
        self.assertNotIn(12, vmap)

    def test_free_head_push_rejects_missing_block_next_proof(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 5
        plan = _verified_free_head_push_plan()
        plan["block_next_proof_valid"] = False
        resolver.fastmem_access_plans_by_site[(0, 5)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="free_head_push_reject_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "block-next-proof-missing"):
            lower_memop(
                builder,
                {"kind": "free_head_push", "operands": [12, 15]},
                {15: ir.Constant(i64, 12288)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_atomic_remote_head_push_lowers_verified_plan_only(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 6
        resolver.fastmem_access_plans_by_site[(0, 6)] = [
            _verified_atomic_remote_head_push_plan()
        ]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="atomic_remote_page_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {15: ir.Constant(i64, 12288)}

        lower_memop(
            builder,
            {"kind": "atomic_remote_head_push", "operands": [12, 15]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        text = str(module)
        self.assertIn("fastmem_atomic_remote_old_head", text)
        self.assertIn("fastmem_atomic_remote_block_next_ptr", text)
        self.assertIn("fastmem_atomic_remote_retry_1", text)
        self.assertIn("extractvalue", text)
        self.assertIn("cmpxchg", text)
        self.assertGreaterEqual(text.count("cmpxchg"), 3)
        self.assertIn("acq_rel", text)
        self.assertNotIn(12, vmap)

    def test_atomic_remote_head_push_fails_fast_when_retry_budget_exhausts(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 6
        plan = _verified_atomic_remote_head_push_plan()
        plan["retry_attempt_limit"] = 1
        resolver.fastmem_access_plans_by_site[(0, 6)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="atomic_remote_exhaust_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {15: ir.Constant(i64, 12288)}

        lower_memop(
            builder,
            {"kind": "atomic_remote_head_push", "operands": [12, 15]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        text = str(module)
        self.assertIn("fastmem_atomic_remote_retry_exhausted", text)
        self.assertIn("br i1", text)
        self.assertIn("unreachable", text)

    def test_atomic_remote_head_push_rejects_missing_remote_owner_proof(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 6
        plan = _verified_atomic_remote_head_push_plan()
        plan["remote_owner_proof_valid"] = False
        resolver.fastmem_access_plans_by_site[(0, 6)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="atomic_remote_reject_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "remote-owner-proof-missing"):
            lower_memop(
                builder,
                {"kind": "atomic_remote_head_push", "operands": [12, 15]},
                {15: ir.Constant(i64, 12288)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_atomic_remote_head_drain_lowers_to_acquire_exchange(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 7
        resolver.fastmem_access_plans_by_site[(0, 7)] = [
            _verified_atomic_remote_head_drain_plan()
        ]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="atomic_remote_drain_page_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {}

        lower_memop(
            builder,
            {"kind": "atomic_remote_head_drain", "dst": 17, "operands": [12]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        text = str(module)
        self.assertIn("atomicrmw xchg", text)
        self.assertIn("acquire", text)
        self.assertIn("fastmem_atomic_remote_drain_xchg_17", text)
        self.assertIn(17, vmap)
        self.assertNotIn(12, vmap)

    def test_atomic_remote_head_drain_rejects_nonlowerable_plan(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 7
        plan = _verified_atomic_remote_head_drain_plan()
        plan["lowerable"] = False
        resolver.fastmem_access_plans_by_site[(0, 7)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="atomic_remote_drain_reject_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(RuntimeError, "drain-plan-not-lowerable"):
            lower_memop(
                builder,
                {"kind": "atomic_remote_head_drain", "dst": 17, "operands": [12]},
                {},
                resolver,
                builder.block,
                {},
                {},
                {},
            )

    def test_drain_remote_list_to_local_lowers_verified_plan_only(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 8
        resolver.fastmem_access_plans_by_site[(0, 8)] = [
            _verified_drain_remote_list_to_local_plan()
        ]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="drain_remote_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }
        vmap = {15: ir.Constant(i64, 12288)}

        lower_memop(
            builder,
            {"kind": "drain_remote_list_to_local", "operands": [12, 15]},
            vmap,
            resolver,
            builder.block,
            {},
            {},
            {},
        )
        builder.ret(ir.Constant(i64, 0))

        text = str(module)
        self.assertIn("fastmem_drain_remote_old_local_head", text)
        self.assertIn("fastmem_drain_remote_tail_next", text)
        self.assertIn("fastmem_drain_remote_done", text)
        self.assertNotIn(12, vmap)

    def test_drain_remote_list_to_local_rejects_missing_block_next_access_resolution(self):
        i64, _module, builder = _new_builder()
        resolver = _DummyResolver()
        resolver.current_instruction_index = 8
        plan = _verified_drain_remote_list_to_local_plan()
        plan["block_next_access_resolved"] = False
        resolver.fastmem_access_plans_by_site[(0, 8)] = [plan]
        resolver.fastmem_layout_refs[12] = {
            "ptr": builder.inttoptr(
                ir.Constant(i64, 8192),
                ir.IntType(8).as_pointer(),
                name="drain_remote_reject_layout_ref",
            ),
            "layout_id": "PageMetaLayoutV0",
        }

        with self.assertRaisesRegex(
            RuntimeError, "drain-remote-list-block-next-access-unresolved"
        ):
            lower_memop(
                builder,
                {"kind": "drain_remote_list_to_local", "operands": [12, 15]},
                {15: ir.Constant(i64, 12288)},
                resolver,
                builder.block,
                {},
                {},
                {},
            )
