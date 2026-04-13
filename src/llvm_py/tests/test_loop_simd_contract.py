#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

from llvmlite import binding as llvm
from llvmlite import ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.loop_simd_contract import build_loop_simd_contract, apply_loop_simd_metadata


class TestLoopSimdContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        llvm.initialize()
        llvm.initialize_native_target()
        llvm.initialize_native_asmprinter()

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
        self.assertEqual(contract["lowering"]["llvm_loop_md"]["vectorize.enable"], True)

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
        self.assertEqual(contract["lowering"]["llvm_loop_md"]["reduction.kind"], "int_add")

    def test_rejects_non_numeric_loop_plan(self):
        self.assertIsNone(build_loop_simd_contract({"header": 1}))

    def test_apply_loop_simd_metadata_sets_llvm_loop_hint_for_int_map_candidate(self):
        module = ir.Module(name="m")
        fnty = ir.FunctionType(ir.VoidType(), [])
        func = ir.Function(module, fnty, name="f")
        entry = func.append_basic_block("entry")
        loop = func.append_basic_block("loop")
        exitb = func.append_basic_block("exit")

        builder = ir.IRBuilder(entry)
        builder.branch(loop)
        builder.position_at_end(loop)
        branch = builder.cbranch(ir.Constant(ir.IntType(1), 1), loop, exitb)

        contract = build_loop_simd_contract(
            {
                "header": 1,
                "numeric_kind": "induction",
                "numeric_proof_source": "simple_while_arithmetic_only",
                "numeric_induction_value_ids": [10],
                "header_phi_value_ids": [10],
                "header_compare_operand_value_ids": [12],
            }
        )

        self.assertTrue(apply_loop_simd_metadata(module, branch, contract))

        builder.position_at_end(exitb)
        builder.ret_void()
        llvm.parse_assembly(str(module))
        self.assertIn("llvm.loop", str(module))

    def test_apply_loop_simd_metadata_sets_llvm_loop_hint_for_reduction_candidate(self):
        module = ir.Module(name="m")
        fnty = ir.FunctionType(ir.VoidType(), [])
        func = ir.Function(module, fnty, name="f")
        entry = func.append_basic_block("entry")
        loop = func.append_basic_block("loop")
        exitb = func.append_basic_block("exit")

        builder = ir.IRBuilder(entry)
        builder.branch(loop)
        builder.position_at_end(loop)
        branch = builder.cbranch(ir.Constant(ir.IntType(1), 1), loop, exitb)

        contract = build_loop_simd_contract(
            {
                "header": 1,
                "numeric_kind": "induction",
                "numeric_proof_source": "simple_while_arithmetic_only",
                "numeric_induction_value_ids": [10],
                "numeric_reduction_value_ids": [40],
                "header_phi_value_ids": [10],
                "header_compare_operand_value_ids": [12],
            }
        )

        self.assertTrue(apply_loop_simd_metadata(module, branch, contract))


if __name__ == "__main__":
    unittest.main()
