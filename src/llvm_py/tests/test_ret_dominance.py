#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.ret import lower_return


class _ResolverStub:
    def __init__(self, i64_type: ir.IntType):
        self._i64_type = i64_type
        self.def_blocks = {11: {1}}
        self.global_vmap = {}
        self.resolve_calls = 0

    def resolve_i64(self, value_id, block, preds, block_end_values, vmap, bb_map):
        self.resolve_calls += 1
        return ir.Constant(self._i64_type, 99)


class TestReturnDominance(unittest.TestCase):
    def test_non_dominating_vmap_value_uses_resolver_path(self):
        i64 = ir.IntType(64)
        module = ir.Module(name="ret_dominance_mod")
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")

        bb0 = fn.append_basic_block("bb0")
        bb1 = fn.append_basic_block("bb1")
        bb2 = fn.append_basic_block("bb2")

        ir.IRBuilder(bb0).branch(bb2)
        ir.IRBuilder(bb1).branch(bb2)
        b2 = ir.IRBuilder(bb2)

        resolver = _ResolverStub(i64)
        vmap = {11: ir.Constant(i64, 42)}
        preds = {2: [0, 1]}
        block_end_values = {}
        bb_map = {0: bb0, 1: bb1, 2: bb2}

        lower_return(
            b2,
            11,
            vmap,
            i64,
            resolver=resolver,
            preds=preds,
            block_end_values=block_end_values,
            bb_map=bb_map,
        )

        ir_txt = str(module)
        self.assertIn("ret i64 99", ir_txt, msg=ir_txt)
        self.assertNotIn("ret i64 42", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.resolve_calls, 1)

    def test_entry_like_block_can_reuse_vmap_value(self):
        i64 = ir.IntType(64)
        module = ir.Module(name="ret_entry_mod")
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb0 = fn.append_basic_block("bb0")
        builder = ir.IRBuilder(bb0)

        resolver = _ResolverStub(i64)
        vmap = {11: ir.Constant(i64, 42)}

        lower_return(
            builder,
            11,
            vmap,
            i64,
            resolver=resolver,
            preds={0: []},
            block_end_values={},
            bb_map={0: bb0},
        )

        ir_txt = str(module)
        self.assertIn("ret i64 42", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.resolve_calls, 0)


if __name__ == "__main__":
    unittest.main()
