#!/usr/bin/env python3
import os
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.mir_call.constructor_call import lower_constructor_call
from src.llvm_py.instructions.mir_call.value_call import lower_value_call
from src.llvm_py.instructions.mir_call.extern_call import lower_extern_call
from src.llvm_py.instructions.mir_call.closure_call import lower_closure_creation
from src.llvm_py.instructions.mir_call.print_marshal import PrintArgMarshallerBox
from src.llvm_py.instructions.mir_call_legacy import lower_constructor_call as lower_constructor_call_legacy
from src.llvm_py.instructions.mir_call_legacy import lower_method_call as lower_method_call_legacy


class _ContextStub:
    def __init__(self):
        self.hot_trace_counts = {}


class _ResolverStub:
    def __init__(self, i64_type: ir.IntType):
        self.context = _ContextStub()
        self._i64_type = i64_type
        self.string_literals = {}
        self.string_ptrs = {}

    def resolve_i64(self, value_id, block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(self._i64_type, int(value_id) + 1)


class _OwnerStub:
    def __init__(self, bb):
        self.preds = {0: []}
        self.block_end_values = {}
        self.bb_map = {0: bb}


def _new_builder():
    i64 = ir.IntType(64)
    module = ir.Module(name="test_mir_call_hot_fallback")
    fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder, bb


class TestMirCallHotFallback(unittest.TestCase):
    def setUp(self):
        self._old_safepoint = os.environ.get("NYASH_LLVM_AUTO_SAFEPOINT")
        os.environ["NYASH_LLVM_AUTO_SAFEPOINT"] = "0"

    def tearDown(self):
        if self._old_safepoint is None:
            os.environ.pop("NYASH_LLVM_AUTO_SAFEPOINT", None)
        else:
            os.environ["NYASH_LLVM_AUTO_SAFEPOINT"] = self._old_safepoint

    def _count(self, resolver, key: str) -> int:
        return int(resolver.context.hot_trace_counts.get(key, 0))

    def test_constructor_call_increments_call_fallback_counter(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_constructor_call(builder, module, "IntegerBox", [11], 21, vmap, resolver, owner)

        self.assertGreaterEqual(self._count(resolver, "resolve_fallback_call"), 1)
        self.assertIn(21, vmap)

    def test_value_call_increments_call_fallback_counter(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_value_call(builder, module, 30, [31, 32], 40, vmap, resolver, owner)

        self.assertGreaterEqual(self._count(resolver, "resolve_fallback_call"), 3)
        self.assertIn(40, vmap)

    def test_extern_call_increments_call_fallback_counter(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_extern_call(builder, module, "print", [50], 60, vmap, resolver, owner)

        self.assertGreaterEqual(self._count(resolver, "resolve_fallback_call"), 1)
        self.assertIn(60, vmap)

    def test_legacy_constructor_call_increments_call_fallback_counter(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_constructor_call_legacy(builder, module, "IntegerBox", [71], 81, vmap, resolver, owner)

        self.assertGreaterEqual(self._count(resolver, "resolve_fallback_call"), 1)
        self.assertIn(81, vmap)

    def test_legacy_method_call_prefers_direct_build_box_alias_when_available(self):
        i64 = ir.IntType(64)
        i8p = ir.IntType(8).as_pointer()
        module = ir.Module(name="test_mir_call_hot_fallback_legacy_build_box")
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)
        owner = _OwnerStub(bb)

        arg_seed = ir.Function(module, ir.FunctionType(i8p, []), name="seed_src_ptr")
        arg_ptr = builder.call(arg_seed, [], name="src_ptr")
        ir.Function(
            module,
            ir.FunctionType(i64, [i64, i64]),
            name="BuildBox.emit_program_json_v0/2",
        )

        resolver = _ResolverStub(i64)
        resolver.string_literals[1] = "lang.compiler.build.build_box"
        vmap = {
            1: ir.Constant(i64, 0),
            2: arg_ptr,
            3: ir.Constant(i64, 0),
        }

        lower_method_call_legacy(
            builder=builder,
            module=module,
            box_name=None,
            method="emit_program_json_v0",
            receiver=1,
            args=[2, 3],
            dst_vid=4,
            vmap=vmap,
            resolver=resolver,
            owner=owner,
        )
        builder.ret(vmap[4])

        ir_text = str(module)
        self.assertIn('call i64 @"BuildBox.emit_program_json_v0/2"', ir_text)
        self.assertNotIn("nyash.plugin.invoke_by_name_i64", ir_text)

    def test_closure_call_increments_call_fallback_counter(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_closure_creation(
            builder,
            module,
            params=[{"id": 1}],
            captures=[{"id": 91}, 92],
            me_capture=93,
            dst_vid=94,
            vmap=vmap,
            resolver=resolver,
            owner=owner,
        )

        self.assertGreaterEqual(self._count(resolver, "resolve_fallback_call"), 3)
        self.assertIn(94, vmap)

    def test_print_marshal_increments_call_fallback_counter(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)

        marshaled = PrintArgMarshallerBox.marshal(
            arg_id=111,
            type_info={"stringish": False},
            builder=builder,
            resolver=resolver,
            module=module,
            vmap={},
            preds={0: []},
            block_end_values={},
            bb_map={0: bb},
        )

        self.assertGreaterEqual(self._count(resolver, "resolve_fallback_call"), 1)
        self.assertIsNotNone(marshaled)


if __name__ == "__main__":
    unittest.main()
