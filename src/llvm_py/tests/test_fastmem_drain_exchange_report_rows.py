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


def _exchange_producer_mir() -> dict:
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
                        }
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
                            }
                        ]
                    }
                ],
            }
        ]
    }


class TestFastMemDrainExchangeReportRows(unittest.TestCase):
    def test_exchange_producer_report_rows_pin_selected_slice(self):
        rows = dict(
            build_report_rows(
                _exchange_producer_mir(),
                object_out=Path(tempfile.gettempdir()) / "fastmem_exchange_producer.o",
                profile="remote-free-drain-exchange",
            )
        )

        self.assertEqual(rows["fastmem_atomic_remote_head_drain_exchange_producer_pilot"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_exchange_selected"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_open"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_lowered_count"], "1")
        self.assertEqual(rows["atomic_remote_head_drain_exchange_order"], "acquire")
        self.assertEqual(rows["atomic_remote_head_drain_result_kind"], "remote_free_list_token")
        self.assertEqual(rows["atomic_remote_head_drain_to_local_route_open"], "0")
        self.assertEqual(rows["remote_owner_branch_routing_open"], "0")


if __name__ == "__main__":
    unittest.main()
