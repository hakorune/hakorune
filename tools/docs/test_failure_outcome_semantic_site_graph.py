#!/usr/bin/env python3
"""Focused contract tests for the Failure/Outcome site graph guard."""

from __future__ import annotations

import unittest
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from failure_outcome_semantic_site_graph import build_graph, validate, validate_site_id


def base_manifest(*sites: dict, current: int = 0, previous: int = 0) -> dict:
    return {
        "schema_version": 0,
        "semantic_activation": 0,
        "pending_counts": {"missing_argument_zero": current},
        "pending_baseline_counts": {"missing_argument_zero": previous},
        "previous_pending_counts": {"missing_argument_zero": previous},
        "evidence_occurrences": [
            {
                "evidence_id": "e1",
                "source_path": "src/example.rs",
                "line": 1,
                "token": "VMValue::Void",
                "evidence_kind": "runtime_value_carrier",
                "evidence": "VMValue::Void",
            }
        ],
        "semantic_sites": list(sites),
    }


def site(site_id: str, **overrides: object) -> dict:
    value = {
        "site_id": site_id,
        "site_kind": "operation_outcome",
        "layer": "runtime_backend",
        "owner_domain": "carrier",
        "operation": "vmvalue_void",
        "outcome_branch": "current_carrier_observation",
        "semantic_class": "",
        "target_carrier": "",
        "owner": "",
        "profile": "implementation",
        "migration_action": "",
        "backend_policy": "",
        "current_carrier": "VMValue::Void",
        "evidence_refs": ["e1"],
        "review_status": "pending",
    }
    value.update(overrides)
    return value


class SemanticSiteGraphGuardTest(unittest.TestCase):
    def test_generated_graph_is_guard_clean(self) -> None:
        self.assertEqual(validate(build_graph()), [])

    def test_site_id_requires_four_known_segments(self) -> None:
        self.assertIsNone(
            validate_site_id("runtime_backend.extern.env_get.provider_missing")
        )
        self.assertIn("exactly four", validate_site_id("runtime_backend.extern.env_get"))
        self.assertIn("unknown site layer", validate_site_id("runtime.extern.env_get.provider_missing"))

    def test_compatibility_site_requires_profile(self) -> None:
        errors = validate(
            base_manifest(
                site(
                    "runtime_backend.compatibility.vmvalue_void.equality_boxing",
                    semantic_class="compatibility_only",
                    profile="",
                )
            )
        )
        self.assertIn("compatibility_only requires profile", " ".join(errors))

    def test_pending_count_cannot_increase(self) -> None:
        errors = validate(
            base_manifest(
                site("runtime_backend.carrier.vmvalue_void.current_carrier_observation"),
                current=3,
                previous=2,
            )
        )
        self.assertIn("pending count increased", " ".join(errors))


if __name__ == "__main__":
    unittest.main()
