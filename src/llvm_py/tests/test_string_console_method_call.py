#!/usr/bin/env python3
import os
import unittest
from unittest.mock import patch

import llvmlite.ir as ir

from src.llvm_py.instructions.mir_call.string_console_method_call import (
    lower_string_search_or_slice_method_call,
    lower_string_or_console_method_call,
)


def _new_builder():
    i64 = ir.IntType(64)
    module = ir.Module(name="test_string_console_method_call")
    fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder


def _new_ptr_builder():
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    module = ir.Module(name="test_string_console_method_call_ptr")
    fn = ir.Function(module, ir.FunctionType(i64, [i8p, i8p]), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder, fn.args[0], fn.args[1]


def _declare(module, name, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


class TestStringConsoleMethodCall(unittest.TestCase):
    def test_substring_search_slice_route_uses_string_kernel_and_marks_receiver(self):
        i64, module, builder = _new_builder()
        marked = {"count": 0}

        result = lower_string_search_or_slice_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            method_name="substring",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            mark_receiver_stringish=lambda: marked.__setitem__("count", marked["count"] + 1),
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertEqual(marked["count"], 1)
        self.assertIn("nyash.string.substring_hii", ir_text)
        self.assertIn("unified_substring", ir_text)

    def test_indexof_search_slice_route_uses_string_kernel(self):
        i64, module, builder = _new_builder()
        marked = {"count": 0}

        result = lower_string_search_or_slice_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            method_name="indexOf",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            mark_receiver_stringish=lambda: marked.__setitem__("count", marked["count"] + 1),
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertEqual(marked["count"], 1)
        self.assertIn("nyash.string.indexOf_hh", ir_text)
        self.assertIn("unified_indexOf", ir_text)

    def test_indexof_search_slice_route_uses_pointer_kernel_when_available(self):
        i64, module, builder, recv_ptr, needle_ptr = _new_ptr_builder()
        marked = {"count": 0}

        with patch.dict(os.environ, {"NYASH_LLVM_FAST": "1"}, clear=False):
            result = lower_string_search_or_slice_method_call(
                builder=builder,
                declare=lambda name, ret, args: _declare(module, name, ret, args),
                method_name="indexOf",
                recv_h=ir.Constant(i64, 1),
                recv_ptr=recv_ptr,
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                ensure_handle=lambda value: value,
                needle_ptr_for_value=lambda vid: needle_ptr if vid == 2 else None,
                mark_receiver_stringish=lambda: marked.__setitem__(
                    "count", marked["count"] + 1
                ),
            )
        builder.ret(result)

        ir_text = str(module)
        self.assertEqual(marked["count"], 1)
        self.assertIn("nyash.string.indexOf_ss", ir_text)
        self.assertIn("unified_indexOf_ss", ir_text)

    def test_lastindexof_search_slice_route_uses_string_kernel(self):
        i64, module, builder = _new_builder()

        result = lower_string_search_or_slice_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            method_name="lastIndexOf",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.string.lastIndexOf_hh", ir_text)
        self.assertIn("unified_lastIndexOf", ir_text)

    def test_compat_wrapper_still_delegates_substring(self):
        i64, module, builder = _new_builder()

        result = lower_string_or_console_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            method_name="substring",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.string.substring_hii", ir_text)
        self.assertIn("unified_substring", ir_text)

    def test_console_log_route_bridges_i64_handles_to_string_ptrs(self):
        i64, module, builder = _new_builder()

        result = lower_string_or_console_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            method_name="log",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.string.to_i8p_h", ir_text)
        self.assertIn("nyash.console.log", ir_text)

    def test_unknown_method_returns_none_without_side_effects(self):
        i64, module, builder = _new_builder()

        result = lower_string_or_console_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            method_name="push",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
        )

        self.assertIsNone(result)
        self.assertNotIn("nyash.console.log", str(module))


if __name__ == "__main__":
    unittest.main()
