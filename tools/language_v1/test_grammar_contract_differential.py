#!/usr/bin/env python3

import unittest

from tools.language_v1.grammar_contract_differential import generate_cases


class GrammarContractDifferentialTests(unittest.TestCase):
    def test_generation_is_deterministic_and_bounded(self) -> None:
        first = generate_cases(seed=3478, max_depth=2, case_count=8)
        second = generate_cases(seed=3478, max_depth=2, case_count=8)
        self.assertEqual(first, second)
        self.assertEqual(len(first), 8)
        self.assertTrue(all(1 <= row["composition_depth"] <= 2 for row in first))
        self.assertEqual(
            [row["profile"] for row in first],
            ["Canonical", "Compat2025"] * 4,
        )

    def test_generation_rejects_unbounded_configuration(self) -> None:
        with self.assertRaises(ValueError):
            generate_cases(seed=1, max_depth=0, case_count=1)
        with self.assertRaises(ValueError):
            generate_cases(seed=1, max_depth=1, case_count=0)


if __name__ == "__main__":
    unittest.main()
