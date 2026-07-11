#!/usr/bin/env python3
"""Focused contract tests for the S5 conflict ledger."""

from __future__ import annotations

import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from failure_outcome_conflict_ledger import CONFLICTS, build_manifest, validate


class ConflictLedgerTest(unittest.TestCase):
    def test_all_eight_conflicts_are_pending(self) -> None:
        manifest = build_manifest()
        self.assertEqual(validate(manifest), [])
        self.assertEqual(len(manifest["conflicts"]), 8)
        self.assertEqual([row["conflict_id"] for row in manifest["conflicts"]], [row[0] for row in CONFLICTS])
        self.assertTrue(all(row["status"] == "pending_consultation" for row in manifest["conflicts"]))

    def test_missing_evidence_rejected(self) -> None:
        broken = copy.deepcopy(build_manifest())
        broken["conflicts"][0]["evidence_refs"] = []
        self.assertIn("conflict evidence missing", " ".join(validate(broken)))

    def test_resolved_owner_claim_rejected(self) -> None:
        broken = copy.deepcopy(build_manifest())
        broken["conflicts"][0]["current_observation"] = "owner resolved"
        self.assertIn("claims resolved owner", " ".join(validate(broken)))

    def test_activation_rejected(self) -> None:
        broken = copy.deepcopy(build_manifest())
        broken["semantic_activation"] = 1
        self.assertIn("semantic activation", " ".join(validate(broken)))


if __name__ == "__main__":
    unittest.main()
