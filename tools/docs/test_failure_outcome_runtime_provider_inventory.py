#!/usr/bin/env python3
"""Focused contract tests for the S2 runtime/provider inventory."""

from __future__ import annotations

import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from failure_outcome_runtime_provider_inventory import FAMILIES, build_manifest, validate


class RuntimeProviderInventoryTest(unittest.TestCase):
    def test_generated_manifest_is_guard_clean(self) -> None:
        manifest = build_manifest()
        self.assertEqual(validate(manifest), [])
        self.assertEqual({row["family"] for row in manifest["runtime_provider_evidence"]}, set(FAMILIES))

    def test_provider_missing_is_pending(self) -> None:
        rows = [
            row
            for row in build_manifest()["runtime_provider_evidence"]
            if row["family"] == "provider_status"
        ]
        self.assertTrue(rows)
        self.assertTrue(all(row["resolution"] == "Pending" for row in rows))
        self.assertTrue(all(row["pending_reason"] == "ProviderContractMissing" for row in rows))

    def test_pending_evidence_cannot_claim_site(self) -> None:
        broken = copy.deepcopy(build_manifest())
        row = next(row for row in broken["runtime_provider_evidence"] if row["resolution"] == "Pending")
        row["site_ref"] = "runtime_backend.extern.hako_mem_free.success"
        self.assertTrue(any("pending evidence has semantic site reference" in error for error in validate(broken)))

    def test_activation_stays_zero(self) -> None:
        broken = copy.deepcopy(build_manifest())
        broken["semantic_activation"] = 1
        self.assertIn("semantic activation must remain 0", validate(broken))


if __name__ == "__main__":
    unittest.main()
