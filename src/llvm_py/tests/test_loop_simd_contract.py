#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.loop_simd_contract import build_loop_simd_contract


class TestLoopSimdContract(unittest.TestCase):
    def test_builds_int_map_candidate_contract(self):
        contract = build_loop_simd_contract(
            {
                "header": 1,
                "numeric_kind": "induction",
                "numeric_proof_source": "simple_while_arithmetic_only",
                "numeric_induction_value_ids": [10, 11],
                "header_phi_value_ids": [10],
                "header_compare_operand_value_ids": [12],
            }
        )

        self.assertEqual(contract["policy"]["mode"], "auto_eligible")
        self.assertEqual(contract["diag"]["accepted_class"], "int_map_candidate")
        self.assertEqual(contract["lowering"]["llvm_loop_md"], "defer")

    def test_builds_int_reduction_candidate_contract(self):
        contract = build_loop_simd_contract(
            {
                "header": 1,
                "numeric_kind": "induction",
                "numeric_proof_source": "simple_while_arithmetic_only",
                "numeric_induction_value_ids": [10, 11],
                "numeric_reduction_value_ids": [40],
                "header_phi_value_ids": [10],
                "header_compare_operand_value_ids": [12],
            }
        )

        self.assertEqual(contract["diag"]["accepted_class"], "int_reduction_candidate")
        self.assertEqual(contract["proof"]["reduction_value_ids"], [40])

    def test_rejects_non_numeric_loop_plan(self):
        self.assertIsNone(build_loop_simd_contract({"header": 1}))


if __name__ == "__main__":
    unittest.main()
