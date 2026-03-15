#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.mir_call.method_fallback_tail import (
    lower_direct_or_plugin_method_call,
)


def _new_builder():
    i64 = ir.IntType(64)
    module = ir.Module(name="test_method_fallback_tail")
    fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder


class TestMethodFallbackTail(unittest.TestCase):
    def test_prefers_direct_known_box_method_when_present(self):
        i64, module, builder = _new_builder()
        ir.Function(module, ir.FunctionType(i64, [i64, i64]), name="StringBox.length/2")

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name="StringBox",
            method_name="length",
            recv_h=ir.Constant(i64, 1),
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_length",
            plugin_call_name="unified_plugin_invoke",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"StringBox.length/2"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_falls_back_to_plugin_invoke_when_direct_target_missing(self):
        i64, module, builder = _new_builder()

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name="StringBox",
            method_name="length",
            recv_h=ir.Constant(i64, 1),
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_length",
            plugin_call_name="unified_plugin_invoke",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.plugin.invoke_by_name_i64", ir_text)
        self.assertNotIn('call i64 @"StringBox.length/2"', ir_text)


if __name__ == "__main__":
    unittest.main()
