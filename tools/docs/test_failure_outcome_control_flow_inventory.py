#!/usr/bin/env python3
"""Focused contract tests for the S3 control-flow inventory."""

from __future__ import annotations

import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from failure_outcome_control_flow_inventory import FAMILIES, build_manifest, validate


class ControlFlowInventoryTest(unittest.TestCase):
    def test_generated_manifest_has_all_pending_families(self) -> None:
        manifest = build_manifest()
        self.assertEqual(validate(manifest), [])
        self.assertEqual({row["family"] for row in manifest["control_flow_evidence"]}, set(FAMILIES))
        self.assertTrue(all(row["resolution"] == "Pending" for row in manifest["control_flow_evidence"]))

    def test_pending_rows_cannot_claim_site(self) -> None:
        broken = copy.deepcopy(build_manifest())
        broken["control_flow_evidence"][0]["site_ref"] = "runtime_backend.example.site"
        self.assertTrue(any("has site reference" in error for error in validate(broken)))

    def test_activation_stays_zero(self) -> None:
        broken = copy.deepcopy(build_manifest())
        broken["semantic_activation"] = 1
        self.assertIn("semantic activation must remain 0", validate(broken))


if __name__ == "__main__":
    unittest.main()
