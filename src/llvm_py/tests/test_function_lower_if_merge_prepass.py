#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.function_lower import _seed_if_merge_ret_phi_incomings


class _ResolverStub:
    def __init__(self):
        self.block_phi_incomings = None


class _BuilderStub:
    def __init__(self):
        self.block_phi_incomings = {}
        self.preds = {4: [1, 2, 4], 5: [3]}
        self.resolver = _ResolverStub()


class TestFunctionLowerIfMergePrepass(unittest.TestCase):
    def test_seed_if_merge_ret_phi_incomings_dedups_non_self_preds(self):
        builder = _BuilderStub()

        _seed_if_merge_ret_phi_incomings(builder, {4: 90, 5: 91})

        self.assertEqual(
            builder.block_phi_incomings,
            {
                4: {90: [(1, 90), (2, 90)]},
                5: {91: [(3, 91)]},
            },
        )
        self.assertIs(builder.resolver.block_phi_incomings, builder.block_phi_incomings)


if __name__ == "__main__":
    unittest.main()
