#!/usr/bin/env python3
"""Unit coverage for the FACT0-P1-G0 semantic partition freeze."""

from __future__ import annotations

import copy
import json
import unittest
from pathlib import Path

from mirbuilder_type_fact_partition_guard import (
    validate_active_cutover_writer_inventory_v1,
    validate_const0_authority_v1,
    validate_fastmem_receipt0_authority_v1,
    validate_p1_g0_profile_freeze_v1,
)


ROOT = Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "tools/checks/fixtures/mirbuilder_type_fact_producer_matrix_v1.json"


def fixture_copy() -> dict[str, object]:
    return json.loads(FIXTURE.read_text(encoding="utf-8"))


def partition(data: dict[str, object], source_file: str) -> dict[str, object]:
    partitions = data["writer_partitions"]
    assert isinstance(partitions, list)
    return next(row for row in partitions if row["source_file"] == source_file)


class PartitionGuardTests(unittest.TestCase):
    def test_live_fixture_passes(self) -> None:
        validate_p1_g0_profile_freeze_v1(fixture_copy())

    def test_live_active_cutover_and_const0_authority_pass(self) -> None:
        validate_active_cutover_writer_inventory_v1(ROOT, fixture_copy())
        validate_const0_authority_v1(ROOT)
        validate_fastmem_receipt0_authority_v1(ROOT)

    def test_profile_prerequisite_drift_rejects(self) -> None:
        data = fixture_copy()
        profiles = data["partition_profiles"]
        assert isinstance(profiles, dict)
        profiles["copy_exact"]["retirement_prerequisite"] = "PHI0-CUT0"
        with self.assertRaises(SystemExit):
            validate_p1_g0_profile_freeze_v1(data)

    def test_schema_valid_slice_rebinding_rejects(self) -> None:
        data = fixture_copy()
        literal = partition(data, "src/mir/builder/builder_build.rs")
        compare = partition(data, "src/mir/builder/emission/compare.rs")
        literal["slices"][0]["producer_profiles"] = ["simple_exact"]
        compare["slices"][0]["producer_profiles"] = ["literal_postemit_exact"]
        with self.assertRaises(SystemExit):
            validate_p1_g0_profile_freeze_v1(data)

    def test_evidence_prose_is_not_freeze_authority(self) -> None:
        data = fixture_copy()
        profiles = data["partition_profiles"]
        assert isinstance(profiles, dict)
        profiles["copy_exact"]["evidence_owner"] = "clarified test prose"
        validate_p1_g0_profile_freeze_v1(data)

    def test_partition_order_is_not_freeze_authority(self) -> None:
        data = fixture_copy()
        partitions = data["writer_partitions"]
        assert isinstance(partitions, list)
        data["writer_partitions"] = list(reversed(copy.deepcopy(partitions)))
        validate_p1_g0_profile_freeze_v1(data)


if __name__ == "__main__":
    unittest.main()
