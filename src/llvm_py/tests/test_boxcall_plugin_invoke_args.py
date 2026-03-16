#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.boxcall import lower_boxcall


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.string_literals = {}


class TestBoxcallPluginInvokeArgs(unittest.TestCase):
    def test_plugin_invoke_boxes_pointer_args_before_dispatch(self):
        module = ir.Module(name="test_boxcall_plugin_invoke_args")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        recv_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_recv_ptr")
        arg_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_arg_ptr")
        recv_ptr = builder.call(recv_seed, [], name="recv_ptr")
        arg_ptr = builder.call(arg_seed, [], name="arg_ptr")

        vmap = {1: recv_ptr, 2: arg_ptr}
        resolver = _ResolverStub()

        lower_boxcall(
            builder=builder,
            module=module,
            box_vid=1,
            method_name="emit_program_json_v0",
            args=[2],
            dst_vid=3,
            vmap=vmap,
            resolver=resolver,
        )
        builder.ret(vmap[3])

        ir_text = str(module)
        self.assertGreaterEqual(ir_text.count("nyash.box.from_i8_string"), 2)
        self.assertIn("pinvoke_by_name", ir_text)
        self.assertIn("nyash.plugin.invoke_by_name_i64", ir_text)


if __name__ == "__main__":
    unittest.main()
