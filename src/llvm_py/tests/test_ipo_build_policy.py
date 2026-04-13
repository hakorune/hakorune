import os
import tempfile
import unittest

from src.llvm_py.builders.closure_split_contract import build_closure_split_contract
from src.llvm_py.builders.ipo_callable_contract import build_ipo_callable_contract
from src.llvm_py.builders.ipo_call_edge_contract import build_ipo_call_edge_contract
from src.llvm_py.builders.ipo_build_policy import (
    IpoBuildPolicy,
    apply_ipo_build_policy,
    resolve_ipo_build_policy,
    summarize_ipo_contracts,
    thinlto_companion_path,
)
from src.llvm_py.llvm_builder import NyashLLVMBuilder


def _thin_candidate_summary():
    closure_contract = build_closure_split_contract(params=[{"id": 1}], captures=[], me_capture=None)
    callable_contract = build_ipo_callable_contract(closure_contract)
    edge_contract = build_ipo_call_edge_contract(callable_contract)
    return summarize_ipo_contracts(
        {"Demo/0": {1: callable_contract}},
        {"Demo/0": {1: edge_contract}},
    )


class TestIpoBuildPolicy(unittest.TestCase):
    def tearDown(self):
        os.environ.pop("NYASH_LLVM_LTO_MODE", None)
        os.environ.pop("HAKO_LLVM_LTO_MODE", None)

    def test_resolve_ipo_build_policy_defaults_off(self):
        policy = resolve_ipo_build_policy()
        self.assertEqual(policy, IpoBuildPolicy(lto_mode="off", pgo_mode="off", thinlto_import_candidate_count=0))

    def test_apply_ipo_build_policy_is_noop_for_phase272x(self):
        kwargs = {"opt": 2, "cpu": "native"}
        policy = IpoBuildPolicy(lto_mode="off", pgo_mode="off", thinlto_import_candidate_count=0)

        applied = apply_ipo_build_policy(dict(kwargs), policy)

        self.assertEqual(applied, kwargs)

    def test_resolve_ipo_build_policy_enables_thin_only_with_candidates(self):
        os.environ["NYASH_LLVM_LTO_MODE"] = "thin"

        policy = resolve_ipo_build_policy(_thin_candidate_summary())

        self.assertEqual(policy.lto_mode, "thin")
        self.assertEqual(policy.thinlto_import_candidate_count, 1)

    def test_resolve_ipo_build_policy_keeps_off_without_candidates(self):
        os.environ["NYASH_LLVM_LTO_MODE"] = "thin"

        policy = resolve_ipo_build_policy({"thinlto_import_candidate_count": 0, "direct_thin_edge_count": 0})

        self.assertEqual(policy.lto_mode, "off")

    def test_thinlto_companion_path_adds_bc_suffix(self):
        policy = IpoBuildPolicy(lto_mode="thin", pgo_mode="off", thinlto_import_candidate_count=1)
        self.assertEqual(thinlto_companion_path("/tmp/demo.o", policy), "/tmp/demo.thinlto.bc")

    def test_compile_to_object_emits_thinlto_companion_when_enabled(self):
        os.environ["NYASH_LLVM_LTO_MODE"] = "thin"
        builder = NyashLLVMBuilder()
        builder.build_from_mir(
            {
                "functions": [
                    {
                        "name": "main",
                        "params": [],
                        "blocks": [
                            {
                                "id": 0,
                                "instructions": [
                                    {"op": "const", "dst": 0, "value": {"type": "i64", "value": 42}},
                                    {"op": "ret", "value": 0},
                                ],
                            }
                        ],
                    }
                ]
            }
        )
        closure_contract = build_closure_split_contract(params=[{"id": 1}], captures=[], me_capture=None)
        callable_contract = build_ipo_callable_contract(closure_contract)
        edge_contract = build_ipo_call_edge_contract(callable_contract)
        builder.ipo_callable_contracts_by_function = {"main": {1: callable_contract}}
        builder.ipo_call_edge_contracts_by_function = {"main": {1: edge_contract}}

        with tempfile.TemporaryDirectory() as tmp:
            output_path = os.path.join(tmp, "demo.o")
            builder.compile_to_object(output_path)
            self.assertTrue(os.path.exists(output_path))
            self.assertTrue(os.path.exists(os.path.join(tmp, "demo.thinlto.bc")))


if __name__ == "__main__":
    unittest.main()
