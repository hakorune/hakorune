#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.numeric_loop_policy import (
    apply_numeric_loop_pass_policy,
    numeric_loop_vectorization_flags,
)


class _DummyPMB:
    def __init__(self):
        self.loop_vectorize = False
        self.slp_vectorize = False


class TestNumericLoopPolicy(unittest.TestCase):
    def test_level_two_enables_vectorization_knobs(self):
        pmb = _DummyPMB()

        apply_numeric_loop_pass_policy(pmb, 2)

        self.assertTrue(pmb.loop_vectorize)
        self.assertTrue(pmb.slp_vectorize)

    def test_lower_opt_levels_keep_vectorization_off(self):
        pmb = _DummyPMB()

        apply_numeric_loop_pass_policy(pmb, 1)

        self.assertFalse(pmb.loop_vectorize)
        self.assertFalse(pmb.slp_vectorize)

    def test_flag_helper_matches_builder_policy_shape(self):
        self.assertEqual(numeric_loop_vectorization_flags(3), (True, True))


if __name__ == "__main__":
    unittest.main()
