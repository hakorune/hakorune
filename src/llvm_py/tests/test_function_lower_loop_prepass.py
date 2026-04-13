#!/usr/bin/env python3
import os
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import builders.function_lower as function_lower


class _ContextStub:
    def __init__(self):
        self.integerish_value_ids = set()
        self.non_negative_value_ids = set()
        self.numeric_loop_plans = {}
        self.loop_simd_contracts = {}


class TestFunctionLowerLoopPrepass(unittest.TestCase):
    def test_run_loop_prepass_returns_none_when_gate_disabled(self):
        prev = os.environ.get("NYASH_LLVM_PREPASS_LOOP")
        os.environ.pop("NYASH_LLVM_PREPASS_LOOP", None)
        try:
            self.assertIsNone(function_lower._run_loop_prepass({}))
        finally:
            if prev is None:
                os.environ.pop("NYASH_LLVM_PREPASS_LOOP", None)
            else:
                os.environ["NYASH_LLVM_PREPASS_LOOP"] = prev

    def test_run_loop_prepass_uses_detect_simple_while_when_enabled(self):
        prev_env = os.environ.get("NYASH_LLVM_PREPASS_LOOP")
        prev_detect = function_lower.detect_simple_while
        os.environ["NYASH_LLVM_PREPASS_LOOP"] = "1"

        try:
            function_lower.detect_simple_while = lambda _blocks: {
                "header": 1,
                "then": 2,
                "latch": 3,
                "exit": 4,
            }
            plan = function_lower._run_loop_prepass({1: {"id": 1, "instructions": []}})
        finally:
            function_lower.detect_simple_while = prev_detect
            if prev_env is None:
                os.environ.pop("NYASH_LLVM_PREPASS_LOOP", None)
            else:
                os.environ["NYASH_LLVM_PREPASS_LOOP"] = prev_env

        self.assertEqual(plan, {"header": 1, "then": 2, "latch": 3, "exit": 4})

    def test_run_loop_prepass_annotates_numeric_induction_when_body_is_arithmetic_only(self):
        prev_env = os.environ.get("NYASH_LLVM_PREPASS_LOOP")
        prev_detect = function_lower.detect_simple_while
        os.environ["NYASH_LLVM_PREPASS_LOOP"] = "1"
        context = _ContextStub()
        context.integerish_value_ids = {7, 10, 11, 12}
        context.non_negative_value_ids = {10, 11, 12}

        try:
            function_lower.detect_simple_while = lambda _blocks: {
                "header": 1,
                "then": 2,
                "latch": 3,
                "exit": 4,
                "cond": 7,
                "body_insts": [
                    {"op": "copy", "dst": 10, "src": 6},
                    {"op": "binop", "dst": 11, "operation": "+", "lhs": 10, "rhs": 12},
                ],
                "skip_blocks": [1, 2, 3],
            }
            plan = function_lower._run_loop_prepass({1: {"id": 1, "instructions": []}}, context)
        finally:
            function_lower.detect_simple_while = prev_detect
            if prev_env is None:
                os.environ.pop("NYASH_LLVM_PREPASS_LOOP", None)
            else:
                os.environ["NYASH_LLVM_PREPASS_LOOP"] = prev_env

        self.assertEqual(plan["numeric_kind"], "induction")
        self.assertEqual(plan["numeric_induction_value_ids"], [10, 11])
        self.assertEqual(context.numeric_loop_plans[1]["numeric_kind"], "induction")
        self.assertEqual(context.loop_simd_contracts[1]["diag"]["accepted_class"], "int_map_candidate")

    def test_run_loop_prepass_skips_numeric_annotation_for_mixed_body(self):
        prev_env = os.environ.get("NYASH_LLVM_PREPASS_LOOP")
        prev_detect = function_lower.detect_simple_while
        os.environ["NYASH_LLVM_PREPASS_LOOP"] = "1"
        context = _ContextStub()
        context.integerish_value_ids = {7, 10}

        try:
            function_lower.detect_simple_while = lambda _blocks: {
                "header": 1,
                "then": 2,
                "latch": 3,
                "exit": 4,
                "cond": 7,
                "body_insts": [
                    {"op": "call", "dst": 10, "func": 5, "args": [6]},
                ],
                "skip_blocks": [1, 2, 3],
            }
            plan = function_lower._run_loop_prepass({1: {"id": 1, "instructions": []}}, context)
        finally:
            function_lower.detect_simple_while = prev_detect
            if prev_env is None:
                os.environ.pop("NYASH_LLVM_PREPASS_LOOP", None)
            else:
                os.environ["NYASH_LLVM_PREPASS_LOOP"] = prev_env

        self.assertNotIn("numeric_kind", plan)
        self.assertEqual(context.numeric_loop_plans, {})
        self.assertEqual(context.loop_simd_contracts, {})

    def test_run_loop_prepass_annotates_numeric_reduction_when_body_updates_non_compare_phi(self):
        prev_env = os.environ.get("NYASH_LLVM_PREPASS_LOOP")
        prev_detect = function_lower.detect_simple_while
        os.environ["NYASH_LLVM_PREPASS_LOOP"] = "1"
        context = _ContextStub()
        context.integerish_value_ids = {7, 20, 22, 30, 32, 40}
        context.non_negative_value_ids = {20, 22, 30, 32, 40}

        try:
            function_lower.detect_simple_while = lambda _blocks: {
                "header": 1,
                "then": 2,
                "latch": 3,
                "exit": 4,
                "cond": 7,
                "body_insts": [
                    {"op": "copy", "dst": 32, "src": 30},
                    {"op": "binop", "dst": 40, "operation": "+", "lhs": 32, "rhs": 22},
                ],
                "header_phi_value_ids": [20, 30],
                "header_compare_operand_value_ids": [20, 31],
                "skip_blocks": [1, 2, 3],
            }
            plan = function_lower._run_loop_prepass({1: {"id": 1, "instructions": []}}, context)
        finally:
            function_lower.detect_simple_while = prev_detect
            if prev_env is None:
                os.environ.pop("NYASH_LLVM_PREPASS_LOOP", None)
            else:
                os.environ["NYASH_LLVM_PREPASS_LOOP"] = prev_env

        self.assertEqual(plan["numeric_kind"], "induction")
        self.assertEqual(plan["numeric_reduction_value_ids"], [40])
        self.assertEqual(context.numeric_loop_plans[1]["numeric_reduction_value_ids"], [40])
        self.assertEqual(context.loop_simd_contracts[1]["diag"]["accepted_class"], "int_reduction_candidate")
        self.assertEqual(
            context.loop_simd_contracts[1]["proof"]["reduction_value_ids"],
            [40],
        )

    def test_run_loop_prepass_skips_numeric_reduction_for_compare_carrier(self):
        prev_env = os.environ.get("NYASH_LLVM_PREPASS_LOOP")
        prev_detect = function_lower.detect_simple_while
        os.environ["NYASH_LLVM_PREPASS_LOOP"] = "1"
        context = _ContextStub()
        context.integerish_value_ids = {7, 20, 22, 40}

        try:
            function_lower.detect_simple_while = lambda _blocks: {
                "header": 1,
                "then": 2,
                "latch": 3,
                "exit": 4,
                "cond": 7,
                "body_insts": [
                    {"op": "binop", "dst": 40, "operation": "+", "lhs": 20, "rhs": 22},
                ],
                "header_phi_value_ids": [20],
                "header_compare_operand_value_ids": [20, 31],
                "skip_blocks": [1, 2, 3],
            }
            plan = function_lower._run_loop_prepass({1: {"id": 1, "instructions": []}}, context)
        finally:
            function_lower.detect_simple_while = prev_detect
            if prev_env is None:
                os.environ.pop("NYASH_LLVM_PREPASS_LOOP", None)
            else:
                os.environ["NYASH_LLVM_PREPASS_LOOP"] = prev_env

        self.assertEqual(plan["numeric_kind"], "induction")
        self.assertNotIn("numeric_reduction_value_ids", plan)
        self.assertEqual(context.numeric_loop_plans[1].get("numeric_reduction_value_ids"), None)
        self.assertEqual(context.loop_simd_contracts[1]["diag"]["accepted_class"], "int_map_candidate")


if __name__ == "__main__":
    unittest.main()
