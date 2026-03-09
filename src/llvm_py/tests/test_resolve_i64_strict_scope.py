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

    def resolve_i64(self, value_id, block, preds, block_end_values, vmap, bb_map):
        self.resolve_calls += 1
        return ir.Constant(self.i64_type, 77)


class TestResolveI64StrictScope(unittest.TestCase):
    def test_non_dominating_global_vmap_value_uses_resolver_when_not_prefer_local(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_scope_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb2")

        resolver = _ResolverStub(i64)
        resolver.global_vmap[11] = ir.Constant(i64, 42)

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

        self.assertEqual(str(got), "i64 77")
        self.assertEqual(resolver.resolve_calls, 1)

    def test_global_phi_placeholder_is_still_reused(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="resolve_i64_phi_mod")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb2")
        builder = ir.IRBuilder(bb)
        phi = builder.phi(i64, name="phi_v11")

        resolver = _ResolverStub(i64)
        resolver.global_vmap[11] = phi

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


if __name__ == "__main__":
    unittest.main()
