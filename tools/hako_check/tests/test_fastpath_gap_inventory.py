from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
TOOL = ROOT / "tools" / "hako_check" / "fastpath_gap_inventory.py"


class FastPathGapInventoryTest(unittest.TestCase):
    def test_reports_known_receiver_routes_without_local_fastpath_fact(self) -> None:
        payload = {
            "functions": [
                {
                    "name": "Main.runOne/2",
                    "metadata": {
                        "user_box_method_routes": [
                            {
                                "block": 10,
                                "instruction_index": 2,
                                "route_kind": "user_box.method",
                                "emit_kind": "direct_function_call",
                                "proof": "typed_user_box_method_same_module",
                                "target_exists": True,
                                "target_body_supported": True,
                                "symbol": "Page.acquire_usize/1",
                            },
                            {
                                "block": 11,
                                "instruction_index": 4,
                                "route_kind": "user_box.method",
                                "emit_kind": "runtime_call",
                                "proof": "typed_user_box_method_same_module",
                                "target_exists": True,
                                "target_body_supported": True,
                                "symbol": "Page.dynamic/0",
                            },
                        ],
                        "thin_entry_selections": [
                            {
                                "surface": "user_box_method",
                                "state": "candidate",
                                "selected_entry": "thin_internal_entry",
                            }
                        ],
                        "local_fastpath_facts": [],
                    },
                }
            ]
        }

        rows = self._run_tool(payload, "Main.runOne/2")

        self.assertEqual(rows["output_contract"], "hako-fastpath-gap-inventory-v0")
        self.assertEqual(rows["known_receiver_direct_method_route_count"], "1")
        self.assertEqual(rows["local_fastpath_fact_count"], "0")
        self.assertEqual(rows["known_receiver_direct_method_without_fact_count"], "1")
        self.assertEqual(rows["thin_entry_method_candidate_count"], "1")
        self.assertEqual(rows["function_0_top_missing_subject"], "Page.acquire_usize/1")
        self.assertEqual(rows["fallback_evidence_fact_enabled"], "0")
        self.assertEqual(rows["backend_lowering_changed"], "0")
        self.assertEqual(rows["winner_claim_allowed"], "0")

    def test_matching_fact_closes_the_gap_for_that_site(self) -> None:
        payload = {
            "functions": [
                {
                    "name": "Main.runOne/2",
                    "metadata": {
                        "user_box_method_routes": [
                            {
                                "block": 10,
                                "instruction_index": 2,
                                "route_kind": "user_box.method",
                                "emit_kind": "direct_function_call",
                                "proof": "typed_user_box_method_same_module",
                                "target_exists": True,
                                "target_body_supported": True,
                                "symbol": "Page.acquire_usize/1",
                            }
                        ],
                        "local_fastpath_facts": [
                            {
                                "block": 10,
                                "instruction_index": 2,
                                "backend_kind": "known_receiver_direct_call",
                            }
                        ],
                    },
                }
            ]
        }

        rows = self._run_tool(payload, "Main.runOne/2")

        self.assertEqual(rows["known_receiver_direct_method_route_count"], "1")
        self.assertEqual(rows["local_fastpath_fact_count"], "1")
        self.assertEqual(rows["known_receiver_direct_method_without_fact_count"], "0")
        self.assertEqual(rows["top_gap_function"], "Main.runOne/2")
        self.assertEqual(rows["top_gap_count"], "0")

    def _run_tool(self, payload: dict, method: str) -> dict[str, str]:
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "mir.json"
            path.write_text(json.dumps(payload), encoding="utf-8")
            result = subprocess.run(
                [
                    sys.executable,
                    str(TOOL),
                    "--mir-json",
                    str(path),
                    "--method",
                    method,
                    "--front",
                    "unit",
                ],
                check=True,
                capture_output=True,
                text=True,
            )
        return dict(
            line.split("=", 1)
            for line in result.stdout.splitlines()
            if "=" in line
        )


if __name__ == "__main__":
    unittest.main()
