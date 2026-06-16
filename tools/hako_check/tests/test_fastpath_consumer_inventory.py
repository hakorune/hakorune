from __future__ import annotations

import subprocess
import sys
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
INVENTORY = ROOT / "tools" / "hako_check" / "fastpath_consumer_inventory.py"


class FastPathConsumerInventoryTest(unittest.TestCase):
    def test_reports_known_consumer_families(self) -> None:
        result = subprocess.run(
            [sys.executable, str(INVENTORY)],
            check=True,
            capture_output=True,
            text=True,
        )
        rows = dict(line.split("=", 1) for line in result.stdout.splitlines() if "=" in line)

        self.assertEqual(rows["output_contract"], "hako-fastpath-consumer-inventory-v0")
        self.assertEqual(rows["consumer_count"], "5")
        self.assertEqual(rows["backend_consumer_code_is_not_reachability"], "1")
        self.assertEqual(rows["winner_claim_requires_reachable_consumer"], "1")
        self.assertEqual(rows["unknown_consumer_winner_claim_allowed"], "0")
        self.assertEqual(rows["consumer_0_family"], "exact_seed")
        self.assertEqual(rows["consumer_1_family"], "local_fastpath_fact")
        self.assertEqual(rows["consumer_2_family"], "local_i64_map_entry_table")
        self.assertEqual(rows["consumer_3_family"], "string_dead_text_region")
        self.assertEqual(
            rows["consumer_3_status"],
            "backend_consumer_exists_reachability_blocked",
        )
        self.assertEqual(rows["consumer_4_family"], "runtime_helper_fallback")
        self.assertEqual(rows["consumer_4_status"], "fallback_not_fastpath")


if __name__ == "__main__":
    unittest.main()
