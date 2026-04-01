#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.boxcall_runtime_data import try_lower_collection_boxcall


class _DummyResolver:
    def __init__(self, value_types=None, integerish_ids=None):
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])


def _new_builder():
    i64 = ir.IntType(64)
    module = ir.Module(name="test_boxcall_collection_policy")
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


class TestBoxcallCollectionPolicy(unittest.TestCase):
    def test_array_get_with_i64_key_uses_slot_load_hi(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(
            value_types={1: {"kind": "handle", "box_type": "ArrayBox"}, 2: "i64"},
            integerish_ids={2},
        )

        result = try_lower_collection_boxcall(
            builder=builder,
            module=module,
            method_name="get",
            recv_val=ir.Constant(i64, 1),
            box_vid=1,
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            declare=_declare,
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.array.slot_load_hi", ir_text)
        self.assertNotIn("nyash.runtime_data.get_hh", ir_text)

    def test_array_set_with_non_i64_key_keeps_runtime_data_fallback(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: {"kind": "handle", "box_type": "StringBox"},
                3: {"kind": "handle", "box_type": "StringBox"},
            }
        )

        result = try_lower_collection_boxcall(
            builder=builder,
            module=module,
            method_name="set",
            recv_val=ir.Constant(i64, 1),
            box_vid=1,
            args=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            declare=_declare,
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.set_hhh", ir_text)
        self.assertNotIn("nyash.array.slot_store_hih", ir_text)
        self.assertNotIn("nyash.array.slot_store_hii", ir_text)

    def test_array_has_keeps_runtime_data_facade(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(
            value_types={1: {"kind": "handle", "box_type": "ArrayBox"}, 2: "i64"},
            integerish_ids={2},
        )

        result = try_lower_collection_boxcall(
            builder=builder,
            module=module,
            method_name="has",
            recv_val=ir.Constant(i64, 1),
            box_vid=1,
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            declare=_declare,
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.has_hh", ir_text)
        self.assertNotIn("nyash.array.slot_load_hi", ir_text)

    def test_map_clear_uses_clear_h(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={1: {"kind": "handle", "box_type": "MapBox"}})

        result = try_lower_collection_boxcall(
            builder=builder,
            module=module,
            method_name="clear",
            recv_val=ir.Constant(i64, 1),
            box_vid=1,
            args=[],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            ensure_handle=lambda value: value,
            declare=_declare,
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.map.clear_h", ir_text)
        self.assertNotIn("nyash.map.probe_hh", ir_text)


if __name__ == "__main__":
    unittest.main()
