#!/usr/bin/env python3

import copy
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from llvm_builder import NyashLLVMBuilder
from ownership_lowering import OwnershipLoweringContractError, verify_ownership_lowering_v1
from pyvm.vm import PyVM


def function_data():
    operations = [
        {"block": 0, "instruction_index": 0, "op": "copy_owned", "dst": 1, "src": 0},
        {"block": 0, "instruction_index": 1, "op": "destroy_owned", "value": 1},
    ]
    return {
        "name": "Ownership.llvm/1",
        "params": [0],
        "blocks": [
            {
                "id": 0,
                "instructions": [
                    {"op": "copy_owned", "dst": 1, "src": 0},
                    {"op": "destroy_owned", "value": 1},
                    {"op": "const", "dst": 2, "value": {"type": "i64", "value": 0}},
                    {"op": "ret", "value": 2},
                ],
            }
        ],
        "metadata": {
            "value_types": {
                "0": {"kind": "handle", "box_type": "OwnedTestBox"},
                "1": {"kind": "handle", "box_type": "OwnedTestBox"},
            },
            "storage_classes": {"0": "box_ref", "1": "box_ref"},
            "ownership_ssa_v1": {
                "schema": "VerifiedOwnershipSsaV1",
                "producer": "rust_ownership_ssa_verifier_v1",
                "owner": 41,
                "backend": "llvm_py",
                "provider": "nyash_kernel",
                "value_kinds": {"0": "borrowed", "1": "owned"},
                "operations": operations,
            },
        },
    }


class TestOwnershipLowering(unittest.TestCase):
    def test_exact_witness_lowers_to_kernel_handle_pair(self):
        builder = NyashLLVMBuilder()
        builder.lower_function(function_data())
        llvm = str(builder.module)

        self.assertIn("nyrt_handle_retain_h", llvm)
        self.assertIn("nyrt_handle_release_h", llvm)
        self.assertIn("call i64", llvm)
        self.assertIn("call void", llvm)

    def test_missing_or_foreign_contract_rejects_before_lowering(self):
        for mutate, reason in [
            (lambda data: data["metadata"].pop("ownership_ssa_v1"), "missing_witness"),
            (
                lambda data: data["metadata"]["ownership_ssa_v1"].update(
                    {"provider": "root_shim"}
                ),
                "provider_missing_capability",
            ),
            (
                lambda data: data["metadata"]["storage_classes"].update({"1": "opaque"}),
                "missing_boxref",
            ),
        ]:
            data = function_data()
            mutate(data)
            with self.subTest(reason=reason):
                with self.assertRaisesRegex(OwnershipLoweringContractError, reason):
                    verify_ownership_lowering_v1(data)

    def test_operation_inventory_is_exact_and_use_ledger_closes(self):
        data = function_data()
        session = verify_ownership_lowering_v1(data)
        self.assertIsNotNone(session)
        first, second = data["blocks"][0]["instructions"][:2]
        session.claim(0, 0, first)
        session.claim(0, 1, second)
        session.finish()

        stale = copy.deepcopy(data)
        stale["metadata"]["ownership_ssa_v1"]["operations"][0]["dst"] = 9
        with self.assertRaisesRegex(OwnershipLoweringContractError, "foreign_operation"):
            verify_ownership_lowering_v1(stale)

    def test_pyvm_rejects_ownership_op_instead_of_skipping(self):
        program = {"functions": [function_data()]}
        with self.assertRaisesRegex(RuntimeError, "pyvm/ownership:missing_capability"):
            PyVM(program).run_args("Ownership.llvm/1", [7])


if __name__ == "__main__":
    unittest.main()
