#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.llvm_attrs import apply_runtime_llvm_attrs
from llvm_builder import NyashLLVMBuilder


def _line_for(ir_text: str, needle: str) -> str:
    for line in ir_text.splitlines():
        if needle in line:
            return line
    raise AssertionError(f"missing line containing: {needle}\n{ir_text}")


class TestLLVMAttrsPolicy(unittest.TestCase):
    def test_runtime_helper_policy_marks_readonly_and_nocapture(self):
        module = ir.Module(name="llvm_attrs_policy")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()

        ir.Function(module, ir.FunctionType(i64, [i64]), name="nyash.string.len_h")
        ir.Function(module, ir.FunctionType(i64, [i8p]), name="nyash.console.log")
        ir.Function(
            module,
            ir.FunctionType(i8p, [i8p, i64, i64]),
            name="nyash.string.substring_sii",
        )

        apply_runtime_llvm_attrs(module)

        ir_text = str(module)
        self.assertIn("readonly", _line_for(ir_text, 'nyash.string.len_h'), msg=ir_text)
        self.assertIn("nocapture", _line_for(ir_text, 'nyash.console.log'), msg=ir_text)
        self.assertIn("nocapture", _line_for(ir_text, 'nyash.string.substring_sii'), msg=ir_text)

    def test_builder_apply_runtime_attrs_uses_the_same_policy(self):
        builder = NyashLLVMBuilder()
        module = builder.module
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()

        ir.Function(module, ir.FunctionType(i64, [i64]), name="nyash.integer.get_h")
        ir.Function(
            module,
            ir.FunctionType(i64, [i8p, i8p]),
            name="nyash.string.indexOf_ss",
        )

        builder.apply_runtime_llvm_attrs()

        ir_text = str(module)
        self.assertIn("readonly", _line_for(ir_text, 'nyash.integer.get_h'), msg=ir_text)
        self.assertIn("nocapture", _line_for(ir_text, 'nyash.string.indexOf_ss'), msg=ir_text)


if __name__ == "__main__":
    unittest.main()
