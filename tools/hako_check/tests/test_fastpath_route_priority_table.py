from __future__ import annotations

import subprocess
import sys
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
TOOL = ROOT / "tools" / "hako_check" / "fastpath_route_priority_table.py"


class FastPathRoutePriorityTableTest(unittest.TestCase):
    def test_route_priority_table_contract(self) -> None:
        result = subprocess.run(
            [sys.executable, str(TOOL)],
            check=True,
            capture_output=True,
            text=True,
        )
        rows = dict(
            line.split("=", 1)
            for line in result.stdout.splitlines()
            if "=" in line
        )

        self.assertEqual(
            rows["output_contract"],
            "hako-fastpath-route-priority-table-v0",
        )
        self.assertEqual(rows["route_priority_table_version"], "v0")
        self.assertEqual(rows["entry_count"], "4")
        self.assertEqual(rows["priority_unique"], "1")
        self.assertEqual(rows["lowest_priority_wins"], "1")
        self.assertEqual(rows["route_priority_changes_backend_lowering"], "0")
        self.assertEqual(rows["route_priority_retires_exact_seed"], "0")
        self.assertEqual(rows["entry_0_family"], "exact_seed")
        self.assertEqual(rows["entry_0_priority"], "10")
        self.assertEqual(rows["entry_1_family"], "local_fastpath_fact")
        self.assertEqual(rows["entry_1_priority"], "20")
        self.assertEqual(rows["entry_2_family"], "string_dead_text_region")
        self.assertEqual(rows["entry_2_priority"], "30")
        self.assertEqual(rows["entry_3_family"], "runtime_helper_fallback")
        self.assertEqual(rows["entry_3_priority"], "90")


if __name__ == "__main__":
    unittest.main()
