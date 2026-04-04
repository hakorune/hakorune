#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from utils.values import resolve_i64_strict


class _ResolverStub:
    def __init__(self, i64_type: ir.IntType):
        self.i64_type = i64_type
        self.global_vmap = {}
        self.def_blocks = {}
        self.resolve_calls = 0
        self.context = None

    def resolve_i64(self, value_id, block, preds, block_end_values, vmap, bb_map):
        self.resolve_calls += 1
        return ir.Constant(self.i64_type, 77)


class _ContextStub:
    def __init__(self, dominators=None):
        self._dominators = dominators or {}

    def dominates(self, def_bid: int, use_bid: int) -> bool:
        return def_bid in self._dominators.get(use_bid, set())


class TestResolveI64StrictScope(unittest.TestCase):
    def test_non_dominating_global_vmap_ssa_value_uses_resolver_when_not_prefer_local(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_scope_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, [i64]), name="main")
        bb1 = fn.append_basic_block("bb1")
        bb2 = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb1)
        inherited = builder.add(fn.args[0], ir.Constant(i64, 1), name="non_dom_add")

        resolver = _ResolverStub(i64)
        resolver.global_vmap[11] = inherited
        resolver.def_blocks[11] = {1}
        resolver.context = _ContextStub({2: {0, 2}})

        got = resolve_i64_strict(
            resolver,
            11,
            bb2,
            preds={2: [0, 1]},
            block_end_values={},
            vmap={},
            bb_map={1: bb1, 2: bb2},
            prefer_local=False,
            hot_scope="unit",
        )

        self.assertEqual(str(got), "i64 77")
        self.assertEqual(resolver.resolve_calls, 1)

    def test_same_block_global_phi_placeholder_is_still_reused(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_phi_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb)
        phi = builder.phi(i64, name="phi_v11")

        resolver = _ResolverStub(i64)
        resolver.global_vmap[11] = phi
        resolver.def_blocks[11] = {2}
        resolver.context = _ContextStub({2: {2}})

        got = resolve_i64_strict(
            resolver,
            11,
            bb,
            preds={2: [0, 1]},
            block_end_values={},
            vmap={},
            bb_map={2: bb},
            prefer_local=False,
            hot_scope="unit",
        )

        self.assertIs(got, phi)
        self.assertEqual(resolver.resolve_calls, 0)

    def test_declared_local_phi_placeholder_without_owner_is_reused(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_declared_phi_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb)
        phi = builder.phi(i64, name="phi_v11")

        resolver = _ResolverStub(i64)
        resolver.block_phi_incomings = {2: {11: [(0, 11), (1, 11)]}}

        got = resolve_i64_strict(
            resolver,
            11,
            bb,
            preds={2: [0, 1]},
            block_end_values={},
            vmap={11: phi},
            bb_map={2: bb},
            prefer_local=True,
            hot_scope="unit",
        )

        self.assertIs(got, phi)
        self.assertEqual(resolver.resolve_calls, 0)

    def test_dominating_global_phi_placeholder_without_owner_is_reused(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_dom_phi_no_owner_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb1 = fn.append_basic_block("bb1")
        bb2 = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb1)
        phi = builder.phi(i64, name="phi_v11")

        resolver = _ResolverStub(i64)
        resolver.global_vmap[11] = phi
        resolver.def_blocks[11] = {1}
        resolver.context = _ContextStub({2: {1}})

        got = resolve_i64_strict(
            resolver,
            11,
            bb2,
            preds={2: [1]},
            block_end_values={},
            vmap={},
            bb_map={1: bb1, 2: bb2},
            prefer_local=False,
            hot_scope="unit",
        )

        self.assertIs(got, phi)
        self.assertEqual(resolver.resolve_calls, 0)

    def test_non_dominating_global_phi_placeholder_uses_resolver(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_phi_non_dom_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb1 = fn.append_basic_block("bb1")
        bb2 = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb1)
        phi = builder.phi(i64, name="phi_v11")

        resolver = _ResolverStub(i64)
        resolver.global_vmap[11] = phi
        resolver.def_blocks[11] = {1}
        resolver.context = _ContextStub({2: {0, 2}})

        got = resolve_i64_strict(
            resolver,
            11,
            bb2,
            preds={2: [0, 1]},
            block_end_values={},
            vmap={},
            bb_map={1: bb1, 2: bb2},
            prefer_local=False,
            hot_scope="unit",
        )

        self.assertEqual(str(got), "i64 77")
        self.assertEqual(resolver.resolve_calls, 1)

    def test_prefer_local_does_not_reuse_inherited_non_dominating_value(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_join_scope_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb2")

        resolver = _ResolverStub(i64)
        inherited = ir.Constant(i64, 42)

        got = resolve_i64_strict(
            resolver,
            11,
            bb,
            preds={2: [0, 1]},
            block_end_values={},
            vmap={11: inherited},
            bb_map={2: bb},
            prefer_local=True,
            hot_scope="unit",
        )

        self.assertEqual(str(got), "i64 77")
        self.assertEqual(resolver.resolve_calls, 1)

    def test_prefer_local_reuses_value_defined_in_current_block(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_current_def_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb2")

        resolver = _ResolverStub(i64)
        resolver.def_blocks[11] = {2}
        local = ir.Constant(i64, 42)

        got = resolve_i64_strict(
            resolver,
            11,
            bb,
            preds={2: [0, 1]},
            block_end_values={},
            vmap={11: local},
            bb_map={2: bb},
            prefer_local=True,
            hot_scope="unit",
        )

        self.assertIs(got, local)
        self.assertEqual(resolver.resolve_calls, 0)

    def test_dominating_global_vmap_ssa_value_is_reused(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_dom_scope_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, [i64]), name="main")
        bb1 = fn.append_basic_block("bb1")
        bb2 = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb1)
        dominating = builder.add(fn.args[0], ir.Constant(i64, 2), name="dom_add")

        resolver = _ResolverStub(i64)
        resolver.global_vmap[11] = dominating
        resolver.def_blocks[11] = {1}
        resolver.context = _ContextStub({2: {1}})

        got = resolve_i64_strict(
            resolver,
            11,
            bb2,
            preds={2: [1]},
            block_end_values={},
            vmap={},
            bb_map={1: bb1, 2: bb2},
            prefer_local=False,
            hot_scope="unit",
        )

        self.assertIs(got, dominating)
        self.assertEqual(resolver.resolve_calls, 0)


if __name__ == "__main__":
    unittest.main()
