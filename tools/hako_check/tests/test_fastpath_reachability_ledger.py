from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
LEDGER = ROOT / "tools" / "hako_check" / "fastpath_reachability_ledger.py"


class FastPathReachabilityLedgerTest(unittest.TestCase):
    def test_reports_exact_seed_preempting_string_dead_text_consumer(self) -> None:
        payload = {
            "functions": [
                {
                    "name": "main",
                    "metadata": {
                        "exact_seed_backend_route": {
                            "tag": "substring_concat_loop_ascii",
                            "source_route": "string_kernel_plans.loop_payload",
                            "proof": "string_kernel_plan_concat_triplet_loop_payload",
                            "selected_value": 35,
                        },
                        "string_dead_text_region_plans": [
                            {
                                "route_id": "string.dead_text_region.plan",
                                "loop_header": 18,
                            }
                        ],
                    },
                }
            ]
        }

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "mir.json"
            path.write_text(json.dumps(payload), encoding="utf-8")
            result = subprocess.run(
                [
                    sys.executable,
                    str(LEDGER),
                    "--mir-json",
                    str(path),
                    "--front",
                    "kilo_micro_substring_concat",
                ],
                check=True,
                capture_output=True,
                text=True,
            )

        rows = dict(
            line.split("=", 1)
            for line in result.stdout.splitlines()
            if "=" in line
        )
        self.assertEqual(rows["output_contract"], "hako-fastpath-reachability-ledger-v1")
        self.assertEqual(rows["route_priority_table_version"], "v0")
        self.assertEqual(rows["selected_route"], "substring_concat_loop_ascii")
        self.assertEqual(rows["selected_route_owner"], "function_level_exact_seed")
        self.assertEqual(rows["selected_route_priority"], "10")
        self.assertEqual(rows["selected_route_priority_source"], "route_priority_table_v0")
        self.assertEqual(rows["new_consumer_exists"], "1")
        self.assertEqual(rows["new_consumer_reachable"], "0")
        self.assertEqual(rows["old_exact_seed_selected"], "1")
        self.assertEqual(rows["preemption_detected"], "1")
        self.assertEqual(rows["winner_claim_allowed"], "0")
        self.assertEqual(rows["candidate_1_family"], "string_dead_text_region")
        self.assertEqual(rows["candidate_1_preempted_by"], "substring_concat_loop_ascii")
        self.assertEqual(rows["candidate_1_preempted_reason"], "lower_priority_selected_route")

    def test_unselected_candidate_is_not_reachable(self) -> None:
        payload = {
            "functions": [
                {
                    "name": "main",
                    "metadata": {
                        "string_dead_text_region_plans": [
                            {
                                "route_id": "string.dead_text_region.plan",
                                "loop_header": 18,
                            }
                        ],
                    },
                }
            ]
        }

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "mir.json"
            path.write_text(json.dumps(payload), encoding="utf-8")
            result = subprocess.run(
                [
                    sys.executable,
                    str(LEDGER),
                    "--mir-json",
                    str(path),
                    "--front",
                    "candidate_only",
                ],
                check=True,
                capture_output=True,
                text=True,
            )

        rows = dict(
            line.split("=", 1)
            for line in result.stdout.splitlines()
            if "=" in line
        )
        self.assertEqual(rows["candidate_count"], "1")
        self.assertEqual(rows["output_contract"], "hako-fastpath-reachability-ledger-v1")
        self.assertEqual(rows["route_priority_table_version"], "v0")
        self.assertEqual(rows["selected_route"], "none")
        self.assertEqual(rows["selected_route_priority_source"], "none")
        self.assertEqual(rows["new_consumer_exists"], "1")
        self.assertEqual(rows["new_consumer_reachable"], "0")
        self.assertEqual(rows["preemption_detected"], "0")
        self.assertEqual(rows["winner_claim_allowed"], "0")
        self.assertEqual(rows["candidate_0_selected"], "0")
        self.assertEqual(rows["candidate_0_reachable"], "0")


if __name__ == "__main__":
    unittest.main()
