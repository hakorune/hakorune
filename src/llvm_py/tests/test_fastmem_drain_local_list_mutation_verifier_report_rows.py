#!/usr/bin/env python3
import tempfile
import sys
import unittest
from pathlib import Path

_REPO_ROOT = Path(__file__).resolve().parents[3]
_HAKO_CHECK_ROOT = _REPO_ROOT / "tools" / "hako_check"
for _path in (str(_REPO_ROOT), str(_HAKO_CHECK_ROOT)):
    if _path not in sys.path:
        sys.path.insert(0, _path)

from fastmem_mir_to_llvm_producer_report_body import build_report_rows
from test_fastmem_drain_local_list_mutation_preflight_report_rows import (
    _drain_local_list_mir,
)


class TestFastMemDrainLocalListMutationVerifierReportRows(unittest.TestCase):
    def test_verifier_report_rows_pin_lowering_slice(self):
        rows = dict(
            build_report_rows(
                _drain_local_list_mir(),
                object_out=Path(tempfile.gettempdir())
                / "fastmem_drain_local_list_mutation_verifier.o",
                profile="remote-free-drain-local-list-mutation-verifier-preconditions",
            )
        )

        self.assertEqual(rows["fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions"], "1")
        self.assertEqual(rows["fastmem_memop_drain_remote_list_to_local_count"], "1")
        self.assertEqual(rows["drain_remote_list_to_local_plan_count"], "1")
        self.assertEqual(rows["drain_remote_list_to_local_token_provenance_valid"], "1")
        self.assertEqual(rows["drain_remote_list_to_local_page_operand_valid"], "1")
        self.assertEqual(rows["drain_remote_list_to_local_head_class_resolved"], "1")
        self.assertEqual(rows["drain_remote_list_to_local_lowerable_count"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_local_list_mutation_open"], "0")
        self.assertEqual(rows["replacement_front_next_producer_slice"], "atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot")
        self.assertEqual(rows["remote_owner_branch_routing_open"], "0")
        self.assertEqual(rows["type_abi_hot_lookup_count"], "0")
        self.assertEqual(rows["provider_abi_hot_dispatch_count"], "0")
        self.assertEqual(rows["product_activation"], "0")
        self.assertEqual(rows["global_allocator_claim"], "0")
        self.assertEqual(rows["winner_claim"], "0")


if __name__ == "__main__":
    unittest.main()
