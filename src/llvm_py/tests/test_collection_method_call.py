#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.mir_call.collection_method_call import (
    lower_collection_method_call,
)


class _DummyResolver:
    def __init__(self, value_types=None, integerish_ids=None):
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])


def _new_builder():
    i64 = ir.IntType(64)
    module = ir.Module(name="test_collection_method_call")
    fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder


def _declare(module, name, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


class TestCollectionMethodCall(unittest.TestCase):
    def test_non_runtime_data_get_falls_back_to_map_raw_kernel(self):
        i64, module, builder = _new_builder()

        result = lower_collection_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="MapBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
        )
        builder.ret(result)

        self.assertIn("nyash.map.slot_load_hh", str(module))

    def test_runtime_data_push_uses_runtime_data_dispatch(self):
        i64, module, builder = _new_builder()

        result = lower_collection_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="RuntimeDataBox",
            method_name="push",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            prefer_array_mono_route=False,
        )
        builder.ret(result)

        self.assertIn("nyash.runtime_data.push_hh", str(module))

    def test_arraybox_get_with_i64_key_uses_array_slot_load_hi(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})

        result = lower_collection_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="ArrayBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.array.slot_load_hi", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)
        self.assertNotIn("nyash.runtime_data.get_hh", ir_text)

    def test_arraybox_get_with_non_i64_key_keeps_runtime_data_facade(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: {"kind": "handle", "box_type": "StringBox"}})

        result = lower_collection_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="ArrayBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.get_hh", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)

    def test_arraybox_set_with_i64_key_and_value_uses_array_set_hii(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: "i64", 3: "i64"}, integerish_ids={2, 3})

        result = lower_collection_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="ArrayBox",
            method_name="set",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.array.set_hii", ir_text)
        self.assertNotIn("nyash.map.slot_store_hhh", ir_text)
        self.assertNotIn("nyash.runtime_data.set_hhh", ir_text)

    def test_arraybox_set_with_non_i64_key_keeps_runtime_data_facade(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(
            value_types={
                2: {"kind": "handle", "box_type": "StringBox"},
                3: {"kind": "handle", "box_type": "StringBox"},
            }
        )

        result = lower_collection_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="ArrayBox",
            method_name="set",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.set_hhh", ir_text)
        self.assertNotIn("nyash.map.slot_store_hhh", ir_text)

    def test_arraybox_has_keeps_runtime_data_facade(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})

        result = lower_collection_method_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="ArrayBox",
            method_name="has",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.has_hh", ir_text)
        self.assertNotIn("nyash.map.probe_hh", ir_text)


if __name__ == "__main__":
    unittest.main()
