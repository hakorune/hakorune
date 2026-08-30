#!/usr/bin/env python3
"""Independent live-owner coverage for the five rehomed FACT0 checks."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "tools/checks/lib"))

from mirbuilder_type_fact_call_post_success_guard import (  # noqa: E402
    validate_call_receipt0_authority_v1,
    validate_const0_authority_v1,
    validate_literal_postemit_retirement_v1,
    validate_map_write_observe0_authority_v1,
    validate_resolved_direct_call_authority_v1,
)


class CallPostSuccessGuardTests(unittest.TestCase):
    def test_const0_live_owner(self) -> None:
        validate_const0_authority_v1(ROOT)

    def test_literal_postemit_live_owner(self) -> None:
        validate_literal_postemit_retirement_v1(ROOT)

    def test_resolved_direct_call_production_owner(self) -> None:
        validate_resolved_direct_call_authority_v1(ROOT)

    def test_call_receipt_live_owner(self) -> None:
        validate_call_receipt0_authority_v1(ROOT)

    def test_map_write_live_owner(self) -> None:
        validate_map_write_observe0_authority_v1(ROOT)


if __name__ == "__main__":
    unittest.main()
