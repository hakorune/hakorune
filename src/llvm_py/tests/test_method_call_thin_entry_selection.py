#!/usr/bin/env python3
import os
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from src.llvm_py.instructions.mir_call.method_call import lower_method_call


class _ResolverStub:
    def __init__(self):
        self.string_literals = {}
        self.string_ptrs = {}
        self.value_types = {}
        self.marked_strings = set()
        self.thin_entry_selection_by_value = {}
        self.thin_entry_selection_by_subject = {}
        self.thin_entry_selections = []

    def mark_string(self, vid):
        self.marked_strings.add(vid)


class _OwnerStub:
    preds = None
    block_end_values = None
    bb_map = None


class TestMethodCallThinEntrySelection(unittest.TestCase):
    def setUp(self):
        self._old_safepoint = os.environ.get("NYASH_LLVM_AUTO_SAFEPOINT")
        os.environ["NYASH_LLVM_AUTO_SAFEPOINT"] = "0"

    def tearDown(self):
        if self._old_safepoint is None:
            os.environ.pop("NYASH_LLVM_AUTO_SAFEPOINT", None)
        else:
            os.environ["NYASH_LLVM_AUTO_SAFEPOINT"] = self._old_safepoint

    def _new_module(self, name: str):
        module = ir.Module(name=name)
        i64 = ir.IntType(64)
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)
        return i64, module, builder

    def test_known_receiver_selection_prefers_thin_direct_call_name(self):
        i64, module, builder = self._new_module("test_method_call_known_receiver_thin")
        ir.Function(module, ir.FunctionType(i64, [i64]), name="Point.step/1")

        resolver = _ResolverStub()
        resolver.thin_entry_selection_by_subject[("user_box_method", "Point.step")] = [
            {
                "surface": "user_box_method",
                "subject": "Point.step",
                "manifest_row": "user_box_method.known_receiver",
                "selected_entry": "thin_internal_entry",
            }
        ]
        vmap = {1: ir.Constant(i64, 77)}

        lower_method_call(
            builder=builder,
            module=module,
            box_name="Point",
            method="step",
            receiver=1,
            args=[],
            dst_vid=2,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[2])

        ir_text = str(module)
        self.assertIn('%"thin_known_receiver_step" = call i64 @"Point.step/1"', ir_text)
        self.assertNotIn('%"known_box_step" = call i64 @"Point.step/1"', ir_text)

    def test_without_selection_method_call_uses_legacy_direct_route_name(self):
        i64, module, builder = self._new_module("test_method_call_known_receiver_legacy")
        ir.Function(module, ir.FunctionType(i64, [i64]), name="Point.step/1")

        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 77)}

        lower_method_call(
            builder=builder,
            module=module,
            box_name="Point",
            method="step",
            receiver=1,
            args=[],
            dst_vid=2,
            vmap=vmap,
            resolver=resolver,
            owner=_OwnerStub(),
        )
        builder.ret(vmap[2])

        ir_text = str(module)
        self.assertIn('%"known_box_step" = call i64 @"Point.step/1"', ir_text)
        self.assertNotIn('%"thin_known_receiver_step" = call i64 @"Point.step/1"', ir_text)
