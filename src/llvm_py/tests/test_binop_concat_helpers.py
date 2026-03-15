#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.binop import _dispatch_string_concat, _materialize_string_concat_handles


class _ResolverStub:
    def __init__(self):
        self._stringish = set()
        self.string_literals = {}
        self.value_types = {}

    def is_stringish(self, value_id: int) -> bool:
        return int(value_id) in self._stringish


class TestBinopConcatHelpers(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="binop_concat_helpers")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), i64

    def test_materialize_concat_handles_bridges_untagged_i64_constant(self):
        mod, builder, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver._stringish.add(10)

        hl, hr = _materialize_string_concat_handles(
            builder,
            resolver,
            10,
            11,
            12,
            ir.Constant(i64, 100),
            ir.Constant(i64, 200),
            ir.Constant(i64, 100),
            ir.Constant(i64, 200),
        )

        self.assertIsNotNone(hl)
        self.assertIsNotNone(hr)
        self.assertIn('@"nyash.any.toString_h"', str(mod))

    def test_dispatch_string_concat_prefers_concat3_for_chain(self):
        mod, builder, i64 = self._make_builder()
        callee = ir.Function(mod, ir.FunctionType(i64, [i64, i64]), name="nyash.string.concat_hh")
        lhs_raw = builder.call(callee, [ir.Constant(i64, 1), ir.Constant(i64, 2)], name="seed_concat")

        result = _dispatch_string_concat(
            builder,
            20,
            lhs_raw,
            ir.Constant(i64, 3),
            ir.Constant(i64, 1),
            ir.Constant(i64, 3),
        )

        self.assertIn("concat3_hhh", getattr(result, "name", ""))
        self.assertIn('@"nyash.string.concat3_hhh"', str(mod))


if __name__ == "__main__":
    unittest.main()
