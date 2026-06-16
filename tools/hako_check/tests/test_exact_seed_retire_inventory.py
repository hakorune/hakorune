from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
INVENTORY = ROOT / "tools" / "hako_check" / "exact_seed_retire_inventory.py"


def run_inventory(payload: dict) -> dict[str, str]:
    with tempfile.TemporaryDirectory() as tmp:
        path = Path(tmp) / "mir.json"
        path.write_text(json.dumps(payload), encoding="utf-8")
        result = subprocess.run(
            [
                sys.executable,
                str(INVENTORY),
                "--mir-json",
                str(path),
                "--front",
                "synthetic_front",
            ],
            check=True,
            capture_output=True,
            text=True,
        )
    return dict(line.split("=", 1) for line in result.stdout.splitlines() if "=" in line)


class ExactSeedRetireInventoryTest(unittest.TestCase):
    def test_exact_seed_without_replacement_is_not_retirable(self) -> None:
        rows = run_inventory(
            {
                "functions": [
                    {
                        "name": "main",
                        "metadata": {
                            "exact_seed_backend_route": {
                                "tag": "substring_concat_loop_ascii",
                                "source_route": "string_kernel_plans.loop_payload",
                                "proof": "string_kernel_plan_concat_triplet_loop_payload",
                                "selected_value": 35,
                            }
                        },
                    }
                ]
            }
        )

        self.assertEqual(rows["output_contract"], "hako-exact-seed-retire-inventory-v0")
        self.assertEqual(rows["route_priority_table_version"], "v0")
        self.assertEqual(rows["exact_seed_present"], "1")
        self.assertEqual(rows["exact_seed_tag"], "substring_concat_loop_ascii")
        self.assertEqual(rows["replacement_candidate_exists"], "0")
        self.assertEqual(rows["replacement_reachable"], "0")
        self.assertEqual(rows["retire_allowed"], "0")
        self.assertEqual(rows["retire_blocker"], "no_replacement_candidate")
        self.assertEqual(rows["exact_seed_retired"], "0")
        self.assertEqual(rows["winner_claim_allowed"], "0")

    def test_preempted_replacement_is_not_retirable(self) -> None:
        rows = run_inventory(
            {
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
        )

        self.assertEqual(rows["replacement_family"], "string_dead_text_region")
        self.assertEqual(rows["replacement_candidate_exists"], "1")
        self.assertEqual(rows["replacement_reachable"], "0")
        self.assertEqual(rows["preemption_detected"], "1")
        self.assertEqual(rows["retire_allowed"], "0")
        self.assertEqual(rows["retire_blocker"], "replacement_not_reachable")
        self.assertEqual(rows["drive_by_retire_allowed"], "0")


if __name__ == "__main__":
    unittest.main()
