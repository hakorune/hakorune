#!/usr/bin/env python3
import os
import unittest
from unittest.mock import patch

import llvmlite.ir as ir

from src.llvm_py.instructions.stringbox import emit_stringbox_call


class _DummyResolver:
    def __init__(self, string_ptrs):
        self.string_ptrs = string_ptrs
        self.marked = []

    def mark_string(self, vid):
        self.marked.append(vid)


def _new_builder():
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    module = ir.Module(name="test_stringbox_indexof_pointer")
    fn = ir.Function(module, ir.FunctionType(i64, [i8p, i8p]), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder, fn.args[0], fn.args[1]


class TestStringBoxIndexOfPointer(unittest.TestCase):
    def test_indexof_fast_path_uses_pointer_symbol(self):
        i64, module, builder, recv_ptr, needle_ptr = _new_builder()
        resolver = _DummyResolver({7: recv_ptr, 11: needle_ptr})
        vmap = {}

        with patch.dict(os.environ, {"NYASH_LLVM_FAST": "1"}, clear=False):
            handled = emit_stringbox_call(
                builder=builder,
                module=module,
                method_name="indexOf",
                recv_val=recv_ptr,
                args=[11],
                dst_vid=22,
                vmap=vmap,
                box_vid=7,
                resolver=resolver,
                preds=None,
                block_end_values=None,
                bb_map=None,
            )

        builder.ret(ir.Constant(i64, 0))
        self.assertTrue(handled)
        self.assertIn(22, vmap)
        self.assertIn("nyash.string.indexOf_ss", str(module))
        self.assertEqual(resolver.marked, [])


if __name__ == "__main__":
    unittest.main()
