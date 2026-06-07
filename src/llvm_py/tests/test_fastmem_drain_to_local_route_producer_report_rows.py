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


def _drain_to_local_mir() -> dict:
    return {
        "functions": [
            {
                "name": "test",
                "metadata": {
                    "fastmem_regions": [
                        {
                            "id": 1,
                            "contract": "PageMetaLayoutV0",
                            "source_span": {"file": "test.hako", "line": 1, "column": 1},
                        }
                    ],
                    "fastmem_access_plans": [
                        {
                            "kind": "atomic_remote_head_drain",
                            "verified": True,
                            "status": "verified",
                            "block": 0,
                            "instruction_index": 0,
                            "region": 1,
                            "page": 12,
                            "result": 17,
                            "remote_head_layout_id": "PageMetaLayoutV0",
                            "remote_head_field_id": "remote_head",
                            "remote_head_field_class": "atomic_remote_head",
                            "remote_head_byte_offset": 32,
                            "remote_head_field_size": 8,
                            "remote_head_field_type": "usize",
                            "remote_head_alignment": 8,
                            "memory_order_policy": "acquire_exchange",
                            "retry_attempt_limit": 0,
                            "lowerable": True,
                        },
                        {
                            "kind": "drain_remote_list_to_local",
                            "verified": True,
                            "status": "verified",
                            "block": 0,
                            "instruction_index": 1,
                            "region": 1,
                            "page": 12,
                            "local_free_head_layout_id": "PageMetaLayoutV0",
                            "local_free_head_field_id": "local_free_head",
                            "local_free_head_field_class": "local_free_head",
                            "local_free_head_byte_offset": 24,
                            "local_free_head_field_size": 8,
                            "local_free_head_field_type": "usize",
                            "local_free_head_alignment": 8,
                            "block_next_layout_id": "FreeBlockNodeLayoutV0",
                            "block_next_field_id": "next",
                            "block_next_field_class": "local_free_block_next",
                            "block_next_byte_offset": 0,
                            "block_next_field_size": 8,
                            "block_next_field_type": "usize",
                            "block_next_alignment": 8,
                            "publication_order": "verifier_owned_acquire_then_owner_local",
                            "token_provenance_valid": True,
                            "page_operand_valid": True,
                            "head_class_resolved": True,
                            "block_next_access_resolved": True,
                            "lowerable": True,
                        },
                    ],
                    "fastmem_remote_owner_facts": [],
                    "fastmem_block_next_facts": [],
                },
                "blocks": [
                    {
                        "instructions": [
                            {
                                "op": "memop",
                                "kind": "atomic_remote_head_drain",
                                "dst": 17,
                                "operands": [12],
                            },
                            {
                                "op": "memop",
                                "kind": "drain_remote_list_to_local",
                                "operands": [12, 17],
                            },
                        ]
                    }
                ],
            }
        ]
    }


class TestFastMemDrainToLocalRouteProducerReportRows(unittest.TestCase):
    def test_selection_report_rows_keep_route_closed(self):
        rows = dict(
            build_report_rows(
                _drain_to_local_mir(),
                object_out=Path(tempfile.gettempdir()) / "fastmem_drain_to_local_route.o",
                profile="remote-free-drain-to-local-selection",
            )
        )

        self.assertEqual(rows["fastmem_atomic_remote_head_drain_to_local_route_selection"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_to_local_route_selected"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_to_local_route_producer_pilot"], "0")
        self.assertEqual(rows["atomic_remote_head_drain_to_local_route_open"], "0")
        self.assertEqual(rows["atomic_remote_head_drain_open"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_lowered_count"], "1")
        self.assertEqual(rows["replacement_front_next_producer_slice"], "atomic_remote_head_drain_to_local_route_producer_pilot")

    def test_producer_report_rows_open_route_and_advance_next_slice(self):
        rows = dict(
            build_report_rows(
                _drain_to_local_mir(),
                object_out=Path(tempfile.gettempdir()) / "fastmem_drain_to_local_route.o",
                profile="remote-free-drain-to-local",
            )
        )

        self.assertEqual(rows["fastmem_atomic_remote_head_drain_to_local_route_producer_pilot"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_to_local_route_selected"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_to_local_route_producer_pilot"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_to_local_route_open"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_open"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_lowered_count"], "1")
        self.assertEqual(rows["replacement_front_next_producer_slice"], "atomic_remote_head_drain_local_list_mutation_preflight")


if __name__ == "__main__":
    unittest.main()
