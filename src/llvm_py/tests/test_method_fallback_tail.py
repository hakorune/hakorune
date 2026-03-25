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

    def test_prefers_direct_known_box_method_without_implicit_receiver_when_present(self):
        i64, module, builder = _new_builder()
        ir.Function(module, ir.FunctionType(i64, []), name="ProbeBox.run/0")

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name="ProbeBox",
            method_name="run",
            recv_h=ir.Constant(i64, 1),
            args=[],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_run",
            plugin_call_name="unified_plugin_invoke",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"ProbeBox.run/0"()', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_prefers_direct_stage1_module_alias_when_receiver_literal_matches(self):
        i64, module, builder = _new_builder()
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64]),
            name="MirBuilderBox.emit_from_program_json_v0/2",
        )

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method_name="emit_from_program_json_v0",
            recv_h=ir.Constant(i64, 0),
            args=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_emit_program_json",
            plugin_call_name="unified_plugin_invoke",
            receiver_literal="lang.mir.builder.MirBuilderBox",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"MirBuilderBox.emit_from_program_json_v0/2"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_prefers_direct_stage1_using_resolver_alias_when_receiver_literal_matches(self):
        i64, module, builder = _new_builder()
        ir.Function(
            module,
            ir.FunctionType(i64, [i64]),
            name="Stage1UsingResolverBox.resolve_for_source/1",
        )

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method_name="resolve_for_source",
            recv_h=ir.Constant(i64, 0),
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_resolve_for_source",
            plugin_call_name="unified_plugin_invoke",
            receiver_literal="lang.compiler.entry.using_resolver_box",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"Stage1UsingResolverBox.resolve_for_source/1"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_prefers_direct_llvm_backend_alias_when_receiver_literal_matches(self):
        i64, module, builder = _new_builder()
        ir.Function(
            module,
            ir.FunctionType(i64, [i64]),
            name="LlvmBackendBox.compile_obj/1",
        )

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method_name="compile_obj",
            recv_h=ir.Constant(i64, 0),
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_compile_obj",
            plugin_call_name="unified_plugin_invoke",
            receiver_literal="selfhost.shared.backend.llvm_backend",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"LlvmBackendBox.compile_obj/1"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_prefers_direct_llvm_backend_link_alias_when_receiver_literal_matches(self):
        i64, module, builder = _new_builder()
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64, i64]),
            name="LlvmBackendBox.link_exe/3",
        )

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method_name="link_exe",
            recv_h=ir.Constant(i64, 0),
            args=[2, 3, 4],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_link_exe",
            plugin_call_name="unified_plugin_invoke",
            receiver_literal="selfhost.shared.backend.llvm_backend",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"LlvmBackendBox.link_exe/3"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_prefers_direct_func_scanner_alias_when_receiver_literal_matches(self):
        i64, module, builder = _new_builder()
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64]),
            name="FuncScannerBox.find_matching_brace/2",
        )

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method_name="find_matching_brace",
            recv_h=ir.Constant(i64, 0),
            args=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_find_matching_brace",
            plugin_call_name="unified_plugin_invoke",
            receiver_literal="lang.compiler.entry.func_scanner",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"FuncScannerBox.find_matching_brace/2"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_prefers_direct_string_helpers_alias_when_receiver_literal_matches(self):
        i64, module, builder = _new_builder()
        ir.Function(
            module,
            ir.FunctionType(i64, [i64]),
            name="StringHelpers.int_to_str/1",
        )

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name=None,
            method_name="int_to_str",
            recv_h=ir.Constant(i64, 0),
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_int_to_str",
            plugin_call_name="unified_plugin_invoke",
            receiver_literal="selfhost.shared.common.string_helpers",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn('call i64 @"StringHelpers.int_to_str/1"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_filebox_known_methods_fall_back_to_plugin_invoke(self):
        i64, module, builder = _new_builder()

        result = lower_direct_or_plugin_method_call(
            builder=builder,
            module=module,
            box_name="FileBox",
            method_name="open",
            recv_h=ir.Constant(i64, 7),
            args=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            direct_call_name="known_box_file_open",
            plugin_call_name="unified_plugin_invoke",
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.plugin.invoke_by_name_i64", ir_text)
        self.assertNotIn('call i64 @"FileBox.open/2"', ir_text)

    def test_filebox_unknown_method_fails_fast_without_by_name_tail(self):
        i64, module, builder = _new_builder()

        with self.assertRaisesRegex(NotImplementedError, "Unsupported MIR method call"):
            lower_direct_or_plugin_method_call(
                builder=builder,
                module=module,
                box_name="FileBox",
                method_name="stat",
                recv_h=ir.Constant(i64, 7),
                args=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                ensure_handle=lambda value: value,
                direct_call_name="known_box_file_stat",
                plugin_call_name="unified_plugin_invoke",
            )

        self.assertNotIn("nyash.plugin.invoke_by_name_i64", str(module))

    def test_filebox_write_fails_fast_without_by_name_tail(self):
        i64, module, builder = _new_builder()

        with self.assertRaisesRegex(NotImplementedError, "Unsupported MIR method call"):
            lower_direct_or_plugin_method_call(
                builder=builder,
                module=module,
                box_name="FileBox",
                method_name="write",
                recv_h=ir.Constant(i64, 7),
                args=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                ensure_handle=lambda value: value,
                direct_call_name="known_box_file_write",
                plugin_call_name="unified_plugin_invoke",
            )

        self.assertNotIn("nyash.plugin.invoke_by_name_i64", str(module))

    def test_unsupported_direct_target_fails_fast_when_plugin_tail_is_retired(self):
        i64, module, builder = _new_builder()

        with self.assertRaisesRegex(NotImplementedError, "Unsupported MIR method call"):
            lower_direct_or_plugin_method_call(
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

    def test_unsupported_direct_target_fails_fast_for_pointer_args(self):
        i64, module, builder = _new_builder()
        i8p = ir.IntType(8).as_pointer()
        str_fn = ir.Function(module, ir.FunctionType(i8p, []), name="seed_str_ptr")
        arg_ptr = builder.call(str_fn, [], name="str_ptr")

        with self.assertRaisesRegex(NotImplementedError, "Unsupported MIR method call"):
            lower_direct_or_plugin_method_call(
                builder=builder,
                module=module,
                box_name="Stage1Box",
                method_name="emit",
                recv_h=ir.Constant(i64, 1),
                args=[2],
                resolve_arg=lambda vid: arg_ptr if vid == 2 else ir.Constant(i64, vid),
                ensure_handle=lambda value: (
                    builder.call(
                        ir.Function(
                            module,
                            ir.FunctionType(i64, [i8p]),
                            name="nyash.box.from_i8_string",
                        ),
                        [value],
                        name="boxed_ptr_arg",
                    )
                    if hasattr(value, "type") and value.type == i8p
                    else value
                ),
                direct_call_name="known_box_emit",
                plugin_call_name="unified_plugin_invoke",
            )

    def test_alias_box_missing_direct_target_fails_fast_without_by_name_tail(self):
        i64, module, builder = _new_builder()

        with self.assertRaisesRegex(NotImplementedError, "Unsupported MIR method call"):
            lower_direct_or_plugin_method_call(
                builder=builder,
                module=module,
                box_name="LlvmBackendBox",
                method_name="compile_obj",
                recv_h=ir.Constant(i64, 1),
                args=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                ensure_handle=lambda value: value,
                direct_call_name="known_box_compile_obj",
                plugin_call_name="unified_plugin_invoke",
                receiver_literal="selfhost.shared.backend.llvm_backend",
            )


if __name__ == "__main__":
    unittest.main()
