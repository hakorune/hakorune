#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

from llvmlite import ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.function_lower import _collect_param_candidate_value_ids, _map_function_params_to_vmap


class _BuilderStub:
    def __init__(self):
        self.vmap = {}


class TestFunctionLowerParamMap(unittest.TestCase):
    def test_collect_param_candidate_value_ids_uses_undefined_value_inputs(self):
        blocks = [
            {
                "id": 1,
                "instructions": [
                    {"op": "const", "dst": 10, "value": {"type": "int", "value": 1}},
                    {"op": "binop", "dst": 11, "lhs": 1, "rhs": 10},
                    {"op": "call", "dst": 12, "args": [2, 11]},
                    {"op": "mir_call", "dst": 13, "mir_call": {"args": [3, 12]}},
                ],
            }
        ]

        self.assertEqual(_collect_param_candidate_value_ids(blocks), [1, 2, 3])

    def test_map_function_params_to_vmap_prefers_explicit_param_ids(self):
        builder = _BuilderStub()
        module = ir.Module(name="test_mod")
        i64 = ir.IntType(64)
        func = ir.Function(module, ir.FunctionType(i64, [i64, i64]), name="Demo/2")

        param_value_ids = _map_function_params_to_vmap(builder, func, [90, 91], [])

        self.assertEqual(param_value_ids, [90, 91])
        self.assertIs(builder.vmap[90], func.args[0])
        self.assertIs(builder.vmap[91], func.args[1])

    def test_map_function_params_to_vmap_uses_heuristic_and_positional_fallback(self):
        builder = _BuilderStub()
        module = ir.Module(name="test_mod")
        i64 = ir.IntType(64)
        func = ir.Function(module, ir.FunctionType(i64, [i64, i64, i64]), name="Demo/3")
        blocks = [
            {
                "id": 1,
                "instructions": [
                    {"op": "binop", "dst": 50, "lhs": 7, "rhs": 8},
                    {"op": "ret", "value": 50},
                ],
            }
        ]

        param_value_ids = _map_function_params_to_vmap(builder, func, [], blocks)

        self.assertEqual(param_value_ids, [7, 8, 2])
        self.assertIs(builder.vmap[7], func.args[0])
        self.assertIs(builder.vmap[8], func.args[1])
        self.assertIs(builder.vmap[2], func.args[2])


if __name__ == "__main__":
    unittest.main()
