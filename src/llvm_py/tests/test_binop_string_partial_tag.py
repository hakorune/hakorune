#!/usr/bin/env python3
import unittest
import sys
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from resolver import Resolver
from instructions.binop import lower_binop


class TestBinopStringPartialTag(unittest.TestCase):
    def test_partial_string_tag_still_materializes_concat(self):
        i64 = ir.IntType(64)
        mod = ir.Module(name="binop_partial_tag")
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        builder = ir.IRBuilder(bb)

        resolver = Resolver({}, {1: bb})
        resolver.builder = builder
        resolver.module = mod
        resolver.mark_string(10)  # lhs is known string-ish

        vmap = {
            10: ir.Constant(i64, 100),  # handle-like
            11: ir.Constant(i64, 200),  # untagged i64 (must bridge via any.toString_h)
        }

        lower_binop(
            builder,
            resolver,
            "+",
            10,
            11,
            12,
            vmap,
            bb,
            preds={},
            block_end_values={},
            bb_map={1: bb},
            dst_type={"kind": "handle", "box_type": "StringBox"},
        )
        builder.ret(ir.Constant(i64, 0))

        ir_txt = str(mod)
        self.assertIn('@"nyash.any.toString_h"', ir_txt, msg=ir_txt)
        self.assertIn('@"nyash.string.concat_hh"', ir_txt, msg=ir_txt)
        self.assertIn(12, vmap)


if __name__ == "__main__":
    unittest.main()
