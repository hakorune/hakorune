#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.mir_call.arg_resolver import resolve_call_arg


class _ResolverStub:
    def __init__(self, i64_type: ir.IntType):
        self.i64_type = i64_type
        self.global_vmap = {}
        self.def_blocks = {}
        self.resolve_calls = 0
        self.context = None

    def resolve_i64(self, value_id, block, preds, block_end_values, vmap, bb_map):
        self.resolve_calls += 1
        return ir.Constant(self.i64_type, 88)


class _ContextStub:
    def __init__(self, dominators=None):
        self._dominators = dominators or {}

    def dominates(self, def_bid: int, use_bid: int) -> bool:
        return def_bid in self._dominators.get(use_bid, set())


class TestMirCallArgResolverScope(unittest.TestCase):
    def test_non_dominating_local_vmap_value_falls_back_to_resolver(self):
        i64 = ir.IntType(64)
        module = ir.Module(name="call_arg_scope_mod")
        fn = ir.Function(module, ir.FunctionType(i64, [i64]), name="main")
        bb1 = fn.append_basic_block("bb1")
        bb2 = fn.append_basic_block("bb2")
        builder1 = ir.IRBuilder(bb1)
        inherited = builder1.add(fn.args[0], ir.Constant(i64, 1), name="non_dom_add")
        builder2 = ir.IRBuilder(bb2)

        resolver = _ResolverStub(i64)
        resolver.def_blocks[11] = {1}
        resolver.context = _ContextStub({2: {0, 2}})

        got = resolve_call_arg(
            11,
            builder2,
            {11: inherited},
            resolver,
            preds={2: [0, 1]},
            block_end_values={},
            bb_map={1: bb1, 2: bb2},
            hot_scope="call",
        )

        self.assertEqual(str(got), "i64 88")
        self.assertEqual(resolver.resolve_calls, 1)

    def test_same_block_local_vmap_value_is_reused(self):
        i64 = ir.IntType(64)
        module = ir.Module(name="call_arg_scope_local_mod")
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb)
        local = ir.Constant(i64, 42)

        resolver = _ResolverStub(i64)
        resolver.def_blocks[11] = {2}
        resolver.context = _ContextStub({2: {2}})

        got = resolve_call_arg(
            11,
            builder,
            {11: local},
            resolver,
            preds={2: [0, 1]},
            block_end_values={},
            bb_map={2: bb},
            hot_scope="call",
        )

        self.assertIs(got, local)
        self.assertEqual(resolver.resolve_calls, 0)


if __name__ == "__main__":
    unittest.main()
