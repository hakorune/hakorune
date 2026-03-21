#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.boxcall import lower_boxcall


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.string_literals = {}
        self.integerish_ids = set()

    def is_arrayish(self, value_id: int) -> bool:
        value = self.value_types.get(int(value_id))
        return isinstance(value, dict) and value.get("box_type") == "ArrayBox"


class TestBoxcallPluginInvokeArgs(unittest.TestCase):
    def test_stage1_build_box_literal_receiver_prefers_direct_call(self):
        module = ir.Module(name="test_boxcall_stage1_build_box_direct")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        recv_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_recv_ptr")
        arg_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_arg_ptr")
        recv_ptr = builder.call(recv_seed, [], name="recv_ptr")
        arg_ptr = builder.call(arg_seed, [], name="arg_ptr")
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64]),
            name="BuildBox.emit_program_json_v0/2",
        )

        vmap = {1: recv_ptr, 2: arg_ptr, 3: ir.Constant(i64, 0)}
        resolver = _ResolverStub()
        resolver.string_literals[1] = "lang.compiler.build.build_box"

        lower_boxcall(
            builder=builder,
            module=module,
            box_vid=1,
            method_name="emit_program_json_v0",
            args=[2, 3],
            dst_vid=4,
            vmap=vmap,
            resolver=resolver,
        )
        builder.ret(vmap[4])

        ir_text = str(module)
        self.assertIn('call i64 @"BuildBox.emit_program_json_v0/2"', ir_text)
        self.assertIn("nyash.box.from_i8_string", ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_unsupported_boxcall_method_fails_fast_before_plugin_invoke(self):
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

        with self.assertRaisesRegex(NotImplementedError, "Unsupported BoxCall method"):
            lower_boxcall(
                builder=builder,
                module=module,
                box_vid=1,
                method_name="emit_program_json_v0",
                args=[2, 3, 4],
                dst_vid=5,
                vmap=vmap,
                resolver=resolver,
            )

    def test_unsupported_boxcall_method_fails_fast_for_pointer_args(self):
        module = ir.Module(name="test_boxcall_plugin_invoke_argc_cap")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        recv_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_recv_ptr")
        arg1_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_arg1_ptr")
        recv_ptr = builder.call(recv_seed, [], name="recv_ptr")
        arg1_ptr = builder.call(arg1_seed, [], name="arg1_ptr")

        vmap = {
            1: recv_ptr,
            2: arg1_ptr,
            3: ir.Constant(i64, 33),
            4: ir.Constant(i64, 44),
        }
        resolver = _ResolverStub()

        with self.assertRaisesRegex(NotImplementedError, "Unsupported BoxCall method"):
            lower_boxcall(
                builder=builder,
                module=module,
                box_vid=1,
                method_name="emit_program_json_v0",
                args=[2],
                dst_vid=5,
                vmap=vmap,
                resolver=resolver,
            )

    def test_arraybox_boxcall_set_prefers_array_route(self):
        module = ir.Module(name="test_boxcall_arraybox_set_route")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        recv_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_recv_ptr")
        recv_ptr = builder.call(recv_seed, [], name="recv_ptr")

        vmap = {
            1: recv_ptr,
            2: ir.Constant(i64, 7),
            3: ir.Constant(i64, 99),
        }
        resolver = _ResolverStub()
        resolver.value_types = {
            1: {"kind": "handle", "box_type": "ArrayBox"},
            2: "i64",
            3: {"kind": "handle", "box_type": "StringBox"},
        }
        resolver.integerish_ids = {2}

        lower_boxcall(
            builder=builder,
            module=module,
            box_vid=1,
            method_name="set",
            args=[2, 3],
            dst_vid=4,
            vmap=vmap,
            resolver=resolver,
        )
        builder.ret(ir.Constant(i64, 0))

        ir_text = str(module)
        self.assertIn('call i64 @"nyash.array.set_hih"', ir_text)
        self.assertNotIn('call i64 @"nyash.map.set_hh"', ir_text)


if __name__ == "__main__":
    unittest.main()
