#!/usr/bin/env python3
import unittest

import llvmlite.ir as ir

from src.llvm_py.builders.closure_split_contract import build_closure_split_contract
from src.llvm_py.instructions.mir_call.closure_call import lower_closure_creation


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
    module = ir.Module(name="test_closure_split_contract")
    fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder, bb


class TestClosureSplitContract(unittest.TestCase):
    def test_builds_empty_env_contract(self):
        contract = build_closure_split_contract(params=[{"id": 1}], captures=[], me_capture=None)

        self.assertEqual(contract["diag"]["accepted_class"], "empty_env")
        self.assertEqual(contract["proof"]["env_capture_count"], 0)
        self.assertEqual(contract["lowering"]["ctor_name"], "nyash.closure.new")

    def test_builds_me_only_env_contract(self):
        contract = build_closure_split_contract(params=[], captures=[], me_capture=44)

        self.assertEqual(contract["diag"]["accepted_class"], "me_only_env")
        self.assertEqual(contract["proof"]["env_capture_value_ids"], [44])
        self.assertTrue(contract["lowering"]["use_capture_ctor"])

    def test_builds_capture_env_with_me_contract(self):
        contract = build_closure_split_contract(
            params=[{"id": 1}],
            captures=[{"id": 40}, 41],
            me_capture=42,
        )

        self.assertEqual(contract["diag"]["accepted_class"], "capture_env_with_me")
        self.assertEqual(contract["proof"]["capture_value_ids"], [40, 41])
        self.assertEqual(contract["proof"]["env_capture_value_ids"], [40, 41, 42])
        self.assertEqual(contract["policy"]["env_scalarization"], "defer")
        self.assertEqual(contract["policy"]["thin_entry_specialization"], "defer")

    def test_lower_closure_creation_uses_simple_ctor_for_empty_env(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_closure_creation(
            builder,
            module,
            params=[{"id": 1}],
            captures=[],
            me_capture=None,
            dst_vid=90,
            vmap=vmap,
            resolver=resolver,
            owner=owner,
        )
        builder.ret(vmap[90])

        ir_text = str(module)
        self.assertIn('call i64 @"nyash.closure.new"(i64 1)', ir_text)
        self.assertNotIn("nyash.closure.new_with_captures", ir_text)

    def test_lower_closure_creation_uses_capture_ctor_for_me_only_env(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_closure_creation(
            builder,
            module,
            params=[],
            captures=[],
            me_capture=93,
            dst_vid=94,
            vmap=vmap,
            resolver=resolver,
            owner=owner,
        )
        builder.ret(vmap[94])

        ir_text = str(module)
        self.assertIn('call i64 @"nyash.closure.new_with_captures"(i64 0, i64 1, i64 94)', ir_text)


if __name__ == "__main__":
    unittest.main()
