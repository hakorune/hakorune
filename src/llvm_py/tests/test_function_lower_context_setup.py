#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import builders.function_lower as function_lower


class _ResolverStub:
    def __init__(self):
        self.bound_context = None

    def bind_context(self, context):
        self.bound_context = context


class _BuilderStub:
    def __init__(self):
        self.vmap = {1: "old"}
        self.bb_map = {2: "old"}
        self.predeclared_ret_phis = {3: "old"}
        self.resolver = _ResolverStub()


class TestFunctionLowerContextSetup(unittest.TestCase):
    def test_reset_function_lower_state_clears_function_local_maps(self):
        builder = _BuilderStub()

        function_lower._reset_function_lower_state(builder)

        self.assertEqual(builder.vmap, {})
        self.assertEqual(builder.bb_map, {})
        self.assertEqual(builder.predeclared_ret_phis, {})

    def test_create_function_context_binds_builder_to_context_storage(self):
        builder = _BuilderStub()

        context = function_lower._create_function_context(builder, "Demo/0")

        self.assertEqual(context.func_name, "Demo/0")
        self.assertIs(builder.context, context)
        self.assertIs(builder.resolver.bound_context, context)
        self.assertIs(builder.block_phi_incomings, context.block_phi_incomings)
        self.assertIs(builder.phi_trivial_aliases, context.phi_trivial_aliases)
        self.assertIs(builder.def_blocks, context.def_blocks)
        self.assertIs(builder.block_end_values, context.block_end_values)
        self.assertIs(builder.phi_manager, context.phi_manager)


if __name__ == "__main__":
    unittest.main()
