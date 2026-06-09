from __future__ import annotations

import sys
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from fastmem_mir_to_llvm_producer_report_body import build_report_rows
from typed_object_exact_slot_inventory import typed_object_exact_slot_inventory
from report_kv import row_key_surface, shared_key_surface


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
            key_sets[profile] = row_key_surface(rows)

        baseline = key_sets[profiles[0]]
        for profile in profiles[1:]:
            with self.subTest(profile=profile):
                self.assertSetEqual(baseline, key_sets[profile])

        self.assertSetEqual(
            baseline,
            shared_key_surface(
                build_report_rows(mir, object_out=object_out, profile=profiles[0]),
                build_report_rows(mir, object_out=object_out, profile=profiles[1]),
                build_report_rows(mir, object_out=object_out, profile=profiles[2]),
            ),
        )

        required_keys = {
            "replacement_front_producer",
            "replacement_front_backend_artifact",
            "fastmem_table_access_plan_count",
            "atomic_remote_head_drain_selected",
            "remote_owner_branch_routing_selected",
            "fastmem_branch_cfg_selected",
        }
        self.assertTrue(required_keys.issubset(baseline))

    def test_typed_object_exact_slot_inventory_reflects_storage_families(self) -> None:
        mir = {
            "user_box_decls": [
                {
                    "name": "Point",
                    "field_decls": [
                        {"name": "x", "declared_type": "i64", "is_weak": False},
                        {"name": "y", "declared_type": "u64", "is_weak": False},
                        {"name": "owner", "declared_type": "handle", "is_weak": False},
                        {"name": "compat_only", "declared_type": "i64", "is_weak": True},
                    ],
                }
            ],
            "typed_object_plans": [
                {
                    "box_name": "Point",
                    "fields": [
                        {"name": "x", "storage": "i64"},
                        {"name": "y", "storage": "u64"},
                        {"name": "owner", "storage": "handle"},
                    ],
                }
            ],
        }

        inventory = typed_object_exact_slot_inventory(mir)
        self.assertEqual(inventory["typed_object_exact_slot_get_i64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_set_i64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_get_u64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_set_u64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_get_handle_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_set_handle_count"], 1)
        self.assertEqual(inventory["typed_object_exact_helper_call_count"], 3)
        self.assertEqual(inventory["typed_object_inline_slot_load_count"], 0)
        self.assertEqual(inventory["typed_object_inline_slot_store_count"], 0)
        self.assertEqual(inventory["typed_object_compat_field_get_count"], 1)
        self.assertEqual(inventory["typed_object_get_compat_i64_count"], 0)
        self.assertEqual(inventory["typed_object_exact_name_lookup_count"], 0)
        self.assertEqual(inventory["typed_object_exact_internal_dispatch_count"], 0)
        self.assertEqual(inventory["typed_object_exact_silent_fallback_count"], 0)
        self.assertEqual(inventory["typed_object_required_route_failfast_count"], 1)


if __name__ == "__main__":
    unittest.main()
