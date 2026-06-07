#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

_REPO_ROOT = Path(__file__).resolve().parents[3]
_HAKO_CHECK_ROOT = _REPO_ROOT / "tools" / "hako_check"
for _path in (str(_REPO_ROOT), str(_HAKO_CHECK_ROOT)):
    if _path not in sys.path:
        sys.path.insert(0, _path)

from fastmem_mir_to_llvm_producer_report_route_rows import build_route_state


def _route_state(
    profile: str,
    *,
    local_free_push_count: int = 0,
    local_free_pop_count: int = 0,
    free_head_push_count: int = 0,
    free_head_pop_count: int = 0,
) -> dict[str, object]:
    return {
        "profile": profile,
        "deferred_local_free_kinds": [],
        "selected_local_free_kinds": [],
        "verified_free_head_pop": [None] * free_head_pop_count,
        "verified_free_head_push": [None] * free_head_push_count,
        "verified_local_free_pop": [None] * local_free_pop_count,
        "verified_local_free_push": [None] * local_free_push_count,
    }


class TestFastMemDrainExchangeRouteState(unittest.TestCase):
    def test_exchange_selection_pins_exchange_order_and_result_kind(self):
        rows = build_route_state(_route_state("remote-free-drain-exchange-selection"))

        self.assertTrue(rows["remote_free_drain_exchange_selection"])
        self.assertFalse(rows["remote_free_drain_exchange_producer"])

    def test_exchange_producer_advances_to_to_local_selection(self):
        rows = build_route_state(_route_state("remote-free-drain-exchange"))

        self.assertFalse(rows["remote_free_drain_exchange_selection"])
        self.assertTrue(rows["remote_free_drain_exchange_producer"])

    def test_route_state_exposes_free_route_candidate_default(self):
        owner_runtime_rows = build_route_state(_route_state("owner-runtime"))
        layout_table_rows = build_route_state(_route_state("layout_table"))

        self.assertIn("free_route_candidate", owner_runtime_rows)
        self.assertEqual(owner_runtime_rows["free_route_candidate"], "none")
        self.assertIn("free_route_candidate", layout_table_rows)
        self.assertEqual(layout_table_rows["free_route_candidate"], "none")

    def test_local_free_route_state_exposes_alloc_candidate(self):
        rows = build_route_state(
            _route_state(
                "local-free",
                local_free_pop_count=1,
            )
        )

        self.assertEqual(rows["route_candidate"], "local_free_alloc")
        self.assertEqual(rows["free_route_candidate"], "none")

    def test_local_free_route_state_exposes_free_route_candidate(self):
        rows = build_route_state(
            _route_state(
                "local-free",
                local_free_push_count=1,
            )
        )

        self.assertEqual(rows["route_candidate"], "none")
        self.assertEqual(rows["free_route_candidate"], "same_owner_local_free")


if __name__ == "__main__":
    unittest.main()
