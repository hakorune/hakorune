#!/usr/bin/env python3
import os
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import builders.function_lower as function_lower


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


if __name__ == "__main__":
    unittest.main()
