#!/usr/bin/env python3
import os
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.mir_call.method_call import lower_method_call


class _ResolverStub:
    def __init__(self, *, value_types=None):
        self.string_literals = {}
        self.string_ptrs = {}
        self.value_types = value_types or {1: {"kind": "handle", "box_type": "ArrayBox"}}
        self.marked_strings = set()

    def mark_string(self, vid):
        self.marked_strings.add(vid)


class _OwnerStub:
    preds = None
    block_end_values = None
    bb_map = None


class TestMethodCallCollectionBirth(unittest.TestCase):
    def setUp(self):
        self._old_safepoint = os.environ.get("NYASH_LLVM_AUTO_SAFEPOINT")
        os.environ["NYASH_LLVM_AUTO_SAFEPOINT"] = "0"

    def tearDown(self):
        if self._old_safepoint is None:
            os.environ.pop("NYASH_LLVM_AUTO_SAFEPOINT", None)
        else:
            os.environ["NYASH_LLVM_AUTO_SAFEPOINT"] = self._old_safepoint

    def test_arraybox_birth_is_lowered_as_noop_initializer(self):
        module = ir.Module(name="test_method_call_collection_birth")
        i64 = ir.IntType(64)
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 77)}

        lower_method_call(
            builder=builder,
            module=module,
            box_name="ArrayBox",
            method="birth",
            receiver=1,
            args=[],
            dst_vid=2,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[2])

        ir_text = str(module)
        self.assertIn("ret i64 0", ir_text, msg=ir_text)
        self.assertNotIn('@"nyash.plugin.invoke_by_name_i64"', ir_text, msg=ir_text)

    def test_arraybox_size_uses_slot_len_h_without_resolver_array_facts(self):
        module = ir.Module(name="test_method_call_arraybox_size")
        i64 = ir.IntType(64)
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        resolver = _ResolverStub(value_types={})
        vmap = {1: ir.Constant(i64, 77)}

        lower_method_call(
            builder=builder,
            module=module,
            box_name="ArrayBox",
            method="size",
            receiver=1,
            args=[],
            dst_vid=2,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[2])

        ir_text = str(module)
        self.assertIn('@"nyash.array.slot_len_h"', ir_text, msg=ir_text)
        self.assertNotIn('@"nyash.any.length_h"', ir_text, msg=ir_text)


if __name__ == "__main__":
    unittest.main()
