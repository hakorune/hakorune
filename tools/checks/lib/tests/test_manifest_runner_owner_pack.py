#!/usr/bin/env python3
"""Focused tests for the smoke-owner-pack argv contract."""

from __future__ import annotations

import copy
import sys
import tomllib
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from manifest_runner import validate_row_kind_contract


BASE_ENTRY = {
    "id": "owner-pack",
    "row_kind": "smoke-owner-pack",
    "cmd": [
        "bash",
        "tools/smokes/v2/run.sh",
        "--profile",
        "quick",
        "--owner-profile",
        "integration",
        "--suite",
        "phase2050-owner-pack",
        "--dry-run",
        "--skip-preflight",
    ],
}

ROOT = Path(__file__).resolve().parents[4]


class ManifestRunnerOwnerPackTest(unittest.TestCase):
    def test_checked_in_registry_row_matches_contract(self) -> None:
        with (ROOT / "tools/checks/guard_rows.toml").open("rb") as handle:
            rows = tomllib.load(handle)["rows"]
        entry = next(row for row in rows if row.get("id") == "smoke-owner-pack-phase2050")
        self.assertEqual(entry.get("row_kind"), BASE_ENTRY["row_kind"])
        self.assertEqual(entry.get("cmd"), BASE_ENTRY["cmd"])
        validate_row_kind_contract(entry, "test")

    def test_exact_runner_contract_is_accepted(self) -> None:
        validate_row_kind_contract(copy.deepcopy(BASE_ENTRY), "test")

    def test_missing_suite_is_rejected(self) -> None:
        entry = copy.deepcopy(BASE_ENTRY)
        suite_index = entry["cmd"].index("--suite")
        del entry["cmd"][suite_index : suite_index + 2]
        with self.assertRaises(SystemExit):
            validate_row_kind_contract(entry, "test")

    def test_filter_is_rejected(self) -> None:
        entry = copy.deepcopy(BASE_ENTRY)
        entry["cmd"].extend(["--filter", "phase2050"])
        with self.assertRaises(SystemExit):
            validate_row_kind_contract(entry, "test")

    def test_non_runner_command_is_rejected(self) -> None:
        entry = copy.deepcopy(BASE_ENTRY)
        entry["cmd"][1] = "tools/smokes/v2/profiles/integration/core/phase2050/run_all.sh"
        with self.assertRaises(SystemExit):
            validate_row_kind_contract(entry, "test")


if __name__ == "__main__":
    unittest.main()
