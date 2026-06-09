from __future__ import annotations

import json
import sys
from pathlib import Path
import unittest
import tempfile
from unittest import mock

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from fastmem_mir_to_llvm_producer_report_body import build_report_rows
import state_explain
from typed_object_exact_slot_inventory import (
    typed_object_exact_route_sample_rows,
    typed_object_exact_bridge_symbol,
    typed_object_exact_slot_route_decisions,
    typed_object_exact_slot_inventory,
)
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

    def test_typed_object_exact_slot_inventory_prefers_route_decisions_when_present(self) -> None:
        mir = {
            "functions": [
                {
                    "metadata": {
                        "route_decisions": [
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "semantic_op": "FieldGet",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_storage": "i64",
                                "selected_route": "hako.typed_object.slot_load_i64",
                            },
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "semantic_op": "FieldSet",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_storage": "u64",
                                "selected_route": "hako.typed_object.slot_store_u64",
                            },
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "semantic_op": "FieldGet",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_storage": "handle",
                                "selected_route": "hako.typed_object.slot_load_handle",
                            },
                        ]
                    }
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
        }

        inventory = typed_object_exact_slot_inventory(mir)
        self.assertEqual(inventory["typed_object_exact_slot_get_i64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_set_i64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_get_u64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_set_u64_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_get_handle_count"], 1)
        self.assertEqual(inventory["typed_object_exact_slot_set_handle_count"], 1)
        self.assertEqual(inventory["typed_object_exact_helper_call_count"], 3)
        self.assertEqual(inventory["typed_object_exact_slot_eligible_count"], 3)
        self.assertEqual(inventory["typed_object_compat_field_get_count"], 1)
        self.assertEqual(inventory["typed_object_required_route_failfast_count"], 1)
        self.assertEqual(inventory["typed_object_exact_route_decision_count"], 3)
        self.assertEqual(
            inventory["typed_object_exact_lowering_forms"], "exact_helper_bridge"
        )
        self.assertEqual(
            inventory["typed_object_exact_bridge_symbols"],
            "hako.object.exact_slot_get_handle_hii,hako.object.exact_slot_get_i64_hii,hako.object.exact_slot_set_u64_hiu",
        )

    def test_typed_object_exact_route_sample_rows_infer_bridge_symbol(self) -> None:
        rows = typed_object_exact_route_sample_rows(
            [
                {
                    "function": "main",
                    "site_id": "site-1",
                    "semantic_op": "FieldGet",
                    "selected_lowering_form": "exact_helper_bridge",
                    "selected_storage": "i64",
                    "selected_route": "hako.typed_object.slot_load_i64",
                }
            ]
        )

        self.assertEqual(
            rows,
            [
                ("typed_object_exact_route_0_function", "main"),
                ("typed_object_exact_route_0_site_id", "site-1"),
                ("typed_object_exact_route_0_selected_route", "hako.typed_object.slot_load_i64"),
                (
                    "typed_object_exact_route_0_selected_lowering_form",
                    "exact_helper_bridge",
                ),
                (
                    "typed_object_exact_route_0_selected_bridge_symbol",
                    "hako.object.exact_slot_get_i64_hii",
                ),
            ],
        )

    def test_typed_object_exact_slot_route_decisions_filter_exact_helper_bridge(self) -> None:
        mir = {
            "functions": [
                {
                    "name": "main",
                    "metadata": {
                        "route_decisions": [
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_route": "hako.typed_object.slot_load_i64",
                            },
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "selected_lowering_form": "native_direct",
                                "selected_route": "hako.typed_object.slot_load_i64",
                            },
                            {
                                "source_plan_kind": "OtherRoute",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_route": "hako.typed_object.slot_load_i64",
                            },
                        ]
                    },
                }
            ]
        }

        decisions = typed_object_exact_slot_route_decisions(mir)
        self.assertEqual(len(decisions), 1)
        self.assertEqual(decisions[0]["selected_route"], "hako.typed_object.slot_load_i64")
        self.assertEqual(decisions[0]["selected_lowering_form"], "exact_helper_bridge")

    def test_typed_object_exact_bridge_symbol_helper(self) -> None:
        self.assertEqual(
            typed_object_exact_bridge_symbol("FieldGet", "i64"),
            "hako.object.exact_slot_get_i64_hii",
        )
        self.assertEqual(
            typed_object_exact_bridge_symbol("FieldSet", "u64"),
            "hako.object.exact_slot_set_u64_hiu",
        )

    def test_build_report_rows_uses_route_decision_typed_object_exact_slot_counts(self) -> None:
        mir = {
            "functions": [
                {
                    "name": "main",
                    "metadata": {
                        "route_decisions": [
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "semantic_op": "FieldGet",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_storage": "i64",
                                "selected_route": "hako.typed_object.slot_load_i64",
                            },
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "semantic_op": "FieldSet",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_storage": "u64",
                                "selected_route": "hako.typed_object.slot_store_u64",
                            },
                        ],
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
                            }
                        ],
                    },
                    "blocks": [
                        {
                            "instructions": [{"op": "branch"}],
                            "terminator": {"op": "branch"},
                        }
                    ],
                }
            ],
            "typed_object_plans": [
                {
                    "box_name": "Point",
                    "fields": [
                        {"name": "x", "storage": "i64"},
                        {"name": "y", "storage": "u64"},
                    ],
                }
            ],
            "user_box_decls": [
                {
                    "name": "Point",
                    "field_decls": [
                        {"name": "x", "declared_type": "i64", "is_weak": False},
                        {"name": "y", "declared_type": "u64", "is_weak": False},
                    ],
                }
            ],
        }

        rows = build_report_rows(
            mir,
            object_out=Path("target/test-fastmem-report-row-uses-route-decisions.o"),
            profile="remote-free-drain-to-local",
        )
        self.assertEqual(
            sum(
                1
                for key, _ in rows
                if key == "typed_object_exact_route_0_selected_bridge_symbol"
            ),
            1,
        )
        row_map = dict(rows)
        self.assertEqual(row_map["typed_object_exact_slot_get_i64_count"], "1")
        self.assertEqual(row_map["typed_object_exact_slot_set_i64_count"], "1")
        self.assertEqual(row_map["typed_object_exact_slot_get_u64_count"], "1")
        self.assertEqual(row_map["typed_object_exact_slot_set_u64_count"], "1")
        self.assertEqual(row_map["typed_object_exact_helper_call_count"], "2")
        self.assertEqual(row_map["typed_object_exact_slot_eligible_count"], "2")
        self.assertEqual(row_map["typed_object_compat_field_get_count"], "0")
        self.assertEqual(row_map["typed_object_required_route_failfast_count"], "0")
        self.assertEqual(row_map["typed_object_exact_route_decision_count"], "2")
        self.assertEqual(row_map["typed_object_exact_lowering_forms"], "exact_helper_bridge")
        self.assertEqual(
            row_map["typed_object_exact_bridge_symbols"],
            "hako.object.exact_slot_get_i64_hii,hako.object.exact_slot_set_u64_hiu",
        )
        self.assertEqual(row_map["typed_object_exact_route_sample_count"], "2")
        self.assertEqual(
            row_map["typed_object_exact_route_0_selected_route"],
            "hako.typed_object.slot_load_i64",
        )
        self.assertEqual(
            row_map["typed_object_exact_route_0_selected_lowering_form"],
            "exact_helper_bridge",
        )
        self.assertEqual(
            row_map["typed_object_exact_route_0_selected_bridge_symbol"],
            "hako.object.exact_slot_get_i64_hii",
        )

    def test_state_explain_emits_route_decision_exact_slot_rows(self) -> None:
        mir = {
            "functions": [
                {
                    "metadata": {
                        "route_decisions": [
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "semantic_op": "FieldGet",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_storage": "i64",
                                "selected_route": "hako.typed_object.slot_load_i64",
                                "selected_bridge_symbol": "hako.object.exact_slot_get_i64_hii",
                            }
                        ]
                    }
                }
            ],
            "typed_object_plans": [
                {
                    "box_name": "Page",
                    "fields": [{"name": "capacity", "storage": "i64"}],
                }
            ],
            "user_box_decls": [
                {
                    "name": "Page",
                    "field_decls": [
                        {"name": "capacity", "declared_type": "i64", "is_weak": False}
                    ],
                }
            ],
        }

        with tempfile.TemporaryDirectory() as tmpdir:
            mir_path = Path(tmpdir) / "mir.json"
            out_path = Path(tmpdir) / "state.kv"
            mir_path.write_text(json.dumps(mir), encoding="utf-8")
            old_argv = sys.argv[:]
            try:
                sys.argv = [
                    "state_explain.py",
                    "--mir-json",
                    str(mir_path),
                    "--out",
                    str(out_path),
                ]
                rc = state_explain.main()
            finally:
                sys.argv = old_argv

            self.assertEqual(rc, 0)
            text = out_path.read_text(encoding="utf-8")
            self.assertIn("typed_object_exact_route_decision_count=1", text)
            self.assertIn("typed_object_exact_lowering_forms=exact_helper_bridge", text)
            self.assertIn(
                "typed_object_exact_bridge_symbols=hako.object.exact_slot_get_i64_hii",
                text,
            )
            self.assertIn(
                "typed_object_exact_route_0_selected_route=hako.typed_object.slot_load_i64",
                text,
            )
            self.assertIn(
                "typed_object_exact_route_0_selected_lowering_form=exact_helper_bridge",
                text,
            )
            self.assertIn(
                "typed_object_exact_route_0_selected_bridge_symbol=hako.object.exact_slot_get_i64_hii",
                text,
            )
            self.assertIn("typed_object_exact_route_sample_count=1", text)

    def test_state_explain_emits_array_text_session_route_rows(self) -> None:
        mir = {
            "functions": [
                {
                    "name": "array_text_indexof",
                    "metadata": {
                        "array_text_state_residence_route": {
                            "selected_route": "hako.array_text.session_indexof_const_utf8",
                            "selected_bridge_symbol": "hako.array_text.session_indexof_const_utf8",
                            "fallback_route": "nyash.array.string_indexof_hisi",
                            "fallback_policy": "fail_fast",
                        },
                        "array_text_residence_sessions": [
                            {
                                "begin_block": 1,
                                "begin_to_header_block": 2,
                                "begin_placement": "begin",
                                "header_block": 3,
                                "body_block": 4,
                                "exit_block": 5,
                                "update_block": 6,
                                "update_instruction_index": 7,
                                "update_placement": "update",
                                "end_block": 8,
                                "end_placement": "end",
                                "route_instruction_index": 9,
                                "array_value": 10,
                                "index_value": 11,
                                "source_value": 12,
                                "result_len_value": 13,
                                "middle_value": 14,
                                "middle_length": 15,
                                "skip_instruction_indices": [],
                                "scope": "selected",
                                "proof": "array_text_session_route",
                                "consumer_capability": "slot_text_len_store_session",
                                "publication_boundary": "none",
                                "carrier": "array_lane_text_cell",
                            }
                        ],
                    },
                }
            ],
            "typed_object_plans": [],
            "user_box_decls": [],
        }

        with tempfile.TemporaryDirectory() as tmpdir:
            mir_path = Path(tmpdir) / "mir.json"
            out_path = Path(tmpdir) / "state.kv"
            mir_path.write_text(json.dumps(mir), encoding="utf-8")
            old_argv = sys.argv[:]
            try:
                sys.argv = [
                    "state_explain.py",
                    "--mir-json",
                    str(mir_path),
                    "--out",
                    str(out_path),
                ]
                rc = state_explain.main()
            finally:
                sys.argv = old_argv

            self.assertEqual(rc, 0)
            text = out_path.read_text(encoding="utf-8")
            self.assertIn("array_text_state_residence_route_count=1", text)
            self.assertIn("array_text_selected_route_count=1", text)
            self.assertIn("array_text_selected_bridge_symbol_count=1", text)
            self.assertIn("array_text_compat_string_indexof_hisi_count=1", text)
            self.assertIn("array_text_session_count=1", text)
            self.assertIn("array_text_session_begin_count=1", text)
            self.assertIn("array_text_session_end_count=1", text)
            self.assertIn(
                "array_text_publication_in_selected_region_count=0",
                text,
            )
            self.assertIn(
                "array_text_registry_carrier_in_selected_region_count=0",
                text,
            )
            self.assertIn(
                "array_text_silent_fallback_after_selected_route_count=0",
                text,
            )
            self.assertIn(
                "array_text_state_residence_route_0_selected_route=hako.array_text.session_indexof_const_utf8",
                text,
            )
            self.assertIn(
                "array_text_state_residence_route_0_selected_bridge_symbol=hako.array_text.session_indexof_const_utf8",
                text,
            )
            self.assertIn(
                "array_text_residence_session_0_carrier=array_lane_text_cell",
                text,
            )

    def test_state_explain_emits_array_text_observer_route_rows(self) -> None:
        mir = {
            "functions": [
                {
                    "name": "array_text_indexof",
                    "metadata": {
                        "array_text_observer_routes": [
                            {
                                "observer_kind": "indexof",
                                "consumer_shape": "found_predicate",
                                "proof_region": "array_get_receiver_indexof",
                                "publication_boundary": "none",
                                "selected_route": "hako.array_text.session_indexof_const_utf8",
                                "selected_bridge_symbol": "hako.array_text.session_indexof_const_utf8",
                                "fallback_route": "nyash.array.string_indexof_hisi",
                                "fallback_policy": "fail_fast",
                                "executor_contract": {
                                    "publication_boundary": "none",
                                    "proof_region": "observe.indexof",
                                    "carrier": "array_lane_text_cell",
                                },
                            }
                        ]
                    },
                }
            ],
            "typed_object_plans": [],
            "user_box_decls": [],
        }

        with tempfile.TemporaryDirectory() as tmpdir:
            mir_path = Path(tmpdir) / "mir.json"
            out_path = Path(tmpdir) / "state.kv"
            mir_path.write_text(json.dumps(mir), encoding="utf-8")
            old_argv = sys.argv[:]
            try:
                sys.argv = [
                    "state_explain.py",
                    "--mir-json",
                    str(mir_path),
                    "--out",
                    str(out_path),
                ]
                rc = state_explain.main()
            finally:
                sys.argv = old_argv

            self.assertEqual(rc, 0)
            text = out_path.read_text(encoding="utf-8")
            self.assertIn("array_text_observer_route_count=1", text)
            self.assertIn("array_text_observer_indexof_count=1", text)
            self.assertIn("array_text_observer_selected_route_count=1", text)
            self.assertIn("array_text_observer_selected_bridge_symbol_count=1", text)
            self.assertIn("array_text_observer_found_predicate_count=1", text)
            self.assertIn(
                "array_text_observer_publication_in_selected_region_count=0",
                text,
            )
            self.assertIn(
                "array_text_observer_registry_carrier_in_selected_region_count=0",
                text,
            )
            self.assertIn("array_text_observer_publication_none_count=1", text)
            self.assertIn("array_text_observer_executor_contract_count=1", text)
            self.assertIn(
                "array_text_observer_route_0_observer_kind=indexof",
                text,
            )
            self.assertIn(
                "array_text_observer_route_0_consumer_shape=found_predicate",
                text,
            )
            self.assertIn(
                "array_text_observer_route_0_publication_boundary=none",
                text,
            )
            self.assertIn(
                "array_text_observer_route_0_selected_route=hako.array_text.session_indexof_const_utf8",
                text,
            )
            self.assertIn(
                "array_text_observer_route_0_selected_bridge_symbol=hako.array_text.session_indexof_const_utf8",
                text,
            )
            self.assertIn(
                "array_text_observer_route_0_fallback_route=nyash.array.string_indexof_hisi",
                text,
            )
            self.assertIn(
                "array_text_observer_route_0_fallback_policy=fail_fast",
                text,
            )
            self.assertIn(
                "array_text_observer_route_0_executor_contract_publication_boundary=none",
                text,
            )


if __name__ == "__main__":
    unittest.main()
