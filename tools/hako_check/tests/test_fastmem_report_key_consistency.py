from __future__ import annotations

import sys
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from fastmem_mir_to_llvm_producer_report_body import build_report_rows


class FastMemReportKeyConsistencyTest(unittest.TestCase):
    def _mir(self) -> dict[str, object]:
        return {
            "functions": [
                {
                    "name": "main",
                    "metadata": {
                        "fastmem_regions": [
                            {
                                "contract": "PageMapV0",
                            }
                        ],
                        "fastmem_access_plans": [
                            {
                                "kind": "table_index",
                                "verified": True,
                                "status": "verified",
                                "lowerable": True,
                                "table_id": "page_table",
                                "field_id": "page",
                                "table_length_resolved": True,
                                "stride_resolved": True,
                                "field_offset_resolved": True,
                                "element_layout_verified": True,
                                "bounds_proof_valid": True,
                                "overflow_proof_valid": True,
                                "alignment_valid": True,
                                "alignment": 8,
                            },
                            {
                                "kind": "field_load",
                                "verified": True,
                                "status": "verified",
                                "lowerable": True,
                                "table_id": "page_table",
                                "field_id": "owner_worker_id",
                                "alignment_valid": True,
                                "alignment": 8,
                                "field_class": "plain_scalar",
                            },
                            {
                                "kind": "field_store",
                                "verified": True,
                                "status": "verified",
                                "lowerable": True,
                                "table_id": "page_table",
                                "field_id": "owner_worker_id",
                                "alignment_valid": True,
                                "alignment": 8,
                                "field_class": "plain_scalar",
                            },
                        ],
                        "fastmem_remote_owner_facts": [
                            {
                                "proof_kind": "source_assume_remote_owner",
                                "same_owner_rejected": True,
                            }
                        ],
                        "fastmem_block_next_facts": [
                            {
                                "proof_kind": "source_assume_remote_free_block_next",
                            }
                        ],
                    },
                    "blocks": [
                        {
                            "instructions": [
                                {"op": "branch"},
                                {"op": "memop", "kind": "current_alloc_owner_id"},
                                {"op": "memop", "kind": "owner_eq"},
                                {"op": "memop", "kind": "atomic_remote_head_push"},
                                {"op": "memop", "kind": "atomic_remote_head_drain"},
                                {"op": "memop", "kind": "drain_remote_list_to_local"},
                            ],
                            "terminator": {"op": "branch"},
                        }
                    ],
                }
            ]
        }

    def test_route_families_share_the_same_report_key_surface(self) -> None:
        mir = self._mir()
        object_out = Path("target/test-fastmem-report-key-consistency.o")
        profiles = [
            "remote-free-drain-to-local",
            "remote-owner-branch-routing-lowering",
            "fastmem-branch-cfg-lowering",
        ]

        key_sets: dict[str, set[str]] = {}
        for profile in profiles:
            rows = build_report_rows(mir, object_out=object_out, profile=profile)
            key_sets[profile] = {key for key, _ in rows}

        baseline = key_sets[profiles[0]]
        for profile in profiles[1:]:
            with self.subTest(profile=profile):
                self.assertSetEqual(baseline, key_sets[profile])

        required_keys = {
            "replacement_front_producer",
            "replacement_front_backend_artifact",
            "fastmem_table_access_plan_count",
            "atomic_remote_head_drain_selected",
            "remote_owner_branch_routing_selected",
            "fastmem_branch_cfg_selected",
        }
        self.assertTrue(required_keys.issubset(baseline))


if __name__ == "__main__":
    unittest.main()
