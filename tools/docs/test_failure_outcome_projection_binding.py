#!/usr/bin/env python3
"""Focused contract tests for the S2 projection binding inventory."""

from __future__ import annotations

import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from failure_outcome_projection_binding import build_manifest, validate


class ProjectionBindingTest(unittest.TestCase):
    def test_generated_manifest_is_guard_clean(self) -> None:
        manifest = build_manifest()
        self.assertEqual(validate(manifest), [])

    def test_hako_mem_free_is_the_only_bound_corridor(self) -> None:
        manifest = build_manifest()
        self.assertEqual(len(manifest["projection_bindings"]), 1)
        binding = manifest["projection_bindings"][0]
        self.assertEqual(
            binding["projects_site"], "runtime_backend.extern.hako_mem_free.success"
        )
        self.assertEqual(binding["encoding"], "VoidSentinelI64Zero")
        self.assertNotIn("semantic_class", binding)
        self.assertNotIn("target_carrier", binding)

    def test_provider_fallbacks_remain_pending(self) -> None:
        manifest = build_manifest()
        observations = manifest["boundary_observations"]
        self.assertEqual(len(observations), 6)
        self.assertTrue(all(row["resolution"] == "Pending" for row in observations))
        self.assertTrue(
            all(row["pending_reason"] == "ProviderContractMissing" for row in observations)
        )

    def test_pending_projection_cannot_become_binding_source(self) -> None:
        manifest = build_manifest()
        broken = copy.deepcopy(manifest)
        broken["projection_bindings"][0]["projects_site"] = (
            "runtime_backend.constant.constant_bridge.source_observation"
        )
        self.assertTrue(
            any("projection source is not classified" in error for error in validate(broken))
        )


if __name__ == "__main__":
    unittest.main()
