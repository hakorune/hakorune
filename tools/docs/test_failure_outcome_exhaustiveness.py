#!/usr/bin/env python3
"""Focused mutation tests for the S4 exhaustiveness checker."""

from __future__ import annotations

import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from failure_outcome_exhaustiveness import (
    BINDING,
    CONTROL,
    GRAPH,
    RUNTIME,
    read,
    validate,
)


def fixtures() -> tuple[dict, dict, dict, dict]:
    return read(GRAPH), read(BINDING), read(RUNTIME), read(CONTROL)


class ExhaustivenessTest(unittest.TestCase):
    def test_clean_manifests_pass(self) -> None:
        self.assertEqual(validate(*fixtures()), [])

    def test_duplicate_site_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        graph["semantic_sites"].append(copy.deepcopy(graph["semantic_sites"][0]))
        self.assertIn("duplicate semantic site id", validate(graph, binding, runtime, control))

    def test_invalid_site_id_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        graph["semantic_sites"][0]["site_id"] = "handwritten.site"
        self.assertTrue(any("four segments" in error for error in validate(graph, binding, runtime, control)))

    def test_unknown_class_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        graph["semantic_sites"][0]["semantic_class"] = "guess"
        self.assertIn("unknown semantic class", " ".join(validate(graph, binding, runtime, control)))

    def test_classified_incomplete_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        site = graph["semantic_sites"][0]
        site["review_status"] = "classified"
        site["owner"] = ""
        self.assertIn("classified site is incomplete", " ".join(validate(graph, binding, runtime, control)))

    def test_compatibility_profile_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        site = graph["semantic_sites"][0]
        site["semantic_class"] = "compatibility_only"
        site["profile"] = ""
        self.assertIn("requires profile", " ".join(validate(graph, binding, runtime, control)))

    def test_foreign_null_policy_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        site = graph["semantic_sites"][0]
        site["semantic_class"] = "foreign_null"
        site["backend_policy"] = ""
        self.assertIn("foreign_null policy missing", " ".join(validate(graph, binding, runtime, control)))

    def test_unit_absence_conflation_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        site = graph["semantic_sites"][0]
        site["semantic_class"] = "optional_absence"
        site["target_carrier"] = "Unit"
        self.assertIn("Unit/absence conflation", " ".join(validate(graph, binding, runtime, control)))

    def test_projection_chain_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        source = copy.deepcopy(graph["semantic_sites"][0])
        source["site_id"] = "reference.option.option_value.unit_projection"
        source["site_kind"] = "boundary_projection"
        source["projects_site"] = graph["semantic_sites"][0]["site_id"]
        graph["semantic_sites"].append(source)
        chained = copy.deepcopy(source)
        chained["site_id"] = "reference.option.option_value.backend_zero_null_projection"
        chained["projects_site"] = source["site_id"]
        graph["semantic_sites"].append(chained)
        self.assertIn("projection chain forbidden", " ".join(validate(graph, binding, runtime, control)))

    def test_missing_argument_zero_increase_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        graph["pending_counts"]["missing_argument_zero"] = 1
        self.assertIn("increased from baseline", " ".join(validate(graph, binding, runtime, control)))

    def test_activation_rejected(self) -> None:
        graph, binding, runtime, control = fixtures()
        runtime["semantic_activation"] = 1
        self.assertIn("semantic activation", " ".join(validate(graph, binding, runtime, control)))


if __name__ == "__main__":
    unittest.main()
