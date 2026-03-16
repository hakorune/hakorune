#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.mir_call.method_call import lower_method_call


class _ResolverStub:
    def __init__(self):
        self.string_literals = {}
        self.string_ptrs = {}
        self.value_types = {}
        self.marked_strings = set()

    def mark_string(self, vid):
        self.marked_strings.add(vid)


class _OwnerStub:
    preds = None
    block_end_values = None
    bb_map = None


class TestMethodCallStage1ModuleAlias(unittest.TestCase):
    def test_mir_builder_literal_receiver_prefers_direct_call(self):
        module = ir.Module(name="test_method_call_stage1_module_alias")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        arg_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_program_json_ptr")
        arg_ptr = builder.call(arg_seed, [], name="program_json_ptr")
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64]),
            name="MirBuilderBox.emit_from_program_json_v0/2",
        )

        resolver = _ResolverStub()
        resolver.string_literals[1] = "lang.mir.builder.MirBuilderBox"
        vmap = {
            1: ir.Constant(i64, 0),
            2: arg_ptr,
            3: ir.Constant(i64, 0),
        }

        lower_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method="emit_from_program_json_v0",
            receiver=1,
            args=[2, 3],
            dst_vid=4,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[4])

        ir_text = str(module)
        self.assertIn('call i64 @"MirBuilderBox.emit_from_program_json_v0/2"', ir_text)
        self.assertIn("nyash.box.from_i8_string", ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_mir_builder_source_literal_receiver_prefers_direct_call(self):
        module = ir.Module(name="test_method_call_stage1_source_module_alias")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        arg_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_source_ptr")
        arg_ptr = builder.call(arg_seed, [], name="source_ptr")
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64]),
            name="MirBuilderBox.emit_from_source_v0/2",
        )

        resolver = _ResolverStub()
        resolver.string_literals[1] = "lang.mir.builder.MirBuilderBox"
        vmap = {
            1: ir.Constant(i64, 0),
            2: arg_ptr,
            3: ir.Constant(i64, 0),
        }

        lower_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method="emit_from_source_v0",
            receiver=1,
            args=[2, 3],
            dst_vid=4,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[4])

        ir_text = str(module)
        self.assertIn('call i64 @"MirBuilderBox.emit_from_source_v0/2"', ir_text)
        self.assertIn("nyash.box.from_i8_string", ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_using_resolver_literal_receiver_prefers_direct_call(self):
        module = ir.Module(name="test_method_call_stage1_using_resolver_alias")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        arg_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_source_ptr")
        arg_ptr = builder.call(arg_seed, [], name="source_ptr")
        ir.Function(
            module,
            ir.FunctionType(i64, [i64]),
            name="Stage1UsingResolverBox.resolve_for_source/1",
        )

        resolver = _ResolverStub()
        resolver.string_literals[1] = "lang.compiler.entry.using_resolver_box"
        vmap = {
            1: ir.Constant(i64, 0),
            2: arg_ptr,
        }

        lower_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method="resolve_for_source",
            receiver=1,
            args=[2],
            dst_vid=3,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[3])

        ir_text = str(module)
        self.assertIn('call i64 @"Stage1UsingResolverBox.resolve_for_source/1"', ir_text)
        self.assertIn("nyash.box.from_i8_string", ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_llvm_backend_literal_receiver_prefers_direct_call(self):
        module = ir.Module(name="test_method_call_llvm_backend_alias")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        arg_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_mir_path_ptr")
        arg_ptr = builder.call(arg_seed, [], name="mir_path_ptr")
        ir.Function(
            module,
            ir.FunctionType(i64, [i64]),
            name="LlvmBackendBox.compile_obj/1",
        )

        resolver = _ResolverStub()
        resolver.string_literals[1] = "selfhost.shared.backend.llvm_backend"
        vmap = {
            1: ir.Constant(i64, 0),
            2: arg_ptr,
        }

        lower_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method="compile_obj",
            receiver=1,
            args=[2],
            dst_vid=3,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[3])

        ir_text = str(module)
        self.assertIn('call i64 @"LlvmBackendBox.compile_obj/1"', ir_text)
        self.assertIn("nyash.box.from_i8_string", ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_llvm_backend_link_literal_receiver_prefers_direct_call(self):
        module = ir.Module(name="test_method_call_llvm_backend_link_alias")
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        obj_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_obj_path_ptr")
        out_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_out_path_ptr")
        libs_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_libs_ptr")
        obj_ptr = builder.call(obj_seed, [], name="obj_path_ptr")
        out_ptr = builder.call(out_seed, [], name="out_path_ptr")
        libs_ptr = builder.call(libs_seed, [], name="libs_ptr")
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64, i64]),
            name="LlvmBackendBox.link_exe/3",
        )

        resolver = _ResolverStub()
        resolver.string_literals[1] = "selfhost.shared.backend.llvm_backend"
        vmap = {
            1: ir.Constant(i64, 0),
            2: obj_ptr,
            3: out_ptr,
            4: libs_ptr,
        }

        lower_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method="link_exe",
            receiver=1,
            args=[2, 3, 4],
            dst_vid=5,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[5])

        ir_text = str(module)
        self.assertIn('call i64 @"LlvmBackendBox.link_exe/3"', ir_text)
        self.assertIn("nyash.box.from_i8_string", ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)


if __name__ == "__main__":
    unittest.main()
