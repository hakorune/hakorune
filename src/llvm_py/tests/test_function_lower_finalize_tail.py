#!/usr/bin/env python3
import sys
import types
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import builders.function_lower as function_lower
import builders.block_lower as block_lower


class _BuilderStub:
    pass


class _ContextStub:
    pass


class TestFunctionLowerFinalizeTail(unittest.TestCase):
    def test_run_finalize_tail_keeps_finalize_order(self):
        builder = _BuilderStub()
        func = types.SimpleNamespace(name="demo")
        context = _ContextStub()
        block_by_id = {1: {"id": 1, "instructions": []}}
        events = []

        prev_finalize = function_lower._finalize_phis
        prev_lower_terms = block_lower.lower_terminators
        prev_phi_contract = function_lower._enforce_phi_ordering_contract
        prev_enforce_terms = function_lower._enforce_terminators
        prev_hot = function_lower._emit_hot_summary

        function_lower._finalize_phis = lambda _builder, _context: events.append("finalize_phis")
        block_lower.lower_terminators = lambda _builder, _func: events.append("lower_terminators")
        function_lower._enforce_phi_ordering_contract = lambda _builder: events.append("phi_contract")
        function_lower._enforce_terminators = lambda _builder, _func, _blocks: events.append("enforce_terminators")
        function_lower._emit_hot_summary = lambda _context: events.append("hot_summary")

        try:
            function_lower._run_finalize_tail(builder, func, block_by_id, context)
        finally:
            function_lower._finalize_phis = prev_finalize
            block_lower.lower_terminators = prev_lower_terms
            function_lower._enforce_phi_ordering_contract = prev_phi_contract
            function_lower._enforce_terminators = prev_enforce_terms
            function_lower._emit_hot_summary = prev_hot

        self.assertEqual(
            events,
            ["finalize_phis", "lower_terminators", "phi_contract", "enforce_terminators", "hot_summary"],
        )

    def test_run_finalize_tail_tolerates_nonfatal_tail_errors(self):
        builder = _BuilderStub()
        func = types.SimpleNamespace(name="demo")
        context = _ContextStub()
        block_by_id = {1: {"id": 1, "instructions": []}}
        events = []

        prev_finalize = function_lower._finalize_phis
        prev_lower_terms = block_lower.lower_terminators
        prev_phi_contract = function_lower._enforce_phi_ordering_contract
        prev_enforce_terms = function_lower._enforce_terminators
        prev_hot = function_lower._emit_hot_summary

        function_lower._finalize_phis = lambda _builder, _context: events.append("finalize_phis")
        block_lower.lower_terminators = lambda _builder, _func: events.append("lower_terminators")
        function_lower._enforce_phi_ordering_contract = lambda _builder: events.append("phi_contract")

        def _boom_terms(_builder, _func, _blocks):
            events.append("enforce_terminators")
            raise RuntimeError("non-fatal")

        def _boom_hot(_context):
            events.append("hot_summary")
            raise RuntimeError("non-fatal")

        function_lower._enforce_terminators = _boom_terms
        function_lower._emit_hot_summary = _boom_hot

        try:
            function_lower._run_finalize_tail(builder, func, block_by_id, context)
        finally:
            function_lower._finalize_phis = prev_finalize
            block_lower.lower_terminators = prev_lower_terms
            function_lower._enforce_phi_ordering_contract = prev_phi_contract
            function_lower._enforce_terminators = prev_enforce_terms
            function_lower._emit_hot_summary = prev_hot

        self.assertEqual(
            events,
            ["finalize_phis", "lower_terminators", "phi_contract", "enforce_terminators", "hot_summary"],
        )


if __name__ == "__main__":
    unittest.main()
