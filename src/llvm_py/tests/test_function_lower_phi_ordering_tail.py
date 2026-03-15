#!/usr/bin/env python3
import contextlib
import io
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import builders.function_lower as function_lower
import phi_placement
import phi_wiring.debug_helper as phi_debug_helper


class _BuilderStub:
    pass


class TestFunctionLowerPhiOrderingTail(unittest.TestCase):
    def test_enforce_phi_ordering_contract_raises_in_strict_mode(self):
        prev_verify = phi_placement.verify_phi_ordering
        prev_strict = phi_debug_helper.is_phi_strict_enabled
        prev_debug = phi_debug_helper.is_phi_debug_enabled
        stderr = io.StringIO()

        phi_placement.verify_phi_ordering = lambda _builder: {2: False}
        phi_debug_helper.is_phi_strict_enabled = lambda: True
        phi_debug_helper.is_phi_debug_enabled = lambda: False

        try:
            with contextlib.redirect_stderr(stderr):
                with self.assertRaises(RuntimeError):
                    function_lower._enforce_phi_ordering_contract(_BuilderStub())
        finally:
            phi_placement.verify_phi_ordering = prev_verify
            phi_debug_helper.is_phi_strict_enabled = prev_strict
            phi_debug_helper.is_phi_debug_enabled = prev_debug

        self.assertIn("[CRITICAL] [function_lower/PHI] 1 blocks have incorrect PHI ordering: [2]", stderr.getvalue())

    def test_enforce_phi_ordering_contract_warns_when_debug_enabled(self):
        prev_verify = phi_placement.verify_phi_ordering
        prev_strict = phi_debug_helper.is_phi_strict_enabled
        prev_debug = phi_debug_helper.is_phi_debug_enabled
        stderr = io.StringIO()

        phi_placement.verify_phi_ordering = lambda _builder: {4: False}
        phi_debug_helper.is_phi_strict_enabled = lambda: False
        phi_debug_helper.is_phi_debug_enabled = lambda: True

        try:
            with contextlib.redirect_stderr(stderr):
                function_lower._enforce_phi_ordering_contract(_BuilderStub())
        finally:
            phi_placement.verify_phi_ordering = prev_verify
            phi_debug_helper.is_phi_strict_enabled = prev_strict
            phi_debug_helper.is_phi_debug_enabled = prev_debug

        self.assertIn("[WARNING] [function_lower/PHI] 1 blocks have incorrect PHI ordering: [4]", stderr.getvalue())

    def test_enforce_phi_ordering_contract_is_silent_on_success_when_debug_disabled(self):
        prev_verify = phi_placement.verify_phi_ordering
        prev_strict = phi_debug_helper.is_phi_strict_enabled
        prev_debug = phi_debug_helper.is_phi_debug_enabled
        stderr = io.StringIO()

        phi_placement.verify_phi_ordering = lambda _builder: {1: True, 2: True}
        phi_debug_helper.is_phi_strict_enabled = lambda: False
        phi_debug_helper.is_phi_debug_enabled = lambda: False

        try:
            with contextlib.redirect_stderr(stderr):
                function_lower._enforce_phi_ordering_contract(_BuilderStub())
        finally:
            phi_placement.verify_phi_ordering = prev_verify
            phi_debug_helper.is_phi_strict_enabled = prev_strict
            phi_debug_helper.is_phi_debug_enabled = prev_debug

        self.assertEqual(stderr.getvalue(), "")


if __name__ == "__main__":
    unittest.main()
